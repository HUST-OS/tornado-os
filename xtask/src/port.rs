// ref: https://github.com/rustsbi/rustsbi-k210/blob/main/xtask/src/detect.rs
use serialport::{SerialPortType, UsbPortInfo};

pub fn detect_serial_ports() -> Option<(String, UsbPortInfo)> {
    let ports = serialport::available_ports().expect("list available ports");
    let mut ans = Vec::new();
    for p in ports {
        if let SerialPortType::UsbPort(info) = p.port_type {
            if info.vid == 0x1a86 && info.pid == 0x7523 {
                ans.push((p.port_name, info));
            }
        }
    }
    if ans.len() == 0 {
        None
    } else {
        Some(ans[0].clone())
    }
}
