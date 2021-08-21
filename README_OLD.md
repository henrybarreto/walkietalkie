![fluxo de trabalho de exemplo](https://github.com/henrybarreto/walkietalkie/actions/workflows/rust.yml/badge.svg)

## What is "walkietalkie"?

It is a Rust crate what provides methods and structure to be able communication between clients and server, load configuration
files, convert data, run commands and send back the results.

## What are the examples?

The examples are a combination of a Soldiers (Clients) and Commander(Server). Each Commander has a list of Commands, which it sends to its Soldiers, what it can have many simultaneously. The Soldiers follow to the Commander's commands orders and, when all orders have been run, it waits a time interval and ask again to the Commander for new on ones.

## Why is it?

When I had have been studying about Linux networks, I have used many utilities what I liked too much, but one have awoken me interest, the netcat. This nc (netcat) is a utility for reading from and writing to network connections using TCP or UDP. 

In somewhere I found someone explained how to use to execute commands directly to Windows's CMD. I thought it is interesting and could have many other good applications, but I have been letting aside, until the Hashnode Hackathon incentives my try to implement what I thought.

Moreover, If there is a file server, HTTP server, ftp server why not a Linux command server? (lol) ~~ssh and another ways to do it does not exist, okay?~~

Technically speaking, the Commander does:

- Each server is listening for TCP connections
- Spawn a new thread
- Send the Commands from the "config.ron" file to Soldier
- Wait for a response
- Show its in terminal.

The Soldier does: 

- Try to connect with a server addr from the config.ron file
- Require Commands
- Executes it
- Send back the Responses
- Write down output, errors in files

It is a simple implementation of what I imagined to the project, the first one, of course. The code can seem a little spaghetti, but as I wanted to see the project running first, I did it for a good cause. I am planning to improving this implementation at a point what it's allowing the use of middleware in TCP connection, thread spawning and others worth points, encrypting the Responses somehow, simplifying the "protocol" of sending and receiving commands, letting the logging more informational and so on...

> *Done Is Better Than Perfect.*

Here I have tried to use less than possible crates because I'm learning processing, and the contact with vanilla code is good for it. Some essential crates would not be ignored to speed up the coding, that was:

- **bincode**
- **ron**
- openssl (it will be used soon)
- **daemonize**

**bincode** is: "a crate for encoding and decoding using a tiny binary serialization strategy".

I have used it to serialize and deserialize the Commands and Responses to send through the TcpStream.

**ron** is: "a simple readable data serialization format that looks similar to Rust syntax".

It is like a JSON for Rust. With it, I built the configuration file of the Commander and Soldier.

**daemonize** is: "a library for writing system daemons".

That is it, does not needs much more explanation. It was used to make the Soldier a daemon to run the Commands in the background.

Rust has a peculiar way to manager memory, its ownership and borrow checker make it does not need a garbage collector what can be suffering in the first contact, but improve the and  development and avoiding some runtimes errors late.

I fought against this. Some times I did not know what to do because the code seems to be ok, but some variable was "borrowed" and could not been used out of the context of borrowing. To solve, either I used reference or I cloned the object of the problem. I still do not know what it causes, or it is right, but I did it.

To try it, just clone my repo in GitHub: 

```bash
git clone https://github.com/henrybarreto/walkietalkie
```

Configure the "config.ron" of the Commander and the Soldier and run it.

```bash
cd examples/soldier/
cargo run --examples soldier
```

```bash
cd examples/commander
cargo run --examples commander 
```

That is it, It was the first thing I built used Rust, so tips are so welcome, I would like it too much. Feel free to comment to correct me. Thank you for reading.
