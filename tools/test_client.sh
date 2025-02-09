#!/bin/zsh

# Run the Rust program in the background
RUST_LOG=DEBUG cargo run -p becks_server &
server_pid=$! # Save the process ID of the Rust program

# Start client in the foreground
echo "Starting client..."
RUST_LOG=becks_client=DEBUG,becks_network=DEBUG,WARN cargo run -p becks_client

echo "Client exited, stopping server..."
kill $server_pid
wait $server_pid 2>/dev/null # Wait for the Rust program to terminate
echo "Server has been stopped."
