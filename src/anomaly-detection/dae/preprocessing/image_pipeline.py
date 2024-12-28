import cv2
import numpy as np

def preprocess_image(image, target_size=(128, 128)):
    if isinstance(image, str):
        image = cv2.imread(image)
    
    image = cv2.resize(image, target_size)
    
    image = image.astype('float32') / 255.0
    
    image = cv2.GaussianBlur(image, (3, 3), 0)
    
    return image

if __name__ == "__main__":
    test_image = np.random.randint(0, 255, (256, 256, 3), dtype='uint8')
    processed = preprocess_image(test_image)
    print(f"Processed image shape: {processed.shape}")