use axum::Json;
use reqwest;
use serde_json::{json, Value};

use axum::extract::Query;
use serde::Deserialize;

use crate::routes::MAPS_API_KEY;

#[derive(Deserialize)]
pub struct LocationQuery {
    fromLng: f32,
    fromLat: f32,
    toLng: f32,
    toLat: f32,
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

pub async fn distance(location_query: Query<LocationQuery>) -> Json<Value> {
    let from = (location_query.fromLat, location_query.fromLng);
    let to = (location_query.toLat, location_query.toLng);

    let result = reqwest::get(format!(
        "https://maps.googleapis.com/maps/api/directions/json?key={}&origin={},{}&destination={},{}&units=metric",
        MAPS_API_KEY.as_str(),
        from.0, from.1,
        to.0, to.1,
    ))
        .await
        .unwrap();

    let result = result.text().await.unwrap().to_string();
    println!("{}", result);
    let result: RootResponse = serde_json::from_str(&result).unwrap();

    let leg = &result.routes[0].legs[0];

    Json(json!({
        "distance": leg.distance.value,
        "duration": leg.duration.value,
    }))
}
