#!/bin/zsh

# Run the Rust program in the background
RUST_LOG=DEBUG cargo run -p becks_server &
rust_pid=$! # Save the process ID of the Rust program

# Run Python in the foreground and execute the import statement
python -c "from tools.requests_test import *; import code; code.interact(local=locals())"

# When Python exits, terminate the Rust program
echo "Python has exited. Stopping the background Rust program..."
kill $rust_pid
wait $rust_pid 2>/dev/null # Wait for the Rust program to terminate
echo "Rust program has been stopped."
