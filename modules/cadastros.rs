use crate::visual::*;
use crate::bd::*;
use crate::modules::{produtos, entregas, cargas, vendas};

/// ================= indicadores  ====================
pub fn qtd_produtos() -> usize {
    bd().produtos.len()
}

pub fn qtd_entregas() -> usize {
    bd().entregas.len()
}

pub fn qtd_cargas() -> usize {
    bd().cargas.len()
}

pub fn qtd_vendas() -> usize {
    bd().vendas.len()
}
/// ====================================================

/// Inserir novo produto
pub fn inserir_produtos() {
    let mut novo_produto = produtos::Produto::novo();

    println!("              ID: {:03}", novo_produto.id);
    novo_produto.nome = input_string("Nome do produto");
    novo_produto.valor = input_float("Valor");

    produtos::validar_produto(&novo_produto);
    bd_mut().produtos.push(novo_produto);

    println!("✅ Produto inserido com sucesso!");
    pausar();
}

/// Buscar produto por ID
pub fn buscar_produtos(id: usize) {
    if let Some(produto) = bd().produtos.iter().find(|p| p.id == id) {
        println!("\nEncontrado: {}", produto.nome);
    } else {
        println!("\n❌ Produto não encontrado.");
    }
    pausar();
}

/// Listar produtos
pub fn listar_produtos() {
    produtos::imprimir_cabecalho_produto();
    for produto in &bd().produtos {
        produtos::imprimir_produto(produto);
    }
    pausar();
}

/// Entregas
pub fn inserir_entregas() {
    entregas::inserir_entrega();
}

pub fn buscar_entregas() {
    entregas::buscar_entrega();
}

pub fn listar_entregas() {
    entregas::listar_entregas();
}

/// Cargas
pub fn inserir_cargas() {
    cargas::inserir_carga();
}

pub fn buscar_cargas() {
    cargas::buscar_carga();
}

pub fn listar_cargas() {
    cargas::listar_cargas();
}

/// Vendas
pub fn inserir_vendas() {
    vendas::inserir_venda();
}

pub fn buscar_vendas() {
    vendas::buscar_venda();
}

pub fn listar_vendas() {
    vendas::listar_vendas();
}

/// Relatórios (placeholder)
pub fn relatorios() {
    println!("🔍 Módulo de relatórios ainda não implementado.");
    pausar();
}
