FROM tensorflow/tensorflow:2.12.0-gpu

WORKDIR /app

COPY . /app

# Install required Python packages (all in one pip install command)
RUN pip install --no-cache-dir \
    numpy \
    pandas \
    scikit-learn \
    matplotlib \
    librosa \
    scipy \
    opencv-python-headless

CMD ["bash"]