import librosa
import numpy as np

def preprocess_audio(audio_path, target_length=16000):
    audio, sr = librosa.load(audio_path, sr=16000)
    
    if len(audio) > target_length:
        audio = audio[:target_length]
    else:
        audio = np.pad(audio, (0, target_length - len(audio)))
    
    mel_spec = librosa.feature.melspectrogram(y=audio, sr=sr, n_mels=128)
    mel_spec = librosa.power_to_db(mel_spec, ref=np.max)
    
    return mel_spec

if __name__ == "__main__":
    test_audio = np.random.rand(16000)
    spectrogram = preprocess_audio(test_audio)
    print(f"Mel spectrogram shape: {spectrogram.shape}")