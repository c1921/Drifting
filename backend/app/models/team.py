# ===========================================
# File: models/team.py
# Description: Pydantic models for Team API responses
# ===========================================

from __future__ import annotations

from typing import List, Optional

from pydantic import BaseModel, Field

from backend.app.models.character import CharacterResponse


class TeamResponse(BaseModel):
    """
    队伍API响应模型
    
    用于API返回队伍数据时的序列化
    """
    name: str = Field(..., description="队伍名称")
    leader: str = Field(..., description="队长名称")
    leader_id: str = Field(..., description="队长ID")
    is_player_controlled: bool = Field(..., description="是否由玩家控制")
    
    # 成员信息
    members: List[CharacterResponse] = Field(default_factory=list, description="队伍成员列表")
    member_count: int = Field(..., description="成员数量")
    
    # 资源
    gold: int = Field(..., description="金币数量")
    food: int = Field(..., description="食物数量")
    capacity: float = Field(..., description="最大负重")
    
    # 状态
    morale: float = Field(..., description="队伍士气")
    speed: float = Field(..., description="移动速度")
    location: str = Field(..., description="当前位置")
    destination: Optional[str] = Field(None, description="目的地")
    distance_to_destination: float = Field(0.0, description="到目的地的距离")
    traveling: bool = Field(..., description="是否正在旅行")
    
    class Config:
        json_schema_extra = {
            "example": {
                "name": "漂流者",
                "leader": "旅行者",
                "leader_id": "123e4567-e89b-12d3-a456-426614174000",
                "is_player_controlled": True,
                "members": [],
                "member_count": 3,
                "gold": 100,
                "food": 50,
                "capacity": 500.0,
                "morale": 100.0,
                "speed": 1.0,
                "location": "Capital",
                "destination": None,
                "distance_to_destination": 0.0,
                "traveling": False
            }
        }


class TeamStatsResponse(BaseModel):
    """
    队伍统计信息API响应模型
    """
    total_health: int = Field(..., description="总生命值")
    average_health: float = Field(..., description="平均生命值")
    total_morale: int = Field(..., description="总士气")
    average_morale: float = Field(..., description="平均士气")
    total_fatigue: int = Field(..., description="总疲劳度")
    average_fatigue: float = Field(..., description="平均疲劳度")
    alive_count: int = Field(..., description="存活人数")
    dead_count: int = Field(..., description="死亡人数")


class GameStateResponse(BaseModel):
    """
    游戏状态API响应模型
    """
    is_initialized: bool = Field(..., description="游戏是否已初始化")
    current_location: str = Field(..., description="当前位置")
    elapsed_hours: float = Field(..., description="已经过的游戏时间（小时）")
    current_day: int = Field(..., description="当前天数")
    current_hour: float = Field(..., description="当天的小时数")
    player_team: Optional[dict] = Field(None, description="玩家队伍信息")
    
    class Config:
        json_schema_extra = {
            "example": {
                "is_initialized": True,
                "current_location": "Capital",
                "elapsed_hours": 0.0,
                "current_day": 1,
                "current_hour": 0.0,
                "player_team": {
                    "name": "漂流者",
                    "members": ["旅行者", "守卫", "商人"],
                    "gold": 100,
                    "food": 50,
                    "morale": 100.0,
                    "location": "Capital",
                    "traveling": False
                }
            }
        }


class InitGameRequest(BaseModel):
    """
    初始化游戏请求模型
    """
    player_name: str = Field(default="旅行者", description="玩家角色名称")
    team_name: str = Field(default="漂流者", description="队伍名称")
    starting_location: str = Field(default="Capital", description="起始位置")
    
    class Config:
        json_schema_extra = {
            "example": {
                "player_name": "旅行者",
                "team_name": "漂流者",
                "starting_location": "Capital"
            }
        }

