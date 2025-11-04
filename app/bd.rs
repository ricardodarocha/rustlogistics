// banco_dados.rs

// Placeholder para o tipo ListaDupla (a ser definido depois)
#[derive(Debug, Default)]
pub struct ListaDupla;

/// Estrutura principal representando o "banco de dados" em memória
#[derive(Debug, Default)]
pub struct BancoDados {
    pub produtos: Option<ListaDupla>,
    pub entregas: Option<ListaDupla>,
    pub cargas: Option<ListaDupla>,
    pub vendas: Option<ListaDupla>,
}

/// Instância global única (equivalente ao `static BancoDados memoria;` do C)
use std::sync::{OnceLock, Mutex};

static BD_GLOBAL: OnceLock<Mutex<BancoDados>> = OnceLock::new();

/// Função que retorna um acesso à instância global de BancoDados
pub fn bd() -> &'static Mutex<BancoDados> {
    BD_GLOBAL.get_or_init(|| {
        let db = BancoDados {
            produtos: None,
            entregas: None,
            cargas: None,
            vendas: None,
        };
        Mutex::new(db)
    })
}

/// Inicializa o banco de dados (equivalente a `conectar()`)
pub fn conectar() {
    let _ = bd(); // Garante que a instância global foi inicializada
    println!("Banco de dados inicializado");
}

/// Finaliza o banco de dados (equivalente a `desconectar()`)
pub fn desconectar() {
    println!("\nBanco de dados finalizado");
}
