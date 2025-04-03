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

[![Build All](https://github.com/Machina-Software/Pantheon/actions/workflows/build_all.yml/badge.svg?branch=main)](https://github.com/Machina-Software/Pantheon/actions/workflows/build_all.yml) [![Discord](https://discord.gg/knyExCkJQD)](https://discord.gg/knyExCkJQD)


</div>

> [!WARNING]  
> Panthon is very much still pre-release software!
> 
> If you're thinking about employing this in a competition environment, please feel free to reach out for help with deployment or with any issues that may arise, we're happy to help :)

## About
**Pantheon** is a C2 Adversary Emulation Framework written entirely in Rust. It strives to be *painlessly interactive*, *performant*, and *portable*.


## Components

```
Panthon/
├── athena/     # web-based frontend built using Dioxus
├── hermes/     # cross-platform agent
├── talaria/    # library implementing shared functionality between other components
└── tartarus/   # server built using Rocket
```

## Getting Started
Up-to-date builds can be found on either the [Releases](https://github.com/Machina-Software/Pantheon/releases) page or by pulling artifacts from [GitHub Actions](https://github.com/Machina-Software/Pantheon/actions).

For proper deployments, it is currently necessary to manually build at least the agent, _Hermes_, from source, as its default configuration is baked into the binary to be as portable as possible. This can be accomplished with the following commands:
```bash
git clone https://github.com/Machina-Software/Pantheon
cd Pantheon/hermes
URL="127.0.0.1:8080" POLL_INTERVAL_MS="10000" cargo build --release
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
