use tokio::{fs::File, io::AsyncReadExt};

#[allow(dead_code)]
async fn tokio_err() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("foo.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    println!("{}", contents);
    Ok(())
}

#[tokio::main]
#[test]
async fn test_tokio_err() {
    let t = tokio_err().await;
    assert!(t.is_err());
}
