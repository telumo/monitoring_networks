use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;

use std::env;

mod packets;
use packets::GettableEndPoints;

mod cli;

const WIDTH: usize = 20;

fn main() {
    let matches = cli::build_cli().get_matches();

    // interface
    let interface_name = matches.value_of("interface").unwrap();
    
    // 利用可能なインターフェースを取得
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter() // 値をわたしてVの要素を一つずつ検討
        .filter(|iface| iface.name == *interface_name)
        .next()
        .expect("Faild to get interface");

    let(_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx), // DataLinkSender, DataLinkReceiver
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create datalink channel {}", e)
    };

    loop {
        match rx.next() {
            Ok(frame) => {
                let frame = EthernetPacket::new(frame).unwrap();
                match frame.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        ipv4_handler(&frame);
                    },
                    EtherTypes::Ipv6 => {
                        ipv6_handler(&frame);
                    }
                    _ => {
                        println!("Not a ipv4 or ipv6");
                    }
                }
            },
            Err(e) => {
                panic!("Failed to read: {}", e);
            }
        }
    }
}

fn ipv4_handler(ethernet: &EthernetPacket) {
    if let Some(packet) = Ipv4Packet::new(ethernet.payload()){
        match packet.get_next_level_protocol() {
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(&packet);
            },
            IpNextHeaderProtocols::Udp => {
                udp_handler(&packet);
            },
            _ => {
                println!("Not a tcp or a udp packet");
            }
        }
    }
}

fn ipv6_handler(ethernet: &EthernetPacket) {
    if let Some(packet) = Ipv6Packet::new(ethernet.payload()){
        match packet.get_next_header() {
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(&packet);
            },
            IpNextHeaderProtocols::Udp => {
                udp_handler(&packet);
            },
            _ => {
                println!("Not a tcp or a udp packet");
            }
        }
    }
}

fn tcp_handler(packet: &GettableEndPoints) {
    let tcp = TcpPacket::new(packet.get_payload());
    if let Some(tcp) = tcp {
        print_packet_info(packet, &tcp, "TCP");
    }
}

fn udp_handler(packet: &GettableEndPoints) {
    let udp = UdpPacket::new(packet.get_payload());
    if let Some(udp) = udp {
        print_packet_info(packet, &udp, "UDP");
    }
}

fn print_packet_info(l3: &GettableEndPoints, l4: &GettableEndPoints, proto: &str) {
    // l3: ネットワーク層
    // l4: トランスポート層
    println!("Captured a {proto} packet from {l3_get_source}|{l4_get_source} to {l3_get_destination}|{l4_get_destination}\n",
        proto = proto,
        l3_get_source = l3.get_source(),
        l4_get_source = l4.get_source(),
        l3_get_destination = l3.get_destination(),
        l4_get_destination = l4.get_destination()
    );
    let payload = l4.get_payload();
    let len = payload.len();

    for i in 0..len {
        // 2桁0埋め, 16進数表示
        print!("{:<02X} ", payload[i]);

        // WIDTHの間隔で区切る
        if i%WIDTH == WIDTH-1 || i == len-1 {

            // もし中途半端だったら、WIDTHまでスペースで埋める
            for _j in 0..WIDTH-1-(i % (WIDTH)) {
                print!("   ");
            }

            // セパレート
            print!("| ");

            // アスキーで表示
            for j in i-i%WIDTH..i+1 {
                if payload[j].is_ascii_alphabetic() {
                    print!("{}", payload[j] as char);
                } else {
                    print!(".");
                }
            }

            // 改行
            print!("\n");
        }
    }
    println!("{}", "=".repeat(WIDTH * 3));
    print!("\n");
}