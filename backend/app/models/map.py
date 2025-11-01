from __future__ import annotations

from enum import Enum
from typing import Dict, List
import math

from pydantic import BaseModel, Field


class Terrain(str, Enum):
    plains = "plains"
    desert = "desert"
    mountain = "mountain"
    forest = "forest"
    hills = "hills"
    swamp = "swamp"
    water = "water"


class Coordinate(BaseModel):
    x: float
    y: float


class CityNode(BaseModel):
    """
    一个地图节点：城市/地点。

    - name: 城市名称
    - coord: 坐标（世界坐标或屏幕坐标，单位自定）
    - terrain: 地形类型
    - neighbors: 邻接节点名称列表（距离不再存储，按坐标动态计算）
    """

    name: str
    coord: Coordinate
    terrain: Terrain
    neighbors: List[str] = Field(default_factory=list)


class Graph(BaseModel):
    """
    简单图结构，使用名称->节点 的映射保存所有城市。
    """

    nodes: Dict[str, CityNode] = Field(default_factory=dict)

    def neighbors_of(self, name: str) -> List[str]:
        node = self.nodes.get(name)
        return list(node.neighbors) if node else []

    def distance_between(self, a: str, b: str) -> float:
        """基于坐标的欧氏距离（动态计算）。"""
        na = self.nodes.get(a)
        nb = self.nodes.get(b)
        if not na or not nb:
            raise KeyError(f"Unknown node(s): {a}, {b}")
        dx = na.coord.x - nb.coord.x
        dy = na.coord.y - nb.coord.y
        return math.hypot(dx, dy)
