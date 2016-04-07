#!/bin/bash

set -e

groupadd dev --gid ${TARGET_GID?:}
useradd dev \
    --uid ${TARGET_UID?:} \
    --gid dev \
    --groups sudo \
    --password $(perl -e'print crypt("dev", "aa")')

echo "dev ALL=(ALL:ALL) NOPASSWD: ALL" >> /etc/sudoers

if [[ $# != 0 ]]; then
    exec chpst -u dev:dev "$@"
else
#    exec bash -i
    exec chpst -u dev:dev bash -i
fi

