<div align="center">
  
<div>
  <img height=200 src="athena/assets/cdo-logo.png" alt="Pantheon logo" />
</div>

# Pantheon
**behold, deus ex machina**\
*for here lies the temple of the gods*

[About](#about) •
[Demo](#demo)

[![Build All](https://github.com/Machina-Software/Pantheon/actions/workflows/main.yml/badge.svg?branch=main)](https://github.com/Machina-Software/Pantheon/actions/workflows/main.yml)

</div>


## About
**Pantheon** is a C2 Adversary Emulation framework written in Rust, with a focus on *interactivity*, *performance*, and *portability*. **Pantheon** has been tested to work with up to ten thousand beacons.


Want to get involved? Join our Discord[test]!

### Components
```
Panthon/
├── athena/            # web-based frontend built using Dioxus
├── hermes/            # cross-platform agent
├── talaria/           # library implementing shared functionality between other components
└── tartarus/          # server built using Rocket
```


## Demo
![image](https://github.com/user-attachments/assets/a409f146-c2b5-46f2-aae6-2007e7216910)



# Quick Start Guide

The quickest way to get the most up to date version of the applciation is by pulling it stright from the github releases, to do this run the below command
```
sudo apt install jq -y

curl -s https://api.github.com/repos/Dack985/Pantheon/releases/latest \
| jq -r '.assets[].browser_download_url' \
| xargs -I {} wget -P $HOME {}

```


Also here is a one-liner to install the installer script 
```
sudo curl -fsSL https://raw.githubusercontent.com/Dack985/Pantheon/main/install.sh | bash

```

From here you can chose wether to run Athena as a web application serving the file with something such as nginx or you can just use the raw binary also downloaded with this
