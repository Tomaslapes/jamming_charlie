// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ sync::Mutex};

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

struct DeviceConnectionStatusState {
    status: Mutex<DeviceConnectionStatus>,
}


struct SerialPortConnection{
    port: Option<serialport::Serial>

}

struct SerialPortConnectionState{
    connection: Mutex<SerialPortConnection>
}

#[tauri::command]
fn get_device_state(state: tauri::State<DeviceConnectionStatusState>) -> DeviceConnectionStatus {
    state.status.lock().unwrap().clone()
}

#[tauri::command]
fn connect(state: tauri::State<DeviceConnectionStatusState>, port_name: String) {
    let res = serialport::new(&port_name, 115_200).open();
    let mut status = state.status.lock().unwrap();
    match res {
        Ok(port) => {
            status.status = ConnectionStatus::Connected;
            status.port_name = Some(port_name.clone());
            status.device_name = port.name().clone();
            status.error = None;
            println!("Connected to port: {}", port_name);
        }
        Err(e) => {
            status.error = Some(format!("Error connecting to port: {}", e));
            println!("Error connecting to port: {}", e);
        }
    }
}

fn main() {
    tauri::Builder::default()
        .manage(DeviceConnectionStatusState {
            status: Mutex::new(DeviceConnectionStatus {
                status: ConnectionStatus::Disconnected,
                port_name: None,
                device_name: None,
                error: None,
            }),
        })
        .manage(SerialPortConnectionState{
            connection: Mutex::new(SerialPortConnection{
                port: None
            })
        })
        .invoke_handler(tauri::generate_handler![
            get_serial_ports,
            get_device_state,
            connect
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
