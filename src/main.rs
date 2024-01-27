use parser_toy::nom::redis::main::redis_cli;

#[tokio::main]
async fn main() {
    redis_cli().await.expect("end the world")
}