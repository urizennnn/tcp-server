use tcp::tcp::tcp::TCP;

#[tokio::main]
async fn main() {
    TCP::run("localhost:8080")
}
