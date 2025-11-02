# ===========================================
# File: services/team_service.py
# Description: Team management service layer
# ===========================================

from __future__ import annotations

from typing import Dict, List, Optional

from backend.app.core.team import Team
from backend.app.core.character import Character


class TeamService:
    """
    队伍管理服务
    
    提供队伍相关的业务逻辑：
    - 队伍信息序列化
    - 成员管理
    - 资源管理
    - 行动处理
    """
    
    @staticmethod
    def serialize_team(team: Team) -> Dict:
        """
        将队伍序列化为完整的字典格式
        
        参数:
            team (Team): 要序列化的队伍
            
        返回:
            Dict: 包含队伍完整信息的字典
        """
        return {
            "name": team.name,
            "leader": team.leader.name,
            "leader_id": team.leader.id,
            "is_player_controlled": team.is_player_controlled,
            "members": [TeamService.serialize_character(member) for member in team.members],
            "member_count": len(team.members),
            "gold": team.gold,
            "food": team.food,
            "capacity": team.capacity,
            "morale": team.morale,
            "speed": team.speed,
            "location": team.location,
            "destination": team.destination,
            "distance_to_destination": team.distance_to_destination,
            "traveling": team.traveling,
        }
    
    @staticmethod
    def serialize_character(character: Character) -> Dict:
        """
        将角色序列化为字典格式
        
        参数:
            character (Character): 要序列化的角色
            
        返回:
            Dict: 包含角色信息的字典
        """
        return {
            "id": character.id,
            "name": character.name,
            "gender": character.gender,
            "age": character.age,
            "control_type": character.control_type,
            "role": character.role,
            "origin": character.origin,
            "attributes": character.attributes,
            "skills": character.skills,
            "traits": character.traits,
            "mood": character.mood,
            "health": character.health,
            "fatigue": character.fatigue,
            "morale": character.morale,
            "loyalty": character.loyalty,
            "is_alive": character.is_alive,
            "reputation": character.reputation,
        }
    
    @staticmethod
    def get_member_by_id(team: Team, member_id: str) -> Optional[Character]:
        """
        根据ID获取队伍成员
        
        参数:
            team (Team): 队伍实例
            member_id (str): 成员ID
            
        返回:
            Optional[Character]: 找到的角色，如果不存在则返回None
        """
        for member in team.members:
            if member.id == member_id:
                return member
        return None
    
    @staticmethod
    def get_player_character(team: Team) -> Optional[Character]:
        """
        获取队伍中的玩家角色
        
        参数:
            team (Team): 队伍实例
            
        返回:
            Optional[Character]: 玩家角色，如果不存在则返回None
        """
        for member in team.members:
            if member.is_player():
                return member
        return None
    
    @staticmethod
    def get_npc_characters(team: Team) -> List[Character]:
        """
        获取队伍中的所有NPC角色
        
        参数:
            team (Team): 队伍实例
            
        返回:
            List[Character]: NPC角色列表
        """
        return [member for member in team.members if not member.is_player()]
    
    @staticmethod
    def calculate_team_stats(team: Team) -> Dict:
        """
        计算队伍的综合统计数据
        
        参数:
            team (Team): 队伍实例
            
        返回:
            Dict: 包含队伍统计信息的字典
        """
        if not team.members:
            return {
                "total_health": 0,
                "average_health": 0,
                "total_morale": 0,
                "average_morale": 0,
                "total_fatigue": 0,
                "average_fatigue": 0,
                "alive_count": 0,
                "dead_count": 0,
            }
        
        alive_members = [m for m in team.members if m.is_alive]
        dead_members = [m for m in team.members if not m.is_alive]
        
        total_health = sum(m.health for m in team.members)
        total_morale = sum(m.morale for m in team.members)
        total_fatigue = sum(m.fatigue for m in team.members)
        
        return {
            "total_health": total_health,
            "average_health": total_health / len(team.members),
            "total_morale": total_morale,
            "average_morale": total_morale / len(team.members),
            "total_fatigue": total_fatigue,
            "average_fatigue": total_fatigue / len(team.members),
            "alive_count": len(alive_members),
            "dead_count": len(dead_members),
        }

