use bigdecimal::BigDecimal;
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use diesel::backend::Backend;
use diesel::deserialize::Queryable;
use diesel::RunQueryDsl;
use diesel::sql_types::Integer;
use serde::{Serialize, Deserialize};
use crate::db;
use crate::error_handler::CustomError;
use crate::schema::measurements_single_location_function;

/*
#[derive(Serialize, Queryable)]
pub struct Reading {
    #[diesel(deserialize_as = "MyDateTimeWrapper")]
    pub measurement_time_default: DateTime<Local>,
    pub id: i32,
    pub data: serde_json::Value,
}

#[derive(Serialize, Queryable)]
pub struct LatestReading {
    #[diesel(deserialize_as = "MyDateTimeWrapper")]
    pub measurement_time_default: DateTime<Local>,
    pub id: i32,
    pub data: serde_json::Value,
}

#[derive(Serialize, Queryable)]
pub struct Location {
    #[diesel(deserialize_as = "MyDateTimeWrapper")]
    pub publication_time: DateTime<Local>,
    pub id: i32,
    pub name: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
}

#[derive(Serialize, Queryable)]
pub struct LocationReading {
    #[diesel(deserialize_as = "MyDateTimeWrapper")]
    pub measurement_time_default: DateTime<Local>,
    pub id: i32,
    pub name: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
    pub data: serde_json::Value,
}

#[derive(Serialize)]
pub struct LocationReadingOut {
    pub measurement_time_default: DateTime<Local>,
    pub id: i32,
    pub name: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
    pub data: String,
}
*/

#[derive(Serialize, Queryable, QueryableByName)]
#[table_name = "measurements_single_location_function"]
pub struct MeasurementsSingleLocation {
    pub id: i32,
    pub name: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
    //#[diesel(deserialize_as = "MyDateTimeWrapper")]
    pub measurement_time_default: NaiveDateTime,
    pub measurements: serde_json::Value,
}

/*
#[derive(Debug, Deserialize)]
pub struct Measurement {
    pub field_description: String,
    pub index:i16,
    pub measurement: BigDecimal,
}

pub struct MyDateTimeWrapper(DateTime<Local>);

impl Into<DateTime<Local>> for MyDateTimeWrapper {
    fn into(self) -> DateTime<Local> {
        self.0
    }
}

impl<DB, ST> Queryable<ST, DB> for MyDateTimeWrapper
where
    DB: Backend,
    DateTime<Utc>: Queryable<ST, DB>,
{
    type Row = <DateTime<Utc> as Queryable<ST, DB>>::Row;

    fn build(row: Self::Row) -> Self {
        Self(<DateTime<Utc> as Queryable<ST, DB>>::build(row).with_timezone(&Local))
    }
}
*/


impl MeasurementsSingleLocation {
    pub fn measurements_single_location(id: i32, rows: i32) -> Result<Vec<MeasurementsSingleLocation>, CustomError> {
        let q = "select * from measurements_single_location_function($1,$2)";
        let mut conn = db::connection()?;
        let m= diesel::sql_query(q)
            .bind::<Integer, _>(id)
            .bind::<Integer, _>(rows)
            .get_results(&mut conn)?;
        Ok(m)
    }
}