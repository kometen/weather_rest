use crate::readings::{MeasurementsSingleLocation};
use crate::error_handler::CustomError;
use actix_web::{get, post, web, HttpResponse};
use crate::schema::measurements_single_location_function::measurements;

#[get("/measurements_single_location/{id}/{rows}")]
async fn measurements_single_location(path: web::Path<(u32, u32)>) -> Result<HttpResponse, CustomError> {
    let (id, rows) = path.into_inner();
    let m = MeasurementsSingleLocation::measurements_single_location(id, rows)?;
    Ok(HttpResponse::Ok().json(m))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(measurements_single_location);
}
