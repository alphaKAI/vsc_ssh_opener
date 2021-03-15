use code_open_common::{CodeOpenInfo, CodeOpenRequest, SerializedDataContainer};
use path_absolutize::*;
use std::io::Write;
use std::{env, net::TcpStream, path::PathBuf};

static TARGET_IP: &str = "0.0.0.0";
static TARGET_PORT: u16 = 3000;

fn send_request_to_server(code_open_req: CodeOpenRequest) {
    let mut connection = TcpStream::connect((TARGET_IP, TARGET_PORT)).unwrap_or_else(|_| {
        panic!(
            "Failed to establish a connection to the server -> {}:{}",
            TARGET_IP, TARGET_PORT
        )
    });

    let sdc = SerializedDataContainer::from_serializable_data(&code_open_req).unwrap();
    let sdc_one_vec = sdc.to_one_vec();
    connection
        .write_all(&sdc_one_vec)
        .expect("Failed to write CodeOpenInfo via TCP Connection");

    println!("Sent a request!");
    println!("Target host: {}:{}", TARGET_IP, TARGET_PORT);
    println!("CodeOpenRequest: {:?}", code_open_req);
}

fn main() {
    let ssh_flag = env::vars().any(|(k, _)| k == "SSH_CONNECTION");

    if !ssh_flag {
        println!("Error this command should be executed in SSH");
        return;
    }

    let args: Vec<String> = env::args().collect();
    let args = args[1..].to_owned();

    let path = if !args.is_empty() {
        let path = &args[0];
        PathBuf::from(
            shellexpand::full(path)
                .expect("failed to tilde expad")
                .to_string(),
        )
    } else {
        env::current_dir().unwrap()
    };

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

    send_request_to_server(code_open_req);
}
