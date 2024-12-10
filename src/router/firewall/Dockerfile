# Use the official Rust image as the base
FROM rust:latest

# Install dependencies for iptables
RUN apt-get update && apt-get install -y iptables

# Set the working directory inside the container
WORKDIR /app

# Copy the project files into the container
COPY . .

# Build the Rust project
RUN cargo build --release

# Expose a volume for logs (optional)
VOLUME ["/app/logs"]

# Set the default command to run the binary
CMD ["./target/release/firewall"]