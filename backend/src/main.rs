mod db;
mod handlers;
mod routes;

use axum::{response::IntoResponse, routing::get, Router};
use clap::Parser;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Opt {
    /// set the listen addrk
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    let app = Router::new().route("/", get(hello)).route("/hi", get(hello_2));

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    println!("listening on http://{}", sock_addr);

    let listener = tokio::net::TcpListener::bind(&sock_addr).await.expect("Unable to bind to socket");

    axum::serve(listener,app).await.expect("Unable to start server");

}

async fn hello() -> impl IntoResponse {
    "hello from server!"
}

async fn hello_2() -> impl IntoResponse{
    "HELLO 2"
}