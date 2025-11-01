from __future__ import annotations

from pathlib import Path
from typing import Any, Dict

from fastapi import FastAPI, WebSocket, WebSocketDisconnect
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse
from fastapi.staticfiles import StaticFiles
from backend.app.data.world_map import load_world_graph
from backend.app.models.map import Graph


def create_app() -> FastAPI:
    app = FastAPI(title="Drifting Backend", version="0.1.0")

    # CORS for Vite dev server
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
        return {"status": "ok"}

    @app.get("/api/hello")
    async def hello(name: str = "World") -> Dict[str, str]:
        return {"message": f"Hello, {name}!"}

    @app.post("/api/echo")
    async def echo(payload: Dict[str, Any]) -> JSONResponse:
        return JSONResponse(payload)

    @app.websocket("/ws")
    async def ws_endpoint(ws: WebSocket) -> None:
        await ws.accept()
        try:
            await ws.send_text("[server] connected")
            while True:
                msg = await ws.receive_text()
                await ws.send_text(f"[echo] {msg}")
        except WebSocketDisconnect:
            # Client disconnected; nothing else to do.
            pass

    # Map endpoints
    @app.get("/api/map", response_model=Graph)
    async def get_map() -> Graph:
        return load_world_graph()

    # Serve built frontend in production if available
    dist_dir = Path(__file__).resolve().parents[2] / "frontend" / "dist"
    index_html = dist_dir / "index.html"
    if index_html.exists():
        # Mount at root so the SPA works with client-side routing
        app.mount("/", StaticFiles(directory=dist_dir, html=True), name="spa")

    return app


app = create_app()
