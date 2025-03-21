use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InfluxdbDetails {
    pub token: String,
    pub url: String,
    pub bucket: String,
    pub measurement: String,
    pub org: String,
}

use futures::prelude::*;
use influxdb2::{models::DataPoint, Client, RequestError};
pub async fn influxdb_connect(conf: &InfluxdbDetails) -> Client {
    let token = &conf.token;
    let host = &conf.url;
    let org = &conf.org;
    let client = Client::new(host, org, token);
    return client;
}

 

pub async fn influxdb2_write_data(client: &Client,config:&InfluxdbDetails, data: &HashMap<String,f64>, source_tag: String) -> Result<(), RequestError>{

    let bucket = config.bucket.as_str();
    let measurement = config.measurement.as_str();

    let mut point = DataPoint::builder(measurement).tag("source", source_tag);
    for (key,value) in data{

        point = point.field(key, *value);
    }

    let data_points = vec![point.build().unwrap()];

    client.write(bucket, stream::iter(data_points),).await
}