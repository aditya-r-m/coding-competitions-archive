# docker build -t cca --network=host .
# docker run --name cca -itd --network=host -v $(pwd):/root/coding-competitions-archive -v $HOME/.ssh:/root/.ssh cca
# docker exec -it cca bash
FROM ubuntu

RUN apt-get update && \
    apt-get install -y \
    build-essential \
    curl \
    git \
    software-properties-common

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup component add rust-analyzer

RUN apt-get update && \
    apt-get install -y python3-pip

RUN pip install -U 'python-lsp-server[all]'

RUN add-apt-repository ppa:maveonair/helix-editor && \
    apt-get update && \
    apt-get install -y helix

RUN apt-get update && \
    apt-get install -y \
    fzf \
    poppler-utils

COPY helix-config.toml /root/.config/helix/config.toml
COPY helix-languages.toml /root/.config/helix/languages.toml
COPY inputrc /root/.inputrc
COPY bashrc /root/.bashrc

