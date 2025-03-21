use std::fs::File;
use serde_json::Value;
use std::io::BufReader;


pub fn read_config(conf_path: &String) -> Value {
    let file = File::open(conf_path).expect("unable to open the file");
    let reader = BufReader::new(file);
    
    let configuration = serde_json::from_reader(reader).expect("Error while reading file");
    // let configuration: Config = serde_json::from_value(configuration).unwrap();
    configuration
}