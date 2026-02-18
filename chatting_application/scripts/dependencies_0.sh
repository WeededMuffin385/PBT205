sudo apt update

# libssl
sudo apt install pkg-config libssl-dev

# who knows
sudo apt install libnss3-tools

# dev
sudo apt install build-essential

# mkcert
sudo apt install mkcert

# curl
sudo apt install curl

# rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.4/install.sh | bash