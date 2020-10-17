#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //非同期で動くタスクを生成する
    let task = tokio::spawn(async {
        let res = match reqwest::get("https://www.google.com").await {
            Ok(x) => x, 
            Err(e) => {
                println!("{}", e.to_string());
                return;
            }
        };
        println!("Status: {}", res.status());
        let body = match res.text().await {
            Ok(x) => x,
            Err(e) => {
                println!("{}", e.to_string());
                return;
            }
        };
        println!("Body:\n\n{}", body);
    });

    //タスクが終了するのを待つ
    task.await?;

    Ok(())
}