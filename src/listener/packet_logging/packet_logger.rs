// Funções para log de pacotes

use std::fs::OpenOptions;
use std::io::Write;

pub fn log_packet(data: &[u8]) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("packet_logs.txt")
        .expect("Unable to open log file");

    writeln!(file, "{:?}", data).expect("Unable to write to log file");
}

