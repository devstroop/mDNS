use crate::config::{Config, Record};
use anyhow::Result;
use dns_parser::{Packet, QueryClass, QueryType};
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::sync::Arc;
use tokio::net::UdpSocket;

const MDNS_MULTICAST_ADDR: &str = "224.0.0.251";
const MDNS_PORT: u16 = 5353;

pub struct MdnsServer {
    config: Arc<Config>,
    records: HashMap<String, Vec<Record>>,
}

impl MdnsServer {
    pub fn new(config: Arc<Config>) -> Self {
        let mut records: HashMap<String, Vec<Record>> = HashMap::new();
        for zone in &config.zones {
            if zone.name == "local" {
                for record in &zone.records {
                    let key = format!("{}.local", record.name).to_lowercase();
                    records.entry(key).or_default().push(record.clone());
                }
            }
        }
        Self { config, records }
    }

    pub async fn run(&self) -> Result<()> {
        if !self.config.mdns.enabled {
            tracing::info!("mDNS disabled in config");
            return Ok(());
        }

        let socket = UdpSocket::bind(format!("0.0.0.0:{}", MDNS_PORT)).await?;

        let multicast_addr: Ipv4Addr = MDNS_MULTICAST_ADDR.parse().unwrap();
        let interface: Ipv4Addr = "0.0.0.0".parse().unwrap();
        socket.join_multicast_v4(multicast_addr, interface)?;

        tracing::info!(
            "mDNS server listening on {}:{}",
            MDNS_MULTICAST_ADDR,
            MDNS_PORT
        );

        let mut buf = [0u8; 65535];
        loop {
            let (len, peer) = socket.recv_from(&mut buf).await?;
            let data = &buf[..len];

            if let Some(response) = self.handle_query(data) {
                socket.send_to(&response, peer).await?;
            }
        }
    }

    fn handle_query(&self, data: &[u8]) -> Option<Vec<u8>> {
        let packet = Packet::parse(data).ok()?;

        if packet.questions.is_empty() {
            return None;
        }

        let question = &packet.questions[0];
        let qname = question
            .qname
            .to_string()
            .trim_end_matches('.')
            .to_lowercase();
        let qtype = question.qtype;

        tracing::debug!("mDNS Query: {} type: {:?}", qname, qtype);

        if question.qclass != QueryClass::IN {
            return None;
        }

        let answers: Vec<_> = self
            .records
            .get(&qname)
            .iter()
            .flat_map(|records| {
                records.iter().filter_map(|r| {
                    let matches = match qtype {
                        QueryType::A => r.record_type == "A",
                        QueryType::All => true,
                        _ => false,
                    };
                    if !matches {
                        return None;
                    }
                    Some(r.clone())
                })
            })
            .collect();

        if answers.is_empty() {
            return None;
        }

        Some(self.build_response(&packet, &answers))
    }

    fn build_response(&self, packet: &Packet, answers: &[Record]) -> Vec<u8> {
        let mut response = Vec::with_capacity(512);

        response.extend_from_slice(&packet.header.id.to_be_bytes());
        response.extend_from_slice(&[
            0x81,
            0x80,
            0x00,
            0x00,
            0x00,
            0x00,
            (answers.len() as u8),
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ]);

        for q in &packet.questions {
            self.write_name(&q.qname.to_string(), &mut response);
            response.extend_from_slice(&[0x00, 0x01]);
            response.extend_from_slice(&[0x00, 0x01]);
        }

        for a in answers {
            self.write_name(&format!("{}.", a.name), &mut response);
            response.extend_from_slice(&[0x00, 0x01]);
            response.extend_from_slice(&[0x00, 0x01]);
            response.extend_from_slice(&self.config.mdns.ttl.to_be_bytes());

            if a.record_type == "A" {
                response.push(4);
                if let Ok(ip) = a.value.parse::<Ipv4Addr>() {
                    response.extend_from_slice(&ip.octets());
                }
            }
        }

        response
    }

    fn write_name(&self, name: &str, buf: &mut Vec<u8>) {
        for label in name.split('.') {
            buf.push(label.len() as u8);
            buf.extend_from_slice(label.as_bytes());
        }
        buf.push(0);
    }
}
