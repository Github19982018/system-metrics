use log::{error, info};
mod logg;
// mod logging;
mod influxdb2_helper;
mod read_config;
mod system_metrics_main;

use crate::influxdb2_helper::{influxdb_connect, InfluxdbDetails};
use clap::Parser;
use read_config::read_config;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    configuration: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    appname: String,
    sleep_interval: String,
    influxdb_conn: Value,
    path_map: Value,
}
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let conf_path = args.configuration;
    let config = read_config(&conf_path);
    let config: Config = serde_json::from_value(config).expect("configuration parse error");
    let appname = &config.appname;
    // logging::log_init(appname);
    logg::log_init(appname.as_str());
    // logging::initiate_logging(appname);
    info!("first info");
    error!("first error");
    let influxdb_connection: InfluxdbDetails;
    let mut path_map: HashMap<String, String> = HashMap::new();

    influxdb_connection = InfluxdbDetails {
        url: config
            .influxdb_conn
            .get("host")
            .unwrap()
            .to_string()
            .replace("\"", ""),
        token: config
            .influxdb_conn
            .get("token")
            .unwrap()
            .to_string()
            .replace("\"", ""),
        bucket: config
            .influxdb_conn
            .get("bucket")
            .unwrap()
            .to_string()
            .replace("\"", ""),
        measurement: config
            .influxdb_conn
            .get("measurement")
            .unwrap()
            .to_string()
            .replace("\"", ""),
        org: config
            .influxdb_conn
            .get("org")
            .unwrap()
            .to_string()
            .replace("\"", ""),
    };
    let sleep_interval = &config.sleep_interval;

    let conf_map = config.path_map;
    if let Value::Object(path_obj) = conf_map {
        for (tag, path) in path_obj.iter() {
            path_map.insert(tag.clone(), path.clone().to_string().replace("\"", ""));
        }
    }
    // println!("pathmap {:#?}", path_map);
    // println!("bucket {}", influxdb_connection.bucket);
    // println!("{:#?}", influxdb_connection);

    let client = influxdb_connect(&influxdb_connection).await;
    loop {
        system_metrics_main::mapdata(&client, &influxdb_connection, &path_map).await;

        if sleep_interval.contains("s") {
            let duration: u64 = sleep_interval.replace("s", "").parse().unwrap_or(10);
            std::thread::sleep(std::time::Duration::from_secs(duration));
        } else if sleep_interval.contains("m") {
            let duration: u64 = sleep_interval.replace("m", "").parse().unwrap_or(10);
            std::thread::sleep(std::time::Duration::from_secs(duration * 60));
        }
    }
}
