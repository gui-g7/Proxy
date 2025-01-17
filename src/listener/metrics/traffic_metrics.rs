mod traffic_metrics;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fmt;

use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;

use crate::listener::protocols::{icmp, tcp, udp};

/// Representa os dados acumulados por protocolo.
#[derive(Default)]
pub struct TrafficMetrics {
    pub data: Mutex<HashMap<String, u64>>, // Armazena bytes por protocolo
}

impl TrafficMetrics {
    pub fn new() -> Self {
        TrafficMetrics {
            data: Mutex::new(HashMap::new()),
        }
    }

    /// Incrementa o contador de bytes para um protocolo específico.
    pub fn add_data(&self, protocol: &str, bytes: u64) {
        let mut data = self.data.lock().unwrap();
        *data.entry(protocol.to_string()).or_insert(0) += bytes;
    }

    /// Retorna as métricas formatadas.
    pub fn formatted(&self) -> String {
        let data = self.data.lock().unwrap();
        data.iter()
            .map(|(protocol, &bytes)| {
                format!(
                    "{}: {:.2} {}",
                    protocol,
                    Self::convert_bytes(bytes).0,
                    Self::convert_bytes(bytes).1
                )
            })
            .collect::<Vec<String>>()
            .join(", ")
    }

    /// Converte bytes para a unidade apropriada.
    fn convert_bytes(bytes: u64) -> (f64, &'static str) {
        const KB: f64 = 1024.0;
        const MB: f64 = 1024.0 * KB;
        const GB: f64 = 1024.0 * MB;

        if bytes as f64 >= GB {
            (bytes as f64 / GB, "GB")
        } else if bytes as f64 >= MB {
            (bytes as f64 / MB, "MB")
        } else if bytes as f64 >= KB {
            (bytes as f64 / KB, "KB")
        } else {
            (bytes as f64, "Bytes")
        }
    }
}

impl fmt::Debug for TrafficMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatted())
    }
}

#[allow(unused)]
pub fn process_ip_packet(packet: &Ipv4Packet, metrics: &Arc<TrafficMetrics>) {
    match packet.get_next_level_protocol() {
        IpNextHeaderProtocols::Tcp => {
            tcp::process_tcp_packet(packet, metrics);
        }
        IpNextHeaderProtocols::Udp => {
            udp::process_udp_packet(packet, metrics);
        }
        IpNextHeaderProtocols::Icmp => {
            icmp::process_icmp_packet(packet, metrics);
        }
        _ => {
            println!("Protocolo não suportado: {:?}", packet.get_next_level_protocol());
        }
    }
}
