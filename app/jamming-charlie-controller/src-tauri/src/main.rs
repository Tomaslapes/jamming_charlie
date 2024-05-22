// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use serde::{Deserialize, Serialize};
use serialport::{available_ports, SerialPortInfo};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_serial_ports() -> Result<Vec<SerialPortInfo>, String> {
    match available_ports() {
        Ok(ports) => Ok(ports),
        Err(_e) => Err("Error getting ports".to_string()),
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeviceConnectionStatus {
    status: ConnectionStatus,
    port_name: Option<String>,
    device_name: Option<String>,
    error: Option<String>,
}

#[tauri::command]
fn get_device_state(state: tauri::State<DeviceConnectionStatus>) -> DeviceConnectionStatus {
    (*state).clone()
}


#[tauri::command]
fn connect(port_name: String) {
    let port = serialport::new(&port_name, 115_200).open().unwrap();
    println!("Connected!");
    println!("{}",&port_name);
    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_serial_ports, get_device_state, connect])
        .manage(DeviceConnectionStatus {
            status: ConnectionStatus::Disconnected,
            port_name: None,
            device_name: None,
            error: None,
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
