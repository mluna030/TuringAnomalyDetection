import os

# Define the file structure
structure = {
    "README.md": "# TuringAnamolyDetection\n\nA fully Rust-based IoT gateway with anomaly detection.",
    "LICENSE": "MIT License",
    "docs": {
        "architecture-diagram.png": "",
        "setup-guide.md": "# Setup Guide\n\nFollow these steps to set up the project.",
        "design-doc.md": "# Design Document\n\nDetails of the architecture and components.",
    },
    "src": {
        "router": {
            "firewall": {
                "Cargo.toml": "[package]\nname = \"firewall\"\nversion = \"0.1.0\"",
                "src": {
                    "main.rs": "// Rust-based firewall entry point",
                    "iptables_integration.rs": "// Functions to manage iptables rules",
                },
            },
        },
        "anomaly-detection": {
            "model": {
                "train_model.rs": "// Training logic for anomaly detection models",
                "model_loader.rs": "// Load and validate trained models",
            },
            "inference.rs": "// Rust logic for anomaly detection inference",
        },
    },
    "dashboard": {
        "Cargo.toml": "[package]\nname = \"dashboard\"\nversion = \"0.1.0\"",
        "src": {
            "main.rs": "// Rocket backend entry point",
            "dashboard.rs": "// Dashboard routes and logic",
            "frontend": {
                "Cargo.toml": "[package]\nname = \"frontend\"\nversion = \"0.1.0\"",
                "src": {
                    "lib.rs": "// Yew frontend entry point",
                    "components.rs": "// Yew components",
                },
            },
        },
    },
    "tests": {
        "traffic_simulation.rs": "// Simulate traffic for testing the system",
        "integration_tests.rs": "// Integration tests for the project",
    },
}

def create_structure(base_path, structure):
    for name, content in structure.items():
        path = os.path.join(base_path, name)
        if isinstance(content, dict):  # If it's a directory
            if not os.path.exists(path):
                os.makedirs(path)
                print(f"Created directory: {path}")
            create_structure(path, content)  # Recurse into subdirectory
        else:  # If it's a file
            if not os.path.exists(path):
                with open(path, 'w') as f:
                    f.write(content)  # Write the placeholder content
                print(f"Created file: {path}")
            else:
                print(f"Skipped existing file: {path}")

if __name__ == "__main__":
    base_dir = "TuringAnamolyDetection"
    if not os.path.exists(base_dir):
        os.makedirs(base_dir)
        print(f"Created base directory: {base_dir}")
    else:
        print(f"Base directory already exists: {base_dir}")
    create_structure(base_dir, structure)
    print(f"Rust-based project structure for '{base_dir}' created successfully!")
