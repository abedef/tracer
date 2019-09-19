#!/bin/bash

clear
cargo run > image.ppm
#tmux kill-pane -t 2
#tmux select-pane -t 1
#tmux split-pane -v /home/abed/Documents/feh/bin/feh -Z -g 300x300 --force-aliasing --auto-reload
tmux select-pane -t 1
