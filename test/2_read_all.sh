#!/bin/bash
# 2. READ ALL (GET)
echo "Lendo todos os registros..."
curl -s https://crude-production.up.railway.app/data | jq 