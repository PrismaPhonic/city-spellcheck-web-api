#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate lazy_static;

use city_spellcheck::*;
use rocket::http::RawStr;
use rocket_contrib::json::{Json, JsonValue};

lazy_static! {
    static ref CITIES: CityData = {
        let mut cities = CityData::new();
        cities
            .populate_from_file("data/cities_canada-usa-filtered.csv")
            .unwrap();
        cities
    };
}


#[get("/suggestions?<q>&<latitude>&<longitude>")]
fn suggestions(q: &RawStr, latitude: Option<f32>, longitude: Option<f32>) -> JsonValue {
    // let mut cities = CityData::new();
    // cities
    //     .populate_from_file("data/cities_canada-usa-filtered.csv")
    //     .unwrap();
    
    let mut coords = None;

    if let Some(lat) = latitude {
        if let Some(long) = longitude {
            coords = Some(Coordinate::new(lat, long));
        } else {
            return json!("If you supply latitude you must also supply longitude")
        }
    }
    let results = CITIES.search(q, coords);
    json!(results)
}

fn main() {
    rocket::ignite().mount("/", routes![suggestions]).launch();
}
