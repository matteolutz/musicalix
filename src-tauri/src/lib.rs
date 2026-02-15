use libwing::{WingConsole, WingResponse};

use crate::wing::WingConsoleExt;

mod wing;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::async_runtime::spawn_blocking(|| {
        let Ok(mut wing) = WingConsole::connect(None) else {
            return;
        };
        println!("Connected to Wing Console");

        let mut channel_one = wing.input_channel(1);

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
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
