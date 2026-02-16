use std::{path::PathBuf, sync::Arc};

use libwing::WingConsole;
use tauri::{
    async_runtime::RwLock,
    menu::{MenuBuilder, SubmenuBuilder},
    Manager, State,
};

use crate::{
    mix::{get_wing_channel_info, ActorEvent},
    show::{get_show, open_show, open_showfile, save_show, save_show_as, Show, ShowEvent},
};

mod cue;
mod mix;
mod show;
mod wing;

pub type MutableState<'a, T> = State<'a, Arc<RwLock<T>>>;

struct AppData {
    show: Show,
    current_show_file_path: Option<PathBuf>,

    console: Option<WingConsole>,
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

        Ok(Self {
            show: Show::default(),
            current_show_file_path: None,
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
            open_showfile,
            get_wing_channel_info
        ])
        .events(tauri_specta::collect_events![ShowEvent, ActorEvent,]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    /*
    tauri::async_runtime::spawn_blocking(|| {
        let Ok(mut wing) = WingConsole::connect(None) else {
            return;
        };
        println!("Connected to Wing Console");

        let mut channel_one = wing.channel(1.try_into().unwrap());

        let tags = channel_one.get_tags().unwrap();
        println!("channel 1 tags: {}", tags);

        let mut dca_one = wing.dca(1.try_into().unwrap());
        dca_one.set_name("Actor 1").unwrap();
        dca_one.set_color(wing::WingColor::Red).unwrap();

        loop {
            if let Ok(WingResponse::NodeData(id, data)) = wing.read() {
                match WingConsole::id_to_defs(id) {
                    None => println!("<Unknown:{}> = {}", id, data.get_string()),
                    Some(defs) if defs.is_empty() => {
                        println!("<Unknown:{}> = {}", id, data.get_string())
                    }
                    Some(defs) if defs.len() == 1 => {
                        println!("{} = {}", defs[0].0, data.get_string());
                    }
                    Some(defs) if (defs.len() > 1) => {
                        let u = std::collections::HashSet::<u16>::from_iter(
                            defs.iter().map(|(_, def)| def.index),
                        );
                        if u.len() == 1 {
                            // let propname = String::from("/") + &defs[0].0.split("/").enumerate().filter(|(i, _)| *i < defs.len()-1).map(|(_, n)| n).collect::<Vec<_>>().join("/") +
                            let propname =
                                String::from("prop") + defs[0].1.index.to_string().as_str();
                            println!("{} = {} (check out propmap.jsonl for more info on property with id {})", propname, data.get_string(), id);
                        } else {
                            println!("<MultiProp:{}> = {} (check out propmap.jsonl for more info on property id {})", id, data.get_string(), id);
                        }
                    }
                    Some(_) => {}
                }
            }
        }
    });*/

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);

            let app_data = AppData::new(cfg!(debug_assertions)).unwrap();
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
