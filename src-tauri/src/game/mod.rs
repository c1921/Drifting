pub mod components;
pub mod resources;
pub mod systems;

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

/// 初始化 Bevy World，注册组件和 Resource
pub fn init_world() -> World {
    let mut world = World::new();

    // 注册组件类型（bevy 0.19 需要显式注册以便 reflect）
    world.register_component::<RoleId>();
    world.register_component::<RoleName>();
    world.register_component::<Gender>();
    world.register_component::<Age>();
    world.register_component::<Attributes>();
    world.register_component::<Traits>();

    // 插入 Resource
    world.insert_resource(RoleIdCounter::default());
    world.insert_resource(NamePool::default());
    world.insert_resource(TraitPool::default());

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
