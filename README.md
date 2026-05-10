# mDNS

A headless, TOML-config-driven DNS/mDNS server written in Rust.

## Features

- DNS server on port 53
- mDNS multicast support (port 5353)
- TOML-based configuration
- Cross-platform (Linux, macOS, Windows)

## Quick Start

```bash
cargo run
```

Test with:
```bash
dig @127.0.0.1 -p 53 host.local +short
# Output: 192.168.1.1
```

## Configuration

Edit `config.toml`:

```toml
[server]
host = "0.0.0.0"
port = 53

[mdns]
enabled = true
ttl = 300

[[zones]]
name = "local"

[[zones.records]]
name = "host"
type = "A"
value = "192.168.1.1"
```

### Record Types

- `A` - IPv4 address
- `AAAA` - IPv6 address
- `PTR` - Pointer record
- `SRV` - Service record
- `TXT` - Text record

## Options

```bash
mDNS --config custom.toml --port 5353
```

- `--config`, `-c`: Config file path (default: config.toml)
- `--port`, `-p`: Override DNS port

## API

None - purely TOML-driven.

## License

MIT