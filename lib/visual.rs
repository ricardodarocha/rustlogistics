use std::io::{self, Write};

#[cfg(windows)]
use windows_sys::Win32::System::Console::{
    GetConsoleMode, SetConsoleMode, GetStdHandle,
    STD_OUTPUT_HANDLE, ENABLE_VIRTUAL_TERMINAL_PROCESSING, ENABLE_PROCESSED_OUTPUT,
};
#[cfg(windows)]
use windows_sys::Win32::System::SystemServices::{CP_UTF8, SetConsoleCP, SetConsoleOutputCP};

/// Habilita suporte ANSI e UTF-8 no terminal do Windows.
/// Em Linux/macOS, não faz nada.
pub fn enable_ansi() {
    #[cfg(windows)]
    unsafe {
        let h_out = GetStdHandle(STD_OUTPUT_HANDLE);
        if h_out == 0 {
            return;
        }

        let mut dw_mode = 0;
        if GetConsoleMode(h_out, &mut dw_mode) == 0 {
            return;
        }

        let _ = SetConsoleMode(
            h_out,
            dw_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT,
        );

        SetConsoleOutputCP(CP_UTF8);
        SetConsoleCP(CP_UTF8);
    }
}

/// Exibe um menu de opções e retorna a opção escolhida (inteiro).
pub fn exibir_menu(operacoes: &str) -> i32 {
    println!("\n\n{operacoes}");
    print!(" Escolha uma opção: ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<i32>().unwrap_or(0)
}

/// Lê uma string simples com prompt.
pub fn input_string(prompt: &str) -> String {
    print!(" {prompt}: ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

/// Lê uma string obrigatória (não vazia).
pub fn input_string_required(title: &str) -> String {
    loop {
        print!(" {title}: ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let text = buffer.trim();

        if !text.is_empty() {
            return text.to_string();
        }
        println!(" campo requerido");
    }
}

/// Lê um valor de moeda (float).
pub fn input_moeda(title: &str) -> f32 {
    print!(" {title}: R$ ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<f32>().unwrap_or(0.0)
}

/// Lê um número inteiro.
pub fn input_inteiro(title: &str) -> i32 {
    print!(" {title}: ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<i32>().unwrap_or(0)
}

/// Lê um número float.
pub fn input_float(title: &str) -> f32 {
    print!(" {title}: ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<f32>().unwrap_or(0.0)
}

/// Pergunta "S/N" e retorna true ou false.
pub fn input_logico(title: &str) -> bool {
    print!(" {title}: (S/N) ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    matches!(buffer.trim().to_lowercase().as_str(), "s")
}

/// Lê senha com máscara (*)
pub fn input_senha(title: &str) -> String {
    // Para mascarar os caracteres (****), use `rpassword`:
    // let senha = rpassword::prompt_password(format!(" {title}: ")).unwrap();
    // senha.trim().to_string()

    print!(" {title}: ");
    io::stdout().flush().unwrap();

    let mut senha = String::new();
    io::stdin().read_line(&mut senha).unwrap();
    senha.trim().to_string()
}
