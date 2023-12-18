#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ..

export SECRET_KEY="secret"
export EXPIRE_MINUTES=120
cargo test