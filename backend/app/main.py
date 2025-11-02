from __future__ import annotations

from pathlib import Path
from typing import Any, Dict
import sys
import os

# 确保从任意目录直接运行时可导入 backend 包
_PROJECT_ROOT = Path(__file__).resolve().parents[2]
if str(_PROJECT_ROOT) not in sys.path:
    sys.path.insert(0, str(_PROJECT_ROOT))

import uvicorn
from fastapi import FastAPI, WebSocket, WebSocketDisconnect, HTTPException, Body, Request
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse
from fastapi.staticfiles import StaticFiles
from pydantic import BaseModel

from backend.app.data.world_map import load_world_graph
from backend.app.models.map import Graph

HOURS_PER_DAY = 24
DEFAULT_MOVE_SPEED = 1.0
DEFAULT_TIME_COEFFICIENT = 1.0


class TimeStatus(BaseModel):
    day: int
    hour: float
    total_hours: float

def create_app() -> FastAPI:
    app = FastAPI(title="Drifting Backend", version="0.1.0")

    # 为 Vite 开发服务器启用 CORS 支持
    app.add_middleware(
        CORSMiddleware,
        allow_origins=[
            "http://localhost:5173",
            "http://127.0.0.1:5173",
        ],
        allow_credentials=True,
        allow_methods=["*"],
        allow_headers=["*"],
    )

    @app.get("/api/health")
    async def health() -> Dict[str, str]:
        """健康检查接口"""
        return {"status": "ok"}

    @app.get("/api/hello")
    async def hello(name: str = "World") -> Dict[str, str]:
        """简单测试接口"""
        return {"message": f"Hello, {name}!"}

    @app.post("/api/echo")
    async def echo(payload: Dict[str, Any]) -> JSONResponse:
        """回声测试接口"""
        return JSONResponse(payload)

    @app.websocket("/ws")
    async def ws_endpoint(ws: WebSocket) -> None:
        """WebSocket 简单回声服务"""
        await ws.accept()
        try:
            await ws.send_text("[server] connected")
            while True:
                msg = await ws.receive_text()
                await ws.send_text(f"[echo] {msg}")
        except WebSocketDisconnect:
            # 客户端断开连接，无需额外处理
            pass

    # 地图数据接口
    @app.get("/api/map", response_model=Graph)
    async def get_map() -> Graph:
        """获取世界地图图数据"""
        return load_world_graph()

    # --- 玩家移动接口 ---

    class PlayerStatus(BaseModel):
        location: str
        time: TimeStatus

    class MoveRequest(BaseModel):
        to: str
        speed: float = DEFAULT_MOVE_SPEED
        coefficient: float = DEFAULT_TIME_COEFFICIENT

    # 简单的内存中保存玩家位置（即用户的当前位置）
    # 默认起点为地图中的 “Capital”
    # 注意：在实际环境中应改为使用会话或数据库进行持久化存储
    app.state.player_location = "Capital"
    app.state.elapsed_hours = 0.0

    def _current_time_status() -> TimeStatus:
        total = getattr(app.state, "elapsed_hours", 0.0)
        day = int(total // HOURS_PER_DAY) + 1
        hour = total % HOURS_PER_DAY
        return TimeStatus(day=day, hour=hour, total_hours=total)

    def _advance_time(hours: float) -> TimeStatus:
        if hours <= 0:
            return _current_time_status()
        app.state.elapsed_hours = getattr(app.state, "elapsed_hours", 0.0) + hours
        return _current_time_status()

    @app.get("/api/player", response_model=PlayerStatus)
    async def get_player() -> PlayerStatus:
        """获取当前玩家位置"""
        return PlayerStatus(location=app.state.player_location, time=_current_time_status())

    @app.get("/api/time", response_model=TimeStatus)
    async def get_time() -> TimeStatus:
        """获取游戏时间信息"""
        return _current_time_status()

    @app.post("/api/player/move", response_model=PlayerStatus)
    async def move_player(
        request: Request,
        req: MoveRequest | None = Body(default=None),
        to: str | None = Body(default=None),
    ) -> PlayerStatus:
        """移动玩家到新的位置"""
        graph = load_world_graph()
        cur = app.state.player_location

        # 兼容多种提交格式：JSON({to}), 原始 JSON 或 body 字段 "to"
        target: str | None = None
        speed = DEFAULT_MOVE_SPEED
        coefficient = DEFAULT_TIME_COEFFICIENT
        if req and getattr(req, "to", None):
            target = req.to
            speed = getattr(req, "speed", DEFAULT_MOVE_SPEED)
            coefficient = getattr(req, "coefficient", DEFAULT_TIME_COEFFICIENT)
        elif to:
            target = to
        else:
            data: Any | None = None
            try:
                data = await request.json()
            except Exception:
                data = None
            if isinstance(data, dict):
                target = data.get("to")
                raw_speed = data.get("speed")
                raw_coefficient = data.get("coefficient")
                if raw_speed is not None:
                    try:
                        speed = float(raw_speed)
                    except (TypeError, ValueError):
                        raise HTTPException(status_code=400, detail="速度格式错误")
                if raw_coefficient is not None:
                    try:
                        coefficient = float(raw_coefficient)
                    except (TypeError, ValueError):
                        raise HTTPException(status_code=400, detail="系数格式错误")

        if not target:
            raise HTTPException(status_code=400, detail="缺少目标字段: to")

        if speed <= 0 or coefficient <= 0:
            raise HTTPException(status_code=400, detail="速度和系数必须大于 0")

        if target not in graph.nodes:
            raise HTTPException(status_code=400, detail=f"未知地点: {target}")

        # 允许移动到当前位置，否则必须为相邻节点
        time_state: TimeStatus | None = None
        if target != cur:
            node = graph.nodes.get(cur)
            if not node:
                # 若当前位置无效（理论上不应发生），直接跳转
                app.state.player_location = target
            else:
                if target not in node.neighbors:
                    raise HTTPException(status_code=400, detail=f"{target} 不是 {cur} 的相邻地点，无法移动")
                app.state.player_location = target
                distance = graph.distance_between(cur, target)
                time_state = _advance_time(distance * speed * coefficient)

        if time_state is None:
            time_state = _current_time_status()

        return PlayerStatus(location=app.state.player_location, time=time_state)

    # 如果前端打包文件存在，则在生产环境中提供静态文件服务
    dist_dir = Path(__file__).resolve().parents[2] / "frontend" / "dist"
    index_html = dist_dir / "index.html"
    if index_html.exists():
        # 挂载到根路径，确保 SPA（单页应用）前端路由可正常工作
        app.mount("/", StaticFiles(directory=dist_dir, html=True), name="spa")

    return app


app = create_app()

if __name__ == "__main__":
    # 直接运行：启动内置 Uvicorn 服务器
    host = os.getenv("HOST", "127.0.0.1")
    port = int(os.getenv("PORT", "8000"))

    # 直接传入 app 实例即可；
    # 如果需要热重载，请改为字符串导入并设置 reload=True
    uvicorn.run(app, host=host, port=port)
