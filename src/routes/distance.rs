use reqwest;
use serde_json::json;
use serde_json::Value;
use crate::g_maps::MAPS_API_KEY;


// (lng, lat)
pub async fn get_distance(from: (f32, f32), to: (f32, f32), travel_mode: String) {
    let body = json!({
        "origins": vec![generate_waypoints(from)],
        "destinations": vec![generate_waypoints(to)],
        "travelMode": travel_mode,
    });

    let client = reqwest::Client::new();
    
    client.post("https://routes.googleapis.com/distanceMatrix/v2:computeRouteMatrix")
        .header("Content-Type", "application/json")
        .header("X-Goog-FieldMask", "originIndex,destinationIndex,duration,distanceMeters,status,condition")
        .header("X-Goog-Api-Key", MAPS_API_KEY.as_str())
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await
        .unwrap();
}

fn generate_waypoints(lng_lat: (f32, f32)) -> Value {
    json!({
      "waypoint": {
        "location": {
          "latLng": {
            "latitude": lng_lat.1,
            "longitude": lng_lat.0
          }
        }
      }
    })
}