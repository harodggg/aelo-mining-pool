#!/bin/bash
SNARK_PATH="../../snarkOS"
WORK_PATH=$(pwd)

cd $SNARK_PATH
nohup cargo run --release -- start --nodisplay --dev 0 --beacon "" >beacon.log 2>&1 &
nohup cargo run --release -- start --nodisplay --dev 1 --prover "" >prover.log 2>&1 &
cd $(pwd)


