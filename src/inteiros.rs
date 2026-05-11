/// # Inteiros em Rust
///
/// Rust possui vários tipos numéricos inteiros. Os principais são:
///
/// | Tipo   | Tamanho | Sinal   | Faixa                                         |
/// |--------|---------|---------|-----------------------------------------------|
/// | i8     | 8 bits  | com     | -128 a 127                                    |
/// | i16    | 16 bits | com     | -32.768 a 32.767                              |
/// | i32    | 32 bits | com     | -2.147.483.648 a 2.147.483.647                |
/// | i64    | 64 bits | com     | ±9,2 × 10¹⁸                                  |
/// | u8     | 8 bits  | sem     | 0 a 255                                       |
/// | u16    | 16 bits | sem     | 0 a 65.535                                    |
/// | u32    | 32 bits | sem     | 0 a 4.294.967.295                             |
/// | u64    | 64 bits | sem     | 0 a 18.446.744.073.709.551.615                |
/// | isize  | *       | com     | depende da arquitetura (32 ou 64 bits)        |
/// | usize  | *       | sem     | depende da arquitetura (usado em índices)     |

pub fn demonstrar_inteiros() {
    println!("=== Inteiros ===\n");

    // Declaração básica: o tipo i32 é o padrão quando não especificamos
    let a: i32 = 42;
    let b: i32 = -10;
    println!("a = {a}  (i32, com sinal)");
    println!("b = {b}  (i32, com sinal, valor negativo)");

    // Inteiro sem sinal (só positivos)
    let c: u32 = 200;
    println!("c = {c}  (u32, sem sinal — só aceita valores >= 0)");

    // Inteiro de 64 bits
    let populacao_mundial: i64 = 8_000_000_000; // underscore melhora a leitura
    println!("populacao_mundial = {populacao_mundial}  (i64)");

    // usize é usado para tamanhos e índices
    let indice: usize = 3;
    println!("indice = {indice}  (usize)");

    // Operações aritméticas básicas
    println!("\n--- Operações aritméticas ---");
    let soma = a + 8;          // adição
    let subtracao = a - b;     // subtração
    let multiplicacao = a * 2; // multiplicação
    let divisao = a / 5;       // divisão inteira (descarta o resto)
    let resto = a % 5;         // módulo (resto da divisão)

    println!("a + 8       = {soma}");
    println!("a - b       = {subtracao}");
    println!("a * 2       = {multiplicacao}");
    println!("a / 5       = {divisao}  (divisão inteira)");
    println!("a % 5       = {resto}   (resto da divisão)");

    // Conversão entre tipos com 'as'
    println!("\n--- Conversão de tipos (casting) ---");
    let x: i32 = 1000;
    let y: u8 = x as u8; // trunca: 1000 % 256 = 232
    println!("i32 {x} convertido para u8 = {y}  (truncamento!)");

    let f: f64 = 9.9;
    let inteiro: i32 = f as i32; // trunca a parte decimal
    println!("f64 {f} convertido para i32 = {inteiro}  (parte decimal descartada)");

    // Verificação de overflow em modo debug
    // Em modo release o Rust faz wrap-around; em debug entra em panic.
    // Para operações seguras existe checked_add, saturating_add, wrapping_add etc.
    println!("\n--- Operações seguras contra overflow ---");
    let max_i8: i8 = i8::MAX; // 127
    println!("i8::MAX = {max_i8}");

    // checked_add retorna None em caso de overflow
    match max_i8.checked_add(1) {
        Some(v) => println!("127 + 1 = {v}"),
        None => println!("127 + 1 causaria overflow em i8! (checked_add retornou None)"),
    }

    // saturating_add limita ao valor máximo/mínimo do tipo
    let saturado = max_i8.saturating_add(10);
    println!("127.saturating_add(10) = {saturado}  (permanece em 127)");

    println!();
}

#[cfg(test)]
mod testes {
    #[test]
    fn operacoes_basicas() {
        let a: i32 = 42;
        assert_eq!(a + 8, 50);
        assert_eq!(a - 2, 40);
        assert_eq!(a * 2, 84);
        assert_eq!(a / 5, 8);
        assert_eq!(a % 5, 2);
    }

    #[test]
    fn overflow_seguro() {
        let max: i8 = i8::MAX;
        assert!(max.checked_add(1).is_none());
        assert_eq!(max.saturating_add(100), i8::MAX);
    }

    #[test]
    fn conversao_de_tipos() {
        let f: f64 = 3.9;
        let i: i32 = f as i32;
        assert_eq!(i, 3); // parte decimal é descartada
    }
}
