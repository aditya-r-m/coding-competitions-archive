force_color_prompt=yes
export COLORTERM=truecolor
PS1='${debian_chroot:+($debian_chroot)}\[\033[01;32m\]\u\[\033[00m\] \[\033[01;34m\]\w\[\033[00m\] '

export LS_OPTIONS='--color=auto'
alias ls='ls $LS_OPTIONS'
alias ll='ls $LS_OPTIONS -l'
alias l='ls $LS_OPTIONS -lA'

set -o vi
export EDITOR=hx

alias sd="cd ~ && cd \$(find * -type d | fzf)"

function pdf() {
  pdftotext -enc ASCII7 -layout -nopgbrk $1 - | less
}

function crs() {
  spath=$(python3 -c 'print("/".join("'$(pwd)'".split("/")[3:]))')
  sname=$(python3 -c 'print("'$(pwd)'".split("/")[-1])')
  if [ ! -f solution.rs ]; then
    touch solution.rs
    printf "\n[[bin]]\n name = \""$sname"\"\n path=\""$spath"/solution.rs\"\n" >> /root/coding-competitions-archive/Cargo.toml
  fi
}

function hxs() {
  if [ -f solution.rs ]; then
    hx solution.rs
  else
    echo -ne "no solution found\n"
  fi
}

function fw() {
  [[ -z "$!" ]] && { return 0; }
  echo -ne "cpu \e[45m \e[0m\nmem \e[44m \e[0m\nsec \e[40m \e[0m\n"

  while true; do
    sleep 1
    outs=""

    cpuf=$(top -b -d1 -n1 | grep $! | awk '{print $9}')
    [[ -z "$cpuf" ]] && { return 0; }
    cpu=${cpuf%.*}
    outs="${outs}cpu "
    for i in $(seq 0 $(($cpu > 100 ? 100 : $cpu))); do outs="${outs}\e[45m \e[0m"; done;
    outs="${outs} $cpu%\n"

    memf=$(ps -q $! -eo %mem= | xargs)
    [[ -z "$memf" ]] && { return 0; }
    rss=$(ps -q $! -eo rss= | xargs)
    [[ -z "$rss" ]] && { return 0; }
    lgrss=0
    while [ $(($rss>>$lgrss)) -gt 0 ]; do lgrss=$(($lgrss+1)); done
    outs="${outs}mem "
    for i in $(seq 0 $(($lgrss > 100 ? 100 : $lgrss))); do outs="${outs}\e[44m  \e[0m"; done;
    outs="${outs} $rss($memf%)\n"

    sec=$(ps -q $! -eo etimes= | xargs)
    [[ -z "$sec" ]] && { return 0; }
    outs="${outs}sec "
    for i in $(seq 0 $(($sec > 100 ? 100 : $sec))); do outs="${outs}\e[40m \e[0m"; done;
    outs="${outs} $sec\n"

    echo -ne '\033[1A\033[K'
    echo -ne '\033[1A\033[K'
    echo -ne '\033[1A\033[K'
    echo -ne $outs
  done
}

s1in='data/secret/subtask1/1.in'
s1out='data/secret/subtask1/1.ans'
s2in='data/secret/subtask2/1.in'
s2out='data/secret/subtask2/1.ans'

function sio() {
  if [ $1 == '1' ]; then
    sin=$s1in
    sout=$s1out
  else
    sin=$s2in
    sout=$s2out
  fi
}

function rn() {(
  crs
  sio $1
  cargo build --color always --release --bin $sname 2> $sout
  if [ $? -eq 101 ]; then
    less -R $sout
    exit 0
  fi
  /root/coding-competitions-archive/target/release/$sname < $sin &> $sout &
  fw
  less $sout
)}

alias rns="rn 1"
alias rnl="rn 2"
alias ga="git add"
alias gc="git commit"
alias gd="git diff"
alias gp="git push"
alias gs="git status"
