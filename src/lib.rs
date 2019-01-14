extern crate rayon;
/**
 * TODOS:
 * 1. Add enum for Region - states and provinces
*/

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Write};

use rayon::prelude::*;
use rff::match_and_score;
use rff::scorer;

/// Data-Oriented Design approach
/// Struct of Arrays (SoA)
#[derive(Debug)]
pub struct CityData {
    pub names: Vec<String>,
    pub countries: Vec<String>,
    pub regions: Vec<String>,
    pub latitudes: Vec<f32>,
    pub longitudes: Vec<f32>,
}

#[derive(Debug, Copy, Clone)]
pub enum Country {
    US,
    CA,
}

#[derive(Debug)]
pub struct City<'a> {
    name: &'a str,
    country: &'a str,
    region: &'a str,
    latitude: f32,
    longitude: f32,
}

#[derive(Debug)]
pub struct FuzzyResult {
    city: String,
    latitude: f32,
    longitude: f32,
    score: f32,
}

impl FuzzyResult {
    fn new(city_data: City, score: f32) -> FuzzyResult {
        let City {
            name,
            country,
            region,
            latitude,
            longitude,
        } = city_data;
        let city = format!("{}, {}, {}", name, region, country);
        FuzzyResult {
            city,
            latitude,
            longitude,
            score,
        }
    }
}

impl CityData {
    fn new() -> Self {
        CityData {
            names: Vec::new(),
            countries: Vec::new(),
            regions: Vec::new(),
            latitudes: Vec::new(),
            longitudes: Vec::new(),
        }
    }

    fn populate_from_file(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let buffer = fs::read_to_string(filename)?;
        let mut lines = buffer.lines();

        // skip header line
        lines.next();

        for line in lines {
            if let [name, country, region, latitude, longitude] =
                line.split(',').collect::<Vec<&str>>()[..]
            {
                let latitude: f32 = latitude.parse()?;
                let longitude: f32 = longitude.parse()?;

                self.add_city(name, country, region, latitude, longitude);
            };
        }

        Ok(())
    }

    fn add_city(&mut self, name: &str, country: &str, region: &str, latitude: f32, longitude: f32) {
        self.names.push(name.to_string());
        self.countries.push(country.to_string());
        self.regions.push(region.to_string());
        self.latitudes.push(latitude);
        self.longitudes.push(longitude);
    }

    fn get_city(&self, idx: usize) -> City {
        City {
            name: &self.names[idx],
            country: &self.countries[idx],
            region: &self.regions[idx],
            latitude: self.latitudes[idx],
            longitude: self.longitudes[idx],
        }
    }

    fn search(&self, term: &str) -> Vec<FuzzyResult> {
        let mut results = vec![];

        let found: Vec<usize> = self
            .names
            .par_iter()
            .enumerate()
            // .map(|(i, city)| (i, scorer::score(term, city)))
            // .filter(|(_, score)| score < &0.5 && score > &0.01)
            .filter(|(i, city)| city.contains(term))
            .map(|(i, _)| i)
            .collect();

        for city_idx in found {
            // let (i, score) = result;
            let city = self.get_city(city_idx);
            let fr_instance = FuzzyResult::new(city, 0.0);
            results.push(fr_instance);
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_citydata_struct_nyc() {
        let mut cities = CityData::new();
        cities.add_city("New York City", "US", "NY", 40.7128, 74.0060);
        assert_eq!(format!("{:?}", cities.get_city(0)), "City { name: \"New York City\", country: \"US\", region: \"NY\", latitude: 40.7128, longitude: 74.006 }");
    }

    #[test]
    fn test_citydata_struct_sf() {
        let mut cities = CityData::new();
        cities.add_city("San Francisco", "US", "CA", 37.7749, 122.4194);
        assert_eq!(format!("{:?}", cities.get_city(0)), "City { name: \"San Francisco\", country: \"US\", region: \"CA\", latitude: 37.7749, longitude: 122.4194 }");
    }

    #[test]
    fn test_populate_from_file() {
        let mut cities = CityData::new();
        cities.populate_from_file("data/cities_canada-usa-filtered.csv");
        assert_eq!(format!("{:?}", cities.get_city(0)), "City { name: \"Abbotsford\", country: \"CA\", region: \"02\", latitude: 49.05798, longitude: -122.25257 }");
    }

    #[test]
    fn test_search() {
        let mut cities = CityData::new();
        cities.populate_from_file("data/cities_canada-usa-filtered.csv");
        let results = cities.search("London");
        assert_eq!(format!("{:?}", results), "City { name: \"Abbotsford\", country: \"CA\", region: \"02\", latitude: 49.05798, longitude: -122.25257 }");
    }

}
