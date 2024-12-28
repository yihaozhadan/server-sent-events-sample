# server-sent-events-sample# Server-Sent Events Rust Sample

This is a Rust implementation of a Server-Sent Events (SSE) server using Actix Web.

## Features

- src/main.rs:
  - Implements a Rust server using Actix Web
  - Provides a /stats endpoint that streams SSE events
  - Sends two types of events: uptime (system load) and time (current server time)
  - Uses async streams to continuously send events
  - Serves static files from the root directory
- index.html:
  - Provides a clean, modern UI with a responsive design
  - Shows system load and server time in separate cards
  - Displays connection status
  - Uses system fonts and modern CSS styling
- client.js:
  - Connects to the SSE endpoint using EventSource
  - Handles connection status updates
  - Processes incoming events and updates the UI
  - Formats timestamps into readable dates

The server is running at http://127.0.0.1:8000. You can open this URL in your browser to see the live updates. The UI will show:

- Connection status (connected/disconnected)
- System load averages (1, 5, and 15 minutes)
- Current server time

The implementation features:

- Real-time updates without polling
- Automatic reconnection if the connection is lost
- Clean error handling
- Modern, responsive UI

## Running the Server

```bash
cargo run
```

## Running the Client

Open the `index.html` file in your browser to see the live updates.