use std::env;
use std::fs;
use std::ops::Not;
use std::path::Path;
use std::process::Command;

use actix_cors;
use actix_cors::Cors;
use actix_web::web::{Data, Json, Query};
use actix_web::{get, main, App, HttpServer, Responder, Result};
use serde::Deserialize;
use voiceover::VoiceOver;

#[derive(Clone)]
struct AppState {
    pub voiceovers: Vec<VoiceOver>,
}

#[get("/get_list")]
async fn get_list(data: Data<AppState>) -> Result<impl Responder> {
    Ok(Json(
        data.get_ref()
            .voiceovers
            .iter()
            .map(|voiceover| voiceover.name.clone())
            .collect::<Vec<String>>(),
    ))
}

#[derive(Deserialize)]
struct VoiceOverInfo {
    pub id: usize,
}

#[get("/get")]
async fn get(
    data: Data<AppState>,
    Query(VoiceOverInfo { id }): Query<VoiceOverInfo>,
) -> Result<impl Responder> {
    if (0..data.voiceovers.len()).contains(&id) {
        Ok(Json(data.voiceovers[id].clone()))
    } else {
        Err(actix_web::error::ErrorBadRequest("id must be greater or equal zero and lower than count of voiceovers (see list by GET /get_list)"))
    }
}

fn parse_all() {
    let dir_name = "voices";
    let cur_dir = env::current_dir().unwrap();
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        Command::new("python")
            .args(vec![Path::new(&cur_dir)
                .join("voices/parse_all.py")
                .to_str()
                .unwrap()])
            .current_dir(Path::new(&cur_dir).join(dir_name))
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success()
            .not()
            .then(|| panic!());
    } else {
        Command::new("python3")
            .args(vec![Path::new(&cur_dir)
                .join("voices/parse_all.py")
                .to_str()
                .unwrap()])
            .current_dir(Path::new(&cur_dir).join(dir_name))
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success()
            .not()
            .then(|| panic!());
    }
}

#[main]
async fn main() {
    parse_all();
    let args = env::args().collect::<Vec<String>>();
    let port = if let Some(i) = args.iter().position(|x| x == "-p" || x == "--port") {
        args[i + 1].parse().unwrap()
    } else {
        8888u16
    };

    let host = if let Some(i) = args.iter().position(|x| x == "-h" || x == "--host") {
        args[i + 1].as_str()
    } else {
        "0.0.0.0"
    };

    println!(
        "Starting on http://{}:{port}/",
        if host == "0.0.0.0" { "localhost" } else { host }
    );

    let jsons_path = Path::new("./voices/jsons");
    if !jsons_path.is_dir() {
        panic!("/voices/jsons must be in env");
    }
    let app_state = AppState {
        voiceovers: fs::read_dir(Path::new("./voices/jsons"))
            .unwrap()
            .map(|x| {
                serde_json::from_str(fs::read_to_string(x.unwrap().path()).unwrap().as_str())
                    .unwrap()
            })
            .collect::<Vec<VoiceOver>>(),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_state.clone()))
            .wrap(Cors::permissive())
            .service(get_list)
            .service(get)
    })
    .workers(4)
    .bind((host, port))
    .unwrap()
    .run()
    .await
    .unwrap();
}
