# rust_basics

Exemplos didáticos e comentados dos conceitos básicos de Rust, escritos em português.

## Conteúdo

| Arquivo | Tema |
|---|---|
| `src/inteiros.rs` | Tipos inteiros, operações aritméticas, casting e proteção contra overflow |
| `src/listas.rs` | `Vec<T>`, arrays de tamanho fixo, slices, iteração, map/filter e ordenação |
| `src/funcoes.rs` | Funções, parâmetros, retorno, `Option`, `Result`, closures e recursão |
| `src/dashmap_exemplo.rs` | `DashMap` — mapa concorrente chave→valor seguro para múltiplas threads |
| `src/async_exemplo.rs` | Programação assíncrona com `async`/`await` e o runtime `tokio` |

---

## Pré-requisitos

Instale o **Rust** e o **Cargo** (gerenciador de pacotes) pelo instalador oficial:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Depois abra um novo terminal (ou rode `source ~/.cargo/env`) e confirme a instalação:

```bash
rustc --version   # ex.: rustc 1.78.0
cargo --version   # ex.: cargo 1.78.0
```

---

## Como compilar e executar

Clone o repositório e entre na pasta:

```bash
git clone https://github.com/ghpascon/rust_basics.git
cd rust_basics
```

### Executar todos os exemplos

```bash
cargo run
```

O programa imprimirá na tela os exemplos de cada módulo, na ordem:
1. Inteiros
2. Listas
3. Funções
4. DashMap
5. Async/Await

### Compilar sem executar

```bash
cargo build          # modo debug (padrão, mais rápido de compilar)
cargo build --release  # modo release (binário otimizado)
```

O binário gerado fica em `target/debug/rust_basics` (ou `target/release/rust_basics`).

### Executar os testes

Cada módulo possui testes unitários. Para rodar todos:

```bash
cargo test
```

Para rodar os testes de um módulo específico (ex.: funções):

```bash
cargo test funcoes
```

---

## Dependências

As dependências são declaradas em `Cargo.toml` e baixadas automaticamente pelo Cargo:

| Crate | Versão | Uso |
|---|---|---|
| [dashmap](https://crates.io/crates/dashmap) | 6 | Mapa concorrente seguro para threads |
| [tokio](https://crates.io/crates/tokio) | 1 | Runtime assíncrono |

---

## Estrutura do projeto

```
rust_basics/
├── Cargo.toml              # Metadados e dependências do projeto
├── Cargo.lock              # Versões exatas das dependências (gerado automaticamente)
├── README.md               # Este arquivo
└── src/
    ├── main.rs             # Ponto de entrada — chama todos os exemplos
    ├── inteiros.rs         # Exemplos de tipos inteiros
    ├── listas.rs           # Exemplos de Vec, arrays e slices
    ├── funcoes.rs          # Exemplos de funções e closures
    ├── dashmap_exemplo.rs  # Exemplos de DashMap
    └── async_exemplo.rs    # Exemplos de async/await com tokio
```
