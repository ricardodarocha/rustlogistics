use std::io::{self, Write};

pub fn boas_vindas() {
    // Limpa a tela e move o cursor para o topo
    print!("\x1b[2J\x1b[H\n");

    println!("   ╔══════════════════════════════════════════════════════════════╗");
    println!("   ║                                                              ║");
    println!("   ║                   ██████╗  ██████╗ ██╗   ██╗                 ║");
    println!("   ║                   ██╔══██╗██╔═══██╗ ██║ ██╔╝                 ║");
    println!("   ║                   ██████╔╝██║   ██║  ████╔╝                  ║");
    println!("   ║                   ██╔══██╗██║   ██║ ██╔═██╗                  ║");
    println!("   ║                   ██████║╚═██████╔╝██║   ██╗                 ║");
    println!("   ║                    ╚════╝  ╚═════╝ ╚═╝   ╚═╝                 ║");
    println!("   ║                                                              ║");
    println!("   ║               BOX LOGISTICS SYSTEM  ▪  v0.1 (2025)           ║");
    println!("   ║               -------------------------------------          ║");
    println!("   ║               © 2025  Box Logistics - The System             ║");
    println!("   ║                                                              ║");
    println!("   ╚══════════════════════════════════════════════════════════════╝");
}

pub fn team() {
    println!("   ╔══════════════════════════════════════════════════════════════╗");
    println!("   ║                                                              ║");
    println!("   ║     ╔══════════════════════╗    ╔══════════════════════╗     ║");
    println!("   ║     ║  Nathan              ║    ║  Ricardo             ║     ║");
    println!("   ║     ║  Product Owner       ║    ║  Tech Lead           ║     ║");
    println!("   ║     ║  Chefe Comercial     ║    ║  Dev Senior          ║     ║");
    println!("   ║     ╚══════════════════════╝    ╚══════════════════════╝     ║");
    println!("   ║                                                              ║");
    println!("   ╚══════════════════════════════════════════════════════════════╝");
    println!();
}


fn main() {
    enable_ansi();
    conectar();

    loop {
        clear_screen();
        boas_vindas();

        let login = input("Login: ");
        let acesso = efetuar_login(&login);

        if !acesso {
            println!("Usuário ou senha inválidos.");
            std::process::exit(0);
        }

        let mut acesso_ativo = acesso;
        while acesso_ativo {
            verificar_permissoes(&login);
            boas_vindas();
            acesso_ativo = executar_menu_principal();
        }

        print_kpis();

        if !input_logico("Deseja entrar com outro usuário? (s/n): ") {
            break;
        }
    }

    boas_vindas();
    team();
    println!("Nosso time agradece a confiança!");

    desconectar();
    pause_terminal();
}

/* ===== Funções auxiliares ===== */

fn enable_ansi() {
    // No Windows 10+, geralmente não precisa. Placeholder:
    println!("\x1b[?25h"); // mostra o cursor ANSI
}

fn conectar() {
    println!("Conectando ao banco de dados...");
}

fn desconectar() {
    println!("Desconectando...");
}


fn efetuar_login(login: &str) -> bool {
    // Simula login
    login == "admin"
}

fn verificar_permissoes(login: &str) {
    println!("Verificando permissões de {login}...");
}

fn executar_menu_principal() -> bool {
    println!("Menu Principal:\n1. Opção A\n2. Sair");
    let opcao = input("Escolha uma opção: ");
    match opcao.trim() {
        "1" => {
            println!("Executando opção A...");
            true
        }
        "2" => false,
        _ => {
            println!("Opção inválida!");
            true
        }
    }
}

fn print_kpis() {
    println!("Mostrando KPIs...");
}


fn input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn input_logico(prompt: &str) -> bool {
    let resposta = input(prompt);
    matches!(resposta.trim().to_lowercase().as_str(), "s" | "sim" | "y" | "yes")
}

fn pause_terminal() {
    println!("Pressione ENTER para continuar...");
    let mut _tmp = String::new();
    io::stdin().read_line(&mut _tmp).unwrap();
}

fn clear_screen() {
    print!("\x1b[2J\x1b[H"); // limpa e move o cursor para o topo
    io::stdout().flush().unwrap();
}
