use pnet::packet::{ipv4::Ipv4Packet, udp::UdpPacket, Packet};
use std::sync::Arc;
use crate::{ api::api::API_CONFIG, listener::metrics::TrafficMetrics };

pub fn process_udp_packet(ipv4_packet: &Ipv4Packet, metrics: &Arc<TrafficMetrics>) {
    if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
        let src_ip = ipv4_packet.get_source();
        let dst_ip = ipv4_packet.get_destination();
        let src_port = udp_packet.get_source();
        let dst_port = udp_packet.get_destination();
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

        println!("=== Pacote UDP ===");
        println!("Endereço IP de Origem: {}", src_ip);
        println!("Endereço IP de Destino: {}", dst_ip);
        println!("Porta de Origem: {}", src_port);
        println!("Porta de Destino: {}", dst_port);
        println!("Domínios Associados à Origem: {:?}", src_domains);
        println!("Domínios Associados ao Destino: {:?}", dst_domains);
        println!("Tamanho do Payload: {} bytes", payload_size);

        // Atualiza as métricas
        metrics.add_data("UDP", payload_size);
    } else {
        println!("Erro ao processar pacote UDP");
    }
}
