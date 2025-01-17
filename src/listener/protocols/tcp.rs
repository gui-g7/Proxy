use pnet::packet::tcp::TcpPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use std::sync::Arc;
use crate::listener::metrics::TrafficMetrics; // Importa TrafficMetrics corretamente

/// Processa pacotes TCP e atualiza as métricas.
pub fn process_tcp_packet(ipv4_packet: &Ipv4Packet, metrics: &Arc<TrafficMetrics>) {
    if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
        let src_port = tcp_packet.get_source();
        let dst_port = tcp_packet.get_destination();
        let seq_number = tcp_packet.get_sequence();
        let ack_number = tcp_packet.get_acknowledgement();
        let window_size = tcp_packet.get_window();

        let payload_size = ipv4_packet.payload().len() as u64;

        println!("=== Pacote TCP ===");
        println!("Endereço IP de Origem: {}", ipv4_packet.get_source());
        println!("Endereço IP de Destino: {}", ipv4_packet.get_destination());
        println!("Porta de Origem: {}", src_port);
        println!("Porta de Destino: {}", dst_port);
        println!("Número de Sequência: {}", seq_number);
        println!("Número de Reconhecimento: {}", ack_number);
        println!("Tamanho da Janela: {}", window_size);
        println!("Tamanho do Payload: {} bytes", payload_size);

        // Atualiza as métricas
        metrics.add_data("TCP", payload_size);
    } else {
        println!("Erro ao processar pacote TCP");
    }
}
