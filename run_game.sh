#!/bin/bash
DIR="$1"
cd "$DIR"
export TERM=vt100
export COLUMNS=80
export LINES=25
clear
reset
stty columns 80
stty rows 25
tput clear
tput sgr0
echo "Starting game with fixed terminal settings..."
sleep 1
clear
cargo run
exit
