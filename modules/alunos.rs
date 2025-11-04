use std::fmt::Debug;
#[macro_use]
pub mod portugol {
    #[macro_export]
    macro_rules! leia {
        ($var:ident : $t:ty) => {
            {
                use std::io::{self, Write};
                let mut input = String::new();
                println!("  informe o campo {}:{} ", stringify!($var), stringify!($t), );
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
                input = input.trim().to_string();
                print!("> {}\n", input);
                $var = input.parse::<$t>().expect("Falha na conversão do valor");
            }
        };
    }
}     


/// Nó da lista simplesmente encadeada
pub struct No<T> {
    pub dado: T,
    pub proximo: Option<Box<No<T>>>,
}

/// Lista simplesmente encadeada genérica
pub struct ListaSimples<T> {
    pub inicio: Option<Box<No<T>>>,
}

impl<T> ListaSimples<T> {
    /// Cria uma nova lista vazia
    pub fn nova() -> Self {
        Self { inicio: None }
    }

    /// Insere um novo elemento no final da lista
    pub fn push(&mut self, dado: T) {
        let novo = Box::new(No { dado, proximo: None });
        match self.inicio.as_mut() {
            Some(mut atual) => {
                while let Some(ref mut prox) = atual.proximo {
                    atual = prox;
                }
                atual.proximo = Some(novo);
            }
            None => {
                self.inicio = Some(novo);
            }
        }
    }

    /// Pesquisa um elemento usando uma função de comparação (cmp)
    /// Retorna referência ao dado se encontrar, ou None
    pub fn pesquisar<F>(&self, alvo: &T, cmp: F) -> Option<&T>
    where
        F: Fn(&T, &T) -> bool,
    {
        let mut atual = self.inicio.as_ref();
        while let Some(no) = atual {
            if cmp(&no.dado, alvo) {
                return Some(&no.dado);
            }
            atual = no.proximo.as_ref();
        }
        None
    }

    /// Imprime todos os elementos da lista (usa uma função callback)
    pub fn imprimir<F>(&self, imprimir: F)
    where
        F: Fn(&T),
    {
        let mut atual = self.inicio.as_ref();
        while let Some(no) = atual {
            imprimir(&no.dado);
            atual = no.proximo.as_ref();
        }
    }

    /// Limpa a lista — em Rust, basta liberar a estrutura
    pub fn limpar(&mut self) {
        self.inicio = None;
    }
}

/// Permite depuração simples da lista
impl<T: Debug> Debug for ListaSimples<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut atual = self.inicio.as_ref();
        write!(f, "ListaSimples [")?;
        while let Some(no) = atual {
            write!(f, "{:?}", no.dado)?;
            atual = no.proximo.as_ref();
            if atual.is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

pub mod aluno {

    pub struct Aluno{
        pub matricula: i32, 
        pub nome: String, 
        pub nota1: f32, 
        pub nota2: f32,
    }

    pub fn novo() -> Aluno {
    
     let (matricula, nome, nota1, nota2):(i32, String, f32, f32);
     leia!(matricula: i32);
     leia!(nome: String);
     leia!(nota1: f32);
     leia!(nota2: f32);
      
      Aluno {
          matricula: 1,
          nome: "Fulano".to_string(),
          nota1: 9.0,
          nota2: 8.5
      }
      
    }
    pub fn imprimir(aluno: &Aluno) {
          println!("{} {}", aluno.nome, (aluno.nota1 + aluno.nota2) / 2.0);
    }
    
    pub fn listar(alunos: &Vec<Aluno>) {
      for aluno in alunos {
          imprimir(&aluno)
      }  
    }
}

pub mod app {
    use crate::leia;
    
    pub enum Acao {
        Cadastrar,
        Listar,
        Sair,
    }

    pub fn menu() -> Acao {
        let opcao: i8;
        println!("
            (1) Cadastrar
            (2) Listar
            (3) Sair
        
        ");
        leia!(opcao: i8);
        
        match opcao {
            1 => Acao::Cadastrar,
            2 => Acao::Listar,
            3 => Acao::Sair,
            _ => menu(),
        }
    }
}

fn main() {
    // let mut alunos: Vec<aluno::Aluno> = vec![];
    let mut alu: ListaSimples<aluno::Aluno> = ListaSimples::<crate::aluno::Aluno>::nova();
    use app::Acao;
    
    loop {
        let acao = app::menu();
        match acao {
            Acao::Cadastrar => alu.push(aluno::novo()),
            Acao::Listar => alu.imprimir(aluno::imprimir),
            Acao::Sair => break,
        }
    }
}
