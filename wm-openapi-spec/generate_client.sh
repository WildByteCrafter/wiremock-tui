#!/bin/bash

# 1. Get the absolute path to the directory where THIS script lives
SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)

# 2. Get the absolute path to the project root (one level up from the script)
PROJECT_ROOT=$(cd "${SCRIPT_DIR}/.." && pwd)

# 3. Use the absolute path for the Docker mount
docker run --rm \
    -v "${PROJECT_ROOT}:/local" \
    openapitools/openapi-generator-cli generate \
    -i "/local/wm-openapi-spec/wm-v3_13_2.json" \
    -g rust \
    -o "/local/wm-client" \
    --skip-validate-spec \
    --additional-properties=packageName=wm-client,supportAsync=true