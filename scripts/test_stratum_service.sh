#!/usr/bin/env bash
grpcurl -plaintext -import-path  ../crates/protos -proto stratum.proto -d '{"id": 10231}' '[::]:50051' stratum.Stratum.MiningShare
