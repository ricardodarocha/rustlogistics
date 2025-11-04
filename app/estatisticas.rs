use std::fmt::Write; // para formatar strings dinamicamente

/// Imprime um card genérico de KPI
pub fn print_kpi_card(icon: &str, nome: &str, valor: &str, arrow: char) {
    println!("╔══════════════════════╗");
    println!("║  {} {:<16}  ", icon, nome);
    println!("║  Valor: {:<10} {}  ", valor, arrow);
    println!("╚══════════════════════╝\n");
}

/// Converte um valor numérico em string e imprime o card
pub fn print_kpis_value(icon: &str, nome: &str, valor: f32, arrow: char) {
    let mut buffer = String::new();
    write!(&mut buffer, "{:.2}", valor).unwrap();
    print_kpi_card(icon, nome, &buffer, arrow);
}

// ======== Simulação das funções de contagem ========
// (depois você pode ligar isso às suas funções reais de `cadastros.rs`)
fn qtd_produtos() -> f32 { 42.0 }
fn qtd_entregas() -> f32 { 18.0 }
fn qtd_cargas() -> f32 { 9.0 }
fn qtd_vendas() -> f32 { 27.0 }

// ======== Funções de KPI específicas ========
pub fn print_kpi_produtos() {
    print_kpis_value("☻", "produtos", qtd_produtos(), '↑');
}

pub fn print_kpi_entregas() {
    print_kpis_value("◙", "entregas", qtd_entregas(), '↑');
}

pub fn print_kpi_cargas() {
    print_kpis_value("♫", "cargas", qtd_cargas(), '↓');
}

pub fn print_kpi_vendas() {
    print_kpis_value("§", "vendas", qtd_vendas(), '↑');
}

/// Imprime todos os KPIs
pub fn print_kpis() {
    print_kpi_produtos();
    print_kpi_entregas();
    print_kpi_cargas();
    print_kpi_vendas();
}
