mod models;
mod handlers;

use actix_web::{web, App, HttpServer};
use actix_files::Files;
use csv::ReaderBuilder;
use std::sync::Arc;
use std::sync::Mutex;
use handlers::{get_categories, get_subcategories, get_random_template};
use models::TemplateRecord;
use actix_cors::Cors;
use crate::handlers::AppState;

fn load_templates(file_path: &str) -> Vec<TemplateRecord> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)
        .expect("Failed to read CSV file");

    reader
        .deserialize()
        .filter_map(|result| result.ok())
        .collect()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let templates = load_templates("output_templates.csv");
    let app_state = AppState {
        templates: Arc::new(Mutex::new(templates)),
    };

    println!("🚀 Server running at: http://127.0.0.1:8080");


    HttpServer::new(move || {
    App::new()
        .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
        .app_data(web::Data::new(app_state.clone()))
        .service(get_categories)
        .service(get_subcategories)
        .service(get_random_template)
        .service(Files::new("/", "frontend").index_file("index.html"))
})

    .bind("127.0.0.1:8080")?
    .run()
    .await
}
