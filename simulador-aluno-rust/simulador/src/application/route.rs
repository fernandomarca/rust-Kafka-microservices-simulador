use serde::{Deserialize, Serialize};
use std::{error, fs::File, io::Read};
#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    #[serde(rename = "routeId")]
    pub id: String,
    #[serde(rename = "clientId")]
    client_id: String,
    #[serde(rename = "position")]
    #[serde(default)]
    positions: Vec<Position>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Position {
    lat: f64,
    long: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct PartialRoutePosition {
    #[serde(rename = "routeId")]
    id: String,
    #[serde(rename = "clientId")]
    client_id: String,
    #[serde(rename = "position")]
    positions: Vec<f64>,
    finished: bool,
}

#[allow(dead_code)]
impl Route {
    pub fn new() -> Route {
        Route {
            id: String::new(),
            client_id: String::new(),
            positions: Vec::new(),
        }
    }
    pub fn load_positions(&mut self) -> Result<(), Box<dyn error::Error>> {
        match &self.id.is_empty() {
            false => {
                let mut s = String::new();
                let path = format!("./src/destinations/{}.txt", self.id);
                File::open(path)?.read_to_string(&mut s)?;
                s.lines().for_each(|l: &str| {
                    let data = l.split(",").collect::<Vec<&str>>();
                    let lat = data[0].parse::<f64>().expect("lat parser error");
                    let long = data[1].parse::<f64>().expect("lat parser error");
                    self.positions.push(Position { lat, long });
                });
                Ok(())
            }
            true => Err("Route id is empty")?,
        }
    }
    pub fn export_json_positions(&self) -> Result<Vec<String>, Box<dyn error::Error>> {
        let total = self.positions.len() - 1;
        let mut result: Vec<String> = Vec::new();
        for (i, p) in self.positions.iter().enumerate() {
            let partial_route = PartialRoutePosition {
                id: self.id.to_owned(),
                client_id: self.client_id.to_owned(),
                positions: vec![p.lat, p.long],
                finished: if total == i { true } else { false },
            };
            // Serialize the given data structure as a String of JSON.
            let json_route = serde_json::to_string(&partial_route)?;
            result.push(json_route)
        }
        Ok(result)
    }
}

// {"routeId":"1","clientId":"c1"}
// {"routeId":"2","clientId":"c2"}
// {"routeId":"3","clientId":"c3"}
