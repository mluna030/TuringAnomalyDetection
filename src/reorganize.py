import os
from pathlib import Path

# Base directory: now local to "src/anomaly-detection"
BASE_DIR = "anomaly-detection"

# Desired file structure for "anomaly-detection"
FILE_STRUCTURE = {
    "dae": [
        "train_dae.py",
        "export_dae_tflite.py",
        "dae_model.tflite"
    ],
    "sparse-autoencoder": [
        "train_sparse_ae.py",
        "export_sparse_tflite.py",
        "sparse_ae_model.tflite"
    ],
    "detection": [
        "detect_anomalies.py",
        "thresholds.json"
    ],
    "inference/rust": [
        "deploy_model.rs",
        "runtime_test.rs"
    ],
    "inference/c++": [],
    "inference": [
        "inference.py"
    ],
    "integration": [
        "pipeline.py"
    ]
}

# Existing files and their new destinations
FILE_MOVEMENTS = {
    "deployment/deploy_model.rs": "inference/rust/deploy_model.rs",
    "deployment/runtime_test.rs": "inference/rust/runtime_test.rs",
    "model/export_model_tflite.rs": "dae/export_dae_tflite.py",  # Convert to Python
    "model/model_loader.rs": "inference/rust/model_loader.rs",
    "model/train_model.rs": "dae/train_dae.py"  # Convert to Python
}

# Function to create directories and files
def setup_project_structure(base_dir, structure):
    for folder, files in structure.items():
        folder_path = Path(base_dir) / folder
        folder_path.mkdir(parents=True, exist_ok=True)  # Create directories
        for file in files:
            file_path = folder_path / file
            if not file_path.exists():  # Skip if file already exists
                if file.endswith(".py") or file.endswith(".json") or file.endswith(".rs"):
                    file_path.touch()  # Create empty files for Python, JSON, Rust
                elif file.endswith(".tflite"):
                    file_path.write_text("")  # Placeholder for model files

# Function to move existing files to their new locations
def move_existing_files(base_dir, movements):
    for src, dest in movements.items():
        src_path = Path(base_dir) / src
        dest_path = Path(base_dir) / dest
        if src_path.exists():
            dest_path.parent.mkdir(parents=True, exist_ok=True)  # Ensure target folder exists
            src_path.rename(dest_path)

# Run the setup process
def main():
    print(f"Setting up file structure in {BASE_DIR}...")
    setup_project_structure(BASE_DIR, FILE_STRUCTURE)
    print("Reorganizing existing files...")
    move_existing_files(BASE_DIR, FILE_MOVEMENTS)
    print("Project structure setup complete!")

if __name__ == "__main__":
    main()
