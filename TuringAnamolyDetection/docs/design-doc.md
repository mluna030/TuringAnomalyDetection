# Design Document: TuringAnamolyDetection

## Purpose
The **TuringAnamolyDetection** project is designed to enhance the security of IoT networks by providing a robust and efficient gateway for traffic routing, anomaly detection, and real-time monitoring. The system leverages the Turing Pi 2 cluster for modularity and scalability.

---

## System Architecture

### Overview
The system comprises three primary components, distributed across the Turing Pi 2 cluster:
1. **Traffic Router (CM4 Node 1)**: Acts as the primary gateway and enforces firewall rules.
2. **Anomaly Detection Node (Jetson Nano)**: Processes traffic data and identifies anomalous patterns using a TensorFlow Lite model.
3. **Logging and Monitoring Server (CM4 Node 2)**: Aggregates data, raises alerts, and provides a real-time monitoring dashboard.

### Data Flow
1. **Ingress Traffic**: Traffic from IoT devices is routed through the firewall on CM4 Node 1.
2. **Anomaly Detection**: Suspicious or flagged traffic is forwarded to the Jetson Nano for anomaly detection.
3. **Alerts and Logs**: Results of anomaly detection are logged and visualized on CM4 Node 2.
4. **Egress Traffic**: Clean traffic is forwarded to external networks.

---

## Key Components

### 1. Rust-Based Firewall (CM4 Node 1)
- **Functionality**:
  - Monitors and filters incoming traffic using `iptables` (via Rust bindings).
  - Implements dynamic rule enforcement based on anomaly detection feedback.
- **Design Decisions**:
  - Chose Rust for safety, performance, and modern tooling.
  - Utilizes a custom rule update mechanism to dynamically adapt to threats.

### 2. Anomaly Detection Engine (Jetson Nano)
- **Functionality**:
  - Runs a TensorFlow Lite model to classify traffic patterns.
  - Detects:
    - Unauthorized access attempts.
    - Port scans.
    - Unusual traffic spikes.
- **Design Decisions**:
  - Jetson Nano is used for its GPU support and efficient ML model inference.
  - TensorFlow Lite chosen for lightweight deployment on edge devices.

### 3. Monitoring and Logging Server (CM4 Node 2)
- **Functionality**:
  - Aggregates data logs from CM4 Node 1 and Jetson Nano.
  - Provides a real-time dashboard built with Rocket (Rust) or Flask (Python).
- **Design Decisions**:
  - Modular design allows for easy extension to include Grafana or other tools.
  - Logs are stored in JSON for compatibility and ease of analysis.

---

## Communication and Protocols

### MQTT with TLS
- **Purpose**: Secure communication between nodes.
- **Details**:
  - Each node publishes and subscribes to specific topics (e.g., logs, anomaly alerts).
  - TLS encryption ensures message security.
- **Libraries**:
  - **rumqtt** (Rust): For handling MQTT messaging.
  - **Paho MQTT** (Python): For Python-based components.

---

## Security Considerations
1. **Firewall**:
   - Prevents unauthorized access and mitigates common attack vectors (e.g., SYN floods).
2. **Anomaly Detection**:
   - Flags traffic anomalies based on pre-trained models.
   - Adaptable to evolving threats through model updates.
3. **Encrypted Communication**:
   - TLS prevents man-in-the-middle attacks and ensures data integrity.

---

## Future Enhancements
- **Scaling**:
  - Add more nodes for larger IoT deployments.
- **Advanced ML Models**:
  - Upgrade to more sophisticated deep learning models for better accuracy.
- **Integration with Threat Feeds**:
  - Incorporate external threat intelligence for dynamic rule updates.
- **Dashboard Improvements**:
  - Add historical data views and customizable alerts.

---

## Conclusion
**TuringAnamolyDetection** combines lightweight design with robust security features to address the challenges of IoT network security. The modular architecture ensures scalability, while the integration of anomaly detection and secure communication provides a strong foundation for protecting IoT environments.