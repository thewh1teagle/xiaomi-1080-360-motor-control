#!/bin/sh

set -e

if [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ] ; then
    echo "Usage: $0 <HOST> <USERNAME> <PASSWORD>"
    echo
    echo "Pull libraries from remote camera over FTP."
    exit 1
fi

HOST=$1 USERNAME=$2 PASSWORD=$3

curl -u "$USERNAME:$PASSWORD" "ftp://$HOST/mnt/sdcard/bin/file' -o file

echo "finished!"
