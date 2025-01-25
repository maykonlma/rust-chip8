# Emulador Chip-8 em Rust

## 🚀 **Meus primeiros passos no desenvolvimento de emuladores**

Este projeto é um estudo prático que explora a criação de um emulador Chip-8. Como muitas pessoas, sempre tive vontade de criar jogos. No entanto, apesar de não ter muito jeito para criações artísticas, me considero bom em escovar bits 🫣🫣🫣. Foi então que, assistindo a um vídeo sobre emuladores do [Fabio Akita](https://www.youtube.com/watch?v=vUqLLpUJ47s), pensei: *por que não tentar desenvolver um emulador?*

Após algumas pesquisas, descobri que o Chip-8, apesar de ser um interpretador, é considerado uma das melhores opções para quem deseja dar os primeiros passos no estudo de emuladores. Esse projeto vem da vontade de aprender mais sobre Rust e computação de baixo nível.

## Observação

Este projeto é puramente um estudo pessoal e tem como objetivo o aprendizado e a exploração do desenvolvimento de emuladores. Não há intenção de gerar lucro ou ganhos financeiros com este projeto. Todo o código e conteúdo deste repositório são de minha autoria e estão disponíveis para fins educacionais e de compartilhamento de conhecimento.  

**Não tenho a intenção de infringir direitos autorais de terceiros. Caso algum conteúdo relacionado a este projeto seja considerado uma violação, por favor, entre em contato para que eu possa tomar as devidas providências.**

## O que é o Chip-8?

O Chip-8 é uma linguagem de programação interpretada projetada nos anos 1970 para facilitar o desenvolvimento de jogos em sistemas simples, como o COSMAC VIP. Ele é frequentemente usado como ponto de partida para estudos de emulação devido à sua simplicidade.  

O Chip-8 possui um conjunto reduzido de instruções (apenas 35 opcodes), o que torna sua implementação mais acessível para iniciantes. Além disso, seus gráficos são baseados em uma tela de 64x32 pixels monocromática e seus jogos usam um teclado de 16 teclas, características que permitem entender e construir um emulador sem precisar lidar com os desafios mais complexos de arquiteturas modernas.  

Este projeto, portanto, explora a implementação de um emulador Chip-8 com o objetivo de entender os fundamentos da emulação, como a execução de opcodes, manipulação de memória e renderização gráfica.

## Pré-requisitos 

Este projeto utiliza a biblioteca **SDL2** para gerar a janela de execução, portanto é necessário que ela esteja instalada no seu sistema operacional.  


**Linux (Debian/Ubuntu):**  
```
  sudo apt update
  sudo apt install libsdl2-dev
```

**Windows:** Baixe e instale o SDL2 a partir do site oficial: [libsdl.org](https://www.libsdl.org/).

## Configuração do Teclado  

Abaixo se encontra o mapeamento entre o teclado físico e o teclado virtual do Chip-8 utilizado no emulador:  

    Teclado                    Chip-8
    +---+---+---+---+           +---+---+---+---+
    | 1 | 2 | 3 | 4 |           | 1 | 2 | 3 | C |
    +---+---+---+---+           +---+---+---+---+
    | Q | W | E | R |           | 4 | 5 | 6 | D |
    +---+---+---+---+     =>    +---+---+---+---+
    | A | S | D | F |           | 7 | 8 | 9 | E |
    +---+---+---+---+           +---+---+---+---+
    | Z | X | C | V |           | A | 0 | B | F |
    +---+---+---+---+           +---+---+---+---+

## Como Usar

1. Clone o repositório:
```
    git clone https://github.com/maykonlma/rust-chip8.git
```
2. Navegue até o diretório do projeto e execute o emulador passando o arquivo da rom:
```
    cd rust-chip8
    cargo run roms/INVADERS 
```

## Contribuições

Sinta-se à vontade para contribuir! Abra uma issue ou envie um pull request.

🔗 Confira o código no [GitHub](https://github.com/maykonlma/rust-chip8.git)

#Rust #RustLang #Emulador #Chip8 #RetroGaming #GameDevelopment #LowLevelProgramming  