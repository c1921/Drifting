# ===========================================
# 文件: core/character.py
# 描述: 定义角色类 - 所有NPC和玩家的基础类
# 功能:
#   - 管理角色的基本信息、属性和技能
#   - 处理角色状态、情绪和关系系统
#   - 实现AI决策和记忆系统
#   - 提供序列化接口用于保存和加载
# ===========================================

import random
import uuid
from typing import Dict, List, Optional


class Character:
    """
    角色类 - 表示游戏世界中的任何人物
    
    角色可以是玩家、商队成员、定居者或流浪者等。
    统一的结构确保所有角色共享相同的行为接口，便于管理和交互。
    
    主要功能:
    - 基础信息管理 (姓名、性别、年龄、角色等)
    - 属性和技能系统 (力量、智力、交易、战斗等)
    - 状态和情绪系统 (健康、疲劳、士气、情绪等)
    - 社交关系系统 (派系、人际关系、声誉等)
    - 事件记忆系统 (记录重要事件和经历)
    - AI决策系统 (基于性格和状态做出决策)
    """

    def __init__(
        self,
        name: str,
        gender: str = "unknown",
        age: int = 18,
        control_type: str = "AI",
        role: str = "traveler",
        origin: Optional[str] = None,
    ):
        # ===== 基础信息 =====
        self.id: str = str(uuid.uuid4())  # 唯一标识符
        self.name: str = name             # 角色姓名
        self.gender: str = gender         # 性别
        self.age: int = age               # 年龄
        self.control_type: str = control_type  # 控制类型: "player"(玩家控制) 或 "AI"(AI控制)
        self.origin: Optional[str] = origin    # 出身/来源地
        self.role: str = role             # 角色职业: "merchant"(商人), "guard"(守卫), "healer"(治疗师)等

        # ===== 属性和技能系统 =====
        # 基础属性 (3-8之间的随机值)
        self.attributes: Dict[str, int] = {
            "strength": random.randint(3, 8),      # 力量: 影响物理攻击和负重
            "intelligence": random.randint(3, 8),  # 智力: 影响学习和魔法能力
            "charisma": random.randint(3, 8),      # 魅力: 影响社交和交易
            "endurance": random.randint(3, 8),     # 耐力: 影响生命值和疲劳恢复
            "perception": random.randint(3, 8),    # 感知: 影响侦查和发现能力
        }

        # 技能等级 (0-5之间的随机值)
        self.skills: Dict[str, int] = {
            "trading": random.randint(0, 5),     # 交易技能: 影响买卖价格
            "combat": random.randint(0, 5),      # 战斗技能: 影响攻击和防御
            "medicine": random.randint(0, 5),    # 医疗技能: 影响治疗效果
            "navigation": random.randint(0, 5),  # 导航技能: 影响旅行效率
            "negotiation": random.randint(0, 5), # 谈判技能: 影响对话结果
        }

        # ===== 性格和情绪系统 =====
        self.traits: List[str] = []  # 性格特质列表: 例如 ["brave"(勇敢), "greedy"(贪婪), "loyal"(忠诚)]
        self.mood: Dict[str, int] = {  # 情绪状态 (0-100)
            "happiness": 70,  # 快乐度: 影响决策和互动
            "fear": 10,       # 恐惧度: 高恐惧会降低技能效果
            "anger": 0,       # 愤怒度: 影响攻击性和冲突倾向
        }

        # ===== 状态和健康系统 =====
        self.health: int = 100    # 生命值 (0-100): 0表示死亡
        self.fatigue: int = 0     # 疲劳度 (0-100): 高疲劳会影响行动能力
        self.morale: int = 100    # 士气 (0-100): 影响战斗表现和忠诚度
        self.loyalty: int = 50    # 忠诚度 (-100到100): 对所属派系的忠诚程度
        self.is_alive: bool = True  # 存活状态

        # ===== 社交关系系统 =====
        self.affiliation = None  # 所属派系: 商队(Caravan) / 定居点(Settlement) / 派系(Faction)
        self.relationships: Dict[str, Dict] = {}  # 人际关系: {角色ID: {"favor": 30, "type": "friend"}}
        self.reputation: int = 0  # 声誉值: 影响其他角色对该角色的态度
        self.known_characters: set[str] = set()  # 已知角色集合: 记录遇到过的其他角色

        # ===== 记忆和事件系统 =====
        self.memories: List[Dict] = []  # 记忆列表: 存储过去发生的重要事件
        self.current_goal: Optional[str] = None  # 当前目标: AI角色的行为目标
        self.pending_event: Optional[Dict] = None  # 待处理事件: 等待处理的事件数据

    # -------------------------------------------------------------------------
    # Core Methods
    # -------------------------------------------------------------------------

    def __repr__(self):
        return f"<Character {self.name} ({self.role}) | HP:{self.health} | Morale:{self.morale}>"

    def is_player(self) -> bool:
        """
        判断角色是否由玩家控制
        
        返回:
            bool: 如果角色由玩家控制返回True，否则返回False
        """
        return self.control_type == "player"

    # -------------------------------------------------------------------------
    # Ability and Skill System
    # -------------------------------------------------------------------------

    def get_effective_skill(self, skill_name: str) -> int:
        """
        计算有效技能值，基于基础技能、属性加成和情绪修正
        
        参数:
            skill_name (str): 技能名称
            
        返回:
            int: 计算后的有效技能值
            
        计算规则:
        - 基础技能值 + 属性加成 + 情绪修正
        - 交易技能: 受智力和魅力影响
        - 战斗技能: 受力量和耐力影响
        - 导航技能: 受感知影响
        - 恐惧情绪过高会降低技能效果
        - 快乐情绪高涨会提升技能效果
        """
        base = self.skills.get(skill_name, 0)  # 基础技能值
        bonus = 0  # 加成值

        # 属性加成计算
        if skill_name == "trading":
            # 交易技能: 智力和魅力各贡献25%
            bonus += (self.attributes["intelligence"] + self.attributes["charisma"]) // 4
        elif skill_name == "combat":
            # 战斗技能: 力量和耐力各贡献25%
            bonus += (self.attributes["strength"] + self.attributes["endurance"]) // 4
        elif skill_name == "navigation":
            # 导航技能: 感知贡献50%
            bonus += self.attributes["perception"] // 2

        # 情绪修正
        if self.mood["fear"] > 60:
            bonus -= 1  # 恐惧过高降低技能
        if self.mood["happiness"] > 80:
            bonus += 1  # 快乐高涨提升技能

        return max(0, base + bonus)  # 确保不会出现负值

    def train_skill(self, skill_name: str, exp: int = 1):
        """
        训练技能，增加技能经验值
        
        参数:
            skill_name (str): 要训练的技能名称
            exp (int): 获得的经验值，默认为1
            
        说明:
        - 技能值有上限(10)，超过上限不再增加
        - 只有已存在的技能才能被训练
        """
        if skill_name in self.skills:
            self.skills[skill_name] = min(10, self.skills[skill_name] + exp)

    # -------------------------------------------------------------------------
    # Status and Mood System
    # -------------------------------------------------------------------------

    def modify_morale(self, delta: int, reason: str = ""):
        """
        调整士气值并记录变化
        
        参数:
            delta (int): 士气变化量 (正数为增加，负数为减少)
            reason (str): 士气变化的原因描述
            
        说明:
        - 士气值被限制在0-100范围内
        - 变化会被记录到记忆系统中
        """
        self.morale = max(0, min(100, self.morale + delta))  # 限制在0-100范围内
        self.add_memory("morale_change", {"amount": delta, "reason": reason})

    def modify_health(self, delta: int, cause: str = ""):
        """
        调整生命值，处理伤害或治疗
        
        参数:
            delta (int): 生命值变化量 (正数为治疗，负数为伤害)
            cause (str): 生命值变化的原因描述
            
        说明:
        - 生命值被限制在0-100范围内
        - 当生命值降为0时，角色死亡
        """
        self.health = max(0, min(100, self.health + delta))  # 限制在0-100范围内
        if self.health == 0:
            self.die(cause)  # 生命值为0时触发死亡

    def die(self, cause: str = "unknown"):
        """
        处理角色死亡
        
        参数:
            cause (str): 死亡原因描述
            
        说明:
        - 设置存活状态为False
        - 记录死亡事件到记忆系统
        """
        self.is_alive = False
        self.add_memory("death", {"cause": cause})

    def rest(self, duration: int = 1):
        """
        休息，恢复疲劳和提升士气
        
        参数:
            duration (int): 休息时长，影响恢复效果
            
        说明:
        - 每单位时长减少10点疲劳度
        - 每单位时长增加5点士气
        - 疲劳度最低为0
        """
        self.fatigue = max(0, self.fatigue - 10 * duration)  # 减少疲劳，最低为0
        self.modify_morale(+5 * duration, reason="rested")   # 提升士气

    # -------------------------------------------------------------------------
    # Relationship System
    # -------------------------------------------------------------------------

    def change_relationship(self, other_char_id: str, delta: int, rel_type: str = "favor"):
        """
        调整与其他角色的关系值
        
        参数:
            other_char_id (str): 目标角色的ID
            delta (int): 关系变化量 (正数为改善，负数为恶化)
            rel_type (str): 关系类型，如 "friend"(朋友), "enemy"(敌人), "neutral"(中立)
            
        说明:
        - 关系值被限制在-100到100范围内
        - 如果目标角色不存在于关系中，会创建新的关系记录
        """
        rel = self.relationships.get(other_char_id, {"favor": 0, "type": "neutral"})
        rel["favor"] = max(-100, min(100, rel["favor"] + delta))  # 限制在-100到100
        rel["type"] = rel_type
        self.relationships[other_char_id] = rel

    def get_relationship(self, other_char_id: str) -> int:
        """
        获取与其他角色的关系值
        
        参数:
            other_char_id (str): 目标角色的ID
            
        返回:
            int: 关系值，如果关系不存在则返回0
        """
        rel = self.relationships.get(other_char_id)
        return rel["favor"] if rel else 0

    # -------------------------------------------------------------------------
    # Event and Memory System
    # -------------------------------------------------------------------------

    def add_memory(self, memory_type: str, details: Dict):
        """
        添加事件到记忆系统
        
        参数:
            memory_type (str): 记忆类型，如 "combat"(战斗), "trade"(交易), "meeting"(会面)
            details (Dict): 记忆的详细信息
            
        说明:
        - 记忆列表有最大长度限制(100条)
        - 超过限制时会移除最早的记忆
        - 自动记录发生时的回合数
        """
        self.memories.append(
            {
                "type": memory_type,
                "details": details,
                "turn": details.get("turn", 0),  # 记录事件发生的回合
            }
        )
        if len(self.memories) > 100:
            self.memories.pop(0)  # 记忆列表过长时移除最早的记忆

    def has_recent_memory(self, memory_type: str) -> bool:
        """
        检查最近是否有特定类型的记忆
        
        参数:
            memory_type (str): 要检查的记忆类型
            
        返回:
            bool: 如果最近10条记忆中存在该类型记忆则返回True
            
        说明:
        - 只检查最近的10条记忆
        - 用于判断角色是否近期经历过某些事件
        """
        for memory in self.memories[-10:]:  # 只检查最近10条记忆
            if memory["type"] == memory_type:
                return True
        return False

    # -------------------------------------------------------------------------
    # AI Decision Example
    # -------------------------------------------------------------------------

    def decide_next_action(self, context):
        """
        AI决策系统：基于角色状态、性格和情境决定下一步行动
        
        参数:
            context (Dict): 当前环境上下文信息
            
        返回:
            str: 决策的行动类型
            
        决策逻辑:
        - 死亡角色不执行任何行动
        - 低士气且胆小的角色会选择休息
        - 贪婪的角色在交易机会出现时会选择交易
        - 忠诚的角色会协助所属派系的领导者
        - 其他情况随机选择空闲行动
        """
        if not self.is_alive:
            return "none"  # 死亡角色无行动

        # 基于性格特质的决策
        if self.morale < 30 and "coward" in self.traits:
            return "rest"  # 低士气且胆小 -> 休息
        if "greedy" in self.traits and context.get("trade_opportunity"):
            return "trade"  # 贪婪且有交易机会 -> 交易
        if "loyal" in self.traits and self.affiliation:
            return "assist_leader"  # 忠诚且有派系 -> 协助领导者

        # 默认随机行动
        return random.choice(["idle", "chat", "travel"])

    # -------------------------------------------------------------------------
    # Serialization Interface (for save/load)
    # -------------------------------------------------------------------------

    def to_dict(self) -> Dict:
        """
        将角色数据转换为可序列化的字典格式
        
        返回:
            Dict: 包含所有角色数据的字典
            
        用途:
        - 用于游戏保存功能
        - 网络传输
        - 数据持久化存储
        """
        return {
            "id": self.id,
            "name": self.name,
            "gender": self.gender,
            "age": self.age,
            "control_type": self.control_type,
            "role": self.role,
            "origin": self.origin,
            "attributes": self.attributes,
            "skills": self.skills,
            "traits": self.traits,
            "mood": self.mood,
            "health": self.health,
            "fatigue": self.fatigue,
            "morale": self.morale,
            "loyalty": self.loyalty,
            "is_alive": self.is_alive,
            "relationships": self.relationships,
            "reputation": self.reputation,
            "memories": self.memories,
            "known_characters": list(self.known_characters),  # 集合转换为列表以便序列化
            "current_goal": self.current_goal,
            "pending_event": self.pending_event,
        }

    @classmethod
    def from_dict(cls, data: Dict):
        """
        从保存的字典数据重建角色对象
        
        参数:
            data (Dict): 包含角色数据的字典
            
        返回:
            Character: 重建的角色对象
            
        说明:
        - 用于游戏加载功能
        - 从持久化存储中恢复角色状态
        - 提供默认值以确保数据完整性
        """
        # 创建基础角色对象
        character = cls(
            name=data["name"],
            gender=data.get("gender", "unknown"),
            age=data.get("age", 18),
            control_type=data.get("control_type", "AI"),
            role=data.get("role", "traveler"),
            origin=data.get("origin"),
        )
        
        # 恢复所有属性值
        character.id = data.get("id", str(uuid.uuid4()))  # 使用原有ID或生成新ID
        character.attributes = data.get("attributes", character.attributes)
        character.skills = data.get("skills", character.skills)
        character.traits = data.get("traits", [])
        character.mood = data.get("mood", character.mood)
        character.health = data.get("health", 100)
        character.fatigue = data.get("fatigue", 0)
        character.morale = data.get("morale", 100)
        character.loyalty = data.get("loyalty", 50)
        character.is_alive = data.get("is_alive", True)
        character.relationships = data.get("relationships", {})
        character.reputation = data.get("reputation", 0)
        character.memories = data.get("memories", [])
        character.known_characters = set(data.get("known_characters", []))  # 列表转集合
        character.current_goal = data.get("current_goal")
        character.pending_event = data.get("pending_event")
        
        return character
