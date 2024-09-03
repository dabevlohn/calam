use clamav_client::{clean, scan_file, Tcp};
use std::path::PathBuf;

/*
pub fn check_avail(clamd_tcp: Tcp) {
    // Ping clamd to make sure the server is available and accepting TCP connections
    let clamd_available = match ping(clamd_tcp) {
        Ok(ping_response) => ping_response == clamav_client::PONG,
        Err(_) => false,
    };

    if !clamd_available {
        println!("Cannot ping clamd at {}", clamd_tcp.host_address);
        return;
    }
    assert!(clamd_available);
}
*/

pub fn clam_scan(host: String, port: u8, file_path: PathBuf) {
    // Scan file for viruses
    let clamd_tcp = Tcp {
        host_address: format!("{}:{}", host, port),
    };

    let scan_file_response = scan_file(file_path, clamd_tcp, None).unwrap();
    let file_clean = clean(&scan_file_response).unwrap();
    if file_clean {
        println!("No virus found");
    } else {
        println!("The file is infected!");
    }
    assert!(!file_clean);
}
