mod search;

use search::{search_apps, SearchResult};
use tauri::Manager;

#[tauri::command]
fn search(query: &str) -> Vec<SearchResult> {
    search_apps(query)
}

#[tauri::command]
fn launch(path: &str) -> Result<(), String> {
    std::process::Command::new("cmd")
        .args(["/C", "start", "", path])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // Setup system tray
            use tauri::tray::{TrayIconBuilder, TrayIconEvent};
            use tauri::menu::{MenuBuilder, MenuItemBuilder};
            use tauri::image::Image;

            let show = MenuItemBuilder::with_id("show", "Show Powerlight").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let menu = MenuBuilder::new(app)
                .items(&[&show, &quit])
                .build()?;

            // Load custom icon
            let icon_bytes = include_bytes!("../icons/PowerLight.png");
            let img = image::load_from_memory(icon_bytes)
                .map_err(|e| format!("Failed to load icon: {}", e))?
                .to_rgba8();
            let (width, height) = img.dimensions();
            let icon = Image::new_owned(img.into_raw(), width, height);

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .icon(icon)
                .on_menu_event({
                    let window = window.clone();
                    move |app, event| {
                        match event.id.as_ref() {
                            "show" => {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                            "quit" => {
                                app.exit(0);
                            }
                            _ => {}
                        }
                    }
                })
                .on_tray_icon_event(|_tray, event| {
                    if let TrayIconEvent::Click { .. } = event {
                        // Optional: handle tray icon click
                    }
                })
                .build(app)?;

            // Register global shortcut - try multiple options
            use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutEvent};

            let shortcuts_to_try = [
                "Ctrl+Alt+Space",
                "Ctrl+`",
                "Alt+Q",
                "Ctrl+Alt+K",
            ];

            let mut registered = false;
            for shortcut_str in shortcuts_to_try {
                if let Ok(shortcut) = shortcut_str.parse::<Shortcut>() {
                    let window_clone = window.clone();

                    let result = app.global_shortcut().on_shortcut(shortcut.clone(), move |_app, _shortcut, event| {
                        // Only toggle on key down, ignore key up
                        if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                            if window_clone.is_visible().unwrap_or(false) {
                                let _ = window_clone.hide();
                            } else {
                                let _ = window_clone.show();
                                let _ = window_clone.set_focus();
                            }
                        }
                    });

                    if result.is_ok() && app.global_shortcut().register(shortcut.clone()).is_ok() {
                        println!("Registered global shortcut: {}", shortcut_str);
                        registered = true;
                        break;
                    }
                }
            }

            if !registered {
                println!("Warning: Could not register any global shortcut. You can manually show the window.");
            }

            // Initialize search index in background
            std::thread::spawn(|| {
                search::init_index();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![search, launch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
