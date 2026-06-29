mod game;

use std::sync::RwLock;
use tauri::Manager;

/// Tauri 命令：生成一个新角色
#[tauri::command]
fn generate_role(world: tauri::State<'_, RwLock<bevy_ecs::world::World>>) -> Result<game::RoleData, String> {
    let mut world = world.write().map_err(|e| e.to_string())?;
    Ok(game::generate_role(&mut world))
}

/// Tauri 命令：列出所有角色
#[tauri::command]
fn list_roles(world: tauri::State<'_, RwLock<bevy_ecs::world::World>>) -> Result<Vec<game::RoleData>, String> {
    let mut world = world.write().map_err(|e| e.to_string())?;
    Ok(game::list_roles(&mut world))
}

/// Tauri 命令：删除指定角色
#[tauri::command]
fn remove_role(id: u64, world: tauri::State<'_, RwLock<bevy_ecs::world::World>>) -> Result<bool, String> {
    let mut world = world.write().map_err(|e| e.to_string())?;
    Ok(game::remove_role(&mut world, id))
}

/// Tauri 命令：列出所有物品
#[tauri::command]
fn list_items(world: tauri::State<'_, RwLock<bevy_ecs::world::World>>) -> Result<Vec<game::ItemData>, String> {
    let mut world = world.write().map_err(|e| e.to_string())?;
    Ok(game::list_items(&mut world))
}

/// Tauri 命令：获取地图数据（首次调用时自动生成）
#[tauri::command]
fn get_map(world: tauri::State<'_, RwLock<bevy_ecs::world::World>>) -> Result<game::map::MapData, String> {
    let mut world = world.write().map_err(|e| e.to_string())?;

    // 如果已有地图则直接返回
    if let Some(data) = &world.resource::<game::MapResource>().0 {
        return Ok(data.clone());
    }

    // 首次调用时生成
    let data = game::store_map(&mut world, 0);
    log::info!("Map generated on first request");
    Ok(data)
}

/// Tauri 命令：重新生成地图（强制使用新随机种子）
#[tauri::command]
fn regenerate_map(world: tauri::State<'_, RwLock<bevy_ecs::world::World>>) -> Result<game::map::MapData, String> {
    let mut world = world.write().map_err(|e| e.to_string())?;
    let data = game::store_map(&mut world, 0);
    log::info!("Map regenerated with new seed");
    Ok(data)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .manage(RwLock::new(game::init_world()))
    .invoke_handler(tauri::generate_handler![generate_role, list_roles, remove_role, list_items, get_map, regenerate_map])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // 启动时自动生成 10 个角色
      let world = app.state::<RwLock<bevy_ecs::world::World>>();
      if let Ok(mut world) = world.write() {
        let count = game::seed_roles(&mut world, 10);
        log::info!("Seeded {} roles on startup", count.len());

        let items = game::seed_items(&mut world);
        log::info!("Seeded {} items on startup", items.len());
      }

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
