use code_open_common::{CodeOpenInfo, CodeOpenRequest, SerializedDataContainer};
use once_cell::sync::Lazy;
use std::process::Command;
use std::{collections::HashMap, fs::File};
use std::{io::Read, net::TcpListener};

static THIS_APP_NAME: &str = "code_open_daemon";
static THIS_APP_CONFIG_BASE_PATH: Lazy<String> =
    Lazy::new(|| format!("$XDG_CONFIG_HOME/{}", THIS_APP_NAME));
static TABLE_FILE_NAME: &str = "table.json";

fn get_table_file_path() -> String {
    let table_file_path = format!("{}/{}", *THIS_APP_CONFIG_BASE_PATH, TABLE_FILE_NAME);

    shellexpand::full(&table_file_path).unwrap().to_string()
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
        .unwrap_or_else(HashMap::new)
    /*
    vec![("AlphaKai-ArchLinux", "sofa")]
        .into_iter()
        .map(|(a, b)| (a.to_owned(), b.to_owned()))
        .collect::<HashMap<_, _>>()
        */
}

fn resolve_host_name_to_local_configured_name(code_open_info: CodeOpenInfo) -> CodeOpenInfo {
    match load_local_configured_name_table().get(&code_open_info.remote_host_name) {
        Some(remote_host_name) => CodeOpenInfo::new(
            remote_host_name.clone(),
            code_open_info.remote_dir_full_path,
        ),
        None => code_open_info,
    }
}

fn main() {
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
                            resolve_host_name_to_local_configured_name(code_open_info);
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
