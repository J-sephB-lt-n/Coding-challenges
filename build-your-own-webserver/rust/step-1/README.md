- <https://codingchallenges.fyi/challenges/challenge-webserver>

- https://doc.rust-lang.org/book/ch20-01-single-threaded.html

# Instructions

In this step your goal is to create a basic HTTP server that listens on port 80 and can handle a single TCP connection at a time. For all requests we’ll return some text that describes the requested path.

For example if our server is running locally, a client might make the curl request below and get back the simple text message.

```
curl http://localhost/
Requested path: /
```

To support this your server will need to create a socket, and bind it to the address of your server. For the purposes of this challenge that can be the IP address: 127.0.0.1 (the loopback address, also known as localhost) and port: 80 (the default HTTP port).

Once that is done, your server will need to listen for requests, accept incoming requests and then receive the incoming data.

You can learn more about sockets in the Wikipedia article on Berkeley Sockets. Your programming language probably provides a wrapper around this API in its standard library for example Python has socket, Rust has std::net and node has node:net.

For an in-depth look at network programming check out Beej’s Guide to Network Programming.

Once you can receive data from the client you’ll need to parse that data to extract the key elements. For this step that is simply taking the first line of the client request, which will look something like this:

```
GET / HTTP/1.1
```

From which you’ll need to recognise that this is the Request value, the type of request is GET, the requested resource is / and the HTTP version is 1.1.

For this step, you’ll need to return the bare minimum HTTP response:

```
HTTP/1.1 200 OK\r\n\r\nRequested path: <the path>\r\n
```

Which you will do by sending that data back over the socket. Once you have that working we have a very basic server, but it’s not much use yet. Don’t forget to close the socket when you’re done sending data.
