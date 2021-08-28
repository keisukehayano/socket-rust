use std::env;

#[macro_use]
extern crate log;

mod tcp_client;
mod tcp_server;
mod udp_client;
mod udp_server;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        error!("Please specity [tcp|udp]  [server|client]  [addr:port].");
        std::process::exit(1);
    }

    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address: &str = &args[3];

    match protocol {
        "tcp" => match role {
            "server" => {
                //サーバ呼び出し
                tcp_server::serve(&address).unwrap_or_else(|e| error!("{}", e));
            }
            "client" => {
                //クライアント呼び出し
                tcp_client::connect(&address).unwrap_or_else(|e| error!("{}", e));
            }
            _ => {
                missing_role();
            }
        },
        "udp" => match role {
            "server" => {
                //サーバ呼び出し
                udp_server::serve(&address).unwrap_or_else(|e| error!("{}", e));
            }
            "client" => {
                //クライアント呼び出し
                udp_client::communicate(&address).unwrap_or_else(|e| error!("{}", e));
            }
            _ => {
                missing_role();
            }
        },
        _ => {
            error!("Please specity tcp or udp on the let argument.");
            std::process::exit(1);
        }
    }

}

/**
 * 第二引数が不正な時にエラー
 * */
fn missing_role() {
    error!("Please specity server or client on the second argument.");
    std::process::exit(1);
}
