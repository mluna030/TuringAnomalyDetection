from modular_dae import ModularDAE
import numpy as np
from enum import Enum

EPOCHS = 50
BATCH_SIZE = 32
LATENT_DIM = 64
MODEL_SAVE_PATH = "dae_model.h5"

class Modality(Enum):
    IMAGE = "image"
    NETWORK = "network"
    LIDAR = "lidar"
    AUDIO = "audio"

MODALITY_SHAPES = {
    Modality.IMAGE: (128, 128, 3),
    Modality.NETWORK: (100,),
    Modality.LIDAR: (32, 32, 32, 1),
    Modality.AUDIO: (50, 20),
}

def generate_data(num_samples=1000, modality=Modality.IMAGE):
    try:
        shape = MODALITY_SHAPES[modality]
        return np.random.rand(num_samples, *shape)
    except:
        raise ValueError(f"Unsupported modality: {modality}")

def add_noise(data):
    noise_factor = 0.1
    noisy_data = data + noise_factor * np.random.normal(loc=0.0, 
                                                        scale=1.0, 
                                                        size=data.shape)
    return np.clip(noisy_data, 0.0, 1.0)

def train_dae():
    data = generate_data()
    noisy_data = add_noise(data)

    dae = ModularDAE(input_shape=MODALITY_SHAPES[Modality.IMAGE], 
                     modality=Modality.IMAGE, 
                     latent_dim=LATENT_DIM)
    model = dae._build_model()
    dae.model = model 
    dae.compile_model()

    history = dae.model.fit(
        noisy_data,
        data,
        epochs=EPOCHS,
        batch_size=BATCH_SIZE,
        validation_split=0.1
    )
    dae.model.save(MODEL_SAVE_PATH)
    print(f"Model saved at: {MODEL_SAVE_PATH}")

    return history

if __name__ == "__main__":
    train_dae()