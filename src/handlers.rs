use actix_web::{get, web, HttpResponse, Responder};
use rand::seq::SliceRandom;
use crate::models::TemplateRecord;

use std::sync::{Arc, Mutex};

#[derive(Clone)]=
pub struct AppState {
    pub templates: Arc<Mutex<Vec<TemplateRecord>>>, 
}

#[get("/categories")]
async fn get_categories(data: web::Data<AppState>) -> impl Responder {
    println!("Fetching categories...");
    let templates = data.templates.lock().unwrap();
    let mut categories: Vec<String> = templates.iter().map(|t| t.Category.clone()).collect();
    categories.sort();
    categories.dedup();
    println!("Categories fetched: {:?}", categories);
    HttpResponse::Ok().json(categories)
}

#[get("/subcategories/{category}")]
async fn get_subcategories(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let category = path.into_inner();
    let templates = data.templates.lock().unwrap();
    let mut subcategories: Vec<String> = templates
        .iter()
        .filter(|t| t.Category == category)
        .map(|t| t.Subcategory.clone())
        .collect();
    subcategories.sort();
    subcategories.dedup();
    HttpResponse::Ok().json(subcategories)
}

#[get("/template/{category}/{subcategory}")]
async fn get_random_template(data: web::Data<AppState>, path: web::Path<(String, String)>) -> impl Responder {
    let (category, subcategory) = path.into_inner();
    let templates = data.templates.lock().unwrap();
    let filtered: Vec<_> = templates
        .iter()
        .filter(|t| t.Category == category && t.Subcategory == subcategory)
        .collect();

    if let Some(random_template) = filtered.choose(&mut rand::thread_rng()) {
        HttpResponse::Ok().json(random_template.Content.clone())
    } else {
        HttpResponse::NotFound().json("No template found")
    }
}
