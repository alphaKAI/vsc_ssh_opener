use clap::{App, Arg};
use code_open_common::*;
use path_absolutize::*;
use std::io::Write;
use std::{env, net::TcpStream, path::PathBuf};

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
    let ssh_flag = env::vars().any(|(k, _)| k == "SSH_CONNECTION");
    let mut code_open_config = CodeOpenConfig::default();

    let default_port_str = &DEFAULT_PORT.to_string();

    let app = App::new("code-open")
        .version("0.1.0")
        .author("Akihiro Shoji <alpha.kai.net@alpha-kai-net.info>")
        .about("open VSCode over SSH")
        .arg(
            Arg::with_name("ip")
                .required(false)
                .short("i")
                .long("ip")
                .takes_value(true)
                .default_value(DEFAULT_IP)
                .help("ip address of the server to be connected"),
        )
        .arg(
            Arg::with_name("port")
                .required(false)
                .short("p")
                .long("port")
                .takes_value(true)
                .default_value(default_port_str)
                .help("port number of the server to be connected"),
        )
        .arg(Arg::with_name("path").required(false));

    let matches = app.get_matches();

    if let Some(ip) = matches.value_of("ip") {
        code_open_config.set_ip(ip.to_owned());
    }

    if let Some(port) = matches.value_of("port") {
        code_open_config.set_port(port.parse().expect("failed to parse given port number"));
    }

    if !ssh_flag {
        println!("Error this command should be executed in SSH");
        return;
    }

    let path = matches.value_of("path").map_or_else(
        || env::current_dir().unwrap(),
        |path| {
            PathBuf::from(
                shellexpand::full(path)
                    .expect("failed to tilde expad")
                    .to_string(),
            )
        },
    );

    let code_open_info = CodeOpenInfo::new(
        gethostname::gethostname()
            .to_str()
            .expect("Failed: to_str")
            .to_owned(),
        path.absolutize()
            .expect("Failed to absolutize")
            .to_str()
            .expect("Failed: to_str")
            .to_owned(),
    );

    let code_open_req = CodeOpenRequest::Open(code_open_info);

    send_request_to_server(&code_open_config, code_open_req);
}
