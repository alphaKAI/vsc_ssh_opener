use clap::Parser;
use code_open_common::*;
use path_absolutize::*;
use std::io::Write;
use std::path::MAIN_SEPARATOR;
use std::{env, net::TcpStream, path::PathBuf};

/// open VSCode over SSH
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// ip address of the server to be connected
    #[clap(short, long, value_parser, default_value_t = DEFAULT_IP.to_string())]
    ip: String,

    /// port number of the server to be connected
    #[clap(short, long, value_parser, default_value_t = DEFAULT_PORT)]
    port: u16,

    #[clap(value_parser)]
    path: Option<String>,
}

fn send_request_to_server(code_open_config: &CodeOpenConfig, code_open_req: CodeOpenRequest) {
    let mut connection = TcpStream::connect((code_open_config.ip.as_str(), code_open_config.port))
        .unwrap_or_else(|_| {
            panic!(
                "Failed to establish a connection to the server -> {}:{}",
                code_open_config.ip, code_open_config.port
            )
        });

    let sdc = SerializedDataContainer::from_serializable_data(&code_open_req).unwrap();
    let sdc_one_vec = sdc.to_one_vec();
    connection
        .write_all(&sdc_one_vec)
        .expect("Failed to write CodeOpenInfo via TCP Connection");

    println!("Sent a request!");
    println!(
        "Target host: {}:{}",
        code_open_config.ip, code_open_config.port
    );
    println!("CodeOpenRequest: {:?}", code_open_req);
}

fn main() {
    let args = Args::parse();

    let code_open_config = CodeOpenConfig {
        ip: args.ip,
        port: args.port,
    };

    if env::var("SSH_CONNECTION").is_err() {
        println!("Error this command should be executed in SSH");
        return;
    }

    let path = args.path.map_or_else(
        || env::current_dir().unwrap(),
        |path| {
            PathBuf::from(
                shellexpand::full(&path)
                    .expect("failed to tilde expad")
                    .to_string(),
            )
        },
    );

    let remote_host_name = gethostname::gethostname()
        .to_str()
        .expect("Failed: to_str")
        .to_owned();
    let mut remote_dir_full_path = path
        .absolutize()
        .expect("Failed to absolutize")
        .to_str()
        .expect("Failed: to_str")
        .to_owned();
    if path.is_dir() {
        remote_dir_full_path.push(MAIN_SEPARATOR);
    }
    let code_open_info = CodeOpenInfo::new(remote_host_name, remote_dir_full_path);

    let code_open_req = CodeOpenRequest::Open(code_open_info);

    send_request_to_server(&code_open_config, code_open_req);
}
