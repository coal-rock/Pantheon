#!/bin/bash

set -e  # Exit on error
trap 'echo "An error occurred. Exiting..." >&2; exit 1' ERR  # Error handling

install_dependencies() {
    echo "Checking system type..."
    OS_NAME=$(awk -F= '/^ID=/{print $2}' /etc/os-release)

    case "$OS_NAME" in
        "rocky" | "fedora" | "centos" | "rhel" | "oracle")
            PKG_MANAGER="dnf"
            ;;
        "ubuntu" | "debian")
            PKG_MANAGER="apt"
            ;;
        "arch")
            PKG_MANAGER="yay"
            ;;
        *)
            echo "Unsupported OS: $OS_NAME"
            exit 1
            ;;
    esac

    echo "Installing required packages..."
    if [ "$PKG_MANAGER" == "yay" ]; then
        yay -S --noconfirm jq
    else
        sudo $PKG_MANAGER install -y jq
    fi
}

download_latest_release() {
    REPO="Dack985/Pantheon"
    DOWNLOAD_DIR="$HOME/Downloads/pantheon-pull-test"
    mkdir -p "$DOWNLOAD_DIR"

    echo "Fetching latest pre-release from GitHub..."
    LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases" | jq -r '[.[] | select(.prerelease == true and .draft == false)] | first')

    if [[ "$LATEST_RELEASE" == "null" ]]; then
        echo "No pre-release found!"
        exit 1
    fi

    ASSET_URLS=$(echo "$LATEST_RELEASE" | jq -r '.assets[].browser_download_url')

    echo "Downloading assets..."
    for URL in $ASSET_URLS; do
        wget -P "$DOWNLOAD_DIR" "$URL"
    done

    echo "Download complete!"
}

install_from_source() {
    echo "Updating system..."
    sudo apt update && sudo apt upgrade -y

    # Install Rust & Cargo
    if ! command -v rustup &> /dev/null; then
        echo "Installing Rust & Cargo..."
        curl https://sh.rustup.rs -sSf | sh -s -- -y
    fi

    # Install Cargo Binstall
    if ! command -v cargo binstall &> /dev/null; then
        echo "Installing Cargo Binstall..."
        curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
    fi

    # Install Dioxus CLI
    if ! command -v dx &> /dev/null; then
        echo "Installing Dioxus CLI..."
        cargo binstall -y dioxus-cli
    fi

    # Install NPM & Tailwind
    if ! command -v npm &> /dev/null; then
        echo "Installing NPM..."
        sudo apt install -y npm
    fi

    echo "Installing Tailwind CLI..."
    npm install -g tailwindcss @tailwindcss/cli

    # Clone and build the project
    echo "Cloning Pantheon repository..."
    git clone https://github.com/Dack985/Pantheon.git "$HOME/Pantheon"
    cd "$HOME/Pantheon"

    echo "Building Tailwind..."
    npx tailwindcss -i ./input.css -o ./assets/tailwind.css

    echo "Building Athena..."
    cd athena
    dx serve &

    echo "Building Tartarus..."
    cd ../tartarus
    cargo build
}

main() {
    echo "Choose installation method: (1) From Source (2) Prebuilt Binaries"
    select option in "From Source" "Prebuilt"; do
        case $REPLY in
            1) install_from_source; break ;;
            2) install_dependencies; download_latest_release; break ;;
            *) echo "Invalid option. Please choose 1 or 2." ;;
        esac
    done
}

main
