#!/bin/bash
source .env

docker build $(pwd) -t users_schema_up
docker run \
    -w /code \
    --env-file .env \
    --network "$NETWORK" \
    users_schema_up \
    npm run migration