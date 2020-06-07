#!/bin/bash
PATH=$PATH:~/.cargo/bin
eink-clock
sshpass -p root scp out.png 192.168.2.2:/dev/shm/
sshpass -p root ssh 192.168.2.2 /usr/sbin/eips -g /dev/shm/out.png
