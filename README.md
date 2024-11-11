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

## ----Running/Development----
Individual applications can be ran using:
```
cargo run -p athena
cargo run -p tartarus
cargo run -p hermes
```
And all packages can be built by simply running:
```
cargo build
```
in the root directory 
