# docker build -t cca --network=host .
# docker run --name cca -itd --network=host -v $(pwd):/root/coding-competitions-archive -v $HOME/.ssh:/root/.ssh cca
# docker exec -it cca bash
FROM ubuntu

RUN apt-get update && \
    apt-get install -y \
    software-properties-common \
    build-essential \
    python3-pip \
    poppler-utils \
    curl \
    git \
    fzf

COPY inputrc /root/.inputrc
COPY bashrc /root/.bashrc

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN pip install -U 'python-lsp-server[all]'
RUN rustup component add rust-analyzer

RUN curl -LJo helix.tar.xz https://github.com/helix-editor/helix/releases/download/23.10/helix-23.10-x86_64-linux.tar.xz
RUN tar -xvf helix.tar.xz
RUN mv helix-*/hx /usr/local/bin
RUN mkdir -p /root/.config/helix && mv helix-*/runtime /root/.config/helix/runtime

COPY helix-config.toml /root/.config/helix/config.toml
COPY helix-languages.toml /root/.config/helix/languages.toml
