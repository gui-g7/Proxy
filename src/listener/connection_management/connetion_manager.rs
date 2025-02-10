use crate::listener::types::{
    ConnectionIdentifier, 
    ConnectionState
};
use crate::listener::metrics::TrafficMetrics;
use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddrV4},
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};
use pnet::packet::ip::IpNextHeaderProtocol;

#[allow(unused)]
#[derive(Debug)]
pub struct Connection {
    pub identifier: ConnectionIdentifier,
    pub state: ConnectionState,
    pub last_activity: SystemTime,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

#[allow(unused)]
pub struct ConnectionManager {
    connections: Arc<Mutex<HashMap<ConnectionIdentifier, Connection>>>,
    max_idle_time: Duration,
    metrics: Arc<TrafficMetrics>,
}

#[allow(unused)]
impl ConnectionManager {
    pub fn new(max_idle_time: Duration, metrics: Arc<TrafficMetrics>) -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
            max_idle_time,
            metrics,
        }
    }

    pub fn update_connection(
        &self,
        protocol: IpNextHeaderProtocol,
        src_ip: Ipv4Addr,
        src_port: u16,
        dst_ip: Ipv4Addr,
        dst_port: u16,
        payload_size: usize,
    ) {
        let identifier = ConnectionIdentifier {
            protocol,
            source: SocketAddrV4::new(src_ip, src_port),
            destination: SocketAddrV4::new(dst_ip, dst_port),
        };

        let mut connections = self.connections.lock().unwrap();
        let now = SystemTime::now();

        let connection = connections.entry(identifier.clone()).or_insert_with(|| Connection {
            identifier: identifier.clone(),
            state: ConnectionState::New,
            last_activity: now,
            bytes_sent: 0,
            bytes_received: 0,
        });

        let is_outgoing = self.is_outgoing_packet(src_ip);
        let data_size = payload_size as u64;

        if is_outgoing {
            connection.bytes_sent += data_size;
            self.metrics.add_data("TCP", data_size);
        } else {
            connection.bytes_received += data_size;
            self.metrics.add_data("TCP", data_size);
        }

        connection.last_activity = now;
        self.update_connection_state(connection);
    }

    fn is_outgoing_packet(&self, src_ip: Ipv4Addr) -> bool {
        src_ip.is_private()
    }

    fn update_connection_state(&self, connection: &mut Connection) {
        connection.state = match connection.state {
            ConnectionState::New => ConnectionState::Established,
            ConnectionState::Closing => ConnectionState::Closed,
            _ => ConnectionState::Established,
        };
    }
}
