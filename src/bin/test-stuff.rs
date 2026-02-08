use std::error::Error;

use broken_bolt::{SERVER_TIME, ServerTime, fetch_params};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let data:ServerTime = fetch_params(SERVER_TIME, vec![]).await?;
    
    println!("Hello world");
    println!("Time: {}", data.result.rfc1123);
    println!("Time: {}", data.result.unixtime);


    Ok(())
}
