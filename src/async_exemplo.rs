/// # Programação Assíncrona em Rust
///
/// Rust suporta programação assíncrona com as palavras-chave `async` e `await`.
/// Para executar código assíncrono é necessário um **runtime** como o `tokio`.
///
/// Conceitos principais:
/// - `async fn` → define uma função assíncrona (retorna um `Future`)
/// - `.await`   → aguarda a conclusão de um `Future`
/// - `tokio::spawn` → executa uma `task` concorrentemente
/// - `tokio::time::sleep` → aguarda um tempo sem bloquear a thread

use std::time::Duration;
use tokio::time::sleep;

/// Ponto de entrada para as demonstrações assíncronas.
/// Deve ser chamada dentro de um contexto assíncrono (ex.: #[tokio::main]).
pub async fn demonstrar_async() {
    println!("=== Programação Assíncrona (async/await) ===\n");

    // -----------------------------------------------------------------------
    // Função async simples
    // -----------------------------------------------------------------------
    println!("--- Função async simples ---");
    let mensagem = buscar_mensagem().await;
    println!("Mensagem recebida: {mensagem}");

    // -----------------------------------------------------------------------
    // Função async com parâmetros e retorno
    // -----------------------------------------------------------------------
    println!("\n--- Cálculo assíncrono ---");
    let resultado = calcular_async(10, 5).await;
    println!("calcular_async(10, 5) = {resultado}");

    // -----------------------------------------------------------------------
    // Executando tarefas concorrentes com tokio::join!
    // -----------------------------------------------------------------------
    println!("\n--- Tarefas concorrentes com tokio::join! ---");
    println!("Iniciando duas tarefas ao mesmo tempo...");
    let (r1, r2) = tokio::join!(
        tarefa_com_demora("Tarefa A", 100),
        tarefa_com_demora("Tarefa B", 50),
    );
    println!("Resultados: {r1}, {r2}");

    // -----------------------------------------------------------------------
    // Spawning de tasks independentes
    // -----------------------------------------------------------------------
    println!("\n--- tokio::spawn (tasks independentes) ---");
    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(10)).await;
        "resultado da task spawnada"
    });
    let valor = handle.await.unwrap();
    println!("Task spawnada retornou: {valor}");

    // -----------------------------------------------------------------------
    // Tratamento de erros em funções async
    // -----------------------------------------------------------------------
    println!("\n--- Tratamento de erros em async ---");
    match buscar_dado(true).await {
        Ok(dado)  => println!("dado recebido: {dado}"),
        Err(erro) => println!("erro esperado: {erro}"),
    }
    match buscar_dado(false).await {
        Ok(dado)  => println!("dado recebido: {dado}"),
        Err(erro) => println!("erro inesperado: {erro}"),
    }

    // -----------------------------------------------------------------------
    // Múltiplas tasks com tokio::spawn e JoinSet
    // -----------------------------------------------------------------------
    println!("\n--- Múltiplas tasks com JoinSet ---");
    let mut set = tokio::task::JoinSet::new();
    for i in 1..=4 {
        set.spawn(async move {
            sleep(Duration::from_millis(10)).await;
            i * i // retorna o quadrado de i
        });
    }

    let mut quadrados: Vec<u32> = Vec::new();
    while let Some(resultado) = set.join_next().await {
        quadrados.push(resultado.unwrap());
    }
    quadrados.sort();
    println!("Quadrados de 1 a 4: {quadrados:?}");

    println!();
}

// -----------------------------------------------------------------------
// Funções auxiliares assíncronas
// -----------------------------------------------------------------------

/// Simula a busca de uma mensagem (ex.: chamada de rede)
async fn buscar_mensagem() -> String {
    sleep(Duration::from_millis(10)).await; // simula latência
    "Olá do mundo assíncrono!".to_string()
}

/// Cálculo simples que aguarda um tempo antes de retornar
async fn calcular_async(a: i32, b: i32) -> i32 {
    sleep(Duration::from_millis(5)).await;
    a + b
}

/// Tarefa que leva `ms` milissegundos e retorna uma descrição
async fn tarefa_com_demora(nome: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("{nome} concluída em {ms}ms")
}

/// Simula uma operação que pode falhar
async fn buscar_dado(simular_erro: bool) -> Result<String, String> {
    sleep(Duration::from_millis(5)).await;
    if simular_erro {
        Err("falha simulada na busca de dados".to_string())
    } else {
        Ok("dado encontrado com sucesso!".to_string())
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[tokio::test]
    async fn test_buscar_mensagem() {
        let msg = buscar_mensagem().await;
        assert!(!msg.is_empty());
        assert!(msg.contains("assíncrono"));
    }

    #[tokio::test]
    async fn test_calcular_async() {
        assert_eq!(calcular_async(3, 4).await, 7);
        assert_eq!(calcular_async(0, 0).await, 0);
    }

    #[tokio::test]
    async fn test_tarefa_com_demora() {
        let r = tarefa_com_demora("Teste", 10).await;
        assert!(r.contains("Teste"));
        assert!(r.contains("10ms"));
    }

    #[tokio::test]
    async fn test_buscar_dado_erro() {
        assert!(buscar_dado(true).await.is_err());
        assert!(buscar_dado(false).await.is_ok());
    }

    #[tokio::test]
    async fn test_join() {
        let (r1, r2) = tokio::join!(
            calcular_async(1, 1),
            calcular_async(2, 2),
        );
        assert_eq!(r1, 2);
        assert_eq!(r2, 4);
    }
}
