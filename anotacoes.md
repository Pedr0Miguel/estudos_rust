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

## Moves and Ownership

Quando temos um tipo que não possue o trait copy o rust faz oq? Ele move a responsabilidade de uma variável para outra.

Exemplo:
```rust
fn main() {
    let person: String = String::from("Pedro");
    let genius: String = person;
}
```

Quando passamos a variável person para a variável genius não só passamos o valor mas movemos a responsabilidade que ela vai ter que limpar a memória heap.

```rust
fn main() {
    let person: String = String::from("Pedro");
    let genius: String = person;

    print!("{person}");
}
```

Ao fazer isso no código ele vai dar erro pois person não possui mais o valor e nem tem mais responsabilidade sobre o que ele possuia e ocorrendo um erro ao compilar o código.


Mas e se mesmo assim quisessemos chamar person? Aí teríamos que fazer o seguinte:

```rust
fn main() {
    let person: String = String::from("Pedro");

    print!("{person}");

    let genius: String = person;

}
```
Assim o código fica válido pois a variável person só perde a responsabilidade do que ela tem quando passamos para genius, podemos manipular person o quanto quisermos desde que seja antes de passarmos para a outra variável.

---

## The push_str Method on a String type

Como podemos concatenar uma string que foi adicionada na Heap? Simples usamos o método push_str para nos fazer esse favor.

---

## The String type

Um datatype que é alocado na memória heap é a String.

Bem, rust tem dois tipos de String um já vimos anteriormente que era o **str**, neste tipo ele não é armazenado nem dentro da Stack ou na Heap ele é anexado ao compilador isso acontece pois o valor de str já é dito no tempo de compilação.

```rust
fn main() {
    let comida = "massa";
}
```

O tipo de string que estamos falando é o ***String***.

Mas pq o Rust precisa dessa diferenciação como dito antes str é bom quando já sabemos o valor que ele tem mas o String é para receber o valor enquanto o programa roda, como se fossemos inserir nome, endereço, CPF, etc.

No meio tradicional adicionamos a string assim:

```rust
fn main() {
    let text: String = String::new();
}
```

Ou podemos fazer dessa forma:

```rust
fn main() {
    let text: String = String::new();
    let candy: String = String::from("Kill");
}
```



---

## The Copy trait

Vamos aprender sobre a copy trait.

O Copy trait mostra que podemos clonar um tipo.

Os tipos primitivos do Rust como Boolean, Ints, Floats e mais, possuem o Copy trait e isso significa vão ser criadas cópias dos valores desses tipos automaticamente em algumas situações que eles precisem ser duplicados.

Um exemplo em código de como podemos ativar essa duplicação automatica.

```rust
fn main() {
    let time = 2025;
    let year = time; // Aplica o Copy trait e faz uma duplicação do valor dentro de time

    println!("tempo é {time} e ano é {year}");
}

```

Neste simples exemplo que pode se aplicar a mesma coisa para a maioria dos tipos, mas isso se torna diferente quando estamos lidando com heap.

---


## Scope e Ownership

o Owner é quem é responsável por limpar os dados de dentro de uma variável.

E como o Owner sabe que precisa lempar os dados dentro da variável?
|O Owner sabe que precisa limpar os dados quando a variável fica fora de escopo.

Que é quando o bloco de código termina {}

Exemplo em código:

```rust
fn main() {
    let idade = 32; // idade é dona do dado 32

    {
        let handsome = true; // Esta variável só existe neste trecho de código
        // Essa variável só vai existir aqui dentro
    } // Aqui ela fica fora de escopo então simplesmente é limpa da memória

    // idade existe aqui.
}
/*
idade à partir daqui está fora de escopo, sendo assim,
ela é responsável por retirar os dados da memória.
*/
```

Isso é um exemplo que se aplica na Stack mais a frente no curso veremos como funciona dentro da Heap.

---

## Ownership: Stack e Heap

Apesar de já sabermos vamos relembrar:

Stack e Heap são duas regiões diferentes da memória do computador.

Ambos escrevem e lêem dados de maneiras deiferentes o que nos da vantagens e desvantagens.

### Stack

Stack é geralmente rápida, mas ela só suporta um conjunto de dados fixos e de tamanho previsível e esse tamanho já tem que ser informado no tempo de compilação.

A Stack possui uma estrutura que armazena os valores em uma ordem sequencial que recebem os valores e já os remove. Como uma pilha de coisas.

Um exemplo:

bom seria uma pilha de pratos de comida o último a ser colocado é o primeiro a ser retirado.

### Stack II

Todo dado da stack é fixo, tem um tamanho consistente e já é conhecido na hora de compilar o programa.

Um exemplo é o tipo i32 ele sempre será um i32 e o compilador já sabe isso quando vai compilar o programa.


Quando um programa em Rust precisa de um tamanho dinâmico, ele requisita isso para a memória heap. O programa chama o alocador de memória que encontra o local certo que tenha tamanho suficiente para armazenar o valor.



### Heap

Geralmente sendo mais devagar que a memória de Stack, mas ela suporta dados dinâmicos que podem mudar de tamanho com o uso do programa.

A Heap é um grande armazenamento como se fosse um armazém de algum supermercado.

A heap só é requisitada pelo rust quando precisa armazenar algo quando é algo que não sabemos o tamanho, algo como o endereço de alguém ou quando pedimos o anexo de um arquivo, nós não sabemos o tamanho deles então podem ter um tamanho grande demais, então não armazenamos isso na Stack armazenamos isso na Heap.

Como isso funciona? Veremos no tópico seguinte:

### Memory Allocator

Um programa chamado **Memory Allocator** e esse programa procura por um espaço dentro da memória heap grande o suficiente para armazenarmos esse dado.

O MA depois de armazenar retorna um endereço/referência, que é o id da memória que foi armazenado o dado.

Esse endereço/referência pode ser chamado de ponteiro pois isso aponta para onde a memória está armazenada.

### Heap II

A heap é lenta em leitura de dados e também em alocação de dados pois comparada com a stack a stack já sabemos o tamnho local e tudo mais.

Na Heap precisamos procurar o tamanho ideal e depois o endereço para procurar.


### Conclusão

Está seção do curso é focada no conceito de ownership e o propósito de ownership é de responsabilizar a desalocação de memória, particularmente a heap.

---

## Ownership: Introdução

Uma feature única da linguagem, Ownership é um conjunto de regras que o compilador checka para ter certeza que o programa final não possui erros de memória.

Para entender isso precisamos ver como erros como esses podem surgir quando estamos trabalhando com memória.

### Memória

Memória como você já sabe é uma parte do seu hardware que é responsável por armazenar a informação que o seu programa usa.

Muitos programas que abrimos como o Vs code faz uma alocação de memória na memória ram de informações que são importantes para esse programa e toda hora solicita pela memória alocada, outros programas também desalocam memórias que estão armazenadas para maior performance pois nossa memória ram é bem limitada.

### Como funcionava o manejo de memória manual

Em linguagens como C e C++, o programador é o responsável para alocar memória e desalocar memória.

Um dos erros que podem ocorrer por deixar o programador decidir no C/C++ é que o programador pode cometer o erro de esquecer de desalocar a memória o que pode ocorrer de o sistema sempre ficar requisitando memória mas nunca a devolvendo... Outro erro pode ser o de desalocar a memória que já estava desalocada.

### Garbage collector

Outra maneira que encontraram de fazer esse manejo de forma simples é o uso de garbage colector como o Java faz, mas pode desalecerar o programa pois o sistema teria que parar para fazer essa limpeza.

Basicamente esse sistema de garbage analiza a memória que não está mais em uso e limpa ela, o problema com isso é que o garbage colector por sí só já usa memória e pode rodar em alguma hora delicada de deixar o programa lento.

### Solução do Rust no gerenciamento de memória

Rust introduz o paradigma de: Ownership

Indo mais a fundo no que Rust oferece:

O Owner é quem ou o que é responsável por limpar a parte da memória que não está mais em uso.

Todo valor em Rust possui um Owner.

O Owner pode mudar enquanto o programa roda mas é um owner por valor por vez.

Mas o que pode ser um Owner? Uma variável e um parâmetro podem ser Owners.

Ownership se extende também para tipos como tupla ou array, eles são considerados os donos daquele conjunto de valores.

---

## Debugging

Usando o vs code e sua funcionalidade de debugg, podemos executar nosso código uma linha por vez.

Ao invés de rodar o código todo de uma vez, podemos designiar algumas linhas que queremos que o código pare e analizar o comportamento do código.

Essas etapas que queremos parar o código se chamam breakpoints

Como demonstrado na aula 93 podemos percorrer o código trecho por trecho vendo todo o código em sua execução.

### Debug II

Neste trecho vai ser dito um pouco mais afundo no debug.

Quando entramos em debug mode temos algumas abas como VARIABLES que mostra as nossas variáveis né.

E temos a aba de "watch", essa aba é chamada assim pois ela observa os valores referenciados e então faz calculos quando esses valores mudam. Isso vai recomputar alguns dados baseado nos dados que existem no programa.

Basicamente no watch é possível colocar alguns trechos de códigos, como um if/else, <variável> * 2, etc.

Mas pode ocorrer alguns bugs pois coisas muito complexas podem não rodar ou ter um comportamento inesperado.

### Debug III

Nesta parte é para detalharmos o que os outros botões do debug mode fazem.

Alguns botões podem ter o comportamento parecido com o continue button como pular de função em função ou de linha por linha


Nós temos o step over que é o que pula de função em função basicamente falando. Ele para onde vc quer e se vc apertar nele pulamos para a próxima função, sem questionarmos o que roda dentro da função que pulamos.

Temos o step into ele faz o contrário do último ele entre dentro do código da função e podemos ver como está funcionando o código, se você continuar usando o step into ele vai seguir linha por linha o que o código está fazendo.

Se estiver dentro de uma função e quiser se retirar dela pois já foi debuggado o que precisava é só apertar em step out, assim o que faltava ser executado na função era executado e seguimos caminho com o debugg.
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