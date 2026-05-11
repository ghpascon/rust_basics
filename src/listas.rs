/// # Listas em Rust
///
/// Em Rust, a estrutura de lista mais usada é o `Vec<T>` — um vetor dinâmico
/// que cresce conforme novos elementos são inseridos. Além do Vec, há arrays
/// de tamanho fixo ([T; N]) e slices (&[T]).

pub fn demonstrar_listas() {
    println!("=== Listas (Vec, Arrays e Slices) ===\n");

    // -----------------------------------------------------------------------
    // Vec<T> — vetor dinâmico (tamanho definido em tempo de execução)
    // -----------------------------------------------------------------------
    println!("--- Vec<T> ---");

    // Criando um Vec vazio e depois inserindo elementos
    let mut numeros: Vec<i32> = Vec::new();
    numeros.push(10);
    numeros.push(20);
    numeros.push(30);
    println!("numeros = {numeros:?}");

    // Forma abreviada com a macro vec!
    let frutas = vec!["maçã", "banana", "laranja", "uva"];
    println!("frutas  = {frutas:?}");

    // Acessando elementos por índice (começa em 0)
    println!("frutas[0] = {}", frutas[0]);
    println!("frutas[2] = {}", frutas[2]);

    // Acesso seguro com .get() — retorna Option<&T>
    match frutas.get(10) {
        Some(f) => println!("fruta no índice 10: {f}"),
        None    => println!("índice 10 não existe (get retornou None)"),
    }

    // Tamanho e capacidade
    println!("len = {}  (elementos presentes)", numeros.len());
    println!("is_empty = {}", numeros.is_empty());

    // Removendo o último elemento (retorna Option<T>)
    let removido = numeros.pop();
    println!("pop() removeu: {removido:?}  — numeros agora = {numeros:?}");

    // Iterando com for
    println!("\n--- Iteração sobre Vec ---");
    let precos = vec![9.90, 14.50, 3.75, 22.00];
    let mut total = 0.0_f64;
    for preco in &precos {
        total += preco;
    }
    println!("precos = {precos:?}");
    println!("total  = {total:.2}");

    // Iterando com índice usando enumerate()
    println!("\n--- Enumerate (índice + valor) ---");
    for (i, fruta) in frutas.iter().enumerate() {
        println!("  [{i}] {fruta}");
    }

    // Transformando com map e coletando em novo Vec
    println!("\n--- Map + Collect ---");
    let dobros: Vec<i32> = vec![1, 2, 3, 4, 5]
        .iter()
        .map(|x| x * 2)
        .collect();
    println!("dobros = {dobros:?}");

    // Filtrando com filter
    let pares: Vec<i32> = vec![1, 2, 3, 4, 5, 6]
        .into_iter()
        .filter(|x| x % 2 == 0)
        .collect();
    println!("pares  = {pares:?}");

    // Ordenação
    println!("\n--- Ordenação ---");
    let mut nomes = vec!["Zara", "Ana", "Carlos", "Beatriz"];
    nomes.sort();
    println!("ordenado (crescente): {nomes:?}");
    nomes.sort_by(|a, b| b.cmp(a)); // ordem reversa
    println!("ordenado (decrescente): {nomes:?}");

    // -----------------------------------------------------------------------
    // Array — tamanho fixo, definido em tempo de compilação
    // -----------------------------------------------------------------------
    println!("\n--- Array de tamanho fixo [T; N] ---");
    let dias_semana: [&str; 7] = ["Dom", "Seg", "Ter", "Qua", "Qui", "Sex", "Sáb"];
    println!("dias_semana = {dias_semana:?}");
    println!("dias_semana.len() = {}", dias_semana.len());

    // Array inicializado com valor padrão
    let zeros: [i32; 5] = [0; 5];
    println!("zeros = {zeros:?}");

    // -----------------------------------------------------------------------
    // Slice — visão (referência) de parte de um array ou Vec
    // -----------------------------------------------------------------------
    println!("\n--- Slice (&[T]) ---");
    let todos = vec![100, 200, 300, 400, 500];
    let fatia: &[i32] = &todos[1..4]; // índices 1, 2, 3
    println!("todos = {todos:?}");
    println!("fatia (&todos[1..4]) = {fatia:?}");

    println!();
}

#[cfg(test)]
mod testes {
    #[test]
    fn push_e_pop() {
        let mut v: Vec<i32> = Vec::new();
        v.push(1);
        v.push(2);
        assert_eq!(v.len(), 2);
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn get_seguro() {
        let v = vec![10, 20, 30];
        assert_eq!(v.get(1), Some(&20));
        assert!(v.get(99).is_none());
    }

    #[test]
    fn map_e_filter() {
        let dobros: Vec<i32> = vec![1, 2, 3].iter().map(|x| x * 2).collect();
        assert_eq!(dobros, vec![2, 4, 6]);

        let pares: Vec<i32> = vec![1, 2, 3, 4].into_iter().filter(|x| x % 2 == 0).collect();
        assert_eq!(pares, vec![2, 4]);
    }

    #[test]
    fn slice() {
        let v = vec![10, 20, 30, 40];
        let s: &[i32] = &v[1..3];
        assert_eq!(s, &[20, 30]);
    }
}
