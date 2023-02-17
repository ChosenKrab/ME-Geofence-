use std::env;
use std::fs::{File, OpenOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{

    // deserializing the JSON into structures for easier access

    #[derive(Debug, Serialize, Deserialize)]
    struct LocationSettings {
        device: HashMap<String, String>
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct IpSettings {
        priority: u64,
        location: HashMap<String, LocationSettings>
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct Settings {
        ip_settings: HashMap<String, IpSettings>
    }

    // accessing the first argument given when starting the program
    let joined = env::args().nth(1).unwrap();
    // accessing the JSON config file, opening it and deserializing it with the the structures above
    let config_path = "/opt/sms_distributor/sms_config.json".to_string();
    let config_file = File::open(config_path)?;
    let config: Settings = serde_json::from_reader(config_file)?;
    // accessing the file meant for storing the current ips in the network
    let mut log_file = OpenOptions::new().read(true).write(true).append(true).create(true).open("").unwrap();
    let mut logs = String::new();
    log_file.read_to_string(&mut logs)?;
    let mut logging: Vec<&str> = logs.lines().map(|l| l).collect();

    // checking if the ip given by the first argument is included in the config file
    if let Some(ip_settings) = config.ip_settings.get(&joined){
        // checking if the given ip is already logged in
        if logging.contains(&joined.as_str()) {
            // iterating through the vec to find the current ip, removing it from the vector and updating the file.
            for (i, adress) in logging.iter().enumerate(){
                logging.remove(i);
                log_file.;
            }
        } 
        // appending the ip to the file
        else {
            write!(log_file,"{}", joined);
        }

        // looping over the ip adress to access every location, device and given setting for said device
        for (location, location_settings) in ip_settings.location.iter(){
            for (device, device_settings) in location_settings.device.iter() {
                println!("{} {} {} {:?}", joined, location, device, device_settings)
            }
        }
    }
    Ok(())
}
