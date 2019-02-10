#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

use city_spellcheck::*;
use rocket::http::RawStr;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::JsonValue;

lazy_static! {
    static ref CITIES: CityData = {
        let mut cities = CityData::new();
        cities
            .populate_from_file("data/cities_canada-usa-filtered.csv")
            .unwrap();
        cities
    };
}

#[derive(Serialize, Deserialize)]
struct CustomError {
    error: &'static str,
}

#[get("/suggestions?<q>&<latitude>&<longitude>")]
fn suggestions(
    q: &RawStr,
    latitude: Option<f32>,
    longitude: Option<f32>,
) -> Result<JsonValue, status::Custom<JsonValue>> {
    let mut coords = None;

    if let Some(lat) = latitude {
        if let Some(long) = longitude {
            coords = Some(Coordinate::new(lat, long));
        } else {
            let custom_error = CustomError {
                error: "If you supply latitude you must also supply longitude!",
            };
            let response = status::Custom(Status::UnprocessableEntity, json!(custom_error));

            return Err(response);
        }
    } else {
        if let Some(_) = longitude {
            let custom_error = CustomError {
                error: "If you supply longitude you must also supply latitude!",
            };
            let response = status::Custom(Status::UnprocessableEntity, json!(custom_error));

            return Err(response);
        }
    }

    let results = CITIES.search(q, coords);

    Ok(json!(results))
}

fn main() {
    // kicking off static lazy so it's pre-loaded before suggestions route
    let _ = CITIES.get_city(0);
    rocket::ignite().mount("/", routes![suggestions]).launch();
}
