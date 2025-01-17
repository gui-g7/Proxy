// Captura de dados brutos

use std::io;
use std::net::{IpAddr, Ipv4Addr};
use std::os::unix::io::AsRawFd;
use std::time::Duration;

use pnet::datalink::{self, Channel::Ethernet, Config, NetworkInterface};
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::Packet;

/// Função principal que captura os pacotes da interface de rede em modo raw.
/// Essa função faz a captura de todos os pacotes e deixa o processamento
/// para um estágio posterior, como a separação por TCP, UDP, ICMP, etc.
pub fn capture_packets(interface_name: &str) -> io::Result<()> {
    // Procurar a interface de rede correta pelo nome
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
        .filter(|iface| iface.name == interface_name)
        .next()
        .expect("Interface de rede não encontrada");

    // Configuração do canal de captura
    let config = Config {
        read_timeout: Some(Duration::from_millis(10)),
        ..Default::default()
    };

    // Abre um canal Ethernet para captura de pacotes em modo raw
    let (_, mut rx) = match datalink::channel(&interface, config) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Erro: tipo de canal não suportado"),
        Err(e) => panic!("Erro ao abrir o canal: {}", e),
    };

    // Loop principal de captura de pacotes
    loop {
        match rx.next() {
            Ok(packet) => {
                let ethernet_packet = EthernetPacket::new(packet).unwrap();
                process_packet(&ethernet_packet);
            }
            Err(e) => {
                eprintln!("Erro ao capturar o pacote: {:?}", e);
            }
        }
    }
}

/// Processa um pacote Ethernet capturado.
/// Aqui, você pode adicionar a lógica de identificação de protocolo e redirecionamento
/// para o módulo correto (TCP, UDP, ICMP).
fn process_packet(packet: &EthernetPacket) {
    println!("Capturado pacote: {:?}", packet);
}

use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::Packet;

use crate::listener::protocols::process_ip_packet;

fn process_packet(packet: &EthernetPacket) {
    match packet.get_ethertype() {
        EtherTypes::Ipv4 => {
            if let Some(ipv4_packet) = Ipv4Packet::new(packet.payload()) {
                process_ip_packet(&ipv4_packet);
            } else {
                println!("Erro ao processar pacote IPv4");
            }
        } _ => {
            println!("Tipo de pacote Ethernet não suportado: {:?}", packet.get_ethertype());
        }
    }
}

