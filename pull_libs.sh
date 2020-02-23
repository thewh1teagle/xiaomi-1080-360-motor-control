#!/usr/bin/bash

if [ "$1" == "" ] || [ "$2" == "" ] || [ "$3" == "" ] 
then
    echo "Usage: ./pull_libs <IP> <FTP USER> <PASS>"
    exit 1
fi

IP=$1 USER=$2 PASS=$3

curl -u $USER:$PASS 'ftp://'$IP'/mnt/sdcard/bin/file' -o file

echo "finished!."
