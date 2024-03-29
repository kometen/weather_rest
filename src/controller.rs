/*
use super::Pool;
use crate::readings::models::{LatestReading, Location, LocationReading, Reading, Measurement, LocationReadingOut, MeasurementsSingleLocation};
use crate::schema::latest_readings::dsl::latest_readings;
use crate::schema::location_readings::dsl::location_readings;
use crate::schema::locations::dsl::locations;
use crate::schema::readings::columns::{id, measurement_time_default};
use crate::schema::readings::dsl::readings;
use actix_web::{web, Error, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use std::vec::Vec;

pub async fn get_root() -> impl Responder {
    HttpResponse::Ok().body("Weather stats from Statens Vegvesen, The Norwegian Public Roads Administration! Updated approximately every ten minutes. Paths: /locations/{id}, /readings/{id}.")
}


pub async fn get_locations(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_locations(db))
        .await
        .map(|station| HttpResponse::Ok().json(station))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_get_locations(pool: web::Data<Pool>) -> Result<Vec<Location>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = locations.load::<Location>(&conn)?;
    Ok(items)
}

pub async fn get_location_by_id(
    db: web::Data<Pool>,
    location_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_location_by_id(db, location_id.into_inner()))
            .await
            .map(|location| HttpResponse::Ok().json(location))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn db_get_location_by_id(
    pool: web::Data<Pool>,
    location_id: i32,
) -> Result<Location, diesel::result::Error> {
    let conn = pool.get().unwrap();
    locations.find(location_id).get_result::<Location>(&conn)
}

pub async fn get_readings(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_readings(db))
        .await
        .map(|reading| HttpResponse::Ok().json(reading))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_get_readings(pool: web::Data<Pool>) -> Result<Vec<LatestReading>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    latest_readings.load::<LatestReading>(&conn)
}

pub async fn get_readings_by_id(
    db: web::Data<Pool>,
    reading_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_readings_by_id(db, reading_id.into_inner()))
            .await
            .map(|reading| HttpResponse::Ok().json(reading))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn db_get_readings_by_id(
    pool: web::Data<Pool>,
    reading_id: i32,
) -> Result<Vec<Reading>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    readings
        .order(measurement_time_default.desc())
        .limit(18)
        .filter(id.eq(reading_id))
        .load::<Reading>(&conn)
}

pub async fn get_location_readings(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_get_location_readings(db))
        .await
        .map(|reading| {
            let mut vec: Vec<LocationReadingOut> = Vec::new();
            for v in &reading {
                let d: Vec<Measurement> = serde_json::from_str(&v.data.to_string()).unwrap();
                let mut s = String::new();
                for e in &d {
                    let m = [&e.field_description, ":", &e.measurement.to_string(), ","].concat();
                    s.push_str(&m);
                }
                let out = LocationReadingOut {
                    measurement_time_default: v.measurement_time_default,
                    id: v.id.clone(),
                    name: v.name.clone(),
                    latitude: v.latitude.clone(),
                    longitude: v.longitude.clone(),
                    data: s.clone(),
                };
                vec.push(out);
            }
            HttpResponse::Ok().json(vec)
        })
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_get_location_readings(
    pool: web::Data<Pool>,
) -> Result<Vec<LocationReading>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    location_readings.load::<LocationReading>(&conn)
}

pub async fn get_measurements_single_location(
    db: web::Data<Pool>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, Error> {
    let (location_id, rows) = params.into_inner();
    Ok(
        web::block(move || db_get_measurements_single_location(db, location_id, rows))
            .await
            .map(|location| HttpResponse::Ok().json(location))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn db_get_measurements_single_location(
    pool: web::Data<Pool>,
    location_id: i32,
    rows: i32,
) -> QueryResult<Vec<MeasurementsSingleLocation>> {
    println!("get_measurements_single_location(), id: {}, rows: {}", location_id, rows);
    let conn = pool.get().unwrap();
    let result=
        diesel::sql_query("select * from measurements_single_location_function(1,10)")
            .get_result(&conn);
    return result;
}
*/
