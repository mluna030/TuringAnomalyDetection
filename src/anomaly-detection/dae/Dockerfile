FROM tensorflow/tensorflow:2.12.0-gpu

WORKDIR /app

COPY . /app

RUN pip install --no-cache-dir \
    numpy \
    pandas \
    scikit-learn \
    matplotlib

CMD ["bash"]