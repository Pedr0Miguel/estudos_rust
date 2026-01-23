# Learn to Code with Rust 🦀

Um ambiente Docker completo para aprender Rust com todas as ferramentas de desenvolvimento pré-configuradas.

## 📋 Estrutura

Este projeto é um repositório educacional com múltiplos exemplos e exercícios Rust:

- **learn-to-code-with-rust/**: Exercícios organizados por tópico (closures, control-flow, data-types, enums, functions, generics, hash-maps, iterators, lifetimes, option-and-result-enums, ownership, random, regular-expressions, slices, smart-pointers, strings, structs, testing, traits, variables-and-mutability, vectors, etc.)
- **.devcontainer/**: Configuração completa para desenvolvimento em VS Code Dev Containers

## 🚀 Usar com VS Code Dev Containers

1. Abra a pasta do projeto no VS Code
2. Clique em "Reabrir em Container" quando aparecer a notificação
3. Pronto! O ambiente Rust está configurado automaticamente

Todas as extensões e ferramentas serão instaladas automaticamente.

## 🐳 Usar com Docker Compose

```bash
# Build e start o container
docker compose up -d

# Abrir um shell no container
docker compose exec rust bash
```

## 📦 Ferramentas Instaladas

**Ambiente:**
- Rust 1.82 stable com rustfmt, clippy, rust-analyzer
- Git, curl, build-essential, pkg-config

**VS Code Extensions:**
- rust-analyzer: IntelliSense e análise de código Rust
- even-better-toml: Suporte a TOML
- Crates: Gerenciador de dependências
- LLDB: Debug integrado
- GitHub Copilot: Assistente de código
- Test Explorer: Interface visual para testes

**Cargo Tools:**
- `cargo-watch`: Recompilar automaticamente ao salvar
- `cargo-edit`: Editar Cargo.toml de forma interativa
- `cargo-tree`: Visualizar árvore de dependências
- `cargo-expand`: Expandir macros
- `cargo-audit`: Auditoria de vulnerabilidades
- `cargo-outdated`: Verificar atualizações de dependências
- `bacon`: Execução contínua de testes

## 📚 Executar Exercícios

Cada pasta em `learn-to-code-with-rust/` contém:
- `src/main.rs`: Exemplo/introdução do tópico
- `src/coding_challenge.rs`: Desafio para praticar
- `Cargo.toml`: Configuração e dependências

### Exemplos:

```bash
# Navegar para um exercício
cd learn-to-code-with-rust/functions

# Executar o exemplo
cargo run

# Executar o desafio
cargo run --bin coding_challenge

# Rodar testes
cargo test

# Com cargo-watch (recompila ao salvar)
cargo watch -x run
```

## 📝 Atalhos Úteis

```bash
# Compilar sem executar
cargo check

# Compilar com otimizações
cargo build --release

# Executar todos os testes
cargo test

# Executar testes com output
cargo test -- --nocapture

# Formatar código
cargo fmt

# Lint (verificar erros)
cargo clippy

# Expandir macros
cargo expand

# Ver estrutura de dependências
cargo tree

# Verificar vulnerabilidades
cargo audit
```

## 🔧 Solução de Problemas

**Container não inicia?**
```bash
docker compose down
docker compose up --build
```

**Ferramentas cargo falhando ao instalar?**
Tente instalar manualmente dentro do container:
```bash
docker compose exec rust cargo install cargo-watch
```

**Precisa limpar cache Rust?**
```bash
docker compose exec rust cargo clean
```

## 📖 Recursos

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
