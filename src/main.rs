#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use city_spellcheck::*;
use rocket::http::RawStr;
use rocket_contrib::json::{Json, JsonValue};

#[get("/suggestions?<q>&<latitude>&<longitude>")]
fn suggestions(q: &RawStr, latitude: f32, longitude: f32) -> JsonValue {
    let mut cities = CityData::new();
    cities
        .populate_from_file("data/cities_canada-usa-filtered.csv")
        .unwrap();
    
    let coords = Coordinate::new(latitude, longitude);
    let results = cities.search(q, Some(coords));
    json!(results)
}

fn main() {
    rocket::ignite().mount("/", routes![suggestions]).launch();
}
