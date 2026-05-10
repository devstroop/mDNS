mod config;
mod dns;
mod mdns;

use anyhow::Result;
use clap::Parser;
use config::Config;
use std::sync::Arc;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::layer::SubscriberExt;

#[derive(Parser, Debug)]
#[command(name = "mdnsd")]
#[command(about = "Headless mDNS/DNS server", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.toml")]
    config: String,

    #[arg(short, long)]
    port: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mdnsd=debug,info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .ok();

    let args = Args::parse();

    let config = Config::from_file(&args.config)?;
    tracing::info!("Loaded config from {}", args.config);

    let config = Arc::new(config);

    let dns_config = config.clone();
    let mdns_config = config.clone();

    let _dns_handle = tokio::spawn(async move {
        let server = dns::DnsServer::new(dns_config);
        if let Err(e) = server.run().await {
            tracing::error!("DNS server error: {}", e);
        }
    });

    let _mdns_handle = tokio::spawn(async move {
        let server = mdns::MdnsServer::new(mdns_config);
        if let Err(e) = server.run().await {
            tracing::error!("mDNS server error: {}", e);
        }
    });

    let _ = tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
        }
    }).await;

    Ok(())
}