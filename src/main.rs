use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    let server = run("127.0.0.1:0".into())?.await;
    Ok(())
}
