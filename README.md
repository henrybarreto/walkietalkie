![fluxo de trabalho de exemplo](https://github.com/henrybarreto/walkietalkie/actions/workflows/rust.yml/badge.svg)

![walkietalkie logo](./logo.png)


## What is "walkietalkie"?
*UNDER DEVELOPMENT*

"Walkietalkie" is a couple of applications to **send**, **receive** and **execute** commands in linux device.

Basically, it was intent to be a client (soldier) and server (commander) application, which the server send a list of single commands, the client execute each one and return the output.

It can be util when you have many devices and just want to execute a few command to each one, instead of open a SSH connection, execute a and exit.

## How it works?

### The Commander:

- Send commands inside "commander.ron" file
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

### Run Commander
```bash
cargo run -p commander 
```

### Run Soldier
```bash
cargo run -p soldier 
```
When Soldier is running, it will produce three files: `soldier.err`, `soldier.out` and `soldier.pid`.
- `soldier.err` is where you can find the erros occur;
- `solider.out` is the soldier's default output;
- `soldier.pid` is the PID of running process.