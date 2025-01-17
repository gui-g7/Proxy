use listener::protocols::icmp::process_icmp_packet;
use listener::protocols::process_ip_packet;
use listener::protocols::tcp::process_tcp_packet;
use listener::protocols::udp::process_udp_packet;
use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::{Packet, ipv4::Ipv4Packet};
use pnet::packet::ethernet::EthernetPacket;
use std::sync::Arc;
use listener::metrics::TrafficMetrics;
mod listener;

fn main() {
    // Inicializa as métricas de tráfego compartilhadas entre os protocolos
    let metrics = Arc::new(TrafficMetrics::new());

    // Lista todas as interfaces de rede disponíveis e seleciona a interface principal.
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
        .filter(|iface| iface.is_up() && !iface.is_loopback() && iface.is_broadcast())
        .next()
        .expect("Não foi possível encontrar uma interface de rede disponível");

    // Inicia a captura de pacotes na interface selecionada
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Canal de captura não é Ethernet"),
        Err(e) => panic!("Erro ao criar canal de captura: {}", e),
    };

    println!("Capturando pacotes na interface: {}", interface.name);

    // Cria uma thread para exibir métricas periodicamente
    let metrics_logger = metrics.clone();
    std::thread::spawn(move || {
        loop {
            println!("Métricas acumuladas: {:?}", metrics_logger);
            std::thread::sleep(std::time::Duration::from_secs(10));
        }
    });

    // Loop principal de captura de pacotes
    loop {
        // Recebe o pacote em baixo nível (Ethernet)
        match rx.next() {
            Ok(frame) => {
                let ethernet_packet = EthernetPacket::new(frame).unwrap();

                // Apenas processa pacotes IPv4
                if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                    let packet_size = ipv4_packet.payload().len() as u64;

                    // Identifica o protocolo (TCP, UDP, ICMP) e processa o pacote
                    match ipv4_packet.get_next_level_protocol() {
                        pnet::packet::ip::IpNextHeaderProtocols::Tcp => {
                            process_tcp_packet(&ipv4_packet, &metrics);
                            metrics.add_data("TCP", packet_size);
                        }
                        pnet::packet::ip::IpNextHeaderProtocols::Udp => {
                            process_udp_packet(&ipv4_packet, &metrics);
                            metrics.add_data("UDP", packet_size);
                        }
                        pnet::packet::ip::IpNextHeaderProtocols::Icmp => {
                            process_icmp_packet(&ipv4_packet, &metrics);
                            metrics.add_data("ICMP", packet_size);
                        }
                        _ => {
                            println!("Protocolo não suportado");
                        }
                    }
                    if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                        process_ip_packet(&ipv4_packet, &metrics); // Passa `metrics` para o processador
                    }
                }
            }
            Err(e) => {
                eprintln!("Erro ao capturar pacote: {}", e);
            }
        }
    }
}
