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

    let mut tmp_output = String::new();

    let mut i = 0;
    for university in universities {
        if university.location != "Poland" {
            continue;
        }

        println!("Parsing: {}...", university.name);
        let addr = utils::get_place_addr(&api_key, &university, &client).await;
        tmp_output += &generate_insert_query(i, addr, &university).await?;

        let subjects: Vec<&str> = university.subjects_offered.split(",").collect();
        for subject in subjects {
            let subject = subject.trim();
            tmp_output += &generate_insert_subject(i, subject).await?;
        }

        i += 1;
    }

    tokio::fs::write("./output.sql", tmp_output).await?;
    Ok(())
}

async fn generate_insert_query(
    id: u32,
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
        "INSERT INTO universities(id, rank, name, academic, url, lng, lat, address, city, number_students) VALUES ({}, {}, '{}', {}, '{}', {}, {}, '{}', '{}', {}); \n",
        id,
        university.rank.parse::<u32>().unwrap_or(rand::thread_rng().gen_range(600..1000)),
        google_info.name,
        rand::thread_rng().gen_bool(0.6),
        university.url,
        google_info.geometry.location.lng,
        google_info.geometry.location.lat,
        google_info.formatted_address,
        city,
        university.stats_number_students.replace(",", "")
    ))
}

async fn generate_insert_subject(id: u32, subject: &str) -> Result<String> {
    Ok(format!(
        "INSERT INTO universities_subjects(u_id, subject) VALUES ({}, '{}'); \n",
        id, subject
    ))
}
