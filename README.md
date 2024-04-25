### Easy ESP

Project to simplify using TCP and to simplify data transfer

This is just meant to simplify running a tcp server a test esp32
client will be provided

**This is a work in progress**







## Structure

*Server*
The server is responsible for handling any new connections 
as well as passing messages to those connections. It gets passed a 
handle function along with a custom messaging fromat for better verstaility and 
upgradeability