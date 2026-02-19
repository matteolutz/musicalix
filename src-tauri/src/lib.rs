use std::{path::PathBuf, sync::Arc};

use libwing::WingConsole;
use tauri::{
    async_runtime::RwLock,
    menu::{MenuBuilder, SubmenuBuilder},
    Manager, State,
};

use crate::{
    mix::{add_actor, add_group, get_wing_channel_info, import_actors, ActorEvent, GroupEvent},
    show::{
        add_cue, get_show, goto_cue, open_show, save_show, save_show_as, Show, ShowEvent,
        ShowState, ShowStateEvent,
    },
    wing::Wing,
};

mod cue;
mod mix;
mod show;
mod utils;
mod wing;

pub type MutableState<'a, T> = State<'a, Arc<RwLock<T>>>;

struct AppData {
    show: Show,
    current_show_file_path: Option<PathBuf>,

    show_state: ShowState,

    console: Option<Wing>,
}

impl AppData {
    fn new(allow_unconnected: bool) -> Result<Self, String> {
        let wing_res = WingConsole::connect(None);

        let wing = match wing_res {
            Ok(wing) => Some(wing),
            Err(err) => {
                if allow_unconnected {
                    println!("Failed to connect to Wing Console: {}", err);
                    None
                } else {
                    return Err(format!("Failed to connect to Wing Console: {}", err));
                }
            }
        };

        let wing = wing.map(|wing| wing.into());

        Ok(Self {
            show: Show::default(),
            current_show_file_path: None,
            show_state: ShowState::default(),
            console: wing,
        })
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri_specta::Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(tauri_specta::collect_commands![
            get_show,
            get_wing_channel_info,
            add_actor,
            import_actors,
            add_group,
            add_cue,
            goto_cue
        ])
        .events(tauri_specta::collect_events![
            ShowEvent,
            ShowStateEvent,
            ActorEvent,
            GroupEvent
        ]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);

            let app_data = AppData::new(cfg!(debug_assertions)).unwrap();

            if let Some(wing) = app_data.console.as_ref().cloned() {
                tauri::async_runtime::spawn_blocking(move || wing.handle_incoming_loop());
            }

            app.manage(Arc::new(RwLock::new(app_data)));

            let file_menu = SubmenuBuilder::new(app, "File")
                .text("save", "Save")
                .text("save-as", "Save As")
                .text("open", "Open")
                .separator()
                .text("quit", "Quit")
                .build()?;

            let menu = MenuBuilder::new(app).item(&file_menu).build()?;
            app.set_menu(menu.clone())?;

            app.on_menu_event(|handle, event| match event.id().0.as_str() {
                "save" => {
                    let handle = handle.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = save_show(handle)
                            .await
                            .inspect_err(|err| println!("Failed to save: {}", err));
                    });
                }
                "save-as" => {
                    let handle = handle.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = save_show_as(handle)
                            .await
                            .inspect_err(|err| println!("Failed to save as: {}", err));
                    });
                }
                "open" => {
                    let handle = handle.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = open_show(handle)
                            .await
                            .inspect_err(|err| println!("Failed to open: {}", err));
                    });
                }
                "quit" => handle.exit(0),
                _ => {}
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
