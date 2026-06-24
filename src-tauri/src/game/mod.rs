pub mod components;
pub mod resources;
pub mod systems;
pub mod map;

use bevy::prelude::*;
use rand::Rng;
use serde::Serialize;

use components::*;
use resources::*;

/// 返回给前端的角色数据结构
#[derive(Debug, Clone, Serialize)]
pub struct RoleData {
    pub id: u64,
    pub name: String,
    pub gender: String,
    pub age: u8,
    pub attributes: Attributes,
    pub traits: Vec<TraitInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TraitInfo {
    pub name: String,
    pub description: String,
}

/// 返回给前端的物品数据结构
#[derive(Debug, Clone, Serialize)]
pub struct ItemData {
    pub id: u64,
    pub name: String,
    pub category: String,
    pub unit_price: u32,
    pub quantity: u32,
    pub unit_weight: f32,
}

/// 初始化 Bevy World，注册组件和 Resource
pub fn init_world() -> World {
    let mut world = World::new();

    // 注册组件类型
    world.register_component::<RoleId>();
    world.register_component::<RoleName>();
    world.register_component::<Gender>();
    world.register_component::<Age>();
    world.register_component::<Attributes>();
    world.register_component::<Traits>();
    world.register_component::<ItemId>();
    world.register_component::<ItemName>();
    world.register_component::<ItemCategory>();
    world.register_component::<UnitPrice>();
    world.register_component::<ItemQuantity>();
    world.register_component::<UnitWeight>();

    // 插入 Resource
    world.insert_resource(RoleIdCounter::default());
    world.insert_resource(NamePool::default());
    world.insert_resource(TraitPool::default());
    world.insert_resource(ItemIdCounter::default());
    world.insert_resource(ItemCatalog::default());
    world.insert_resource(MapResource::default());

    world
}

/// 生成一个随机角色，返回序列化数据
pub fn generate_role(world: &mut World) -> RoleData {
    let mut rng = rand::thread_rng();

    // 读取 Resource
    let name_pool = world.resource::<NamePool>();
    let trait_pool = world.resource::<TraitPool>();

    // 随机性别
    let gender = if rng.gen_bool(0.5) {
        Gender::Male
    } else {
        Gender::Female
    };

    // 随机姓名
    let name = match &gender {
        Gender::Male => {
            let idx = rng.gen_range(0..name_pool.male.len());
            name_pool.male[idx].to_string()
        }
        Gender::Female => {
            let idx = rng.gen_range(0..name_pool.female.len());
            name_pool.female[idx].to_string()
        }
    };

    // 随机年龄 18~65
    let age = rng.gen_range(18..=65);

    // 随机属性 3~18
    let attributes = Attributes {
        strength: rng.gen_range(3..=18),
        dexterity: rng.gen_range(3..=18),
        constitution: rng.gen_range(3..=18),
        intelligence: rng.gen_range(3..=18),
        charisma: rng.gen_range(3..=18),
    };

    // 随机选取 3~6 个特质
    let trait_count = rng.gen_range(3..=6);
    let mut indices: Vec<usize> = (0..trait_pool.traits.len()).collect();
    // Fisher-Yates 部分打乱
    for i in (1..indices.len()).rev() {
        let j = rng.gen_range(0..=i);
        indices.swap(i, j);
    }
    let selected_traits: Vec<Trait> = indices[..trait_count]
        .iter()
        .map(|&idx| {
            let t = &trait_pool.traits[idx];
            Trait {
                name: t.name.to_string(),
                description: t.description.to_string(),
            }
        })
        .collect();

    // 获取自增 ID
    let id = world.resource_mut::<RoleIdCounter>().next();

    // 创建 Entity
    world.spawn((
        RoleId(id),
        RoleName(name.clone()),
        match &gender {
            Gender::Male => Gender::Male,
            Gender::Female => Gender::Female,
        },
        Age(age),
        attributes.clone(),
        Traits(selected_traits.clone()),
    ));

    // 构建返回数据
    let gender_str = match gender {
        Gender::Male => "Male".to_string(),
        Gender::Female => "Female".to_string(),
    };

    let trait_infos: Vec<TraitInfo> = selected_traits
        .iter()
        .map(|t| TraitInfo {
            name: t.name.clone(),
            description: t.description.clone(),
        })
        .collect();

    RoleData {
        id,
        name,
        gender: gender_str,
        age,
        attributes,
        traits: trait_infos,
    }
}

/// 启动时批量生成 count 个角色
pub fn seed_roles(world: &mut World, count: usize) -> Vec<RoleData> {
    let mut roles = Vec::with_capacity(count);
    for _ in 0..count {
        roles.push(generate_role(world));
    }
    roles
}

/// 列出所有角色
pub fn list_roles(world: &mut World) -> Vec<RoleData> {
    let mut roles: Vec<RoleData> = Vec::new();

    for (role_id, role_name, gender, age, attrs, traits) in
        world.query::<(&RoleId, &RoleName, &Gender, &Age, &Attributes, &Traits)>().iter(world)
    {
        let gender_str = match gender {
            Gender::Male => "Male".to_string(),
            Gender::Female => "Female".to_string(),
        };

        let trait_infos: Vec<TraitInfo> = traits
            .0
            .iter()
            .map(|t| TraitInfo {
                name: t.name.clone(),
                description: t.description.clone(),
            })
            .collect();

        roles.push(RoleData {
            id: role_id.0,
            name: role_name.0.clone(),
            gender: gender_str,
            age: age.0,
            attributes: attrs.clone(),
            traits: trait_infos,
        });
    }

    roles
}

/// 根据 ID 删除角色
pub fn remove_role(world: &mut World, target_id: u64) -> bool {
    let mut found = false;
    let mut to_despawn = Vec::new();

    for (entity, role_id) in world.query::<(Entity, &RoleId)>().iter(world) {
        if role_id.0 == target_id {
            to_despawn.push(entity);
            found = true;
            break;
        }
    }

    for entity in to_despawn {
        world.despawn(entity);
    }

    found
}

/// 从模板池启动时批量生成物品
pub fn seed_items(world: &mut World) -> Vec<ItemData> {
    let catalog = world.resource::<ItemCatalog>().templates.clone();
    let mut items = Vec::with_capacity(catalog.len());

    for template in &catalog {
        let id = world.resource_mut::<ItemIdCounter>().next();

        let category_str = match template.category {
            ItemCategory::Weapon => "Weapons".to_string(),
            ItemCategory::Armor => "Armor".to_string(),
            ItemCategory::Potion => "Potions".to_string(),
            ItemCategory::Material => "Materials".to_string(),
            ItemCategory::Scroll => "Scrolls".to_string(),
            ItemCategory::Treasure => "Treasure".to_string(),
        };

        let quantity = 1 + (id as u32 % 5); // 1~5 的随机库存

        world.spawn((
            ItemId(id),
            ItemName(template.name.to_string()),
            match template.category {
                ItemCategory::Weapon => ItemCategory::Weapon,
                ItemCategory::Armor => ItemCategory::Armor,
                ItemCategory::Potion => ItemCategory::Potion,
                ItemCategory::Material => ItemCategory::Material,
                ItemCategory::Scroll => ItemCategory::Scroll,
                ItemCategory::Treasure => ItemCategory::Treasure,
            },
            UnitPrice(template.unit_price),
            ItemQuantity(quantity),
            UnitWeight(template.unit_weight),
        ));

        items.push(ItemData {
            id,
            name: template.name.to_string(),
            category: category_str,
            unit_price: template.unit_price,
            quantity,
            unit_weight: template.unit_weight,
        });
    }

    items
}

/// 列出所有物品
pub fn list_items(world: &mut World) -> Vec<ItemData> {
    let mut items: Vec<ItemData> = Vec::new();

    for (item_id, item_name, category, price, qty, weight) in
        world.query::<(&ItemId, &ItemName, &ItemCategory, &UnitPrice, &ItemQuantity, &UnitWeight)>().iter(world)
    {
        let category_str = match category {
            ItemCategory::Weapon => "Weapons".to_string(),
            ItemCategory::Armor => "Armor".to_string(),
            ItemCategory::Potion => "Potions".to_string(),
            ItemCategory::Material => "Materials".to_string(),
            ItemCategory::Scroll => "Scrolls".to_string(),
            ItemCategory::Treasure => "Treasure".to_string(),
        };

        items.push(ItemData {
            id: item_id.0,
            name: item_name.0.clone(),
            category: category_str,
            unit_price: price.0,
            quantity: qty.0,
            unit_weight: weight.0,
        });
    }

    items
}

// ── 地图相关 ──────────────────────────────────────

/// Bevy Resource 包装地图数据
#[derive(Resource)]
pub struct MapResource(pub Option<map::MapData>);

impl Default for MapResource {
    fn default() -> Self {
        Self(None)
    }
}

/// 生成地图并存入 World Resource
pub fn store_map(world: &mut World, seed: u32) -> map::MapData {
    let data = map::generate_map(seed);
    world.insert_resource(MapResource(Some(data.clone())));
    data
}
