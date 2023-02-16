<p align="center">
    <img src="assets/logo.png" alt="walkietalkie logo" />
</p>

<p align="center">
    <img src="https://img.shields.io/badge/stability-experimental-orange.svg" alt="walkietalkie stability" />
</p>

## What is Walkietalkie?
Walkietalkie is a simple remote command runner. The main idea is to help system admins to execute simple commands in many
remote devices at once. It is a composition of a server (soldier) and a client (commander).The server is set up and 
configured in the remote device with the commands and the client request the execution of those commands.

### Soldier (server)
The Soldier is the Walkietalkie's server, and it is responsible to receive, execute and return the result from the 
commands.

The Soldier configuration file is `soldier.ron`.
```ron 
(
    name: "S. Buck",            // It should be unique and it used to indefy the Soldier.
    addr: "127.0.0.1:14114",    // It is the host what the Soldier listen.
    group: "root",              // It is the unix group who will use the Soldier.
    user: "root",               // It is the unix user who will use the Soldier.
    seal: (                     // It defines credentials to acess the Soldier.
       username: "root",
       password: "root",
    ),
    commands: [                     // List of commands to be executed.
        Command (
            id: "echo",
            name: "echo",
            args: [
                "Hello, world!"
            ]),
        Command (
            id: "curl",
            name: "curl",
            args: [
                "www.google.com"
            ])
        ],
    )
```

### Commander (client)
Commander is the Walkietalkie's client, therefore it sends the commands to be executed in the Soldier 
(Server) and deal with output.

```ron
(
  name: "Cpt. Steven Rogers",       // It should be unique and it used to indefy the Commander.
  devices: [                        // List of devices what the commander can connect and execute payloads.
    Device (
      address: "127.0.0.1:14114",   // Device address.
      seal: Seal (                  // It defines credentials to acess the Soldier.
        username: "root",
        password: "root"
      )
    ),
  ],
 commands: ["echo", "google"],                       // List of commands to be executed in the Soldier.
)
```

### The Soldier:
- Listening for Commander's connection
- Authenticate
- Require commands
- Executes commands
- Send back the Responses
- Wait new commands

### The Commander:

- Send the commands to Soldiers configured in `commander.ron` 
- Wait for a response
- Show the output

## How to use?
To start a soldier with the right `soldier.ron` configuration file:
```sh
walkietalkie soldier
```

To send a commands from a Commander to all Soldiers configured in `commander.ron` file:
```sh
walkietalkie commander
```
