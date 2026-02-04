# anotacoes

Este arquivo serve para fazer anotações sobre o aprendizado em rust.

---


## Como funciona?

Funciona da seguinte forma tudo vai ser escrito de uma maneira diferente ao invés de irmos digitando de cima para baixo como acontece normalmente em arquivos de texto vamos digitar de baixo para cima.

Usando o template abaixo:

---

## Titulo

Descricao 

---

## Debugging

Usando o vs code e sua funcionalidade de debugg, podemos executar nosso código uma linha por vez.

Ao invés de rodar o código todo de uma vez, podemos designiar algumas linhas que queremos que o código pare e analizar o comportamento do código.

Essas etapas que queremos parar o código se chamam breakpoints

Como demonstrado na aula 93 podemos percorrer o código trecho por trecho vendo todo o código em sua execução.

---

## Recursão em Rust

Uma das principais funcionalidades da programação é a recursão, basicamente quando uma função chama ela mesma, é uma forma de loop bem sofisticada e complexa.

---

## Loop

O rust tem sua forma de iterar as coisas e de como funciona os loops.

Aqui está o loop tradicional em rust:

```rust
loop {
    // Bloco de código que queira repetir
}
```

Para parar o loop usamos a palavra chave: 

```rust
break;
```

e se quiser fazer um bloco de código que faça alguma modificação e recomece o loop utilize o:

```rust
continue;
```


---

## Funções

As funções se mantém a mesma coisa das outras linguagens só tem alguns diferenciais:


```rust

fn teste(variavel: i32)-> i32 // Isso aqui quer dizer que vai retornar um inteiro de 32 bits

fn teste(variavel: i32)-> i32{
    variavel * 2
}

// Se colocar dessa maneira o rust vai entender que vai ter que retornar sem precisa
// necessáriamente colocar o return explicitamente.

// E tem como fazer uma função dentro de uma variável assim:

let multiplicador = 3;

let calculadora ={
    let value = 5 +4;

    value * multiplicador
};




/// valor de calculadora vai ser 27

```

Aí de resto se mantem tudo igual.

---


## Range type

Um range representa uma sequencia ou intervalo de valores consecutivos.

Por exemplo um range pode pegar todos os números entre 15 e 21, ou isso pode listar todos os caracteres entre "b" e "h"


exemplo:

```rust

fn main() {
    let month_days = 1..26;

    println!("{month_days:?}");

    let month_days = 1..=26;
    println!("{month_days:?}");

    for day in month_days {
        println!("{day}");
    }

    let letras = 'b'..'p';

    for letra in letras {
        println!("{letra}");
    }
}


```

E aqui aprendemos a primeira forma de iterar as coisas em Rust.

---

## Tuple Type

Introdução ao tipo Tuples é mais um tipo de collection como array, diferente do array que necessita que todos os dados dentro dele sejam do mesmo tipo, uma tupla não tem essa limitação então pode ser inserido dentro dela dados de diferentes tipos.



exemplo:

```rust
fn main() {
    let empregado = ("MOLLY", 32, "Marketing");

    // Forma mais simples para associar os dados de uma tupla para variaveis

    let (nome, idade, setor) = empregado;

    // let nome = empregado.0;
    // let idade = empregado.1;
    // let setor = empregado.2;

    println!("{} tem {} e trabalha no {}", nome, idade, setor);

    dbg!(empregado);
}
```

---



## Macro em Rust.

O que são os Macros?

Macro em Rust é código que escreve código.

Por que Rust tem macros?

Rust é bem rigoroso com tipos e não tem coisas como:

- funções variádicas (printf(...))

- reflexão em runtime

As macros resolvem isso:

- evitam repetição de código

- geram código eficiente

- permitem sintaxe mais flexível que funções normais


Macro

```rust
println!("Olá {}", nome);
```

- Executa em tempo de compilação

- Recebe tokens de código, não valores

- Expande para código Rust válido

---


## dbg! Macro

O dbg! é semelhante ao println! ele funciona como uma função mas não é uma função é uma maneira auxiliar.

Mesmo existindo o debug trait ele é como se fosse um acessório, se quiser realmente algo pra poder debuggar é o dbg! que temos que utilizar.

```rust
fn main() {
    let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let marcas: [&str; 3] = ["apple", "Samsung", "Motorola"];

    println!("{}", marcas.len());

    println!("{:?}", marcas);
    // OU
    println!("{numbers:?}");
    println!("{numbers:#?}");

    dbg!(2 + 2);
    dbg!(numbers);
}

```

Ele é mais fácil de utilzar do que puxar o println! e tudo mais.

---


## Debug Trait

Diferente da display trait o debug serve para nós programadores usarmos.

Ele é comumente usado nos arrays para melhorar a visão do que ele possue dentro dele.

Tem as formas de como fazer isso em código:

```rust
fn main() {
    let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let marcas: [&str; 3] = ["apple", "Samsung", "Motorola"];

    println!("{}", marcas.len());

    println!("{:?}", marcas);
    // OU
    println!("{marcas:?}");

    // Tem como deixar de forma bonita:

    println!("{marcas:#?}");

}

```

---

## The Display Traits

Uma trait em Rust é como um contrato.

Ela define quais métodos um tipo deve ter para dizer:
    “Eu sei fazer isso”

Se um tipo implementa uma trait, ele promete que possui aqueles métodos.

👉 O como o método funciona pode mudar
👉 Mas o nome e a assinatura do método são os mesmos

🧾 Analogia simples

Trait = contrato
Tipo = pessoa/objeto que assina o contrato

Exemplo do mundo real:

“Você promete chegar às 9h”

Estudante → chega na aula

Funcionário → chega no trabalho

Avião → chega no aeroporto

Todos cumprem o mesmo contrato, mas de formas diferentes.


trait Display

A trait Display diz:

“Este tipo pode ser mostrado como texto legível para humanos”

Ela é usada quando você faz:

```rust
println!("Valor: {}", valor);
```

O {} só funciona se o tipo implementar Display.

✅ Tipos que implementam Display

- i32

- f64

- bool

- String

- &str

Exemplo:

```rust
fn main() {
    let idade = 25;
    let altura = 1.75;
    let ativo = true;

    println!("Idade: {}", idade);
    println!("Altura: {}", altura);
    println!("Ativo: {}", ativo);
}

```

---

## Array, o primeiro scalar/compound type

Array é um tipo escalar ou seja podemos colocar inúmeros dados dentro dele do mesmo tipo que ele funcionara tranquilamente.

exemplo em codigo:

```rust
fn main() {
    let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let marcas: [&str; 3] = ["apple", "Samsung", "Motorola"];

    println!("{}", marcas.len());

    for number in numbers {
        println!("{}", number);
    }
}
```


Arrays tbm possuem métodos próprios.


---


## Caracteres

O tipo caractere em rust  é  representado de maneira simples:

```rust
let exe = 'v';
```

Só pode ser usado uma letra para este tipo.


alguns métodos de uso:

```rust
fn main() {
    let first_initial = 'b';
    let emoji = '🙂';

    println!(
        "{}, {}",
        first_initial.is_alphabetic(),
        emoji.is_alphabetic()
    );

    println!("{}, {}", first_initial.is_uppercase(), emoji.is_lowercase());

    println!("{}, {}", first_initial.is_uppercase(), emoji.is_lowercase());
}

```

---

## Igual ou diferente, && e ||

Operadores que dizem se algo é igual ou diferente de alguma coisa.

exemplo:

```rust
fn main() {
    println!("{}", "coke" == "pepsi"); // falsse
    println!("{}", "coke" != "coke"); // falsse
    println!("{}", "coke" == "coke"); // true
}


```

Aqueles simbolos de AND (&&) e de OR (||) 


---

## Booleans

Valores booleanos em Rust

```rust
let bonito:bool = true;
let bobo:bool = false;

println!("{bonito}"); // Saida true
println!("{bobo}"); // Saida false


let idade = 17;
let pode_dirigir = idade < 18;

println!("{pode_dirigir}"); // Saida true.

```

### Inversão de Booleans

Conseguimos alterar o valor de um boolean apenas usando o valor !

Assim invertemos os seu valores:


```rust
println!("{}", !true); // Saida false
println!("{}", !false); // Saida true
```

---

## Augmented Assigment Operator

Operações matemáticas mais comuns ou mais simples é assim:

Uma forma de fazermos uma variável mais um em linguagens comuns é:

```C++

int i;
i+=1;
i++;

```
Uma forma mais simples que i = i + 1;

Em rust temos isso também mas apenas isso:


```rust
let mut i = 34;

i = i + 1;

i += 1;

println!({i});

i -= 1;
println!({i});

i *= 2;
println!({i});

year /= 4;
println!({i});



```

---


## Operações matematicas

Operações matemáticas assim como em toda linguagem é os principais:

```rust
let adicao: i32 = 5+9;
let subtracao: i32 = 5-9;
let multiplicacao: i32 = 5*9;

let floor_divisao = 5/3;
println!("{floor_divisao}"); // isso vai printar 1 pois podemos dividir 5 por 3 apenas uma vez

let floor_float = 5.00/3.00;
println!("{floor_float}"); // vai retornar o calculo completo 1.6666

let resto = 8%2;
let resto2 = 9%2;

println!("{resto}"); // vai retornar o resto da divisão de 8 por 2 que seria 0
println!("{resto2}"); // vai retornar o resto da divisão de 9 por 2 que seria 1

```

---

## Título do update

Descrição do que é existe. Neste momento vai ser feito assim...

---

## Casting types

Cast é quando convertemos um tipo de dado para outro no exemplo:


```rust
fn main() {
    let miles_away = 50;
    let miles_away_i8 = miles_away as i8;
    let miles_away_u8 = miles_away as u8;

    let miles_float: f64 = 100.34234;
    let miles_f32 = miles_away as f32;
    let miles_int = miles_away as i32;

    println!("{miles_int}");
}

```

O que fizemos aqui? Se o código for printar o miles_int ele vai trazer como 100 ao invés de ser o número grande que está ligado.

---

## Floating Point types

Algumas manipulções que podemos fazer com os floats


Rust tem o f64 que nos dá uns 15 digitos de precisão para um numero enquanto o 32 vai para apenas 6 ou 7 digitos de precisão.


E conseguimos formatar esses valores para quantos digitos depois da virgula precisamos:


exemplo:
```rust
fn main() {
    let value = 3.1458955885;

    println!("{}", value);
    println!("{value:.2}");
    println!("{0:.4}", value);
    println!("{0:.4}", value);
}
```


---

## Introdução a métodos

Métodos são basicamente funções que ficam dentro de classes, em rust os valores que colocamos como integer também possuem métodos.

Ex:

```rust
fn main() {
    let value: i32 = -15;

    println!("{}", value.abs());
}
```

Neste exemplo o que fizemos? Adicionamos o valor -15 ao value e ao printarmos nós colocamos o .abs() isso é um método do tipo int que abs é o valor absoluto de um numero ou seja, basicamente falando a distancia que esse número tem de zero.

Ao rodar o programa ele vai printar como 15.


Bem tranquilo por enquanto.

---


## Datatypes e seus diferenciais - Strings e Raw Strings

Algumas strings tem seu comportamento diferente.

Temos string que sabemos os seu valores no tempo de compilação:

```rust
println!("HELLO");
```

### valores especiais em strings:

Para dar enter é só colocar o "\n" dentro da string no Rust:

```rust
println!("HELLO \n World");
```

Para dar um tab é só colocar o "\t" dentro da string no Rust:

```rust
println!("HELLO \t World");
```

Para escapar algum caracter é só colocar um "\" dentro.

### Raw string

É basicamente as string que tem ' assim como em outras linguagens esse ' vai ignorar todo e qualquer caractere que seja especial.

Mas em rust fica assim:

```rust
let filepath = r"Hello \n world";
```

O compilador vai interpretar isso literalmente e vai ignorar o \n.


Essas string sabemos no tempo de compilação pois está chumbado no código mas tem strings que não o valor essas são strings que o usuário vai inserir no código quando ele estiver rodando.



---


## Datatypes e seus diferenciais - Inteiros e Floats

Rust tem uma variedade de tipos de dados.

## Scalar types

São tipos que carregam apenas um valor, sendo eles:

- Integers
- floating-points numbers
- Booleans
- Characters

## Container types

Ainda não foi mencionado no curso mas são tipos de dados que carregam mais de um tipo de dado como um "container type".

### Inteiros no Rust

Inteiros nos rust funciona de forma parecida com outras linguagens mas possui um diferencial, conseguimos escolher se queremos guardar numeros inteiros e negativos ou apenas numeros positivos!

E isso está nessa tabela de inteiro como funciona os inteiros:

![Tabela Ints](images/data-types/image.png)


#### Facilidade na leitura de um numero inteiro

Em rust para facilitar a leitura de um numero apenas adicionar um "_".

Ex:
```rust
// Aqui temos na maneira tradicional de se ler um inteiro
let exemplo = 1000000;
let exemplo = 99999;

// Aqui a maneira facilitada
let exemplo = 1_000_000;
let exemplo = 99_000;
// O compilador vai ignorar esses "_" é como se não existisse para ele  

```

## Usize e Isize 

Todos os tipos de dado que foram abordados até agora ocupam um certo espaço na memória.

Ex: Um F64 ocupa 64 bits de memória independentemente do computador ou architetura que o código está rodando.

Mas temos 2 tipos especiais em rust Usize e Isize

Usize é para unsigned values (apenas numeros do zero para cima)
Isize é para signed values (Ambos os numeros negativos e positivos)

Obs: Eles não são tipos na verdade são apelidos/aliases para:

- usize -> u32 ou u64
- isize -> i32 ou i64


Mas porque um ou outro? Pq a própria linguagem Rust identifica a arquitetura ou o computador e adapta o tipo para u32/u64 para máquinas de 32/64 bits.


### Float no rust

Temos no rust dois tipos de float mas eles mudam apenas a precisão de quantos numeros após o a virgula queremos mostrar.

![Tabela Float](images/data-types/float.png)

---

## Diretivas ou Instruções para o Compilador

Em Rust podemos adicionar um comando ou uma instrução ao nosso compilador, no exemplo abaixo:

```rust

type Metros = i32;
fn main() {
    
    #[allow(unused_variables)] // Basicamente aqui neste trecho dizemos para o compilador tolerar uma variável que não usamos para nada.
    let altura: Metros = 1600;
    let largura: Metros = 100;
}

```

Isso não se aplica a apenas uma linha podemos aplicar em uma função inteira.

exemplo:

```rust

type Metros = i32;

#![allow(unused_variables)] // Basicamente aqui neste trecho dizemos para o compilador tolerar uma variável que não usamos para nada.    
fn main() {
    
let altura: Metros = 1600;
    let largura: Metros = 100;
}
```

E por fim podemos adicionar isso para o arquivo de código INTEIRO.
Ex:
```rust
#[allow(unused_variables)] // Basicamente aqui neste trecho dizemos para o compilador tolerar uma variável que não usamos para nada. Para todo o código!
type Metros = i32;


fn main() {
    
    let altura: Metros = 1600;
    let largura: Metros = 100;
}
```



---


## Apelidos para tipos

Em rust existe um meio de dar apelidos para tipos!!

Muito passa!

exemplo em codigo:
```rust
type palavra = &str;
type inteiro = i32;
type float = f64;

fn main(){
    let string: palavra = "Exemplo";
    let inteiros: inteiro = 21;
    let numero_com_virgula: float = 21.00;
}

```

parece bobo mas parece muito top isso!
---

## Constantes e Escopos no rust

Constantes assim como em outras linguagens são imutáveis não podem ser ter o seu valor alterado de maneira alguma.

Mas quais são as diferenças entre uma variavel imutável e uma constante?

Uma constante pode ser usada no arquivo inteiro, independente do escopo que ela é declarada.

ex:

```rust
const TAX_RATE: f64 = 7.25;

fn main(){

    println!("The tax rate is {TAX_RATE}");
}
```

---

## Variable Shadowing

Isso é quando fazemos uma nova declaração de uma nova variável sendo assim a primeira vez que
declaramos ela se torna inválida.
exemplo:

```rust
/*
Primeira vez que declaramos a variável

O que podemos tirar  disso? A variável abaixo gramas é um float que recebeu um valor
10.22 portanto ela se comporta apenas como float.
*/
let gramas = 10.22;

/*
Segunda vez que declaramos a variável

O que podemos tirar  disso? 
A variável abaixo gramas é uma string agora "10.22"
portanto ela se comporta apenas como string até o final do código.
*/
let gramas = "10.22";

/*
Daqui pra baixo ela se comparta como string até declararmos ela pela terceira vez que podemos alterar o tipo dela.

Aqui alteramos o valor dela mais uma vez e dessa vez para int e sendo mutáve,

não se formos colocar um valor nela novamente, precisaria ser um inteiro pois aqui ela foi alterada.

e assim termina a explicação
*/

let mut gramas = 10;


```