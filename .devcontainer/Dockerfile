FROM rust:1.82-bullseye

# Install zsh and set it to the default shell
RUN apt install zsh
RUN chsh -s $(which zsh)

# Install OhMyZSH
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
