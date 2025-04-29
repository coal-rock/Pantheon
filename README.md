<div align="center">
  
<div>
  <img height=200 src="athena/assets/cdo-logo.png" alt="Pantheon logo" />
</div>

# Pantheon
**behold, deus ex machina**\
*for here lies the temple of the gods*


[About](#about) •
[Components](#components) •
[Getting Started](#getting-started) •
[Supported Platforms](#supported-platforms) •
[Demo](#demo)

[![Build All](https://github.com/coal-rock/Pantheon/actions/workflows/tests.yml/badge.svg?branch=main)](https://github.com/Machina-Software/Pantheon/actions/workflows/build_all.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)


</div>

> [!WARNING]  
> Pantheon is very much still pre-release software!
> 
> If you're thinking about employing this in a competition environment, please feel free to reach out for help with deployment or with any issues that may arise, we're happy to help :)

## About
**Pantheon** is a C2 Adversary Emulation Framework written entirely in Rust. It strives to be *painlessly interactive*, *performant*, and *portable*.


## Components

```
Pantheon/
├── athena/     # web-based frontend built using Dioxus
├── hermes/     # cross-platform agent
├── talaria/    # library implementing shared functionality between other components
└── tartarus/   # backend server built using Rocket
```

## Getting Started
Up-to-date builds can be found on either the [Releases](https://github.com/Machina-Software/Pantheon/releases) page or by pulling artifacts from [GitHub Actions](https://github.com/Machina-Software/Pantheon/actions).

### Agent
For proper deployments, it is currently necessary to manually build at least the agent, _Hermes_, from source, as its default configuration is baked into the binary to be as portable as possible. This can be accomplished with the following commands:
```bash
git clone https://github.com/coal-rock/Pantheon
cd Pantheon/hermes
URL="http://localhost:8000/api/agent" POLL_INTERVAL_MS=10000 cargo build --release
```
Additionally, an agent build prioritizing minimal binary size can be achieved through the `build.sh` file present in the `hermes` directory.

### Server
For deploying the entire server stack, the only supported route of installation is through Docker Compose:
```bash
git clone https://github.com/coal-rock/Pantheon
cd Pantheon/docker
docker compose up -d
```

## Supported Platforms
Athena (Frontend):
- Windows ✅
- Linux ✅
- macOS ✅

Tartarus (Backend):
- Windows ✅
- Linux ✅
- macOS ✅

Hermes (Agent):
- Windows ❓
- Linux ✅
- macOS ❓

> [!NOTE]  
> Greater support for Windows and macOS is planned and will be coming in the future.

---

## Demo
![image](https://github.com/user-attachments/assets/a409f146-c2b5-46f2-aae6-2007e7216910)
