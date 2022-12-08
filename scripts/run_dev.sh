#!/bin/bash
SNARK_PATH="$HOME/snarkOS"
WORK_PATH=$(pwd)

function start_up_devnet {
    if [ $1 == 1 ]; then
        cd $SNARK_PATH
        echo "Start Up Update"
        nohup git pull >$SNARK_PATH/update.log 2>&1 &
        echo "Start Up Beacon"
        nohup cargo run --release -- start --nodisplay --dev 0 --beacon "" >$WORK_PATH/beacon.log 2>&1 &
        echo "Start Up Prover"
        nohup cargo run --release -- start --nodisplay --dev 1 --prover "" >$WORK_PATH/prover.log 2>&1 &
        echo "Start Up Validator"
        nohup cargo run --release -- start --nodisplay --dev 2 --validator "" >$WORK_PATH/validator.log 2>&1 &
        cd $(pwd)
    fi
}

function start_up_pools {
    if [ $1 == 2 ]; then
        echo "Compile Aelo Pools"
        nohup cargo build --release >cargo.log 2>&1 &
        echo "Start Up Pool"
        nohup $WORK_PATH/target/release/pool >pool.log 2>&1 &
        echo "Start Up Client"
        nohup $WORK_PATH/target/release/client >client.log 2>&1 &
    fi
}

function kill_devnet {
    if [ $1 == 0 ]; then
        echo "Killall Beacon and Prover"
        killall snarkos
        echo "Clear Beacon log and Prover log"
        rm $SNARK_PATH/beacon.log $SNARK_PATH/prover.log

    fi
}

function kill_pools {
    if [ $1 == 3 ]; then
        echo "Kill Pool"
        killall pool
        echo "Kill Client"
        killall client
        echo "Rm All Log File"
        rm *.log
    fi
}

function print_log {
    if [ $1 == "prover" ]; then
        tail -f $SNARK_PATH/prover.log
    elif [ $1 == "beacon" ]; then
        tail -f $SNARK_PATH/beacon.log
    fi
}

start_up_devnet $1
kill_devnet $1
print_log $1
start_up_pools $1
kill_pools $1
