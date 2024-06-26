# Hermes
LAN communication via the TCP protocol in the form of a simple chatroom.

## Rust
I chose rust because I think the language has potential and I want to learn it. I'm not a rust advocate, I'm just rust-curious and I'm not ashamed of it. Help me at your own risk.

## Architecture
Chatrooms will act as 'hubs' that listen for data from their connected sockets and broadcasts that data to each connected socket. It is not meant to be safe yet, so I want a very simple implementation. Clients will have a CLI ui that displays received messages and reads user input. The ui should not wait for the connection respond. I chose Ratatui for the UI and tokio for the concurrent TCP action. Let's see where it takes me!

I moved the text input example from ratatui a bit to make it to my liking, now I need to implement the server.

## brainstorm
I want multiple users to connect. My idea is that the server simply echoes received messages to every connection for simplicity. This means that users must not display their own messages, they must wait for the server to display what they wrote. CLIents are already setup that way, now I need to handle multiple connections on the server.

Server has a listener and starts a tokio task everytime a new connection appears. This taks loops the read action and mirrors the data sent to the server. That means that every client can see what they write, but not what other connections write. To handle that I need every connection to be an object with a listen method and a write method. I also need a broadcast method that holds all connection objects and writes to them what one connection has sent to the server. Each connection has to make itself known to every one on the server. 

I saw an example where the server holds a state object with a hashmap and a broadcast method that goes through each item in the hashmap and send them data. They used a mutex to add or remove clients from the hashmap. The server has a task by itself that listens for data from any of the connections. It seems useful. I will study it further : [tokio chat example](https://github.com/tokio-rs/tokio/blob/master/examples/chat.rs)

## Milestones
- [X] Client has simple UI
- [X] Client can send messages to server
- [X] Server can reveive messages from client
- [X] Server can send messages to client
- [X] Client can receive messages from server
- [X] Client can display received messages from server
- [X] Client can setup rich messaging context (username, etc)
- [X] Messages are rich
- [X] Server can send messages to multiple clients
- [X] Client UI is separated from control
