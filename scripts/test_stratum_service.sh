#!/usr/bin/env bash
WORK_PATH="/Users/g/aelo-mining-pool"
grpcurl -plaintext -import-path $WORK_PATH/crates/protos -proto stratum-pool.proto -d '{"id": 10231}' '[::]:50051' stratum_pool.StratumPool.MiningAuthorize
