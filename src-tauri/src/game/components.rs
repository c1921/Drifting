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
