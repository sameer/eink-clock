#!/bin/bash
cargo run
sshpass -p root scp out.png 192.168.2.2:/dev/shm/
sshpass -p root ssh 192.168.2.2 /usr/sbin/eips -g /dev/shm/out.png
