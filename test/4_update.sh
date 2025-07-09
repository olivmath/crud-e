#!/bin/bash
# 4. UPDATE (PUT)

if [ -z "$1" ]; then
  read -p "Digite o id do registro a ser atualizado: " id
else
  id="$1"
fi

echo "Atualizando o registro (id=$id)..."
curl -s -X PUT https://crude-production.up.railway.app/data/$id \
  -H 'Content-Type: application/json' \
  -d '{"data1": ["atualizado"], "data2": [9,8,7]}'
echo

echo "Lendo novamente o registro atualizado (id=$id)..."
curl -s https://crude-production.up.railway.app/data/$id | jq 