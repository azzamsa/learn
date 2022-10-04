#!/bin/sh

os=$(lsb_release -d)
cpu=$(grep 'model name' /proc/cpuinfo | uniq)
memory=$(grep MemTotal /proc/meminfo)

os=$(echo "$os" | xargs) # remove whitspace after `Description:`
echo "$os" | sed 's/Description:/OS/'
echo "Kernel: $( uname -srm)"

cpu=$(echo "$cpu" | xargs)
echo "$cpu" | sed 's/model name/CPU/'

memory=$(echo "$memory" | xargs)
echo "$memory" | sed 's/MemTotal:/Memory:/'
