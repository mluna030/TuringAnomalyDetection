import tensorflow as tf
from tensorflow.python.keras import layers, models

class ModularDAE:
    def __init__(self, input_shape, modality="image", latent_dim=64):
        self.input_shape = input_shape
        self.modality = modality
        self.latent_dim = latent_dim
    def _build_model(self):
        """
        Builds the AutoEncoder model based on the specified modality.
        """
        inputs = tf.keras.Input(shape=self.input_shape)

        if self.modality == "image":
            # Encoder
            x = layers.Conv2D(32, (3, 3), activation='relu', padding='same')(inputs)
            x = layers.MaxPooling2D((2, 2), padding='same')(x)
            x = layers.Conv2D(64, (3, 3), activation='relu', padding='same')(x)
            x = layers.MaxPooling2D((2, 2), padding='same')(x)
            x = layers.Flatten()(x)
            latent = layers.Dense(self.latent_dim, activation='relu', name="latent")(x)
            # Decoder
            x = layers.Dense(16 * 16 * 64, activation='relu')(latent)
            x = layers.Reshape((16, 16, 64))(x)
            x = layers.Conv2DTranspose(64, (3, 3), activation='relu', padding='same')(x)
            x = layers.UpSampling2D((2, 2))(x)
            x = layers.Conv2DTranspose(32, (3, 3), activation='relu', padding='same')(x)
            outputs = layers.Conv2DTranspose(3, (3, 3), activation='sigmoid', padding='same')(x)

        elif self.modality == "network":
            # Encoder
            x = layers.Dense(128, activation='relu')(inputs)
            x = layers.Dense(64, activation='relu')(x)
            latent = layers.Dense(self.latent_dim, activation='relu', name="latent")(x)
            # Decoder
            x = layers.Dense(64, activation='relu')(latent)
            x = layers.Dense(128, activation='relu')(x)
            outputs = layers.Dense(self.input_shape[0], activation='sigmoid')(x)

        elif self.modality == "lidar":
            # Encoder
            x = layers.Conv3D(32, (3, 3, 3), activation='relu', padding='same')(inputs)
            x = layers.MaxPooling3D((2, 2, 2), padding='same')(x)
            x = layers.Conv3D(64, (3, 3, 3), activation='relu', padding='same')(x)
            x = layers.MaxPooling3D((2, 2, 2), padding='same')(x)
            x = layers.Flatten()(x)
            latent = layers.Dense(self.latent_dim, activation='relu', name="latent")(x)
            # Decoder
            x = layers.Dense(8 * 8 * 8 * 64, activation='relu')(latent)
            x = layers.Reshape((8, 8, 8, 64))(x)
            x = layers.Conv3DTranspose(64, (3, 3, 3), activation='relu', padding='same')(x)
            x = layers.UpSampling3D((2, 2, 2))(x)
            outputs = layers.Conv3DTranspose(1, (3, 3, 3), activation='sigmoid', padding='same')(x)

        elif self.modality == "audio":
            # Encoder
            x = layers.LSTM(128, return_sequences=True)(inputs)
            x = layers.LSTM(64)(x)
            latent = layers.Dense(self.latent_dim, activation='relu', name="latent")(x)
            # Decoder
            x = layers.RepeatVector(self.input_shape[0])(latent)
            x = layers.LSTM(64, return_sequences=True)(x)
            x = layers.LSTM(128, return_sequences=True)(x)
            outputs = layers.TimeDistributed(layers.Dense(self.input_shape[1], activation='sigmoid'))(x)

        else:
            raise ValueError(f"Unsupported modality: {self.modality}")

        return models.Model(inputs, outputs, name=f"DAE_{self.modality}")
    def compile_model(self):
        """
        Compile the model with an appropriate optimizer and loss function.
        """
        self.model.compile(optimizer="adam", loss="mse")

    def summary(self):
        """
        Print the model summary for inspection.
        """
        self.model.summary()