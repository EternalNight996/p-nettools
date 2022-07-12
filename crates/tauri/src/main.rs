#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::Manager;

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
      .setup(|app| {
          Ok({
              // 获取窗口
              let _window = app.get_window("main").unwrap();
              
              // 调试工具
              // window.open_devtools();
              // window.close_devtools();
          })
      })
      .menu(tauri::Menu::os_default(&context.package_info().name))
      .run(context)
      .expect("error while running tauri application");
}
