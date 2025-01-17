use crate::listener::metrics::TrafficMetrics;
use pnet::packet::icmp::IcmpPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use std::sync::Arc;

/// Processa pacotes ICMP e atualiza as métricas.
pub fn process_icmp_packet(ipv4_packet: &Ipv4Packet, metrics: &Arc<TrafficMetrics>) {
    if let Some(icmp_packet) = IcmpPacket::new(ipv4_packet.payload()) {
        let payload_size = ipv4_packet.payload().len() as u64;

        println!("=== Pacote ICMP ===");
        println!("Endereço IP de Origem: {}", ipv4_packet.get_source());
        println!("Endereço IP de Destino: {}", ipv4_packet.get_destination());
        println!("Tamanho do Payload: {} bytes", payload_size);

        // Atualiza as métricas
        metrics.add_data("ICMP", payload_size);

        // Informações adicionais do pacote ICMP
        println!("Tipo: {:?}", icmp_packet.get_icmp_type());
        println!("Código: {:?}", icmp_packet.get_icmp_code());
    } else {
        println!("Erro ao processar pacote ICMP");
    }
}
