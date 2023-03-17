# Actix File Server with API Key Authentication

This is a simple, yet powerful Actix file server written in Rust, which provides file access through a secure API key authentication mechanism. It serves files from a local directory while ensuring that only trusted web services can access them.

## Features

- Actix Web framework for efficient and fast web server
- API key authentication for secure file access
- Customizable local directory for serving files
- Path sanitization to prevent directory traversal attacks
- Environment variable-based configuration

## Setup

1. Clone the repository to your local machine.
2. Set the required environment variables:
  - APIKEY: The API key to be used for authentication.
  - SOCKET: The address and port the server should bind to (e.g., 127.0.0.1:8080).
3. Run the server using cargo run.

## Usage

To access a file from the server, send a GET request to the server with the desired file path and include the x-api-key header with the correct API key.

Example:

```http
GET /path/to/file.txt
x-api-key: yourapikey
```

If the provided API key is correct and the file exists, the server will return the file. Otherwise, it will return an error with the appropriate status code.
