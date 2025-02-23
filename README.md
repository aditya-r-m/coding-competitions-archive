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

Code Jam Solution are implemented in Rust for problem-sets from 2008,9 timeframe, and in Python for the problem-sets that follow.

The following convenience commands can be used to create a tmux based IDE setup with shortcuts such as `Ctrl+b 1` to test the solution implementations against the provided answers,

```
alias sd='cd $(find * -type d | fzf)'
alias gs='git status'
alias gd='git diff --word-diff'
alias ga='git add -A'
alias gc='git commit'
alias gp='git push'
alias gl='git log'
function rn() {
    python3 solution.py < data/secret/subtask$1/1.in > data/secret/subtask$1/1.ans && gd --color data/secret/subtask$1/1.ans | head -n 32
}
alias tx='tmux new-session \; set -s escape-time 0 \; split-window -h -l 150 \; set -g status off \; bind-key 1 send-keys -t 0 "rn 1" Enter \; bind-key 2 send-keys -t 0 "rn 2" Enter \; attach'
```
