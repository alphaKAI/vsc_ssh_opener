use code_open_common::{CodeOpenInfo, CodeOpenRequest, SerializedDataContainer};
use once_cell::sync::Lazy;
use std::{collections::HashMap, fs::File};
use std::{io::Read, net::TcpListener};
use std::{io::Write, process::Command};

static THIS_APP_NAME: &str = "code-open-server";
static THIS_APP_CONFIG_BASE_PATH: Lazy<String> =
    Lazy::new(|| shellexpand::full(&format!("$XDG_CONFIG_HOME/{}", THIS_APP_NAME)).unwrap().to_string());
static TABLE_FILE_NAME: &str = "table.json";

fn get_table_file_path() -> String {
    format!("{}/{}", *THIS_APP_CONFIG_BASE_PATH, TABLE_FILE_NAME)
}

fn open_vscode_in_other_process(code_open_info: CodeOpenInfo) {
    Command::new("code")
        .arg("--remote")
        .arg(format!("ssh-remote+{}", code_open_info.remote_host_name))
        .arg(code_open_info.remote_dir_full_path)
        .spawn()
        .expect("Failed to exec VSCode");
}

fn load_local_configured_name_table() -> HashMap<String, String> {
    File::open(get_table_file_path())
        .ok()
        .and_then(|mut f| {
            let mut buf = String::new();
            f.read_to_string(&mut buf).ok()?;
            serde_json::from_str(&buf).ok()
        })
        .unwrap_or_else(|| {
            let ret = HashMap::new();

            let serialized = serde_json::to_string(&ret)
                .expect("Failed to serialize empty hashmap with serde_json");

            std::fs::create_dir_all(&*THIS_APP_CONFIG_BASE_PATH).expect("failed to create directory for config files");

            let mut f = File::create(get_table_file_path())
                .unwrap_or_else(|_| panic!("Failed to create {}", get_table_file_path()));

            f.write_all(serialized.as_bytes())
                .expect("failed to write serialized bytes");

            println!("There is no table file, so created empty table file at {}", get_table_file_path());

            ret
        })
}

fn resolve_host_name_to_local_configured_name(code_open_info: CodeOpenInfo, table: &HashMap<String, String>) -> CodeOpenInfo {
    match table.get(&code_open_info.remote_host_name) {
        Some(remote_host_name) => CodeOpenInfo::new(
            remote_host_name.clone(),
            code_open_info.remote_dir_full_path,
        ),
        None => code_open_info,
    }
}

fn main() {
    let table = load_local_configured_name_table();
    println!("Actual host name to locally configured host name in .ssh/config table:");
    for (k, v) in table.iter() {
        println!("* {} -> {}", k, v);
    }


    let listener = TcpListener::bind(("0.0.0.0", 3000)).unwrap();
    println!("Server is started!");

    for stream in listener.incoming() {
        println!("{:?}", stream);
        match stream {
            Ok(mut stream) => {
                let sdc = SerializedDataContainer::from_reader(&mut stream)
                    .expect("Failed to receive SDC from a client");
                let code_open_req = sdc
                    .to_serializable_data::<CodeOpenRequest>()
                    .expect("Failed to deserialize received data to CodeOpenRequest");

                match code_open_req {
                    CodeOpenRequest::Open(code_open_info) => {
                        let code_open_info =
                            resolve_host_name_to_local_configured_name(code_open_info, &table);
                        println!("Open VSCode! {:?}", code_open_info);
                        open_vscode_in_other_process(code_open_info)
                    }
                }
            }
            Err(_) => {
                panic!("Connection failed")
            }
        }
    }
}
