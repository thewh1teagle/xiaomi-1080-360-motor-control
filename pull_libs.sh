#!/bin/bash

if [ "$1" == "" ] || [ "$2" == "" ] || [ "$3" == "" ] 
then
    echo "Usage: ./pull_libs <IP> <FTP USER> <PASS>"
    exit 1
fi

IP=$1 USER=$2 PASS=$3



declare -a StringArray=("libdevice_kit.so.0.0.1" "libmortox.so.0.0.0" "libboardav.so.1.0.0" "libmortox_share.so.0.0.0" "libmiio_util.so.0.0.1" "libmortoxev.so.0.0.0" "libmi_isp.so" "libMTE_LINUX.so" )


for val in ${StringArray[@]}; do
   echo $val
done

# curl -u $USER:$PASS 'ftp://'$IP'/mnt/sdcard/bin/file' -o file

# echo "finished!."


# /mnt/data/lib
# libdevice_kit.so.0.0.1
# libmiio_util.so.0.0.1
# libboardav.so.1.0.0
# libmortox.so.0.0.0
# libmortox_share.so.0.0.0
# libmortoxev.so.0.0.0
# libmi_isp.so
# libMTE_LINUX.so

# /lib
# libOMX_BELA.so
# libOMX_AVQE_A.so
# libpthread-2.25.so
# librt-2.25.so
