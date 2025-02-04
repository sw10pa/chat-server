# Chat Server

This is a simple chat server implemented in Rust using the Tokio asynchronous runtime.

## Features

- Handles multiple clients concurrently;
- Supports basic chat functionalities;
- Uses asynchronous I/O for better performance.

## Requirements

- Rust (latest stable version).

## Running

To run the server, use the following command:

```
cargo run
```

## Connecting to the Server

After running the project, you can connect to the server using the local terminal with the following command:

```
telnet localhost 8080
```

You can open as many connections as you wish.
