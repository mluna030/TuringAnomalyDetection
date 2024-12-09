import os
from diagrams import Diagram, Cluster
from diagrams.custom import Custom

# Try to import specific icons, fallback to defaults if not found
try:
    from diagrams.programming.language import Rust
except ImportError:
    print("Warning: 'Rust' icon not found. Using 'Server' as a fallback.")
    from diagrams.onprem.compute import Server as Rust

try:
    from diagrams.onprem.analytics import Jupyter
except ImportError:
    print("Warning: 'Jupyter' icon not found. Using 'Server' as a fallback.")
    from diagrams.onprem.compute import Server as Jupyter

try:
    from diagrams.onprem.iac import Ansible
except ImportError:
    print("Warning: 'Ansible' icon not found. Using 'Server' as a fallback.")
    from diagrams.onprem.compute import Server as Ansible

try:
    from diagrams.generic.database import Storage
except ImportError:
    print("Warning: 'Storage' icon not found. Using 'Server' as a fallback.")
    from diagrams.onprem.compute import Server as Storage

# File name for the generated diagram
OUTPUT_FILE = "architecture-diagram.png"

# Overwrite existing diagram
if os.path.exists(OUTPUT_FILE):
    print(f"File '{OUTPUT_FILE}' already exists. It will be replaced.")
    os.remove(OUTPUT_FILE)

print("Starting diagram generation...")

try:
    with Diagram("Turing Anamoly Detection Architecture", show=False, filename="architecture-diagram", outformat="png"):
        # Cluster representing the Turing Pi 2 setup
        with Cluster("Turing Pi 2 Cluster"):
            router = Custom("Traffic \nRouter \n(RPI)", "/home/micha/TuringAnomalyDetection/TuringAnamolyDetection/docs/resources/rpi.png")
            firewall = Rust("Rust-based \nFirewall")
            detection = Custom("Anomaly Detection Node \n(Jetson Nano)", "/home/micha/TuringAnomalyDetection/TuringAnamolyDetection/docs/resources/jetson.png")
            logging = Custom("Logging Server \n(RPI)", "/home/micha/TuringAnomalyDetection/TuringAnamolyDetection/docs/resources/rpi.png")

            # Data flow connections within the cluster
            router >> firewall >> detection
            detection >> logging

        # External tools used in the development and deployment process
        with Cluster("Development Environment"):
            dev = Jupyter("Training")
            ansible = Ansible("Deployment")
            dev >> ansible >> router

        # Logging and traffic data storage
        logging >> Storage("Traffic Logs")

    print(f"Diagram generation completed. Verifying '{OUTPUT_FILE}'...")

    # Check if the file is created
    if os.path.exists(OUTPUT_FILE):
        print(f"Success! Diagram '{OUTPUT_FILE}' has been generated.")
    else:
        print(f"Error: Diagram '{OUTPUT_FILE}' was not generated.")

except Exception as e:
    print(f"An error occurred during diagram generation: {e}")