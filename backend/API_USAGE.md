# 游戏初始化系统 API 使用指南

## 概述

本系统实现了游戏初始化功能，可以在游戏开始时自动生成：

- 1个玩家角色（职业：旅行者）
- 2个NPC队友（职业：守卫和商人）
- 组成一个由玩家控制的队伍

## API 端点

### 1. 初始化游戏

**POST** `/api/game/init`

创建新游戏，生成玩家角色和队伍。

**请求体：**

```json
{
  "player_name": "旅行者",
  "team_name": "漂流者",
  "starting_location": "Capital"
}
```

**响应示例：**

```json
{
  "is_initialized": true,
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
    "traveling": false
  }
}
```

### 2. 获取游戏状态

**GET** `/api/game/state`

获取当前游戏状态信息。

**响应示例：**

```json
{
  "is_initialized": true,
  "current_location": "Capital",
  "elapsed_hours": 0.0,
  "current_day": 1,
  "current_hour": 0.0,
  "player_team": { ... }
}
```

### 3. 重置游戏

**POST** `/api/game/reset`

重置游戏状态，清空所有数据。

**响应示例：**

```json
{
  "status": "ok",
  "message": "游戏已重置"
}
```

### 4. 获取队伍信息

**GET** `/api/team`

获取玩家队伍的详细信息。

**响应示例：**

```json
{
  "name": "漂流者",
  "leader": "旅行者",
  "leader_id": "uuid-here",
  "is_player_controlled": true,
  "members": [
    {
      "id": "uuid-1",
      "name": "旅行者",
      "control_type": "player",
      "role": "traveler",
      "health": 100,
      "morale": 100,
      ...
    },
    {
      "id": "uuid-2",
      "name": "守卫",
      "control_type": "AI",
      "role": "guard",
      ...
    },
    {
      "id": "uuid-3",
      "name": "商人",
      "control_type": "AI",
      "role": "merchant",
      ...
    }
  ],
  "member_count": 3,
  "gold": 100,
  "food": 50,
  "morale": 100.0,
  "location": "Capital",
  "traveling": false
}
```

### 5. 获取队伍成员列表

**GET** `/api/team/members`

获取队伍所有成员的列表。

**响应示例：**

```json
{
  "characters": [
    {
      "id": "uuid-1",
      "name": "旅行者",
      "control_type": "player",
      "role": "traveler",
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
      "health": 100,
      "morale": 100,
      ...
    },
    ...
  ],
  "total": 3
}
```

### 6. 获取指定队员信息

**GET** `/api/team/members/{member_id}`

获取指定ID的队员详细信息。

**路径参数：**

- `member_id`: 队员的UUID

### 7. 获取队伍统计

**GET** `/api/team/stats`

获取队伍的统计信息。

**响应示例：**

```json
{
  "total_health": 300,
  "average_health": 100.0,
  "total_morale": 300,
  "average_morale": 100.0,
  "total_fatigue": 0,
  "average_fatigue": 0.0,
  "alive_count": 3,
  "dead_count": 0
}
```

### 8. 队伍休息

**POST** `/api/team/rest?days=1`

让队伍休息指定天数，恢复疲劳和士气。

**查询参数：**

- `days`: 休息天数（默认为1）

**响应示例：**

```json
{
  "status": "ok",
  "message": "队伍休息了 1 天",
  "current_day": 2,
  "current_hour": 0.0,
  "team_morale": 100.0
}
```

## 使用流程

### 典型的游戏开始流程

1. **初始化游戏**

   ```bash
   POST /api/game/init
   {
     "player_name": "我的角色",
     "team_name": "我的队伍",
     "starting_location": "Capital"
   }
   ```

2. **获取队伍信息**

   ```bash
   GET /api/team
   ```

3. **查看队伍成员**

   ```bash
   GET /api/team/members
   ```

4. **开始游戏**
   - 使用现有的 `/api/player/move` 进行移动
   - 使用 `/api/team/rest` 进行休息
   - 后续可以添加更多玩法功能

## 数据结构

### 角色属性

- **strength**: 力量 (3-8)
- **intelligence**: 智力 (3-8)
- **charisma**: 魅力 (3-8)
- **endurance**: 耐力 (3-8)
- **perception**: 感知 (3-8)

### 角色技能

- **trading**: 交易 (0-10)
- **combat**: 战斗 (0-10)
- **medicine**: 医疗 (0-10)
- **navigation**: 导航 (0-10)
- **negotiation**: 谈判 (0-10)

### 角色职业

- **traveler**: 旅行者（玩家默认职业）
- **guard**: 守卫（高战斗力）
- **merchant**: 商人（高交易能力）
- **scout**: 侦察兵（高导航能力）
- **healer**: 治疗师（高医疗能力）

### 性格特质

- **brave**: 勇敢
- **cautious**: 谨慎
- **greedy**: 贪婪
- **loyal**: 忠诚
- **stoic**: 坚忍
- **curious**: 好奇
- **coward**: 懦弱
- **charismatic**: 魅力

## 注意事项

1. **游戏必须先初始化**：在调用其他队伍相关API之前，必须先调用 `/api/game/init`
2. **内存存储**：当前使用内存存储，服务器重启后数据会丢失
3. **向后兼容**：保留了原有的 `/api/player` 和 `/api/player/move` 接口
4. **时间系统**：游戏时间会随着移动和休息而推进

## 测试示例

使用 curl 测试：

```bash
# 初始化游戏
curl -X POST http://localhost:8000/api/game/init \
  -H "Content-Type: application/json" \
  -d '{"player_name":"测试玩家","team_name":"测试队伍"}'

# 获取队伍信息
curl http://localhost:8000/api/team

# 获取队伍成员
curl http://localhost:8000/api/team/members

# 队伍休息
curl -X POST "http://localhost:8000/api/team/rest?days=1"
```

使用 PowerShell 测试：

```powershell
# 初始化游戏
Invoke-RestMethod -Uri "http://localhost:8000/api/game/init" `
  -Method POST `
  -ContentType "application/json" `
  -Body '{"player_name":"测试玩家","team_name":"测试队伍"}'

# 获取队伍信息
Invoke-RestMethod -Uri "http://localhost:8000/api/team" -Method GET

# 获取队伍成员
Invoke-RestMethod -Uri "http://localhost:8000/api/team/members" -Method GET
```
