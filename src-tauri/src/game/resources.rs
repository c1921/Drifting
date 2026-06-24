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
