use pnet::packet::{ipv4::Ipv4Packet, tcp::TcpPacket, Packet};
use std::sync::Arc;
use crate::{ api::api::API_CONFIG, listener::metrics::TrafficMetrics };

pub fn process_tcp_packet(ipv4_packet: &Ipv4Packet, metrics: &Arc<TrafficMetrics>) {
    if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
        let src_ip = ipv4_packet.get_source();
        let dst_ip = ipv4_packet.get_destination();
        let src_port = tcp_packet.get_source();
        let dst_port = tcp_packet.get_destination();
        let seq_number = tcp_packet.get_sequence();
        let ack_number = tcp_packet.get_acknowledgement();
        let window_size = tcp_packet.get_window();
        let payload_size = ipv4_packet.payload().len() as u64;

        let src_domains = API_CONFIG.ip_api_lookup(&src_ip.to_string())
            .unwrap_or_else(|e| {
                eprintln!("Erro na consulta de origem {}: {}", src_ip, e);
                Vec::new()
            });
        
        let dst_domains = API_CONFIG.ip_api_lookup(&dst_ip.to_string())
            .unwrap_or_else(|e| {
                eprintln!("Erro na consulta de destino {}: {}", dst_ip, e);
                Vec::new()
            });

        println!("=== Pacote TCP ===");
        println!("Endereço IP de Origem: {}", src_ip);
        println!("Endereço IP de Destino: {}", dst_ip);
        println!("Porta de Origem: {}", src_port);
        println!("Porta de Destino: {}", dst_port);
        println!("Número de Sequência: {}", seq_number);
        println!("Número de Reconhecimento: {}", ack_number);
        println!("Tamanho da Janela: {}", window_size);
        println!("Tamanho do Payload: {} bytes", payload_size);
        println!("Domínios Associados à Origem: {:?}", src_domains);
        println!("Domínios Associados ao Destino: {:?}", dst_domains);
        metrics.add_data("TCP", payload_size);
    } else {
        println!("Erro ao processar pacote TCP");
    }
}
