use anyhow::Result;
use utils::Univertsity;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let data = tokio::fs::read_to_string("./data.json").await?;
    let universities: Vec<Univertsity> = serde_json::from_str(&data)?;

    println!("{:#?}", universities);

    Ok(())
}
