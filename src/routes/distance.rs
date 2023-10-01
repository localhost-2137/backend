use anyhow::Result;
use reqwest;
use serde::Deserialize;

use crate::routes::MAPS_API_KEY;

#[derive(Deserialize)]
pub struct LocationQuery {
    pub fromLng: f64,
    pub fromLat: f64,
    pub toLng: f64,
    pub toLat: f64,
}

#[derive(Deserialize)]
struct RootResponse {
    routes: Vec<RoutesResponse>,
}

#[derive(Deserialize)]
struct RoutesResponse {
    legs: Vec<LegResponse>,
}

#[derive(Deserialize)]
struct LegResponse {
    distance: MeasureResponse,
    duration: MeasureResponse,
}

#[derive(Deserialize)]
struct MeasureResponse {
    value: u32,
}

pub async fn distance(location_query: LocationQuery) -> Result<(u32, u32)> {
    let from = (location_query.fromLat, location_query.fromLng);
    let to = (location_query.toLat, location_query.toLng);

    let result = reqwest::get(format!(
        "https://maps.googleapis.com/maps/api/directions/json?key={}&origin={},{}&destination={},{}&units=metric",
        MAPS_API_KEY.as_str(),
        from.0, from.1,
        to.0, to.1,
    ))
        .await?;

    let result = result.text().await.unwrap().to_string();
    let result: RootResponse = serde_json::from_str(&result).unwrap();

    if result.routes.len() == 0 {
        return Ok((0, 0));
    }

    let leg = &result.routes[0].legs[0];
    Ok((leg.distance.value, leg.duration.value))
}
