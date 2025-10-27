```markdown
┌─────────────────────────────────────────────────────────────┐
│                     Traffic Router (CM4 #1)                 │
│  ┌────────────┐    ┌──────────────┐    ┌────────────────┐   │
│  │  Packet    │───▶│   Firewall   │───▶│  MQTT Client   │   │
│  │  Capture   │    │   Engine     │    │  (Publisher)   │   │
│  └────────────┘    └──────────────┘    └────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼ (MQTT/TLS)
┌─────────────────────────────────────────────────────────────┐
│              Anomaly Detection (Jetson Nano)                │
│  ┌────────────┐    ┌──────────────┐    ┌────────────────┐   │
│  │   MQTT     │───▶│  TFLite      │───▶│  MQTT Client   │   │
│  │ Subscriber │    │  Inference   │    │  (Publisher)   │   │
│  └────────────┘    └──────────────┘    └────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼ (MQTT/TLS)
┌─────────────────────────────────────────────────────────────┐
│              Logging Server (CM4 #2)                        │
│  ┌────────────┐    ┌──────────────┐    ┌────────────────┐   │
│  │   MQTT     │───▶│  Log Store   │───▶│   Dashboard    │   │
│  │ Subscriber │    │  (Time-Series│    │    (Rocket)    │   │
│  └────────────┘    └──────────────┘    └────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## TSA Precheck model

```Rust
    Packet Arrives
            ↓
┌─────────────────────────┐
│  FAST PATH (TSA Pre✓)  │  ← Quick checks (IP/Port/Protocol)
│  - 95% of packets       │     Most traffic goes through here
│  - Microseconds         │
└─────────────────────────┘
            ↓ (Pass) ──→ ALLOW
            ↓ (Suspicious)
┌─────────────────────────┐
│  SLOW PATH (TSA Deep)   │  ← Detailed inspection
│  - 5% of packets        │     Geo-IP, Deep packet inspection
│  - Milliseconds         │     Pattern matching
└─────────────────────────┘
            ↓ (Still suspicious)
┌─────────────────────────┐
│  ANOMALY DETECTION      │  ← Send to Jetson Nano via MQTT
│  - <1% of packets       │     ML model analysis
│  - Seconds              │
└─────────────────────────┘
```