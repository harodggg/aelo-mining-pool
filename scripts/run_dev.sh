#!/bin/bash
SNARK_PATH="$HOME/snarkOS"
WORK_PATH=$(pwd)

function start_up {
    if [ $1 == 1 ]; then
        cd $SNARK_PATH
        echo "Start up beacon"
        nohup cargo run --release -- start --nodisplay --dev 0 --beacon "" >$SNARK_PATH/beacon.log 2>&1 &
        echo "Start up prover"
        nohup cargo run --release -- start --nodisplay --dev 1 --prover "" >$SNARK_PATH/prover.log 2>&1 &
        cd $(pwd)
    fi
}

function kill_devnet {
    if [ $1 == 0 ]; then
        echo "Killall beacon and prover"
        killall snarkos
        echo "clear beacon log and prover log"
        rm $SNARK_PATH/beacon.log $SNARK_PATH/prover.log

    fi
}

start_up $1
kill_devnet $1
