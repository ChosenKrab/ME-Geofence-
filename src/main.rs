use std::env;
use std::fs::File;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use mosquitto_client::Mosquitto;

fn main() -> std::io::Result<()>{

    // deserializing the JSON into structures for easier access

    #[derive(Debug, Serialize, Deserialize)]
    struct LocationSettings {
        device: HashMap<String, String>
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct IpSettings {
        location: HashMap<String, LocationSettings>
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct Settings {
        ip_settings: HashMap<String, IpSettings>,
        client : String
    }

    // accessing the first argument given when starting the program
    let joined = env::args().nth(1).unwrap();
    // accessing the JSON config file, opening it and deserializing it with the the structures above
    let config_path = "/mnt/c/Users/dda/rust_projects/raspberrypi/config/config.json".to_string();
    let config_file = File::open(config_path)?;
    let config: Settings = serde_json::from_reader(config_file)?;

    // creating a new client and connecting it to mosquitto
    let m = Mosquitto::new(&config.client);
    m.connect("localhost", 1883).expect("can't connect");

    // checking if the ip given by the first argument is included in the config file
    if let Some(ip_settings) = config.ip_settings.get(&joined){
         // looping over the ip adress to access every location, device and given setting for said device
        for (location, location_settings) in ip_settings.location.iter(){
            for (device, device_settings) in location_settings.device.iter() {
                // saving the current values as variables to be used in the pubish funktion
                let topic:&str = &(location.to_owned()+ "/" + device);
                let payload = device_settings.as_bytes();
                // sending the values over the mosquitto mqtt broker
                m.publish(topic, payload, 1, false).unwrap();
            }
        }
        // disconnects the cloned client
        m.disconnect().unwrap();
    }
    Ok(())
}
