# Easy ESP

*This is a work in progress*

This Crate is just a wrapper of TCP socket functionality to make a simpler interface and format for dealing with socket connections 

This crate was initially created to simplify communication with the ESP microcontroller but has turned to a more general and friendlier solution for most other applications

There is a specific format that this messaging system follows so we have listed on this projects GitHub simple socket programs to run on a variety of micro controllers and devices

## Structure
Its been decided that this server will use a simple Request Response protocol for easier usage

In the system this Crate uses the Server has been broken up into two parts.
- [ ] Server (Responsible for handling sockets, message transfer, and clients)
- [ ] Handler (Responsible for dealing with any client request and producing a response)

This Crate has implemented the Server component so user of this Crate must create their own 
custom implementation of a handler. 

To aid people in creating handlers and to offer a guide there are a few handlers already made for convenience.
- [ ] Chat room
- [ ] Database
- [ ] Secure

## Handler
A component a user must build themselves to handle any message that comes and return some response message with a optional server command
Handlers must implement the handler trait to handle and String request and return a String response

## Server CMD System
Server can receive requests from handler to do some command server side
from a list of commands

server is capable of:
    - [ ] ShutDown (Shuts down server or client)
    - [ ] SendAll (Sends some message to all clients)
