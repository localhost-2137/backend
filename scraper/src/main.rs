use anyhow::Result;
use utils::Univertsity;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::args().nth(1);
    if api_key.is_none() {
        println!(
            "Usage: {} <google_places_api_key>",
            std::env::args().nth(0).unwrap_or("./binary".to_string())
        );

        return Ok(());
    }
    let api_key = api_key.expect("Shouldnt fail");

    let data = tokio::fs::read_to_string("./data.json").await?;
    let universities: Vec<Univertsity> = serde_json::from_str(&data)?;

    let mut all_subjects: Vec<String> = Vec::new();

    let mut tmp_output = String::new();
    for university in universities {
        if university.location != "Poland" {
            continue;
        }

        let addr = utils::get_place_addr(&api_key, university.clone()).await;
        println!("{:?}", addr);

        let subjects: Vec<&str> = university.subjects_offered.split(",").collect();
        for subject in subjects {
            let subject = subject.trim().to_string();

            if all_subjects.contains(&subject) || subject.is_empty() {
                continue;
            }

            all_subjects.push(subject);
        }

        tmp_output += &generate_insert_query(university).await?;
    }

    tokio::fs::write("./output.sql", tmp_output).await?;
    println!("{:#?}", all_subjects);
    Ok(())
}

async fn generate_insert_query(university: Univertsity) -> Result<String> {
    Ok(format!("INSERT INTO universities({}); \n", university.name))
}
