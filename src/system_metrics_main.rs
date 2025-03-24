use fs_extra::dir::get_size;
use influxdb2::{Client, RequestError};
use std::{collections::HashMap, fs::read};
use sysinfo::{CpuRefreshKind, Disks, RefreshKind, System, ProcessesToUpdate, ProcessRefreshKind};

use crate::influxdb2_helper::{influxdb2_write_data, InfluxdbDetails};

const GB: u64 = 1024 * 1024 * 1024;

fn read_system(source_hashmap: &mut HashMap<String, HashMap<String, f64>>) {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut tags_hashmap: HashMap<String, f64> = HashMap::new();
    let source_tag = String::from("system");
    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        let available_disk = (disk.total_space() / GB) as f64;
        let used_disk = (available_disk as u64 - disk.available_space() / GB) as f64;
        tags_hashmap.insert("system_available_disk_gb".to_string(), available_disk);
        tags_hashmap.insert("system_used_disk_gb".to_string(), used_disk);
    }
    tags_hashmap.insert(
        "system_used_ram_gb ".to_string(),
        sys.used_memory() as f64 / GB as f64,
    );
    tags_hashmap.insert(
        "system_available_ram_gb ".to_string(),
        sys.total_memory() as f64 / GB as f64,
    );

    sys.refresh_cpu_specifics(CpuRefreshKind::everything());
    // Wait a bit because CPU usage is based on diff.
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    // Refresh CPUs again to get actual value.
    sys.refresh_cpu_usage();
    tags_hashmap.insert(
        "system_used_cpu_percent ".to_string(),
        sys.global_cpu_usage() as f64,
    );
    tags_hashmap.insert(
        "system_available_cpu_percent ".to_string(),
        100.0 as f64 - sys.global_cpu_usage() as f64,
    );
    source_hashmap.insert(source_tag, tags_hashmap);


    let source_tag = String::from("service");
    let mut tags_hashmap: HashMap<String, f64> = HashMap::new();

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
// Refresh CPU usage to get actual value.
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing().with_cpu()
    );
    
    let mut ram_usage: u64 = 0;
    let mut cpu_usage: f32 = 0.0;
    for process in sys.processes_by_name("mongod".as_ref()) {
        ram_usage += process.memory();
        cpu_usage += process.cpu_usage();
    }
    tags_hashmap.insert(
        "mongodb_service_ram_gb".to_string(),
        ram_usage as f64/GB as f64,
    );
    tags_hashmap.insert(
        "mongodb_service_cpu_percent".to_string(),
        cpu_usage as f64,
    );


    let mut ram_usage: u64 = 0;
    let mut cpu_usage: f32 = 0.0;
    for process in sys.processes_by_name("influxd".as_ref()) {
        ram_usage += process.memory();
        cpu_usage += process.cpu_usage();
    }
    tags_hashmap.insert(
        "influxd_service_ram_gb".to_string(),
        ram_usage as f64/GB as f64,
    );
    tags_hashmap.insert(
        "influxd_service_cpu_percent".to_string(),
        cpu_usage as f64,
    );

    
    let mut ram_usage: u64 = 0;
    let mut cpu_usage: f32 = 0.0;
    for process in sys.processes_by_exact_name("nmea-parser".as_ref()) {
        ram_usage += process.memory();
        cpu_usage += process.cpu_usage();
    }
    tags_hashmap.insert(
        "adaq_nmea_5001_service_ram_gb".to_string(),
        ram_usage as f64/GB as f64,
    );
    tags_hashmap.insert(
        "adaq_nmea_5001_service_cpu_percent".to_string(),
        cpu_usage as f64,
    );
    source_hashmap.insert(source_tag, tags_hashmap);
    // source_hashmap
}




fn read_storage(
    source_hashmap: &mut HashMap<String, HashMap<String, f64>>,
    path_map: &HashMap<String, String>,
) {
    let source = String::from("storage");
    let mut tags_hashmap: HashMap<String, f64> = HashMap::new();
    let path_map = path_map;
    for (key, value) in path_map.iter() {
        let size = get_size(value).unwrap_or_default();
        tags_hashmap.insert(key.to_string(), size as f64 / GB as f64);
    }
    source_hashmap.insert(source, tags_hashmap);
    // return  source_hashmap;
}




pub async fn mapdata(
    client: &Client,
    influxdb_details: &InfluxdbDetails,
    path_map: &HashMap<String, String>,
) {
    let mut source_hashmap: HashMap<String, HashMap<String, f64>> = HashMap::new();
    read_system(&mut source_hashmap);
    read_storage(&mut source_hashmap, path_map);
    // println!("{:#?}", source_hashmap);
    for data in source_hashmap.iter() {
        let result =
            influxdb2_write_data(&client, influxdb_details, data.1, data.0.to_string()).await;
        match result {
            Ok(_) => {}
            Err(err) => {
                log::error!("error writing to influxdb {}", err)
            }
        };
    }
}
