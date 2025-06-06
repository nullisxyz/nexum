use clap::Parser;

#[derive(Parser, Debug)]
#[command(about)]
pub struct Cli {
    /// Address to listen for browser extension WebSocket requests
    #[arg(short, long, value_name = "ADDR", default_value = "127.0.0.1:1248")]
    pub listen_addr: String,

    /// Node JSON-RPC URL capable of supporting WebSockets
    #[arg(
        short,
        long,
        value_name = "RPC_URL",
        default_value = "wss://eth.drpc.org"
    )]
    pub rpc_url: String,
}

// const VERSION_MESSAGE: &str = concat!(
//     env!("CARGO_PKG_VERSION"),
//     "-",
//     env!("VERGEN_GIT_DESCRIBE"),
//     " (",
//     env!("VERGEN_BUILD_DATE"),
//     ")"
// );
