FROM rust:1.82-bullseye

# Fetch packages and upgrade
RUN apt update -y && apt upgrade -y

# Install zsh and sudo
RUN apt install zsh sudo -y

# Set it to the default shell
RUN sudo chsh -s $(which zsh)

# Install OhMyZSH
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# Install rustfmt
RUN rustup component add rustfmt