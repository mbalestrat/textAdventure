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
    # Create new window with proper settings
    do script "cd \"'"$DIR"'\" && clear && export TERM=xterm-256color && stty rows 35 cols 100 && cargo run"
    
    # Wait a moment for window to be created
    delay 0.5
    
    # Configure the new window
    tell front window
        set background color to {0, 0, 0}
        set normal text color to {65535, 65535, 65535}
        set custom title to "Text Adventure"
        set font name to "Menlo"
        set font size to 14
        
        # Set window position and size more reliably
        set position to {100, 100}
        set size to {800, 600}
        
        # Force terminal dimensions
        do script "resize -s 35 100" in front tab
    end tell
    
    # Bring Terminal to front
    activate
end tell
'

echo "Game launched successfully!"