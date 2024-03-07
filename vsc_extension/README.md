The VSC-extension provides convenience commands in for the following,
- Dark mode reader for problem statement & analysis
- Create solution template
- Edit & Run test sets


Package & Install the vsix file locally,
```
vsce package
codium --install-extension coding-competitions-archive-1.0.0.vsix
```


The dependencies include rust, poppler-utils, & imagemagick,
```
sudo apt-get update
sudo apt-get install -y \
    software-properties-common \
    build-essential \
    poppler-utils \
    imagemagick

curl https://sh.rustup.rs -sSf | bash -s -- -y
rustup component add rust-analyzer # for rust-analyzer extension
```

If you use VS-code, please edit the `codium` commands in `extension.js` with `code`.
