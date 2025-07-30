# ==============================================================================
# CMake Build Script para o Módulo Nativo 'git_optim'
#
# Este arquivo define como o código C++ neste diretório deve ser compilado.
# Ele é invocado pelo script `build.rs` do nosso projeto Rust.
# O objetivo é produzir uma biblioteca ESTÁTICA que será lincada ao
# binário final do `gitph`.
# ==============================================================================

# Especifica a versão mínima do CMake necessária para processar este arquivo.
# Isso garante que não sejam usadas features incompatíveis com versões mais antigas.
cmake_minimum_required(VERSION 3.10)

# Define o nome do projeto e especifica que a linguagem principal é C++.
# O nome do projeto aqui, 'git_optim', é importante, pois ele influencia
# o nome do arquivo de saída da biblioteca.
project(git_optim LANGUAGES CXX)

# ------------------------------------------------------------------------------
# Configuração do Padrão C++
# ------------------------------------------------------------------------------
# É uma boa prática definir explicitamente o padrão da linguagem C++ a ser usado.
# Isso garante consistência na compilação em diferentes ambientes.
# Usamos C++17, um padrão moderno e amplamente suportado.
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# ------------------------------------------------------------------------------
# Definição da Biblioteca
# ------------------------------------------------------------------------------
# Este é o comando principal. Ele cria um alvo de biblioteca chamado 'git_optim'.
#
# - 'git_optim': O nome da nossa biblioteca. O Cargo/rustc irá procurar por este nome.
# - 'STATIC':    Instrui o CMake a criar uma biblioteca estática (.a no Linux/macOS,
#                .lib no Windows). O código desta biblioteca será incorporado
#                diretamente no executável Rust, o que é crucial para a
#                distribuição de um único arquivo binário.
# - Fontes:      A lista de arquivos de código-fonte que compõem a biblioteca.
add_library(
    git_optim
    STATIC
    src/optim.cpp
    src/optim.h
)

# Nota: Não precisamos especificar o diretório de saída aqui. O crate `cmake`
# no `build.rs` gerencia isso e informa o caminho correto para o `rustc`.
# Isso mantém a integração entre os dois sistemas de build limpa e robusta.