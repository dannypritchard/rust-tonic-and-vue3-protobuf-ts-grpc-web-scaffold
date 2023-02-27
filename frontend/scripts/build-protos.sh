#!/bin/bash
ROOT="$(git rev-parse --show-toplevel)"
IN_DIR="${ROOT}/server/_protos/"
OUT_DIR="${ROOT}/frontend/src/protos/"

mkdir -p $OUT_DIR

npx protoc \
    --ts_out $OUT_DIR \
    --proto_path $IN_DIR \
    $IN_DIR/*.proto
