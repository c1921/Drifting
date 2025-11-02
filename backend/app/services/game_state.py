# ===========================================
# File: services/game_state.py
# Description: Global game state management
# ===========================================

from __future__ import annotations

from typing import Optional

from backend.app.core.team import Team
from backend.app.core.character_factory import CharacterFactory


class GameState:
    """
    全局游戏状态管理器
    
    管理游戏的核心状态，包括：
    - 玩家队伍
    - 游戏时间
    - 当前位置
    - 游戏进度
    
    设计为单例模式，确保全局只有一个游戏状态实例
    """
    
    def __init__(self):
        # ===== 核心状态 =====
        self.player_team: Optional[Team] = None  # 玩家队伍
        self.elapsed_hours: float = 0.0  # 已经过的游戏时间（小时）
        self.current_location: str = "Capital"  # 当前位置
        self.is_initialized: bool = False  # 游戏是否已初始化
        
        # ===== 游戏配置 =====
        self.hours_per_day: int = 24  # 每天小时数
        
    def initialize_new_game(
        self, 
        player_name: str = "旅行者",
        team_name: str = "漂流者",
        starting_location: str = "Capital"
    ) -> None:
        """
        初始化新游戏
        
        参数:
            player_name (str): 玩家角色名称
            team_name (str): 队伍名称
            starting_location (str): 起始位置
            
        说明:
            - 创建玩家角色（旅行者职业）
            - 生成2个随机NPC队友（守卫和商人）
            - 组建玩家队伍
            - 设置初始资源和位置
        """
        # 使用角色工厂生成角色
        factory = CharacterFactory()
        
        # 创建玩家角色
        player = factory.create_player(name=player_name, role="traveler")
        
        # 创建2个NPC队友
        npc_guard = factory.create_character(role="guard", control_type="AI")
        npc_merchant = factory.create_character(role="merchant", control_type="AI")
        
        # 创建玩家队伍
        self.player_team = Team(
            name=team_name,
            leader=player,
            is_player_controlled=True
        )
        
        # 添加NPC到队伍
        self.player_team.add_member(npc_guard)
        self.player_team.add_member(npc_merchant)
        
        # 设置初始位置和资源
        self.player_team.location = starting_location
        self.current_location = starting_location
        self.player_team.gold = 100  # 初始金币
        self.player_team.food = 50   # 初始食物
        
        # 重置游戏时间
        self.elapsed_hours = 0.0
        
        # 标记为已初始化
        self.is_initialized = True
        
    def reset(self) -> None:
        """
        重置游戏状态
        
        说明:
            清空所有游戏数据，回到未初始化状态
        """
        self.player_team = None
        self.elapsed_hours = 0.0
        self.current_location = "Capital"
        self.is_initialized = False
        
    def advance_time(self, hours: float) -> None:
        """
        推进游戏时间
        
        参数:
            hours (float): 要推进的小时数
            
        说明:
            更新游戏时间计数器
        """
        if hours > 0:
            self.elapsed_hours += hours
            
    def get_current_day(self) -> int:
        """
        获取当前游戏天数
        
        返回:
            int: 当前天数（从第1天开始）
        """
        return int(self.elapsed_hours // self.hours_per_day) + 1
    
    def get_current_hour(self) -> float:
        """
        获取当前一天中的小时数
        
        返回:
            float: 当天的小时数（0-23.99）
        """
        return self.elapsed_hours % self.hours_per_day
    
    def to_dict(self) -> dict:
        """
        将游戏状态序列化为字典
        
        返回:
            dict: 包含游戏状态的字典
        """
        return {
            "is_initialized": self.is_initialized,
            "current_location": self.current_location,
            "elapsed_hours": self.elapsed_hours,
            "current_day": self.get_current_day(),
            "current_hour": self.get_current_hour(),
            "player_team": self.player_team.summary() if self.player_team else None,
        }

