#!/bin/bash

# ------------------------------
# Validação de argumento
# ------------------------------
if [ -z "$1" ]; then
    echo "Uso: ./build.sh <arquivo.cpp> (relativo a src/)"
    echo "Exemplo: ./build.sh main.cpp"
    exit 1
fi

# ------------------------------
# Resolve caminho do arquivo
# ------------------------------
ARG="$1"
if [[ "$ARG" = /* || "$ARG" == src/* ]]; then
    ARQUIVO="$ARG"
else
    ARQUIVO="src/$ARG"
fi

SAIDA="bin/$(basename "${ARQUIVO%.cpp}")"

if [ ! -f "$ARQUIVO" ]; then
    echo "Erro: arquivo '$ARQUIVO' não encontrado"
    exit 1
fi

# ------------------------------
# Compilação
# ------------------------------
echo "🛠 Compilando $ARQUIVO..."

INICIO_COMP=$(date +%s%N)
g++ "$ARQUIVO" -o "$SAIDA"
STATUS_COMP=$?
FIM_COMP=$(date +%s%N)

if [ $STATUS_COMP -ne 0 ]; then
    echo "✗ Erro na compilação"
    exit 1
fi

DURACAO_COMP_NS=$((FIM_COMP - INICIO_COMP))
DURACAO_COMP_MS=$((DURACAO_COMP_NS / 1000000))

echo "✓ Compilado com sucesso!"
echo "⏱ Tempo de compilação: ${DURACAO_COMP_MS} ms"

# ------------------------------
# Execução
# ------------------------------
echo "▶ Executando..."

INICIO_EXEC=$(date +%s%N)
./"$SAIDA"
FIM_EXEC=$(date +%s%N)

DURACAO_EXEC_NS=$((FIM_EXEC - INICIO_EXEC))
DURACAO_EXEC_MS=$((DURACAO_EXEC_NS / 1000000))

echo "⏱ Tempo de execução: ${DURACAO_EXEC_MS} ms"
