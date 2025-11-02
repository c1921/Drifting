# ===========================================
# File: core/character_factory.py
# Description: Provides helpers to build Character instances with presets.
# ===========================================

from __future__ import annotations

import random
from typing import Dict, Iterable, List, Optional, Sequence

from .character import Character


def _clamp(value: int, minimum: int = 0, maximum: int = 10) -> int:
    """
    将整数值限制在指定范围内
    
    参数:
        value (int): 需要限制的值
        minimum (int): 最小值，默认为0
        maximum (int): 最大值，默认为10
        
    返回:
        int: 限制在[min, max]范围内的值
        
    说明:
        用于确保角色属性和技能值在合理的游戏范围内
    """
    return max(minimum, min(maximum, value))


class CharacterFactory:
    """用于通过可重复的模板或随机化生成 Character 实例的工厂。"""

    # 默认名称池，按性别分类
    DEFAULT_NAME_POOL: Dict[str, Sequence[str]] = {
        "male": ("Arin", "Lukan", "Marek", "Daro", "Renn"),      # 男性名字
        "female": ("Seris", "Lyra", "Mira", "Elin", "Tala"),     # 女性名字
        "unknown": ("Ash", "Rune", "Sol", "Drift", "Vale"),      # 中性/未知性别名字
    }

    # 默认出身地选项
    DEFAULT_ORIGINS: Sequence[str] = (
        "Sunset Caravan",    # 日落商队
        "Harbor of Myria",   # 米里亚港
        "Shifting Sands",    # 流沙之地
        "Northern Frontier", # 北方边境
        "Unknown",           # 未知
    )

    # 默认性格特质选项
    DEFAULT_TRAITS: Sequence[str] = (
        "brave",        # 勇敢 - 更倾向于冒险和战斗
        "cautious",     # 谨慎 - 更倾向于规避风险
        "greedy",       # 贪婪 - 更关注财富和奖励
        "loyal",        # 忠诚 - 更重视团队和承诺
        "stoic",        # 坚忍 - 情绪波动较小
        "curious",      # 好奇 - 更倾向于探索未知
        "coward",       # 懦弱 - 倾向于逃避危险
        "charismatic",  # 魅力 - 社交能力更强
    )

    # 角色职业模板 - 定义不同职业的属性和技能加成
    ROLE_TEMPLATES: Dict[str, Dict[str, Dict[str, int]]] = {
        "merchant": {  # 商人
            "attributes": {"charisma": +2, "intelligence": +1},  # 魅力+2, 智力+1
            "skills": {"trading": +3, "negotiation": +2},        # 交易+3, 谈判+2
        },
        "guard": {     # 守卫
            "attributes": {"strength": +2, "endurance": +2},     # 力量+2, 耐力+2
            "skills": {"combat": +3},                            # 战斗+3
        },
        "scout": {     # 侦察兵
            "attributes": {"perception": +2, "endurance": +1},   # 感知+2, 耐力+1
            "skills": {"navigation": +3},                        # 导航+3
        },
        "healer": {    # 治疗师
            "attributes": {"intelligence": +2, "charisma": +1},  # 智力+2, 魅力+1
            "skills": {"medicine": +4},                          # 医疗+4
        },
        "traveler": {  # 旅行者 - 无特殊加成的基础职业
            "attributes": {},
            "skills": {},
        },
    }

    def __init__(self, seed: Optional[int] = None):
        """
        初始化角色工厂
        
        参数:
            seed (Optional[int]): 随机数种子，用于生成可重复的随机结果
                                  如果为None，则使用系统时间作为种子
        """
        self.random = random.Random(seed)  # 创建独立的随机数生成器

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------

    def create_character(
        self,
        name: Optional[str] = None,
        gender: Optional[str] = None,
        age: Optional[int] = None,
        role: Optional[str] = None,
        control_type: str = "AI",
        origin: Optional[str] = None,
        traits: Optional[Iterable[str]] = None,
    ) -> Character:
        """
        创建角色实例，支持参数覆盖
        
        参数:
            name (Optional[str]): 角色名称，None则随机生成
            gender (Optional[str]): 性别，None则随机选择
            age (Optional[int]): 年龄，None则随机生成(16-60)
            role (Optional[str]): 职业，None则随机选择
            control_type (str): 控制类型，"AI"或"player"
            origin (Optional[str]): 出身地，None则随机选择
            traits (Optional[Iterable[str]]): 性格特质，None则随机分配
            
        返回:
            Character: 创建的角色实例
            
        说明:
            - 所有可选参数如果为None，都会使用合理的默认值
            - 自动应用职业模板的属性/技能加成
            - 自动分配性格特质
        """
        # 选择职业（如果未指定则随机选择）
        chosen_role = role or self.random.choice(list(self.ROLE_TEMPLATES.keys()))
        
        # 选择性别（如果未指定则随机选择）
        chosen_gender = gender or self.random.choice(list(self.DEFAULT_NAME_POOL.keys()))
        
        # 根据性别确定名称池的键（如果性别不在预定义中则使用"unknown"）
        gender_key = chosen_gender if chosen_gender in self.DEFAULT_NAME_POOL else "unknown"
        
        # 选择名称（如果未指定则根据性别随机选择）
        chosen_name = name or self.random.choice(self.DEFAULT_NAME_POOL[gender_key])
        
        # 选择年龄（如果未指定则随机生成16-60岁）
        chosen_age = age if age is not None else self.random.randint(16, 60)
        
        # 选择出身地（如果未指定则随机选择）
        chosen_origin = origin or self.random.choice(self.DEFAULT_ORIGINS)

        # 创建基础角色实例
        character = Character(
            name=chosen_name,
            gender=chosen_gender,
            age=chosen_age,
            control_type=control_type,
            role=chosen_role,
            origin=chosen_origin,
        )

        # 应用职业模板的属性和技能加成
        self._apply_role_template(character, chosen_role)
        
        # 分配性格特质
        self._assign_traits(character, traits)
        
        return character

    def create_player(self, name: str, **overrides) -> Character:
        """
        创建玩家控制角色的便捷方法
        
        参数:
            name (str): 玩家角色名称（必需）
            **overrides: 其他传递给create_character的参数
            
        返回:
            Character: 玩家控制的角色实例
            
        说明:
            - 自动设置控制类型为"player"
            - 默认职业为"traveler"（旅行者）
            - 支持所有create_character的参数覆盖
        """
        overrides["control_type"] = "player"  # 强制设置为玩家控制
        overrides.setdefault("role", "traveler")  # 默认职业为旅行者
        return self.create_character(name=name, **overrides)

    def create_party(
        self,
        size: int,
        roles: Optional[Sequence[str]] = None,
        origin: Optional[str] = None,
    ) -> List[Character]:
        """
        创建角色队伍
        
        参数:
            size (int): 队伍大小
            roles (Optional[Sequence[str]]): 指定的职业列表，None则使用所有可用职业
            origin (Optional[str]): 统一的出身地，None则每个角色随机
            
        返回:
            List[Character]: 角色队伍列表
            
        说明:
            - 尽量确保队伍职业多样性
            - 当指定职业列表用尽时会重新填充
            - 自动建立队伍成员间的相互认识关系
        """
        roster: List[Character] = []  # 队伍列表
        available_roles = list(roles or self.ROLE_TEMPLATES.keys())  # 可用职业列表

        for index in range(size):
            # 如果可用职业列表为空，重新填充
            if not available_roles:
                available_roles = list(self.ROLE_TEMPLATES.keys())

            # 从可用职业中随机选择一个
            role_choice = self.random.choice(available_roles)
            
            # 尽量避免连续使用相同职业（当有多个选择时）
            if len(available_roles) > 1:
                available_roles.remove(role_choice)

            # 创建角色并添加到队伍
            character = self.create_character(role=role_choice, origin=origin)
            roster.append(character)

        # 建立队伍成员间的相互认识关系
        self._link_known_characters(roster)
        return roster

    # ------------------------------------------------------------------
    # Internal helpers
    # ------------------------------------------------------------------

    def _apply_role_template(self, character: Character, role: str) -> None:
        """
        应用职业模板的属性和技能加成
        
        参数:
            character (Character): 要应用模板的角色
            role (str): 职业名称
            
        说明:
            - 根据职业模板调整角色的属性和技能值
            - 使用_clamp函数确保值在合理范围内
            - 技能值的上限为10
        """
        template = self.ROLE_TEMPLATES.get(role, {})  # 获取职业模板
        
        # 应用属性加成
        for attr_name, delta in template.get("attributes", {}).items():
            if attr_name in character.attributes:
                character.attributes[attr_name] = _clamp(character.attributes[attr_name] + delta)

        # 应用技能加成（技能上限为10）
        for skill_name, delta in template.get("skills", {}).items():
            if skill_name in character.skills:
                character.skills[skill_name] = _clamp(character.skills[skill_name] + delta, maximum=10)

    def _assign_traits(self, character: Character, explicit_traits: Optional[Iterable[str]]) -> None:
        """
        为角色分配性格特质
        
        参数:
            character (Character): 要分配特质的角色
            explicit_traits (Optional[Iterable[str]]): 显式指定的特质列表，None则随机分配
            
        说明:
            - 如果指定了特质列表，直接使用指定的特质
            - 否则随机分配1-3个特质
            - 特质会影响角色的AI决策和行为模式
        """
        if explicit_traits is not None:
            character.traits = list(explicit_traits)  # 使用指定的特质
            return

        # 随机分配1-3个特质
        count = self.random.randint(1, 3)
        character.traits = self.random.sample(self.DEFAULT_TRAITS, count)

    def _link_known_characters(self, roster: List[Character]) -> None:
        """
        建立队伍成员间的相互认识关系
        
        参数:
            roster (List[Character]): 角色队伍列表
            
        说明:
            - 模拟队伍成员之间预先存在的熟悉关系
            - 每个角色都会认识队伍中的其他所有成员
            - 这会影响社交互动和关系系统
        """
        ids = [member.id for member in roster]  # 收集所有角色ID
        for member in roster:
            # 将其他所有角色的ID添加到当前角色的known_characters集合中
            member.known_characters.update(other_id for other_id in ids if other_id != member.id)
