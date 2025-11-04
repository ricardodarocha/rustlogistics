use std::sync::atomic::{AtomicUsize, Ordering};
use std::fmt;

// ==========================
// Estrutura principal Produto
// ==========================
#[derive(Clone)]
pub struct Produto {
    pub id: usize,
    pub nome: String,
    pub valor: f32,
}

// ==========================
// Gerador de ID global
// ==========================
static PROXIMO_PRODUTO: AtomicUsize = AtomicUsize::new(0);

pub fn cursor_proximo_produto() -> usize {
    PROXIMO_PRODUTO.load(Ordering::SeqCst)
}

// ==========================
// Criação e validação
// ==========================
impl Produto {
    pub fn novo() -> Self {
        let id = PROXIMO_PRODUTO.fetch_add(1, Ordering::SeqCst) + 1;
        Produto {
            id,
            nome: String::new(),
            valor: 0.0,
        }
    }
}

pub fn validar_produto(_produto: &Produto) -> bool {
    // Pode ser personalizada depois
    true
}

// ==========================
// Impressão formatada
// ==========================
pub fn imprimir_cabecalho_produto() {
    println!(
        "  {:>3} {:<49} {:>9}\n{}",
        "id",
        "nome",
        "valor",
        " ═══════════════════════════════════════════════════════════════════"
    );
}

pub fn imprimir_produto(produto: &Produto) {
    println!(
        "  {:03} {:<49} {:>9.2}",
        produto.id,
        produto.nome,
        produto.valor
    );
}

// ==========================
// Busca e comparação
// ==========================
pub fn localizar_produto(a: &Produto, b: &Produto) -> bool {
    a.id == b.id
}
