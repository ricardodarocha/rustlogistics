use crate::visual::*;
use crate::cadastros::*;

pub fn boas_vindas_simplificado(title: &str) {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║ {:<60}      ║", title);
    println!("║ {:<40} {:>25} ║", "BOX LOGISTICS", "Versão 0.1 (2025)");
    println!("╚═══════════════════════════════════════════════════════════════════╝");
}

const MENU_PRINCIPAL: &str = r#"
   (1) Cadastrar produtos     (4) Formulário de vendas
   (2) Cadastrar entregas     (5) Relatórios
   (3) Cadastrar cargas
 ═══════════════════════════════════════════════════════════════════
   (9) Configurações          (0) Sair
"#;

const MENU_CADASTRAR_PRODUTOS: &str = r#"
   (1) Inserir produto
   (2) Buscar produto
   (3) Listar produtos
 ═══════════════════════════════════════
   (9) Configurações
   (0) Voltar
"#;

const MENU_CADASTRAR_ENTREGAS: &str = r#"
   (1) Inserir entrega
   (2) Buscar entrega
   (3) Listar entregas
 ═══════════════════════════════════════
   (9) Configurações
   (0) Voltar
"#;

const MENU_CADASTRAR_CARGAS: &str = r#"
   (1) Inserir carga
   (2) Buscar carga
   (3) Listar cargas
 ═══════════════════════════════════════
   (9) Configurações
   (0) Voltar
"#;

const MENU_VENDAS: &str = r#"
   (1) Inserir venda
   (2) Buscar venda
   (3) Listar vendas
 ═══════════════════════════════════════
   (9) Configurações
   (0) Voltar
"#;

const MENU_RELATORIO: &str = r#"
   (1) Vendas do Dia
   (2) Estoque Atual
 ═══════════════════════════════════════
   (0) Voltar
"#;

/// Menu de cadastro de produtos
pub fn cadastrar_produtos() {
    let operacao = exibir_menu(MENU_CADASTRAR_PRODUTOS);
    match operacao {
        1 => inserir_produtos(),
        2 => {
            let id = input_inteiro("ID");
            buscar_produtos(id);
        }
        3 => {
            clear_screen();
            boas_vindas_simplificado("RELATÓRIO DE PRODUTOS");
            listar_produtos();
        }
        0 => return,
        _ => println!("Operação inválida"),
    }
}

/// Menu principal do sistema
pub fn executar_menu_principal(continuar: &mut bool) {
    boas_vindas_simplificado("MENU PRINCIPAL");
    let operacao = exibir_menu(MENU_PRINCIPAL);
    clear_screen();

    match operacao {
        1 => {
            boas_vindas_simplificado("CADASTRO DE PRODUTOS");
            if qtd_produtos() == 0 {
                inserir_produtos();
            }
            cadastrar_produtos();
        }
        2 => {
            boas_vindas_simplificado("CADASTRO DE ENTREGAS");
            // cadastrar_entregas();
        }
        3 => {
            boas_vindas_simplificado("CADASTRO DE CARGAS");
            // cadastrar_cargas();
        }
        4 => {
            boas_vindas_simplificado("VENDAS");
            // formulario_vendas();
        }
        0 => *continuar = false,
        _ => println!("Operação inválida"),
    }
}
