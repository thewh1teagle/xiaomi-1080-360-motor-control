#!/bin/sh

set -e

if [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ] ; then
    echo "Usage: $0 <HOST> <USERNAME> <PASSWORD>"
    echo
    echo "Pull libraries from remote camera over FTP."
    exit 1
fi

HOST="$1" USERNAME="$2" PASSWORD="$3"


curl -u "$USERNAME:$PASSWORD" "ftp://$HOST/mnt/sdcard/bin/file' -o file

echo "finished!"


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
