#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

use city_spellcheck::*;
use rocket::http::RawStr;
use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::JsonValue;

#[derive(Serialize, Deserialize)]
struct CustomError {
    error: &'static str,
}

#[get("/suggestions?<q>&<latitude>&<longitude>")]
fn suggestions(
    q: &RawStr,
    latitude: Option<f32>,
    longitude: Option<f32>,
    state: State<CityData>,
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

    let results = state.search(q, coords);

    Ok(json!(results))
}

fn main() {
    let mut cities = CityData::new();
    cities
        .populate_from_file("data/cities_canada-usa-filtered.csv")
        .unwrap();
    rocket::ignite()
        .mount("/", routes![suggestions])
        .manage(cities)
        .launch();
}
