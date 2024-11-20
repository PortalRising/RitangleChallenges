FROM rust:1.82-bullseye

# Install zsh and sudo
RUN apt install zsh sudo

# Set it to the default shell
RUN sudo chsh -s $(which zsh)

# Install OhMyZSH
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
