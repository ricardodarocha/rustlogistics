use crate::visual::*;
use std::cmp::Ordering;

const ACESSO_NEGADO: i32 = 0;
const ACESSO_CONCEDIDO: i32 = 1;

/// Exibe uma mensagem de boas-vindas.
pub fn boas_vindas_ao_sistema(login: &str) {
    println!("\nBem-vindo ao sistema, {login}!\n");
}

/// Faz uma comparação *case-insensitive* entre duas strings.
fn confere(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

/// Realiza login de um usuário comum.
fn user(login: &str) -> i32 {
    let senha = input_senha("senha");

    if confere(login, "jacimar") && confere(&senha, "jacimar") {
        ACESSO_CONCEDIDO
    } else if confere(login, "wemerson") && confere(&senha, "wemerson") {
        ACESSO_CONCEDIDO
    } else if confere(login, "teste") && confere(&senha, "123") {
        ACESSO_CONCEDIDO
    } else if confere(login, "sistema") && confere(&senha, "123") {
        ACESSO_CONCEDIDO
    } else {
        ACESSO_NEGADO
    }
}

/// Cria um novo usuário admin (simulação).
fn admin(login: &mut String) -> i32 {
    let mut email;
    let mut senha;
    let mut repete_senha;

    *login = input_string_required("novo login");
    email = input_string_required("email");

    loop {
        senha = input_senha("senha");
        repete_senha = input_senha("repita a senha");

        if confere(&senha, &repete_senha) {
            break;
        } else {
            println!(" senha inválida, tente novamente");
        }
    }

    boas_vindas_ao_sistema(login);
    ACESSO_CONCEDIDO
}

/// Verifica permissões do usuário.
/// (Aqui seria feita a checagem de módulos ativos, permissões, etc.)
pub fn verificar_permissoes(_login: &str) {
    // Placeholder
}

/// Efetua o processo completo de login (usuário ou admin).
pub fn efetuar_login(login: &mut String) -> i32 {
    let mut tentativas = 3;

    *login = input_string_required("login");

    // Caso admin
    if confere(login, "admin") {
        loop {
            let senha = input_senha("senha");

            if confere(&senha, "admin") {
                return admin(login);
            } else {
                tentativas -= 1;
                println!("\nSenha inválida ({tentativas} tentativas restantes)");

                if tentativas == 0 {
                    return ACESSO_NEGADO;
                }
            }
        }
    } else {
        // Usuário comum
        loop {
            if user(login) == ACESSO_CONCEDIDO {
                boas_vindas_ao_sistema(login);
                return ACESSO_CONCEDIDO;
            } else {
                tentativas -= 1;
                if tentativas == 0 {
                    return ACESSO_NEGADO;
                }
                println!(" verifique usuário e senha");
                *login = input_string_required("login");
            }
        }
    }
}
