use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::process;

// use comfy_table::modifiers::UTF8_ROUND_CORNERS;

// https://transform.tools/json-to-rust-serde
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertDataResponse {
    pub status: String,
    pub data: AlertData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertData {
    pub alerts: Vec<Alert>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    pub labels: Labels,
    pub annotations: Annotations,
    pub state: String,
    #[serde(rename = "activeAt")]
    pub active_at: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Labels {
    pub alertname: String,
    pub instance: String,
    pub job: Option<String>,
    pub name: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Annotations {
    pub description: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!(
            "No prometheus alerts endpoint provided. Usage:\npromterm https://myhost/api/v1/alerts"
        );
        process::exit(1);
    }

    let endpoint = &args[1];
    let response = reqwest::blocking::get(endpoint);

    let f = match response {
        Ok(stuff) => stuff,
        Err(error) => panic!("Failed to retrieve alert data: {:?}", error),
    };

    let mut alert_data: AlertDataResponse = match f.json() {
        Ok(var) => var,
        Err(error) => panic!("Failed to parse response: {:?}", error),
    };

    let mut table = Table::new();
    table
        // .apply_modifier(UTF8_ROUND_CORNERS)
        // .set_width(200)
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Instance", "State", "Alert", "Description"]);

    // Sort alerts by instance
    alert_data
        .data
        .alerts
        .sort_by(|a, b| a.labels.instance.cmp(&b.labels.instance));

    for alert in alert_data.data.alerts {
        table.add_row(vec![
            Cell::new(alert.labels.instance),
            match alert.state.as_str() {
                "firing" => Cell::new(alert.state).fg(Color::Red),
                "pending" => Cell::new(alert.state).fg(Color::Yellow),
                _ => Cell::new(alert.state),
            },
            Cell::new(alert.labels.alertname),
            Cell::new(alert.annotations.description),
        ]);
    }

    println!("{table}");
}
