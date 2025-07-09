#!/bin/bash
# 1. CREATE (POST)
echo "Criando um registro..."
resp=$(curl -s -X POST https://crude-production.up.railway.app/data \
  -H 'Content-Type: application/json' \
  -d '{"data1": ["primeiro", "segundo"], "data2": [1,2,3]}' )
echo "Resposta: $resp"
id=$(echo $resp | grep -oE '"id": *[0-9]+' | grep -oE '[0-9]+')
echo "ID criado: $id" 