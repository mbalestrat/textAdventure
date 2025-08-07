#!/bin/bash

# Get the directory where this script is located
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Change to the project directory
cd "$DIR"

# Clear terminal and display a launching message
clear
echo "================================"
echo "   Launching Text Adventure..."
echo "================================"
echo ""
echo "A new terminal window will open with the game."
echo "Close that window to exit the game."
echo ""

# Launch a new terminal window with the game
osascript -e '
tell application "Terminal"
    do script "cd \"'"$DIR"'\" && clear && TERM=xterm-256color && cargo run"
    set background color of first window to {0, 0, 0}
    set normal text color of first window to {32768, 32768, 32768}
    set custom title of first window to "Text Adventure"
    set font name of first window to "Menlo"
    set font size of first window to 14
    set position of first window to {100, 100}
    set size of first window to {100, 35}
    set number of columns of first window to 100
    set number of rows of first window to 35
end tell
'

echo "Game launched successfully!"