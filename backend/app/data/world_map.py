from __future__ import annotations

from typing import Dict, List

from backend.app.models.map import CityNode, Coordinate, Graph, Terrain


def load_world_graph() -> Graph:
    """返回一个示例世界地图（无方向加权图，距离单位自定）。"""

    nodes: Dict[str, CityNode] = {}

    def add_city(
        name: str,
        x: float,
        y: float,
        terrain: Terrain,
        neighbors: List[str] | None = None,
    ) -> None:
        nodes[name] = CityNode(
            name=name,
            coord=Coordinate(x=x, y=y),
            terrain=terrain,
            neighbors=neighbors or [],
        )

    # 定义若干城市与连接（距离为示例值）
    add_city("Capital", 0, 0, Terrain.plains, ["Harbor", "Farmland"])
    add_city("Harbor", 10, -2, Terrain.water, ["Capital", "ForestGate"])
    add_city("Farmland", -4, 3, Terrain.plains, ["Capital", "Oasis"])
    add_city("Oasis", -16, 6, Terrain.desert, ["Farmland", "MountPass"])
    add_city("MountPass", -6, 14, Terrain.mountain, ["Oasis", "ForestGate"])
    add_city("ForestGate", 4, 8, Terrain.forest, ["MountPass", "Harbor"])

    # 确保邻接是双向（若需要无向图）
    for name, node in list(nodes.items()):
        for neigh in list(node.neighbors):
            if neigh in nodes and name not in nodes[neigh].neighbors:
                nodes[neigh].neighbors.append(name)

    return Graph(nodes=nodes)
