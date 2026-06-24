use bevy::prelude::Component;
use serde::Serialize;

/// 角色 ID
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct RoleId(pub u64);

/// 角色姓名
#[derive(Component, Debug, Clone, Serialize)]
pub struct RoleName(pub String);

/// 性别
#[derive(Component, Debug, Clone, Serialize)]
pub enum Gender {
    Male,
    Female,
}

/// 年龄
#[derive(Component, Debug, Clone, Serialize)]
pub struct Age(pub u8);

/// 五项基本属性
#[derive(Component, Debug, Clone, Serialize)]
pub struct Attributes {
    pub strength: u8,     // STR 力量
    pub dexterity: u8,    // DEX 敏捷
    pub constitution: u8, // CON 体质
    pub intelligence: u8, // INT 智力
    pub charisma: u8,     // CHA 魅力
}

/// 单个特质
#[derive(Debug, Clone, Serialize)]
pub struct Trait {
    pub name: String,
    pub description: String,
}

/// 角色特质列表
#[derive(Component, Debug, Clone, Serialize)]
pub struct Traits(pub Vec<Trait>);

// ── 物品相关组件 ────────────────────────────────

/// 物品 ID
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct ItemId(pub u64);

/// 物品名称
#[derive(Component, Debug, Clone, Serialize)]
pub struct ItemName(pub String);

/// 物品种类
#[derive(Component, Debug, Clone, Serialize)]
pub enum ItemCategory {
    Weapon,
    Armor,
    Potion,
    Material,
    Scroll,
    Treasure,
}

/// 物品单价 (金币)
#[derive(Component, Debug, Clone, Serialize)]
pub struct UnitPrice(pub u32);

/// 物品库存数量
#[derive(Component, Debug, Clone, Serialize)]
pub struct ItemQuantity(pub u32);

/// 物品单位重量
#[derive(Component, Debug, Clone, Serialize)]
pub struct UnitWeight(pub f32);
