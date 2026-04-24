#!/bin/bash

# Configuration
DATA_PLANE="http://localhost:3000"
CONTROL_PLANE="http://localhost:3001"
TIER_NAME="test_tier_$RANDOM"

echo "1. Creating a '$TIER_NAME' tier (Global 5 req/60s, /api/expensive 2 req/60s)..."
curl -s -X POST $CONTROL_PLANE/tiers \
     -H "Content-Type: application/json" \
     -d "{
           \"name\": \"$TIER_NAME\",
           \"limit\": 5,
           \"window_seconds\": 60,
           \"route_overrides\": [
               { \"path\": \"/api/expensive\", \"limit\": 2, \"window_seconds\": 60 }
           ]
         }"

echo -e "\n\n2. Creating an API Key..."
KEY_RESP=$(curl -s -X POST $CONTROL_PLANE/api-keys \
     -H "Content-Type: application/json" \
     -d "{\"user_id\": \"00000000-0000-0000-0000-000000000001\", \"tier\": \"$TIER_NAME\"}")

API_KEY=$(echo $KEY_RESP | grep -o '"key":"[^"]*' | cut -d'"' -f4)
echo "Generated Key: $API_KEY"

echo -e "\n3. Waiting for Data Plane to sync (30s)..."
sleep 30
echo "--- Testing Global Limit (5 req) on / ---"
for i in {1..6}
do
   echo -n "Request $i: "
   curl -s -o /dev/null -D - $DATA_PLANE/ \
        -H "X-API-Key: $API_KEY" | grep -E "HTTP/|x-ratelimit|error"
   sleep 0.5
done

echo -e "\n--- Testing Route Override (2 req) on /api/expensive ---"
for i in {1..3}
do
   echo -n "Request $i: "
   curl -s -o /dev/null -D - $DATA_PLANE/api/expensive \
        -H "X-API-Key: $API_KEY" | grep -E "HTTP/|x-ratelimit|error"
   sleep 0.5
done
