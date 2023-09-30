use anyhow::{anyhow, Result};
use rand::Rng;
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
    let client = reqwest::Client::new();

    let data = tokio::fs::read_to_string("./data.json").await?;
    let universities: Vec<Univertsity> = serde_json::from_str(&data)?;

    let mut all_subjects: Vec<String> = Vec::new();

    let mut tmp_output = String::new();
    for university in universities {
        if university.location != "Poland" {
            continue;
        }

        println!("Parsing: {}...", university.name);
        let addr = utils::get_place_addr(&api_key, &university, &client).await;

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
    let google_info = maps_resp.ok_or_else(|| anyhow!("Address not found!"))?;
    let splitted = google_info.formatted_address.split(",");
    let mut city_splitted = splitted
        .clone()
        .nth(splitted.count() - 2)
        .ok_or_else(|| anyhow!("Cannot get city"))?
        .split(" ")
        .into_iter()
        .collect::<Vec<&str>>();

    city_splitted.remove(0);
    if city_splitted.len() > 1 {
        city_splitted.remove(0);
    }
    let city = city_splitted.join(" ");
    let city = city.trim();

    Ok(format!(
        "INSERT INTO universities(rank, name, academic, url, lng, lat, address, city, number_students, subjects) VALUES ({}, '{}', {}, '{}', {}, {}, '{}', '{}', {}, '{}'); \n",
        university.rank.parse::<u32>().unwrap_or(rand::thread_rng().gen_range(600..1000)),
        google_info.name,
        rand::thread_rng().gen_bool(0.6),
        university.url,
        google_info.geometry.location.lng,
        google_info.geometry.location.lat,
        google_info.formatted_address,
        city,
        university.stats_number_students.replace(",", ""),
        university.subjects_offered
    ))
}
