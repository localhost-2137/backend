use anyhow::Result;
use utils::{Candidate, Univertsity};

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

        println!("Parsing: {}...", university.name);
        let addr = utils::get_place_addr(&api_key, &university).await;

        let subjects: Vec<&str> = university.subjects_offered.split(",").collect();
        for subject in subjects {
            let subject = subject.trim().to_string();

            if all_subjects.contains(&subject) || subject.is_empty() {
                continue;
            }

            all_subjects.push(subject);
        }

        tmp_output += &generate_insert_query(addr, &university).await?;
    }

    tokio::fs::write("./output.sql", tmp_output).await?;
    println!("{:#?}", all_subjects);
    Ok(())
}

async fn generate_insert_query(
    maps_resp: Option<Candidate>,
    university: &Univertsity,
) -> Result<String> {
    let google_info = maps_resp.ok_or_else(|| anyhow::anyhow!("Address not found!"))?;

    Ok(format!(
        "INSERT INTO univeristies(name, url, lng, lat, address, number_students, subjects) VALUES ('{}', '{}', {}, {}, '{}', {}, '{}'); \n",
        //university.rank,
        google_info.name,
        university.url,
        google_info.geometry.location.lng,
        google_info.geometry.location.lat,
        google_info.formatted_address,
        university.stats_number_students.replace(",", ""),
        university.subjects_offered
    ))
}
