**Introdução**

Este projeto contém um exemplo mínimo para programar em C++ usando Docker, sem precisar instalar compiladores localmente.

Arquivos criados:
- [docker-compose.yml](docker-compose.yml)
- [src/main.cpp](src/main.cpp)

**Como funciona (resumo)**
- A imagem Docker fornece o compilador/ambiente.
- No desenvolvimento usamos um volume (bind mount) `./src:/app`, então você edita no PC e o container vê as mudanças.
- O comando do container compila e executa `main.cpp` a cada start.

**Usar (C++)**
1. Abra um terminal na pasta do projeto.
2. Suba o container (build + run):
```bash
docker compose up --build
```
3. Edite `src/main.cpp` localmente. Para ver alterações, pare (`Ctrl+C`) e rode novamente.
4. Para abrir um shell no container em execução:
```bash

docker compose exec cpp sh
```

**Modelos rápidos para outras linguagens**

Rust (desenvolvimento, bind mount):
Dockerfile exemplo:
```Dockerfile
FROM rust:1.72
WORKDIR /app
CMD ["sh", "-c", "cargo run"]
```
docker-compose.yml snippet:
```yaml
services:
  rust:
    build: .
    volumes:
      - ./src:/app
    command: cargo run
```

C (gcc) — praticamente igual ao C++:
```Dockerfile
FROM gcc:12
WORKDIR /app
CMD ["sh", "-c", "gcc main.c -o app && ./app"]
```

Go (execução rápida):
```Dockerfile
FROM golang:1.21
WORKDIR /app
CMD ["sh", "-c", "go run ."]
```

Observações:
- Para produção, prefira copiar o código para a imagem (`COPY`) e construir o binário durante o `docker build`.
- Se mudar dependências (ex.: `Cargo.toml`, `go.mod`, `requirements.txt`), recrie a imagem com `--build`.
- Use `docker compose exec <service> sh` para depurar/interagir.

Se quiser, eu posso também adicionar um `Dockerfile` que faz um build multi-stage (compilação dentro do container e imagem final enxuta) para C++.

**Fluxo básico de programação**

- 1) Edite e salve seus arquivos dentro da pasta `src/` no seu computador.
- 2) Suba o container (se for a primeira vez ou se tiver mudado dependências/code que fazem parte da imagem, adicione `--build`):
```bash
docker compose up --build
```
- 3) Fluxo para ver as alterações:
  - Linguagens interpretadas (ex.: Python, Go `run`, Rust com `cargo run` em dev): salve o arquivo e o processo dentro do container normalmente recarrega/usa a nova versão automaticamente (dependendo do app). Se não, reinicie o serviço com `docker compose restart`.
  - Linguagens compiladas (C/C++): após salvar, é necessário recompilar dentro do container. Opções:
    - Pare e suba novamente: `docker compose down` então `docker compose up` (ou `--build` quando mudar dependências).
    - Build/exec one-off (compilar e rodar manualmente):
```bash
docker compose run --rm cpp sh -c "g++ main.cpp -o app && ./app"
```
- 4) Parar e remover containers (limpar):
```bash
docker compose down
```

Dicas rápidas:
- Use `docker compose exec <service> sh` para abrir um shell e depurar dentro do container.
- Para desenvolvimento use `volumes: - ./src:/app` (bind mount). Para imagem de produção, prefira `COPY` no `Dockerfile` e build multi-stage.



ATALHOS:

```bash

# Docker Compose shortcuts
alias dcu='docker compose up --build'        # sobe e builda
alias dcud='docker compose up -d --build'    # sobe em background
alias dcd='docker compose down'               # para e remove
alias dcr='docker compose restart'            # reinicia serviço
alias dcl='docker compose logs -f'            # ver logs em tempo real
alias dcb='docker compose up -d --build'      # rebuild + start detached
alias dce='docker compose exec'      # rebuild + start detached
alias dcude='dcud && dce'

```

Depois de colar rode:

```bash

source ~/.bashrc

```


Alterado como funciona:

Faça o container subir sem compilar nem executar nada, apenas ficando disponível.

Dockerfile
FROM gcc:12
WORKDIR /app
CMD ["bash"]


Agora o container só abre um shell e fica aguardando comandos.

```bash
docker-compose.yml
version: "3.8"
services:
  cpp:
    build: .
    volumes:
      - ./src:/app
    working_dir: /app
    tty: true
    stdin_open: true
  ```

  ```

# Use
docker compose up -d
docker compose exec cpp bash

#OU

dcude cpp bash
```


e depois para executar um arquivo cpp só dizer o caminho e o nome do arquivo.

assim:

```bash
b main.cpp # se estiver dentro do src.

# Se estiver dentro de uma pasta aí tem que dizer a pasta e tudo mais.
```
