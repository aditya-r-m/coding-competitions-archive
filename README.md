# coding-competitions-archive

This repository contains an archive of competition problems for
Coding Competitions, Google-organized programming competitions. 
Problem data for problems used in the following contests is updated:

 - Distributed Code Jam
 - Code Jam
 - Code Jam to I/O for Women
 - Hash Code
 - Kick Start

The statement and analysis HTML files may have CSS or JS dependencies that are not provided here. Some of the custom judging code may also have library dependencies that are not provided, so they may not work as is. All problem-specific logic is there, however, so it should be useful if you intend to reproduce the behavior.

Some analysis for Distributed Code Jam problems could not be added, so a commented solution for the problem was added instead.

There is no plan to add data for problems for other contests (like Code Jam for Europython, or local/regional Code Jams) at this time.

## Solutions

Code Jam Solutions are implemented in Rust for problem sets from 2008,9.

Convenience scripts such as the following can be used to create an IDE setup using [tmux](https://github.com/tmux/tmux)+[helix](https://github.com/helix-editor/helix),

```
alias sd='cd $(find * -type d | fzf)'
alias gd='git diff --word-diff'
function rn() {
    cargo run --release --bin "$(basename $(pwd))" < data/secret/subtask$1/1.in > data/secret/subtask$1/1.ans &&
        gd --color data/secret/subtask$1/1.ans | tail -n +6 | head -n 32
    if [ -z "$(gd data/secret/subtask$1/1.ans)" ]; then
        echo "Validation successful!"
    fi
}
alias tx='tmux new-session \; set -s escape-time 0 \; split-window -h -l 150 \; set -g status off \; bind-key 1 send-keys -t 0 "rn 1" Enter \; bind-key 2 send-keys -t 0 "rn 2" Enter \; attach'
alias xr='hx solution.rs'
```

This includes the following commands,
  - `sd` for navigating repository
  - `tx` to start tmux split-pane session for code execution and editor views
  - `xr` to start helix editor for solution.rs
  - `Ctrl+b 1`,`Ctrl+b 2` to test the solution implementations against the provided answers
