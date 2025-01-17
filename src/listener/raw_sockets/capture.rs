// Captura de dados brutos

use std::io;
use std::sync::Arc;
use std::time::Duration;
use pnet::datalink::{self, Channel::Ethernet, Config};
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::Packet;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ethernet::EtherTypes;
use crate::listener::metrics::TrafficMetrics;
use crate::listener::protocols::process_ip_packet;


/// Função principal que captura os pacotes da interface de rede em modo raw.
/// Essa função faz a captura de todos os pacotes e deixa o processamento
/// para um estágio posterior, como a separação por TCP, UDP, ICMP, etc.
#[allow(unused)]
pub fn capture_packets(interface_name: &str, metrics: &Arc<TrafficMetrics>) -> io::Result<()> {
    // Configuração do canal Ethernet
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
        .filter(|iface| iface.name == interface_name)
        .next()
        .expect("Interface de rede não encontrada");

    let config = Config {
        read_timeout: Some(Duration::from_millis(10)),
        ..Default::default()
    };

    let (_, mut rx) = match datalink::channel(&interface, config) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Erro: tipo de canal não suportado"),
        Err(e) => panic!("Erro ao abrir o canal: {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let ethernet_packet = EthernetPacket::new(packet).unwrap();
                process_packet(&ethernet_packet, metrics); // Passa métricas aqui
            }
            Err(e) => {
                eprintln!("Erro ao capturar o pacote: {:?}", e);
            }
        }
    }
}


/// Processa um pacote Ethernet capturado.
#[allow(unused)]
fn low_raw_packet(packet: &EthernetPacket) {
    println!("Capturado pacote: {:?}", packet);
}

#[allow(unused)]
fn process_packet(packet: &EthernetPacket, metrics: &Arc<TrafficMetrics>) {
    match packet.get_ethertype() {
        EtherTypes::Ipv4 => {
            if let Some(ipv4_packet) = Ipv4Packet::new(packet.payload()) {
                process_ip_packet(&ipv4_packet, metrics); // Passa `metrics`
            } else {
                println!("Erro ao processar pacote IPv4");
            }
        }
        _ => {
            println!("Tipo de pacote Ethernet não suportado: {:?}", packet.get_ethertype());
        }
    }
}
