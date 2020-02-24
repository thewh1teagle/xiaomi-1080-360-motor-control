#!/bin/bash

# set -e
set -x

if [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ] ; then
    echo "Usage: $0 <HOST> <USERNAME> <PASSWORD> <OUTPATH>"
    echo
    echo "Pull libraries from remote camera over FTP."
    exit 1
fi

HOST="$1" USERNAME="$2" PASSWORD="$3"
PATH="$3"

fetch_libs() { # params : ftp folder path, libs strings array
    path=$1 arr=$2
    for LIB in "${"$arr"[@]}"
    do
        curl -u "$USERNAME:$PASSWORD" "ftp://${HOST}$1/$LIB" --output "./lib/$LIB" 
    done
}


declare -a mnt_libs=("libdevice_kit.so.0.0.1" "libmortox.so.0.0.0" "libboardav.so.1.0.0" \
                    "libmortox_share.so.0.0.0" "libmiio_util.so.0.0.1" "libmortoxev.so.0.0.0" \
                    "libmi_isp.so" "libMTE_LINUX.so")


declare -a root_libs=("")

fetch_libs "/mnt/data/lib" mnt_libs
