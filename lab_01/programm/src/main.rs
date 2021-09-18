use sysinfo::{SystemExt, DiskExt, ProcessExt};
use reqwest;
use serde::{
    Deserialize,
    Serialize,
};
use serde_json;
use std::io::stdin;
use std::fs::File;
use std::io::prelude::*;

const SERVER_URI: &str = "http://localhost:8000";

#[derive(Deserialize)]
pub struct Response {
    is_allowed: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Request {
    code: String,
    hash: String,
}

const MACHINE_ID_FILE: &str = "/var/lib/dbus/machine-id";

fn main() {

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Expected to write registration code");
    let code = buffer.trim_end();
    if code.len() == 0 {
        println!("Expected to write registration code");
        return;
    }

    if let Ok(is_register) = check_registration(String::from(code)) {
        if is_register {
                println!("Check passed!");
                worker();
        } else {
            println!("Secure error!");
            insult();
        }
    } else {
        println!("Error in server access, try again later");
    }
}

fn worker() {
    println!("Hello, sweety!")
}

fn insult() {
    println!("Dirty pirot, ama call FBI!");
}

fn check_registration(code: String) -> Result<bool, String> {
    let hardware_hashkey = get_dbus_machine_id();
    let request = form_request(code, hardware_hashkey);

    let uri = reqwest::Url::parse(SERVER_URI).unwrap();
    let client = reqwest::blocking::Client::new();
    let res = client.post(uri)
        .timeout(std::time::Duration::from_millis(100))
        .body(serde_json::to_string(&request).unwrap())
        .send();
    if let Ok(response) = res {
        if let Ok(text_response) = response.text() {
            if let Ok(response) = serde_json::from_str::<Response>(text_response.as_str()) {
                return Ok(response.is_allowed);
            }
        }
    }

    Err(String::from("Error in server access"))
}

fn form_request(code: String, hardware_hashkey: String) -> Request {
    Request {
        code: code,
        hash: hardware_hashkey,
    }
}

fn get_hardware_hash() -> String {
    let mut system = sysinfo::System::new_all();
    let mut hash = String::new();
    system.refresh_all();
    for disk in system.get_disks() {
        let disk_type = disk.get_type();
        let hash_disk_type = match disk_type {
            sysinfo::DiskType::HDD => 0,
            sysinfo::DiskType::SSD => 1,
            sysinfo::DiskType::Unknown(id) => id
        };

        hash.push_str(hash_disk_type.to_string().as_str());
        hash.push_str(std::str::from_utf8(disk.get_file_system()).expect("Error in gathering sys info"));
        hash.push_str(disk.get_total_space().to_string().as_str());
    }

    hash
}

fn get_dbus_machine_id() -> String {
    let mut cred: Vec<u8> = Vec::new();
    { 
        if let Ok(mut file) = File::open(MACHINE_ID_FILE) {
            let _ = file.read_to_end(&mut cred);
        };
    };
    if let Ok(str_res) = std::str::from_utf8(cred.as_slice()) {
        String::from(str_res)
    } else {
        String::default()
    }
}