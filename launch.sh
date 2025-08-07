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

# Set environment variables for better font rendering
export TERM=xterm-256color

# Launch in a new terminal window based on available terminal emulator
if command -v gnome-terminal &> /dev/null; then
    # GNOME Terminal (Ubuntu, Fedora, etc.)
    gnome-terminal --title="Text Adventure" \
                  --geometry=120x35 \
                  --font="Monospace 12" \
                  -- bash -c "cd \"$DIR\" && clear && TERM=xterm-256color cargo run; read -p 'Press Enter to exit...'"
elif command -v konsole &> /dev/null; then
    # KDE Konsole
    konsole --title="Text Adventure" \
           --profile="Text Adventure" \
           --hide-menubar \
           --hide-tabbar \
           --workdir="$DIR" \
           -e bash -c "cd \"$DIR\" && clear && TERM=xterm-256color cargo run; read -p 'Press Enter to exit...'"

    # Create a temporary profile for Konsole if it doesn't exist
    if [ ! -f ~/.local/share/konsole/TextAdventure.profile ]; then
        mkdir -p ~/.local/share/konsole
        cat > ~/.local/share/konsole/TextAdventure.profile << EOF
[Appearance]
ColorScheme=GreenOnBlack
Font=Monospace,12,-1,5,50,0,0,0,0,0

[General]
Name=Text Adventure
Parent=FALLBACK/
EOF
    fi
elif command -v xterm &> /dev/null; then
    # xterm (fallback)
    xterm -title "Text Adventure" \
          -fa "Monospace" \
          -fs 12 \
          -geometry 120x35 \
          -bg black \
          -fg green \
          -e "cd \"$DIR\" && clear && TERM=xterm-256color cargo run; read -p 'Press Enter to exit...'"
elif command -v x-terminal-emulator &> /dev/null; then
    # Debian/Ubuntu alternative
    x-terminal-emulator -title "Text Adventure" \
                       -geometry 120x35 \
                       -e "cd \"$DIR\" && clear && TERM=xterm-256color cargo run; read -p 'Press Enter to exit...'"
else
    # If no suitable terminal is found, run in the current terminal
    echo "No suitable terminal emulator found. Running in current terminal..."
    clear
    TERM=xterm-256color cargo run
fi

echo "Game launched successfully!"