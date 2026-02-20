// ICMP Ping Flood
/*
Contar cuántos paquetes ICMP tipo Echo Request (type == 8) se enviaron por cada IP.
Si una IP ha enviado más de X (e.g., 100) se considerará sospechosa.
*/


use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;


use pcap::Capture;


use colored::*;


use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::icmp::IcmpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;


fn display_icmp_packet(icmp_packet: &IcmpPacket) {
    println!(
        "        {}",
        format!(
            "ICMP Packet: {:?}, {:?}",
            icmp_packet.get_icmp_type(),
            icmp_packet.get_icmp_code()
        )
        .yellow()
    );
}


fn main() -> Result<(), String> {
    let mut icmp_ping_flood: HashMap<Ipv4Addr, u32> = HashMap::new();


    let Some(file_name) = env::args().nth(1) else {
        return Err(String::from("Tienes que especificar en la línea de comandos el nombre del fichero pcap."))
    };


    let Ok(mut cap) = Capture::from_file(&file_name) else {
        return Err(format!("El fichero {} no existe o no contiene una captura pcap.", file_name))
    };


    while let Ok(packet) = cap.next_packet() {
        if let Some(ethernet_packet) = EthernetPacket::new(&packet.data) {
            match ethernet_packet.get_ethertype() {
                EtherTypes::Ipv4 => {
                    if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                        match ipv4_packet.get_next_level_protocol() {
                            IpNextHeaderProtocols::Icmp => {
                                if let Some(icmp_packet) = IcmpPacket::new(ipv4_packet.payload()) {
                                    // display_icmp_packet(&icmp_packet);


                                    if icmp_packet.get_icmp_type() == pnet::packet::icmp::IcmpType(8) {
                                        if let Some(total) = icmp_ping_flood.get(&ipv4_packet.get_source()) {
                                            icmp_ping_flood.insert(ipv4_packet.get_source(), total + 1);
                                        } else {
                                            icmp_ping_flood.insert(ipv4_packet.get_source(), 1);
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }


    for e in icmp_ping_flood { // si me piden que a partir de las 25 pings es sospechoso, se pone un if englobando a este print, if e.1 >= 25 print...
        println!("({}: {})", e.0, e.1);
    }


    return Ok(())
} // Al terminar el ámbito en el que se declara arp_file se hace drop, lo que cierra el fichero
