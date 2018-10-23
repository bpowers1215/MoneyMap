#!/bin/bash
set -m

mongodb_cmd="mongod --storageEngine $STORAGE_ENGINE --bind_ip_all --profile=1 --slowms=1"
cmd="$mongodb_cmd"
if [ "$AUTH" == "yes" ]; then
    cmd="$cmd --auth"
fi

if [ "$JOURNALING" == "no" ]; then
    cmd="$cmd --nojournal"
fi

if [ "$OPLOG_SIZE" != "" ]; then
    cmd="$cmd --oplogSize $OPLOG_SIZE"
fi

$cmd &

if [ ! -f /data/db/mongodb_password_set ]; then
    echo "========================================================================"
    echo ""
    RET=1
    while [[ RET -ne 0 ]]; do
        echo "=> Waiting for confirmation of MongoDB service startup"
        sleep 5
        mongo admin --eval "help" >/dev/null 2>&1
        RET=$?
    done

    echo "=> INITIALIZE DATABASE AND CREATE USERS"
    mongo admin /db_config/init_db.js

    echo "=> Done!"
    touch /data/db/mongodb_password_set
    echo "========================================================================"
fi

fg
