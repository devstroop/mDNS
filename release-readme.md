# mdnsd - DNS/mDNS Server

A headless, TOML-config-driven DNS/mDNS server.

## Files Included

- `mdnsd` - Binary executable
- `config.toml` - Configuration template
- `mdnsd.service` - Systemd service unit (Linux)

## Configuration

Edit `config.toml` to add DNS records:

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
name = "myhost"
type = "A"
value = "192.168.1.100"
```

### Record Types
- `A` - IPv4 address
- `AAAA` - IPv6 address
- `PTR` - Pointer record
- `SRV` - Service record
- `TXT` - Text record

---

## Installation

### Linux (systemd)

1. Create directory and copy files:
```bash
sudo mkdir -p /etc/mdnsd
sudo cp mdnsd /usr/local/bin/
sudo cp config.toml /etc/mdnsd/config.toml
sudo cp mdnsd.service /etc/systemd/system/
```

2. Enable and start:
```bash
sudo systemctl daemon-reload
sudo systemctl enable mdnsd
sudo systemctl start mdnsd
```

3. Check status:
```bash
sudo systemctl status mdnsd
```

### macOS (Homebrew service)

1. Copy binary and config:
```bash
sudo cp mdnsd /usr/local/bin/
sudo mkdir -p /usr/local/etc/mdnsd
sudo cp config.toml /usr/local/etc/mdnsd/config.toml
```

2. Create launchd plist:
```bash
sudo nano /Library/LaunchDaemons/com.devstroop.mdnsd.plist
```

Add:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.devstroop.mdnsd</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/mdnsd</string>
        <string>-c</string>
        <string>/usr/local/etc/mdnsd/config.toml</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
```

3. Load service:
```bash
sudo launchctl load /Library/LaunchDaemons/com.devstroop.mdnsd.plist
```

### Windows (NSSM service)

1. Create directories:
```cmd
mkdir C:\mdnsd
copy mdnsd.exe C:\mdnsd\
copy config.toml C:\mdnsd\
```

2. Download NSSM: https://nssm.cc/download

3. Install service:
```cmd
nssm install mdnsd C:\mdnsd\mdnsd.exe "-c C:\mdnsd\config.toml"
nssm start mdnsd
```

Or use PowerShell service:
```powershell
New-Service -Name mdnsd -BinaryPathName "C:\mdnsd\mdnsd.exe -c C:\mdnsd\config.toml" -StartupType Automatic
Start-Service mdnsd
```

---

## Testing

```bash
dig @127.0.0.1 -p 53 myhost.local
```

---

## Troubleshooting

### Check if port 53 is available
```bash
sudo lsof -i :53
```

### View logs (Linux)
```bash
sudo journalctl -u mdnsd -f
```

### Check config syntax
```bash
./mdnsd --config /etc/mdnsd/config.toml
```

---

## Ports Used

- DNS: 53 (UDP)
- mDNS: 5353 (UDP) - only if enabled