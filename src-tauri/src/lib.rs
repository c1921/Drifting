mod game;

use std::sync::RwLock;
use tauri::Manager;

/// Tauri 命令：生成一个新角色
#[tauri::command]
fn generate_role(world: tauri::State<'_, RwLock<bevy::prelude::World>>) -> Result<game::RoleData, String> {
    let mut world = world.write().map_err(|e| e.to_string())?;
    Ok(game::generate_role(&mut world))
}

/// Tauri 命令：列出所有角色
#[tauri::command]
fn list_roles(world: tauri::State<'_, RwLock<bevy::prelude::World>>) -> Result<Vec<game::RoleData>, String> {
    let mut world = world.write().map_err(|e| e.to_string())?;
    Ok(game::list_roles(&mut world))
}

/// Tauri 命令：删除指定角色
#[tauri::command]
fn remove_role(id: u64, world: tauri::State<'_, RwLock<bevy::prelude::World>>) -> Result<bool, String> {
    let mut world = world.write().map_err(|e| e.to_string())?;
    Ok(game::remove_role(&mut world, id))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .manage(RwLock::new(game::init_world()))
    .invoke_handler(tauri::generate_handler![generate_role, list_roles, remove_role])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // 启动时自动生成 10 个角色
      let world = app.state::<RwLock<bevy::prelude::World>>();
      if let Ok(mut world) = world.write() {
        let count = game::seed_roles(&mut world, 10);
        log::info!("Seeded {} roles on startup", count.len());
      }

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
