
/***************************************************************************************************
*                                                                                                  *
*                                        ARRAYS                                                    *
*                                                                                                  *
***************************************************************************************************/

/***************************************************************************************************
Exercício 1: Encontrar o Máximo
Enunciado: Escreva um programa em Rust que encontre o valor máximo em um array de inteiros.
***************************************************************************************************/

use std::{array, fmt::format};

pub fn encontraMaximo(){
	let int_array: [i32; 6] = [0, 9, 2, 3, 4, 5];
	let mut maior_num = int_array[0];
	for num in int_array.iter(){
		if num > &maior_num{
			maior_num = *num;
		}
	}
	println!("O maior número do array é {}", maior_num);
}

/***************************************************************************************************
Exercício 2: Reverter um Array
Enunciado: Escreva um programa em Rust que reverta um array de inteiros sem usar métodos prontos da
linguagem para isso.
***************************************************************************************************/

pub fn reverterArray(){
	let mut  int_array: [i32; 5] = [1, 2, 3, 4, 5];
	let array_len = int_array.len();
	let mut aux = 1;
	for num in 0..4{
		int_array.swap(num, array_len - aux);
		aux += 1;
	}
	println!("O array invertido é {:?}", int_array);
}

/***************************************************************************************************
Exercício 3: Calcular a Média
Enunciado: Escreva um programa em Rust que calcule a média dos valores armazenados em um
array de números de ponto flutuante.
***************************************************************************************************/

pub fn calcularMedia(){
	let float_array: [f32; 5] = [1.0, 2.0, 3.0, 3.8, 7.12];
	let soma: f32 = float_array.iter().sum();
	let res: f32 = soma / float_array.len() as f32;
	println!("A média dos valores do array é {:.2}", res)

}
/***************************************************************************************************
Exercício 4: Contar Elementos Negativos
Enunciado: Escreva um programa em Rust que conte quantos números negativos existem em um
array de inteiros.
***************************************************************************************************/

pub fn contarNumerosNegativos(){
	let int_array: [i32; 6] = [-21, 0, 45, 99, -9, -1];
	let mut contador: i32 = 0;
	for num in 0..int_array.len(){
		if int_array[num] < 0{
			contador += 1;
		}
	}
	println!("A quantidade de valores negativos é {}", contador);
}

/***************************************************************************************************
Exercício 5: Verificar Presença de Elemento
Enunciado: Escreva um programa em Rust que verifique se um determinado inteiro está
presente em um array.
***************************************************************************************************/

pub fn verificarPresencaElemento(){
	let int_array = [1, 2, 3, 4, 5];
	let target = 3;
	let exist = int_array.iter().position(|&r| r == target);
	if exist != None {
		println!("O elemento {} existe no array", target);
	} else{
		println!("O elemento {} não exite no array.", target);
	}
}

/***************************************************************************************************
*                                                                                                  *
*                                        VETORES                                                   *
*                                                                                                  *
***************************************************************************************************/

/***************************************************************************************************
Exercício 1: Adicionar Elementos a um Vetor
Enunciado: Escreva um programa em Rust que crie um vetor vazio e adicione a ele os
números de 1 a 5, um de cada vez, usando um loop.
***************************************************************************************************/

pub fn adicionarElemento(){
	let mut int_array: Vec<i32> = Vec::new();
	for num in 1..6{
		int_array.push(num);
	}
	println!("Array preenchido {:?}", int_array);
}

/***************************************************************************************************
Exercício 2: Remover Elemento Específico
Enunciado: Escreva um programa em Rust que remova o primeiro elemento de
valor 3 de um vetor de inteiros.
***************************************************************************************************/

pub fn removerElementoEspecifico(){
	let mut int_array: Vec<i32> = vec![1, 2, 3, 4, 5];
	let target = 3;
	int_array.remove(int_array.iter().position(|&r| r == target).unwrap());
	println!("Novo array após remoção {:?}", int_array);
}

/***************************************************************************************************
Exercício 3: Calcular a Soma de Elementos
Enunciado: Escreva um programa em Rust que calcule a soma de todos os elementos
em um vetor de números inteiros.
***************************************************************************************************/

pub fn calcularSomaElementos(){
	let int_array: Vec<i32> = vec![1, 2, 3, 4, 5];
	let sum: i32 = int_array.iter().sum();
	println!("A soma dos valores do vector é {}", sum);
}

/***************************************************************************************************
Exercício 4: Encontrar o Menor Elemento
Enunciado: Escreva um programa em Rust que encontre o menor elemento em um vetor de números
inteiros.
***************************************************************************************************/

pub fn encontrarMenorNumero(){
	let int_array = vec![0, 9, 2, 3, 4, 5, -99];
	let mut menor_num = int_array[0];
	for num in int_array.iter(){
		if num < &menor_num{
			menor_num = *num;
		}
	}
	println!("O menor número do array é {}", menor_num);

}
//Código Resposta

/***************************************************************************************************
Exercício 5: Filtrar Elementos Pares e Criar um Novo Vetor
Enunciado: Escreva um programa em Rust que, dado um vetor de números inteiros, crie
um novo vetor contendo apenas os elementos pares do vetor original.
***************************************************************************************************/

pub fn filtrarELementosPares(){
	let int_array = vec![0, 9, 2, 3, 4, 5, 99, 44];
	let even_vec: Vec<i32> = int_array.into_iter().filter(|num| num % 2 == 0).collect();
	println!("Valores pares do vetor de inteiros {:?}", even_vec)
}


/***************************************************************************************************
*                                                                                                  *
*                                        STRUCT                                                    *
*                                                                                                  *
***************************************************************************************************/

/***************************************************************************************************
Exercício 1: Definir e Instanciar uma Struct
Enunciado: Defina uma struct Carro que tenha três campos: marca, modelo, e ano.
Crie uma instância dessa struct e imprima seus valores no console.
***************************************************************************************************/

struct Carro{
	 marca: String,
		modelo: String,
		ano: i16
}

/***************************************************************************************************
Exercício 2: Adicionar Método à Struct
Enunciado: Utilizando a struct Carro do exercício anterior, adicione um método descricao que
retorna uma string formatada com todos os dados do carro. Chame este método para uma
instância de Carro e imprima o resultado.
***************************************************************************************************/

impl Carro{
	fn show_details(&self) -> String{
		format!("{} {} {}", self.marca, self.modelo, self.ano)
	}
}

pub fn carro(){
	let new_carro: Carro = Carro{
		marca: String::from("Fiat"),
		modelo: String::from("Uno"),
		ano: 2007
	};
	println!("{}", new_carro.show_details())
}


/***************************************************************************************************
Exercício 3: Struct com Enum
Enunciado: Crie uma struct Pedido que contém nome_do_item (String) e status (um enum StatusPedido
com variantes Pendente, Concluido). Adicione um método à struct Pedido que imprime uma
mensagem diferente dependendo do status do pedido.
***************************************************************************************************/

struct Pedido{
	nome_item: String,
	status: StatusPedido
}

enum StatusPedido {
	Pendente,
	Concluido
}

impl Pedido {
	fn verificar_pedido(&self){
		match self.status {
			StatusPedido::Pendente => println!("{} está pendente.", self.nome_item),
			StatusPedido::Concluido => println!("{} está concluído!", self.nome_item)
		}
	} 				
}

pub fn criar_pedido() {
	let meu_pedido = Pedido {
		nome_item: String::from("Livro Rust"),
		status: StatusPedido::Concluido,
	};

	meu_pedido.verificar_pedido();
}

/***************************************************************************************************
Exercício 4: Struct com Lifetimes
Enunciado: Defina uma struct Livro que tenha dois campos: titulo e autor, ambos
sendo referências a strings com um lifetime específico. Crie uma instância dessa struct e
uma função que aceita uma referência a Livro e imprime o título e o autor.
***************************************************************************************************/

struct Livro<'a>{
	titulo: &'a str,
	autor: &'a str
}

fn print_livro(livro: &Livro){
	println!("O autor {} escreveu o livro {}.", livro.autor, livro.titulo);
}

pub fn lifetimes(){
	let livro = Livro {
		titulo: "O Silmarillion",
		autor: "J.R.R Tolkien",
	};
	print_livro(&livro);
}

/***************************************************************************************************
Exercício 5: Struct com Vários Métodos
Enunciado: Crie uma struct Contador com um campo valor do tipo i32. Adicione métodos para
incrementar e decrementar o valor, além de um método que retorna o valor atual. Demonstre o uso
dessa struct incrementando, decrementando o valor e exibindo o valor atual.
***************************************************************************************************/

struct Contador{
	valor: i32
}

impl Contador {
	fn incrementar(&mut self){
		self.valor += 1
	}
	fn decrementar(&mut self){
		self.valor -= 1
	}
	fn mostrar_valor(&self){
		println!("O valor atual é {}", self.valor)
	}
}

pub fn contador(){
	let mut new_contador = Contador{
		valor: 0
	};
	new_contador.mostrar_valor();
	new_contador.incrementar();
	new_contador.mostrar_valor();
	new_contador.decrementar();
	new_contador.mostrar_valor();
}