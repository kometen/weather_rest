use crate::readings::{Location, MeasurementsSingleLocation};
use crate::error_handler::CustomError;
use actix_web::{get, post, web, HttpResponse};
use crate::schema::measurements_single_location_function::measurements;

#[get("/measurements_single_location/{id}/{rows}")]
async fn measurements_single_location(path: web::Path<(i32, i32)>) -> Result<HttpResponse, CustomError> {
    let (id, rows) = path.into_inner();
    let m = MeasurementsSingleLocation::measurements_single_location(id, rows)?;
    Ok(HttpResponse::Ok().json(m))
}

#[get("/locations")]
async fn locations() -> Result<HttpResponse, CustomError> {
    let m = Location::locations()?;
    Ok(HttpResponse::Ok().json(m))
}

#[get("/locations/{id}")]
async fn location_by_id(path: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let id = path.into_inner();
    let m = Location::location_by_id(id)?;
    Ok(HttpResponse::Ok().json(m))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(measurements_single_location);
    config.service(locations);
    config.service(location_by_id);
}
