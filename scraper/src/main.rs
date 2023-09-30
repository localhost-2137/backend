use anyhow::Result;
use utils::Univertsity;

mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let data = tokio::fs::read_to_string("./data.json").await?;
    let universities: Vec<Univertsity> = serde_json::from_str(&data)?;

    let mut all_subjects: Vec<String> = Vec::new();

    let mut tmp_output = String::new();
    for university in universities {
        if university.location != "Poland" {
            continue;
        }

        let addr = utils::get_place_addr(
            "AIzaSyDMqb-6S20ryR7NcSQDleb5voFJR2ZUCIc",
            university.clone(),
        )
        .await;
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
