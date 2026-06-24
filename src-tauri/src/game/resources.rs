use bevy::prelude::Resource;

/// 自增角色 ID 计数器
#[derive(Resource, Debug)]
pub struct RoleIdCounter(pub u64);

impl RoleIdCounter {
    pub fn next(&mut self) -> u64 {
        let id = self.0;
        self.0 += 1;
        id
    }
}

impl Default for RoleIdCounter {
    fn default() -> Self {
        Self(1)
    }
}

/// 名称池
#[derive(Resource, Debug)]
pub struct NamePool {
    pub male: Vec<&'static str>,
    pub female: Vec<&'static str>,
}

impl Default for NamePool {
    fn default() -> Self {
        Self {
            male: vec![
                "Aldric", "Borin", "Cedric", "Darian", "Eldrin",
                "Finnian", "Gareth", "Holden", "Ivor", "Jorik",
                "Kaelen", "Lian", "Maren", "Niall", "Orin",
                "Peregrin", "Quillan", "Ronan", "Soren", "Theron",
            ],
            female: vec![
                "Aria", "Brienne", "Celeste", "Dahlia", "Elara",
                "Fiona", "Gwendolyn", "Haven", "Ivy", "Juniper",
                "Kaela", "Lilith", "Marin", "Nyx", "Oriana",
                "Pearl", "Quinn", "Rowan", "Seraphina", "Tessa",
            ],
        }
    }
}

/// 特质池
#[derive(Resource, Debug)]
pub struct TraitPool {
    pub traits: Vec<TraitDef>,
}

#[derive(Debug, Clone)]
pub struct TraitDef {
    pub name: &'static str,
    pub description: &'static str,
}

impl Default for TraitPool {
    fn default() -> Self {
        Self {
            traits: vec![
                TraitDef { name: "Tenacity", description: "Unwavering willpower in the face of adversity" },
                TraitDef { name: "Alertness", description: "Keen perception; difficult to catch off guard" },
                TraitDef { name: "Leadership", description: "Born leader who inspires allies" },
                TraitDef { name: "Luck", description: "Fortune favors the bold — lucky breaks come often" },
                TraitDef { name: "Iron Stomach", description: "Extreme resistance to toxins and harmful substances" },
                TraitDef { name: "Night Vision", description: "Can see clearly even in total darkness" },
                TraitDef { name: "Fleet-Footed", description: "Moves significantly faster than an ordinary person" },
                TraitDef { name: "Quick Hands", description: "Nimble fingers, skilled at delicate tasks" },
                TraitDef { name: "Hardy", description: "Exceptional physique and remarkable recovery" },
                TraitDef { name: "Keen Mind", description: "Sharp intellect and excellent memory" },
                TraitDef { name: "Silver Tongue", description: "Gifted with persuasive and eloquent speech" },
                TraitDef { name: "Shadow Walker", description: "Natural talent for moving unseen and unheard" },
                TraitDef { name: "Survivalist", description: "Thrives in the wilderness; knows how to live off the land" },
                TraitDef { name: "Arcane Affinity", description: "Innate connection to magical forces" },
                TraitDef { name: "Berserker Rage", description: "When wounded, fights with savage ferocity" },
            ],
        }
    }
}

// ── 物品相关 Resource ────────────────────────────

/// 自增物品 ID 计数器
#[derive(Resource, Debug)]
pub struct ItemIdCounter(pub u64);

impl ItemIdCounter {
    pub fn next(&mut self) -> u64 {
        let id = self.0;
        self.0 += 1;
        id
    }
}

impl Default for ItemIdCounter {
    fn default() -> Self {
        Self(1)
    }
}

/// 物品模板定义
#[derive(Debug, Clone)]
pub struct ItemTemplate {
    pub name: &'static str,
    pub category: crate::game::components::ItemCategory,
    pub unit_price: u32,
    pub unit_weight: f32,
}

/// 物品模板池（启动时从这些模板生成物品）
#[derive(Resource, Debug)]
pub struct ItemCatalog {
    pub templates: Vec<ItemTemplate>,
}

impl Default for ItemCatalog {
    fn default() -> Self {
        Self {
            templates: vec![
                ItemTemplate { name: "Iron Longsword",  category: crate::game::components::ItemCategory::Weapon,   unit_price: 120, unit_weight: 3.5 },
                ItemTemplate { name: "Elven Bow",       category: crate::game::components::ItemCategory::Weapon,   unit_price: 250, unit_weight: 2.0 },
                ItemTemplate { name: "Steel Dagger",    category: crate::game::components::ItemCategory::Weapon,   unit_price: 60,  unit_weight: 0.8 },
                ItemTemplate { name: "War Hammer",       category: crate::game::components::ItemCategory::Weapon,   unit_price: 200, unit_weight: 6.0 },
                ItemTemplate { name: "Leather Armor",   category: crate::game::components::ItemCategory::Armor,    unit_price: 80,  unit_weight: 5.0 },
                ItemTemplate { name: "Steel Shield",     category: crate::game::components::ItemCategory::Armor,    unit_price: 180, unit_weight: 4.5 },
                ItemTemplate { name: "Chainmail",        category: crate::game::components::ItemCategory::Armor,    unit_price: 350, unit_weight: 8.0 },
                ItemTemplate { name: "Health Potion",    category: crate::game::components::ItemCategory::Potion,   unit_price: 25,  unit_weight: 0.3 },
                ItemTemplate { name: "Antidote",         category: crate::game::components::ItemCategory::Potion,   unit_price: 40,  unit_weight: 0.2 },
                ItemTemplate { name: "Stamina Elixir",   category: crate::game::components::ItemCategory::Potion,   unit_price: 60,  unit_weight: 0.3 },
                ItemTemplate { name: "Moonstone",        category: crate::game::components::ItemCategory::Material, unit_price: 150, unit_weight: 0.8 },
                ItemTemplate { name: "Mana Crystal",     category: crate::game::components::ItemCategory::Material, unit_price: 300, unit_weight: 0.5 },
                ItemTemplate { name: "Iron Ore",         category: crate::game::components::ItemCategory::Material, unit_price: 30,  unit_weight: 2.0 },
                ItemTemplate { name: "Fireball Scroll",  category: crate::game::components::ItemCategory::Scroll,   unit_price: 200, unit_weight: 0.1 },
                ItemTemplate { name: "Teleport Scroll",  category: crate::game::components::ItemCategory::Scroll,   unit_price: 350, unit_weight: 0.1 },
                ItemTemplate { name: "Gold Coins (bag)", category: crate::game::components::ItemCategory::Treasure, unit_price: 500, unit_weight: 1.2 },
                ItemTemplate { name: "Ruby Pendant",     category: crate::game::components::ItemCategory::Treasure, unit_price: 800, unit_weight: 0.3 },
            ],
        }
    }
}
