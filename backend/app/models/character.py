# ===========================================
# File: models/character.py
# Description: Pydantic models for Character API responses
# ===========================================

from __future__ import annotations

from typing import Dict, List, Optional

from pydantic import BaseModel, Field


class CharacterResponse(BaseModel):
    """
    角色API响应模型
    
    用于API返回角色数据时的序列化
    """
    id: str = Field(..., description="角色唯一标识符")
    name: str = Field(..., description="角色姓名")
    gender: str = Field(..., description="性别")
    age: int = Field(..., description="年龄")
    control_type: str = Field(..., description="控制类型: player或AI")
    role: str = Field(..., description="职业角色")
    origin: Optional[str] = Field(None, description="出身地")
    
    # 属性和技能
    attributes: Dict[str, int] = Field(default_factory=dict, description="基础属性")
    skills: Dict[str, int] = Field(default_factory=dict, description="技能等级")
    traits: List[str] = Field(default_factory=list, description="性格特质")
    
    # 状态
    mood: Dict[str, int] = Field(default_factory=dict, description="情绪状态")
    health: int = Field(..., description="生命值 (0-100)")
    fatigue: int = Field(..., description="疲劳度 (0-100)")
    morale: int = Field(..., description="士气 (0-100)")
    loyalty: int = Field(..., description="忠诚度 (-100到100)")
    is_alive: bool = Field(..., description="存活状态")
    reputation: int = Field(..., description="声誉值")
    
    class Config:
        json_schema_extra = {
            "example": {
                "id": "123e4567-e89b-12d3-a456-426614174000",
                "name": "旅行者",
                "gender": "unknown",
                "age": 25,
                "control_type": "player",
                "role": "traveler",
                "origin": "Capital",
                "attributes": {
                    "strength": 5,
                    "intelligence": 6,
                    "charisma": 7,
                    "endurance": 5,
                    "perception": 6
                },
                "skills": {
                    "trading": 3,
                    "combat": 2,
                    "medicine": 1,
                    "navigation": 3,
                    "negotiation": 2
                },
                "traits": ["curious", "brave"],
                "mood": {
                    "happiness": 70,
                    "fear": 10,
                    "anger": 0
                },
                "health": 100,
                "fatigue": 0,
                "morale": 100,
                "loyalty": 50,
                "is_alive": True,
                "reputation": 0
            }
        }


class CharacterListResponse(BaseModel):
    """
    角色列表API响应模型
    """
    characters: List[CharacterResponse] = Field(default_factory=list, description="角色列表")
    total: int = Field(..., description="角色总数")

