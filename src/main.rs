use std::fs;
use std::path::Path;
use std::env;

use actix_web::{App, get, HttpServer, main, Responder, Result};
use actix_web::web::{Data, Json, Query};
use serde::Deserialize;
use voiceover::VoiceOver;
use actix_cors;
use actix_cors::Cors;

#[derive(Clone)]
struct AppState {
    pub voiceovers: Vec<VoiceOver>,
}

#[get("/get_list")]
async fn get_list(data: Data<AppState>) -> Result<impl Responder> {
    Ok(Json(data.get_ref().voiceovers.iter()
        .map(|voiceover| voiceover.name.clone())
        .collect::<Vec<String>>()
    ))
}

#[derive(Deserialize)]
struct VoiceOverInfo {
    pub id: usize,
}

#[get("/get")]
async fn get(data: Data<AppState>, Query(VoiceOverInfo { id }): Query<VoiceOverInfo>) -> Result<impl Responder> {
    if (0..data.voiceovers.len()).contains(&id) {
        Ok(Json(data.voiceovers[id].clone()))
    } else {
        Err(actix_web::error::ErrorBadRequest("id must be greater or equal zero and lower than count of voiceovers (see list by GET /get_list)"))
    }
}

#[main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();
    let port =
        if let Some(i) = args.iter().position(|x| x == "-p" || x == "--port") {
            args[i + 1].parse().unwrap()
        } else {
            8888u16
        };

    let host =
        if let Some(i) = args.iter().position(|x| x == "-h" || x == "--host") {
            args[i + 1].as_str()
        } else {
            "0.0.0.0"
        };

    println!("Starting on http://{}:{port}/", if host == "0.0.0.0" {"localhost"} else {host});

    let jsons_path = Path::new("./voices/jsons");
    if !jsons_path.is_dir() {
        panic!("/voices/jsons must be in env");
    }
    let app_state = AppState {
        voiceovers: fs::read_dir(
            Path::new("./voices/jsons")).unwrap()
            .map(|x| serde_json::from_str(fs::read_to_string(x.unwrap().path()).unwrap().as_str()).unwrap())
            .collect::<Vec<VoiceOver>>()
    };

    HttpServer::new(move ||
        App::new()
            .app_data(Data::new(app_state.clone()))
            .wrap(Cors::permissive())
            .service(get_list)
            .service(get))
        .workers(4)
        .bind((host, port))
        .unwrap().run()
        .await
        .unwrap();
}
