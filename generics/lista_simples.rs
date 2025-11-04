use std::fmt::Debug;

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
    pub fn inserir(&mut self, dado: T) {
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
