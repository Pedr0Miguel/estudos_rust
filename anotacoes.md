# anotacoes

Este arquivo serve para fazer anotações sobre o aprendizado em rust.

---


## Como funciona?

Funciona da seguinte forma tudo vai ser escrito de uma maneira diferente, para melhor documentar ou anotar, então fica o template:

---

## Título do update

Descrição do que é existe. Neste momento vai ser feito assim...

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