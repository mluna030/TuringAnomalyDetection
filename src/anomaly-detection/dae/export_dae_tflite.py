import tensorflow as tf
import os 

MODEL_SAVE_PATH = "dae_model.h5"
TFLITE_MODEL_PATH = "dae_model.tflite"
def load_model(model_path):
    if os.path.exists(model_path):
        print(f"Loading model from {model_path}...")
        return tf.keras.models.load_model(model_path)
    else:
        raise FileNotFoundError(f"No model found at {model_path}")

def convert_to_tflite(model, quantize=False):
    converter = tf.lite.TFLiteConverter.from_keras_model(model)
    
    if quantize:
        print("Applying post-training quantization...")
        converter.optimizations = [tf.lite.Optimize.DEFAULT]
    
    tflite_model = converter.convert()
    print("Model successfully converted to TensorFlow Lite format.")
    
    return tflite_model

def save_tflite_model(tflite_model, save_path):
    with open(save_path, "wb") as f:
        f.write(tflite_model)
    print(f"TFLite model saved to {save_path}")

if __name__ == "__main__":
    model = load_model(MODEL_SAVE_PATH)
    tflite_model = convert_to_tflite(model, quantize=True)
    
    save_tflite_model(tflite_model, TFLITE_MODEL_PATH)