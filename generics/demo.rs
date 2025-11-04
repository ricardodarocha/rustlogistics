#[derive(Debug)]
struct Produto {
    nome: String,
    valor: f32,
}

fn main() {
    let mut lista = ListaSimples::nova();

    lista.inserir(Produto { nome: "Caixa".into(), valor: 10.0 });
    lista.inserir(Produto { nome: "Palete".into(), valor: 25.5 });
    lista.inserir(Produto { nome: "Etiqueta".into(), valor: 1.2 });

    println!("Lista atual: {:?}", lista);

    // Exemplo de busca
    let alvo = Produto { nome: "Palete".into(), valor: 0.0 };
    if let Some(p) = lista.pesquisar(&alvo, |a, b| a.nome == b.nome) {
        println!("Produto encontrado: {:?} (valor = {})", p.nome, p.valor);
    }

    // Imprimir todos
    println!("--- Produtos ---");
    lista.imprimir(|p| println!("{} - R${:.2}", p.nome, p.valor));

    // Limpar lista
    lista.limpar();
    println!("Lista após limpar: {:?}", lista);
}
