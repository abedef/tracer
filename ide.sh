#!/bin/bash

tmux split-window -h
tmux run-shell 'rustup docs --std'
tmux run-shell -b '/home/abed/Documents/feh/bin/feh -Z -g 300x300 --force-aliasing --auto-reload -x'
tmux select-pane -t 0
tmux resize-pane -x80
tmux send-keys 'echo src/* | entr ./run.sh'
tmux send-keys Enter
tmux select-pane -t 1
tmux send-keys 'vim src'
tmux send-keys Enter
