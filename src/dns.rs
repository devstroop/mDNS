use crate::config::{Config, Record};
use anyhow::Result;
use dns_parser::{Packet, QueryClass, QueryType};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::UdpSocket;

pub struct DnsServer {
    config: Arc<Config>,
    records: HashMap<String, Vec<Record>>,
}

impl DnsServer {
    pub fn new(config: Arc<Config>) -> Self {
        let mut records: HashMap<String, Vec<Record>> = HashMap::new();
        for zone in &config.zones {
            for record in &zone.records {
                let key = format!("{}.{}", record.name, zone.name).to_lowercase();

                records.entry(key).or_default().push(record.clone());
            }
        }
        Self { config, records }
    }

    pub async fn run(&self) -> Result<()> {
        let addr = format!("{}:{}", self.config.server.host, self.config.server.port);
        let socket = UdpSocket::bind(&addr).await?;
        tracing::info!("DNS server listening on {}", addr);

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

        tracing::debug!("Query: {} type: {:?}", qname, qtype);

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
                        QueryType::AAAA => r.record_type == "AAAA",
                        QueryType::PTR => r.record_type == "PTR",
                        QueryType::SRV => r.record_type == "SRV",
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

        let qd_count = packet.questions.len() as u16;
        let an_count = answers.len() as u16;

        response.extend_from_slice(&packet.header.id.to_be_bytes());
        response.extend_from_slice(&[0x81, 0x80]);
        response.extend_from_slice(&qd_count.to_be_bytes());
        response.extend_from_slice(&an_count.to_be_bytes());
        response.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);

        let question_name_offset = 12;

        for q in &packet.questions {
            self.write_name(&q.qname.to_string(), &mut response);
            response.extend_from_slice(&[0x00, 0x01]);
            response.extend_from_slice(&[0x00, 0x01]);
        }

        for a in answers {
            if a.record_type == "A" {
                if let Ok(ip) = a.value.parse::<std::net::Ipv4Addr>() {
                    response.extend_from_slice(&[0xc0, question_name_offset]);
                    response.extend_from_slice(&[0x00, 0x01]);
                    response.extend_from_slice(&[0x00, 0x01]);
                    response.extend_from_slice(&a.ttl.to_be_bytes());
                    response.extend_from_slice(&[0x00, 0x04]);
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
