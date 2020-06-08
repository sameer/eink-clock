#!/bin/bash
target/release/eink-clock | sshpass -p root ssh root@192.168.2.2 'cat > /dev/shm/out.png'
sshpass -p root ssh 192.168.2.2 /usr/sbin/eips -g /dev/shm/out.png
