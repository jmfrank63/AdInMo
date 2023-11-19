use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct CreateData {
    value: i32,
    response_body: String,
}

pub(crate) async fn create_handler(item: web::Json<CreateData>) -> HttpResponse {
    let request = database::crud::Request {
        id: 0, // Assuming the ID is auto-generated
        value: item.value,
        response_body: item.response_body.clone(),
    };

    match database::crud::create_request(&request).await {
        Ok(_) => HttpResponse::Created().json("Request created"),
        Err(_) => HttpResponse::InternalServerError().json("Error creating request"),
    }
}

pub(crate) async fn read_handler(path: web::Path<(i32,)>) -> HttpResponse {
    let id = path.into_inner().0;
    match database::crud::get_request(id).await {
        Ok(request) => HttpResponse::Ok().json(request),
        Err(_) => HttpResponse::NotFound().json("Request not found"),
    }
}

#[derive(Deserialize)]
pub(crate) struct UpdateData {
    id: i32,
    value: i32,
    response_body: String,
}

pub(crate) async fn update_handler(item: web::Json<UpdateData>) -> HttpResponse {
    let request = database::crud::Request {
        id: item.id,
        value: item.value,
        response_body: item.response_body.clone(),
    };

    match database::crud::update_request(&request).await {
        Ok(_) => HttpResponse::Ok().json("Request updated"),
        Err(_) => HttpResponse::InternalServerError().json("Error updating request"),
    }
}

pub(crate) async fn delete_handler(path: web::Path<(i32,)>) -> HttpResponse {
    let id = path.into_inner().0;
    match database::crud::delete_request(id).await {
        Ok(_) => HttpResponse::Ok().json("Request deleted"),
        Err(_) => HttpResponse::InternalServerError().json("Error deleting request"),
    }
}
