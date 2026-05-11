/// # Rust Básico — Ponto de Entrada
///
/// Este arquivo importa e chama os exemplos de cada módulo.
/// Execute com:  cargo run

mod async_exemplo;
mod dashmap_exemplo;
mod funcoes;
mod inteiros;
mod listas;

#[tokio::main]
async fn main() {
    inteiros::demonstrar_inteiros();
    listas::demonstrar_listas();
    funcoes::demonstrar_funcoes();
    dashmap_exemplo::demonstrar_dashmap();
    async_exemplo::demonstrar_async().await;
}
