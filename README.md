# Hermes
What I want to do is an application that permits local network communication as a chatroom.

## Rust
I chose rust because I think the language has potential and I want to learn it. I'm not a rust advocate, I'm just rust-curious and I'm not ashamed of it. Help me at your own risk.

## Architecture
Chatrooms will act as 'hubs' that listen for data from their connected sockets and broadcasts that data to each connected socket. It is not meant to be safe yet, so I want a very simple implementation. Clients will have a CLI ui that displays received messages and reads user input. The ui should not wait for the connection respond. I chose Ratatui for the UI and tokio for the concurrent TCP action. Let's see where it takes me!

I moved the text input example from ratatui a bit to make it to my liking, now I need to implement the server.
