# Pantheon
C2 sever in development by the Cyber Defense Organization (CDO) from University at Albany                                

- overall - pantheon
- frontend - athena 
- backend - tartarus
- agent - hermes 
- network lib - talaria


## ----Format For Code Commenting----

### FILE STARTERS
//! Filename: example.rs
//! Author: [Your Name]
//! Purpose: Functions for processing data in the Hermes project
//! Last Updated: [Date]

### INLINE COMMENTING
/// Use this for inline commenting 

### COMMENTING TODO and FIXME
/// TODO: A comment to build something later
/// FIXME: A comment documenting a known problem

COMMENT VARIABLES, FUNCTIONS, ARGUMENTS with in line or before the start of a new section

## ----Project Structure----

### Athena
The Greek goddess of wisdom, war strategy, and skill.
Often seen as an advisor and protector, Athena serves as the frontend for Pantheon.

### Tartarus
The Greek god of the deepest region of the world: the lowest part of the underworld.
Often serving as a hidden punisher, Tartarus serves as the backend for Pantheon.

### Hermes
The Greek god of mediation and messaging.
Often carrying out communication between the gods, Heres serves as the agent for Pantheon, carrying out messages to the backend.

### Talaria
The trusty winged sandals worn by Hermes, representing speed, agility, and reliability.
Talaria allows for Hermes to carry out messages between the rest of the gods, serving as the networking library for Pantheon.

## Dependencies
`trunk` and the WASM build target are required by `yew` which is used in `athena`.

```
cargo install trunk
rustup target add wasm32-unknown-unknown
```

## ----Running/Development----
Individual applications can be ran using:
```
cd athena && trunk serve
cargo run -p tartarus
cargo run -p hermes
```
And all packages can be built by simply running:
```
cargo build
```
in the root directory 




you can now create and log into a tartarus console by running
```
cargo run --bin tartarus console

```

this is still a work in progress. need to have the responses run in the background of the custom console. At the moment they cut off your commands. Also the agent and server connection shows the agent id based on the connection made, but im not able to have the console register this for some reason. TODO!


in the console one created with cargo you can now see agent registration
this will allow you to be able to identify and see all of your agents based on a unique id created for each one
![image](https://github.com/user-attachments/assets/e8ffa773-ac81-4d5b-b434-8e3a0d4e7670)



also here is a side by side view showing the agent making a connection to the backend server

![image](https://github.com/user-attachments/assets/3e8f465e-6801-49a0-bb6f-136b543f293b)
