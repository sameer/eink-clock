while :
do
    cargo run
    sshpass -p root scp out.png 192.168.2.2:/mnt/us/
    sshpass -p root ssh 192.168.2.2 /usr/sbin/eips -g /mnt/us/out.png
    sleep 55s
done