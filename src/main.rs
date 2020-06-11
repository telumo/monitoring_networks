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

    
}
