#!/bin/bash
PWD=$(pwd)

echo $(basename $PWD)
if [[ $(basename $PWD) -ne "Pantheon" ]]; then
	echo "Please run this script from the root project directory using:"
	echo "./tools/$(basename "$0")"
	exit 1
fi

tmux new-session \; split-window -h \; send-keys 'cd $PWD && clear && cargo run -p tartarus' C-m \; select-pane -L \; send-keys 'cargo run -p hermes' C-m \; attach
