/// # DashMap em Rust
///
/// `DashMap` é um mapa (chave→valor) **concorrente e seguro para múltiplas threads**,
/// semelhante a um `HashMap`, mas projetado para ser acessado e modificado
/// simultaneamente por várias threads sem necessidade de um `Mutex` ou `RwLock` externo.
///
/// Crate: https://crates.io/crates/dashmap
/// Adicione ao Cargo.toml:  dashmap = "6"

use dashmap::DashMap;

pub fn demonstrar_dashmap() {
    println!("=== DashMap ===\n");

    // -----------------------------------------------------------------------
    // Criação e inserção básica
    // -----------------------------------------------------------------------
    println!("--- Criação e inserção ---");
    let estoque: DashMap<String, u32> = DashMap::new();

    estoque.insert("maçã".to_string(), 50);
    estoque.insert("banana".to_string(), 30);
    estoque.insert("laranja".to_string(), 20);

    println!("Estoque após inserções: {:?}", mostrar_mapa(&estoque));

    // -----------------------------------------------------------------------
    // Leitura de valores
    // -----------------------------------------------------------------------
    println!("\n--- Leitura de valores ---");

    // .get() retorna uma referência imutável (Option<Ref<K, V>>)
    if let Some(qtd) = estoque.get("banana") {
        println!("banana em estoque: {}", *qtd);
    }

    // Chave inexistente
    match estoque.get("manga") {
        Some(v) => println!("manga: {}", *v),
        None    => println!("'manga' não está no estoque"),
    }

    // -----------------------------------------------------------------------
    // Atualização de valores
    // -----------------------------------------------------------------------
    println!("\n--- Atualização ---");

    // .get_mut() retorna referência mutável
    if let Some(mut qtd) = estoque.get_mut("maçã") {
        *qtd += 10;
    }
    println!("maçã após incremento: {:?}", estoque.get("maçã").map(|v| *v));

    // .entry() para inserir somente se ausente (padrão or_insert)
    estoque.entry("manga".to_string()).or_insert(15);
    estoque.entry("banana".to_string()).or_insert(999); // banana já existe, não altera
    println!("manga (inserida via entry): {:?}", estoque.get("manga").map(|v| *v));
    println!("banana (entry não alterou): {:?}", estoque.get("banana").map(|v| *v));

    // -----------------------------------------------------------------------
    // Remoção
    // -----------------------------------------------------------------------
    println!("\n--- Remoção ---");
    let removida = estoque.remove("laranja");
    println!("removida: {removida:?}");
    println!("'laranja' ainda existe? {}", estoque.contains_key("laranja"));

    // -----------------------------------------------------------------------
    // Iteração
    // -----------------------------------------------------------------------
    println!("\n--- Iteração ---");
    println!("Estoque atual:");
    let mut itens: Vec<(String, u32)> = estoque.iter()
        .map(|entry| (entry.key().clone(), *entry.value()))
        .collect();
    itens.sort_by_key(|(k, _)| k.clone()); // ordena para saída determinística
    for (produto, qtd) in &itens {
        println!("  {produto}: {qtd}");
    }

    // -----------------------------------------------------------------------
    // Uso concorrente com threads
    // -----------------------------------------------------------------------
    println!("\n--- Uso concorrente com threads ---");
    use std::sync::Arc;
    use std::thread;

    let contador: Arc<DashMap<&str, u32>> = Arc::new(DashMap::new());
    contador.insert("votos", 0);

    let mut handles = vec![];
    for _ in 0..5 {
        let contador_clone = Arc::clone(&contador);
        let handle = thread::spawn(move || {
            // Cada thread incrementa 'votos' de forma segura
            if let Some(mut v) = contador_clone.get_mut("votos") {
                *v += 1;
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("votos após 5 threads: {:?}", contador.get("votos").map(|v| *v));

    println!();
}

// Função auxiliar para exibir o mapa de forma ordenada
fn mostrar_mapa(mapa: &DashMap<String, u32>) -> Vec<(String, u32)> {
    let mut itens: Vec<(String, u32)> = mapa.iter()
        .map(|e| (e.key().clone(), *e.value()))
        .collect();
    itens.sort_by_key(|(k, _)| k.clone());
    itens
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn inserir_e_ler() {
        let m: DashMap<&str, i32> = DashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        assert_eq!(*m.get("a").unwrap(), 1);
        assert_eq!(*m.get("b").unwrap(), 2);
        assert!(m.get("c").is_none());
    }

    #[test]
    fn atualizar() {
        let m: DashMap<&str, i32> = DashMap::new();
        m.insert("x", 10);
        *m.get_mut("x").unwrap() += 5;
        assert_eq!(*m.get("x").unwrap(), 15);
    }

    #[test]
    fn entry_or_insert() {
        let m: DashMap<&str, i32> = DashMap::new();
        m.insert("existe", 7);
        m.entry("existe").or_insert(99);
        m.entry("novo").or_insert(42);
        assert_eq!(*m.get("existe").unwrap(), 7);  // não sobrescrito
        assert_eq!(*m.get("novo").unwrap(), 42);   // inserido
    }

    #[test]
    fn remover() {
        let m: DashMap<&str, i32> = DashMap::new();
        m.insert("k", 1);
        let v = m.remove("k");
        assert_eq!(v, Some(("k", 1)));
        assert!(!m.contains_key("k"));
    }

    #[test]
    fn concorrencia() {
        use std::sync::Arc;
        use std::thread;

        let m: Arc<DashMap<&str, u32>> = Arc::new(DashMap::new());
        m.insert("n", 0);

        let mut handles = vec![];
        for _ in 0..10 {
            let mc = Arc::clone(&m);
            handles.push(thread::spawn(move || {
                *mc.get_mut("n").unwrap() += 1;
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(*m.get("n").unwrap(), 10);
    }
}
