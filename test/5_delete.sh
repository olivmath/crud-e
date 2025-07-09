#!/bin/bash
# 5. DELETE (DELETE)

if [ -z "$1" ]; then
  read -p "Digite o id do registro a ser deletado: " id
else
  id="$1"
fi

echo "Deletando o registro (id=$id)..."
curl -s -X DELETE https://crude-production.up.railway.app/data/$id -w "Status: %{http_code}\n"
echo

echo "Tentando ler o registro deletado (id=$id)..."
curl -s https://crude-production.up.railway.app/data/$id -w "\nStatus: %{http_code}\n" 