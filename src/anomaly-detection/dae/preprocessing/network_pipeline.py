import pandas as pd
import numpy as np

def preprocess_network_data(log_file):
    data = pd.read_csv(log_file)
    
    data = data.drop(columns=['timestamp', 'protocol'], errors='ignore')
    
    data = data.fillna(0)
    
    data = (data - data.min()) / (data.max() - data.min())
    
    return data.values

if __name__ == "__main__":
    dummy_logs = pd.DataFrame({
        'src_ip': np.random.rand(100),
        'dst_ip': np.random.rand(100),
        'packets': np.random.randint(1, 500, 100),
        'bytes': np.random.randint(100, 10000, 100)
    })
    dummy_logs.to_csv("test_logs.csv", index=False)
    
    processed_logs = preprocess_network_data("test_logs.csv")
    print(f"Processed log shape: {processed_logs.shape}")