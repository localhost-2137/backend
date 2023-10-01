use anyhow::Result;
use axum::{extract::State, Json};
use openai::chat::{ChatCompletion, ChatCompletionMessage};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::routes::distance::{distance, LocationQuery};

const MODEL_MSG: &'static str = "Your task is to select the best university based on user inputs. You should also consider the subjects and distances to that universities. I will pre-selected set of schools in that format:\nID|GLOBAL_RANK|NAME|DISTANCE IN METERS|TIME OF TRAVEL IN SECONDS|SUBJECTS\nYou must return only id's of that universities in order from best to worse!\nUniversities to consider:\n{UNIS}\n\nUser preferences:\n{PREF}";
const AI_LIMIT: i32 = 5;
const OPENAI_MODEL: &'static str = "gpt-3.5-turbo";

#[derive(Debug, Deserialize, Serialize)]
pub struct AIInput {
    pub questions: Vec<Vec<String>>,
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AIUni {
    pub id: i32,
    pub rank: i32,
    pub name: String,
    pub lng: f64,
    pub lat: f64,
    pub subjects: Vec<String>,
}

pub async fn ai(State(pool): State<PgPool>, Json(input): Json<AIInput>) -> Json<Value> {
    let mut model_msg = String::from(MODEL_MSG);

    let mut tmp_questions = String::new();
    for q in &input.questions {
        if q.len() != 2 {
            continue;
        }

        tmp_questions += &format!("{}: {}\n", q[0], q[1]);
    }
    model_msg = model_msg.replace("{PREF}", &tmp_questions);
    let unis: Vec<AIUni> = sqlx::query_as("SELECT id, rank, name, lng, lat, array((SELECT subject FROM universities_subjects WHERE universities_subjects.u_id = universities.id)) as subjects 
        FROM universities ORDER BY ST_DistanceSphere(ST_MakePoint(lat, lng), ST_MakePoint($1, $2)) LIMIT $3")
        .bind(input.lat)
        .bind(input.lng)
        .bind(AI_LIMIT)
        .fetch_all(&pool)
        .await.unwrap();

    let mut tmp_unis = String::new();
    for uni in unis {
        let gmaps = distance(LocationQuery {
            fromLng: input.lng,
            fromLat: input.lat,
            toLng: uni.lng,
            toLat: uni.lat,
        })
        .await;
        let gmaps = if let Ok(gmaps) = gmaps {
            (gmaps.0.to_string(), gmaps.1.to_string())
        } else {
            ("NONE".to_string(), "NONE".to_string())
        };

        tmp_unis += &format!(
            "{}|{}|{}|{}|{}|{}\n",
            uni.id,
            uni.rank,
            uni.name,
            gmaps.0,
            gmaps.1,
            uni.subjects.join(",")
        );
    }

    model_msg = model_msg.replace("{UNIS}", &tmp_unis);
    println!("{}", model_msg);

    Json(json!(vec![""]))
}

async fn get_ai_res(input: &str) -> Result<String> {
    let messages: Vec<ChatCompletionMessage> = vec![construct_system_msg(), construct_msg(&input)];
    let chat_completion = ChatCompletion::builder(OPENAI_MODEL, messages.clone())
        .create()
        .await?;

    let choice = chat_completion
        .choices
        .first()
        .ok_or_else(|| anyhow::anyhow!(""))?;

    let msg = &choice.message;
    let resp = msg
        .to_owned()
        .content
        .ok_or_else(|| anyhow::anyhow!("Response content was null"))?;

    Ok(resp)
}

fn construct_msg(msg: &str) -> ChatCompletionMessage {
    ChatCompletionMessage {
        role: openai::chat::ChatCompletionMessageRole::User,
        content: Some(msg.to_string()),
        name: None,
        function_call: None,
    }
}

fn construct_system_msg() -> ChatCompletionMessage {
    ChatCompletionMessage {
        role: openai::chat::ChatCompletionMessageRole::System,
        content: Some(MODEL_MSG.to_string()),
        name: None,
        function_call: None,
    }
}
