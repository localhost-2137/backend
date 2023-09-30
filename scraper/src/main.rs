use anyhow::Result;
use utils::Univertsity;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let data = tokio::fs::read_to_string("./data.json").await?;
    let universities: Vec<Univertsity> = serde_json::from_str(&data)?;

    let mut tmp_output = String::new();
    for university in universities {
        tmp_output += &generate_insert_query(university).await?;
    }

    tokio::fs::write("./output.sql", tmp_output).await?;
    Ok(())
}

async fn generate_insert_query(university: Univertsity) -> Result<String> {
    Ok(format!("INSERT INTO universities({}); \n", university.name))
}
