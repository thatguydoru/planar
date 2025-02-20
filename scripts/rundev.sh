#!/usr/bin/bash

db_name='db.sqlite3'

export DATABASE_URL="sqlite:${db_name}"
export PLANAR_LOG='debug'

if [ ! -f ${db_name} ]; then
    if ! cargo sqlx database setup ; then
        rm ${db_name}*
        exit 1
    fi
fi

cargo run
