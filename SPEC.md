# mdnsd - Specification

## Project Overview
- **Project Name**: mdnsd
- **Type**: Headless DNS/mDNS server
- **Core Functionality**: TOML-config-driven DNS server with mDNS multicast support
- **Target Users**: Developers needing a lightweight local DNS/mDNS solution

## Technology Stack
- **Language**: Rust
- **Async Runtime**: Tokio
- **Configuration**: TOML

## Configuration (config.toml)

```toml
[server]
host = "0.0.0.0"
port = 53

[mdns]
enabled = false
ttl = 300

[[zones]]
name = "local"

[[zones.records]]
name = "host"
type = "A"
value = "192.168.1.1"
ttl = 300
```

## Supported Record Types
- A (IPv4)
- AAAA (IPv6)
- PTR
- SRV
- TXT

## Project Structure
```
mdnsd/
├── src/
│   ├── main.rs       # Entry point
│   ├── config.rs     # TOML config parsing
│   ├── dns.rs        # DNS server
│   └── mdns.rs       # mDNS multicast
├── config.toml       # Default config
├── Cargo.toml
├── README.md
└── .gitignore
```

## Acceptance Criteria
- [x] Builds with `cargo build`
- [x] Loads configuration from TOML
- [x] Serves DNS queries
- [x] Returns correct DNS responses
- [x] Cross-platform