import numpy as np
from scipy.ndimage import gaussian_filter

def preprocess_lidar(lidar_data, target_shape=(32, 32, 32)):
    lidar_data = lidar_data / np.max(lidar_data)
    
    lidar_data = np.resize(lidar_data, target_shape)
    
    lidar_data = gaussian_filter(lidar_data, sigma=1)
    
    return lidar_data

if __name__ == "__main__":
    test_lidar = np.random.rand(64, 64, 64)
    processed_lidar = preprocess_lidar(test_lidar)
    print(f"Processed LIDAR shape: {processed_lidar.shape}")