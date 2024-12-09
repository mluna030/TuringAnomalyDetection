# Setup Guide

This guide will walk you through the steps to set up the IoT-Security-Gateway project on the Turing Pi 2 cluster.

## Prerequisites
1. **Hardware**:
    - Turing Pi 2 board.
    - 1 Jetson Nano.
    - 2 Raspberry Pi CM4 modules.

2. **Software**:
    - Install Rust: https://rustup.rs/
    - Install Python 3.8+ with TensorFlow.
    - MQTT broker (e.g., Mosquitto).

## Steps
1. Set up the Turing Pi 2 cluster.
2. Flash the Raspberry Pi CM4 modules with a suitable OS.
3. Install the required dependencies on each node:
    - Rust, TensorFlow, and necessary libraries.
4. Clone this repository on all nodes.

## Next Steps
Follow the component-specific guides in the `docs/` folder.