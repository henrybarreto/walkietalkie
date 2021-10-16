[![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental)

![walkietalkie logo](./logo.png)


## What is "walkietalkie"?
"Walkietalkie" is a couple of applications to send and run commands in linux's devices".

Basically, it was intent to be a client (commander) and server (soldier) application, which the client sends a list of single commands, the server executes each one and return the output.

It can be useful when you have many devices and just want to execute a few command to each one, instead of open a SSH connection, execute a and exit.

### Soldier
Soldier is the walkietalkie's server, so it needs to be installed in the device what will execute the commands.  

### Commander 
Commander is the wakietalkie's client, therefore it sends the commands to be executed in the Soldier (Server) side and deal with output.

> note: Soldier and Commander is still not implemented, but there are examples inside examples' folder.

## How it works?

### The Commander:

- Send the commands configured inside "commander.ron" file
- Wait for a response
- Show its in terminal
  - *This output is actually just for debug*

### The Soldier:

- Listening for TCP connections
- Require commands
- Executes it
- Send back the Responses
- Wait new commands

## How to use?

First, clone the repository:
```bash
git clone https://github.com/henrybarreto/walkietalkie
```

Configure the "commander.ron" to Commander and "soldier.ron" to Soldier.

### Run Soldier
```bash
cargo run --examples soldier 
```

### Run Commander
```bash
cargo run --examples commander 
```

When Soldier is running, it will produce three files: `soldier.err`, `soldier.out` and `soldier.pid`.
- `soldier.err` is where you can find the erros occur;
- `solider.out` is the soldier's default output;
- `soldier.pid` is the PID of running process.
