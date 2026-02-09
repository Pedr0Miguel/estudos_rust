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
