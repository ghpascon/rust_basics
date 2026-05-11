/// # Funções em Rust
///
/// Funções são blocos de código reutilizáveis. Em Rust:
/// - Declaradas com a palavra-chave `fn`
/// - Parâmetros têm tipos obrigatórios
/// - O tipo de retorno é declarado após `->`
/// - A última expressão (sem `;`) é o valor retornado automaticamente

pub fn demonstrar_funcoes() {
    println!("=== Funções ===\n");

    // Chamada de função simples
    saudar("Mundo");

    // Função com retorno
    let resultado = somar(5, 3);
    println!("somar(5, 3) = {resultado}");

    // Função com múltiplos parâmetros e retorno
    let area = calcular_area_retangulo(4.0, 7.5);
    println!("area do retângulo 4.0 × 7.5 = {area:.2}");

    // Funções que retornam Option
    println!("\n--- Funções que podem falhar (Option) ---");
    match dividir(10.0, 2.0) {
        Some(v) => println!("10 / 2 = {v}"),
        None    => println!("divisão por zero!"),
    }
    match dividir(10.0, 0.0) {
        Some(v) => println!("10 / 0 = {v}"),
        None    => println!("10 / 0 → divisão por zero detectada!"),
    }

    // Funções que retornam Result
    println!("\n--- Funções que retornam erros (Result) ---");
    match parsear_numero("42") {
        Ok(n)  => println!("parsear_numero(\"42\") = {n}"),
        Err(e) => println!("erro: {e}"),
    }
    match parsear_numero("abc") {
        Ok(n)  => println!("parsear_numero(\"abc\") = {n}"),
        Err(e) => println!("parsear_numero(\"abc\") → erro: {e}"),
    }

    // Closures — funções anônimas
    println!("\n--- Closures (funções anônimas) ---");
    let dobrar = |x: i32| x * 2;
    println!("dobrar(7) = {}", dobrar(7));

    // Closure capturando variável do escopo externo
    let taxa = 1.5;
    let aplicar_taxa = |preco: f64| preco * taxa;
    println!("aplicar_taxa(100.0) = {}", aplicar_taxa(100.0));

    // Passando função como argumento (higher-order functions)
    println!("\n--- Funções de ordem superior ---");
    let numeros = vec![1, 2, 3, 4, 5];
    let soma = reduzir_soma(&numeros);
    println!("soma de {numeros:?} = {soma}");

    // Recursão
    println!("\n--- Recursão ---");
    let n = 6;
    println!("fatorial({n}) = {}", fatorial(n));

    println!();
}

// Função sem retorno (retorna `()` implicitamente)
fn saudar(nome: &str) {
    println!("Olá, {nome}!");
}

// Função com parâmetros e retorno explícito
// A última expressão sem `;` é o valor retornado
fn somar(a: i32, b: i32) -> i32 {
    a + b
}

// Parâmetros f64 e retorno f64
fn calcular_area_retangulo(largura: f64, altura: f64) -> f64 {
    largura * altura
}

// Retorno Option<f64>: Some(valor) ou None em caso de erro
fn dividir(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

// Retorno Result<T, E>: Ok(valor) ou Err(mensagem)
fn parsear_numero(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| format!("'{s}' não é um número válido: {e}"))
}

// Função que recebe uma slice e retorna a soma
fn reduzir_soma(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

// Função recursiva para calcular fatorial
fn fatorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * fatorial(n - 1)
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn test_somar() {
        assert_eq!(somar(2, 3), 5);
        assert_eq!(somar(-1, 1), 0);
    }

    #[test]
    fn test_area() {
        assert_eq!(calcular_area_retangulo(3.0, 4.0), 12.0);
    }

    #[test]
    fn test_dividir() {
        assert_eq!(dividir(10.0, 2.0), Some(5.0));
        assert!(dividir(5.0, 0.0).is_none());
    }

    #[test]
    fn test_parsear_numero() {
        assert_eq!(parsear_numero("99"), Ok(99));
        assert!(parsear_numero("xyz").is_err());
    }

    #[test]
    fn test_fatorial() {
        assert_eq!(fatorial(0), 1);
        assert_eq!(fatorial(1), 1);
        assert_eq!(fatorial(5), 120);
        assert_eq!(fatorial(6), 720);
    }

    #[test]
    fn test_reduzir_soma() {
        assert_eq!(reduzir_soma(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(reduzir_soma(&[]), 0);
    }
}
