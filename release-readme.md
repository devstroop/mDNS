# mDNS - DNS/mDNS Server

A headless, TOML-config-driven DNS/mDNS server.

## Files Included

- `mDNS` - Binary executable
- `config.toml` - Configuration template
- `mDNS.service` - Systemd service unit (Linux)

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
sudo mkdir -p /etc/mDNS
sudo cp mDNS /usr/local/bin/
sudo cp config.toml /etc/mDNS/config.toml
sudo cp mDNS.service /etc/systemd/system/
```

2. Enable and start:
```bash
sudo systemctl daemon-reload
sudo systemctl enable mDNS
sudo systemctl start mDNS
```

3. Check status:
```bash
sudo systemctl status mDNS
```

### macOS (Homebrew service)

1. Copy binary and config:
```bash
sudo cp mDNS /usr/local/bin/
sudo mkdir -p /usr/local/etc/mDNS
sudo cp config.toml /usr/local/etc/mDNS/config.toml
```

2. Create launchd plist:
```bash
sudo nano /Library/LaunchDaemons/com.devstroop.mDNS.plist
```

Add:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.devstroop.mDNS</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/mDNS</string>
        <string>-c</string>
        <string>/usr/local/etc/mDNS/config.toml</string>
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
sudo launchctl load /Library/LaunchDaemons/com.devstroop.mDNS.plist
```

### Windows (NSSM service)

1. Create directories:
```cmd
mkdir C:\mDNS
copy mDNS.exe C:\mDNS\
copy config.toml C:\mDNS\
```

2. Download NSSM: https://nssm.cc/download

3. Install service:
```cmd
nssm install mDNS C:\mDNS\mDNS.exe "-c C:\mDNS\config.toml"
nssm start mDNS
```

Or use PowerShell service:
```powershell
New-Service -Name mDNS -BinaryPathName "C:\mDNS\mDNS.exe -c C:\mDNS\config.toml" -StartupType Automatic
Start-Service mDNS
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
sudo journalctl -u mDNS -f
```

### Check config syntax
```bash
./mDNS --config /etc/mDNS/config.toml
```

---

## Ports Used

- DNS: 53 (UDP)
- mDNS: 5353 (UDP) - only if enabled