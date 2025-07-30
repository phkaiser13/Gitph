// ==============================================================================
// Script de Build do Cargo para o Projeto 'gitph'
//
// Este script é executado pelo Cargo antes de compilar o código-fonte principal
// em `src/`. Sua responsabilidade é compilar e preparar quaisquer dependências
// nativas (não-Rust) que o projeto necessite.
//
// Neste caso, ele é responsável por:
// 1. Invocar o CMake para compilar o módulo C++ em `native_modules/git_optim`.
// 2. Instruir o `rustc` (o compilador Rust) a lincar a biblioteca estática
//    resultante (`libgit_optim.a` ou `git_optim.lib`) no binário final.
// ==============================================================================

// Importamos o crate `cmake`, que foi definido em `[build-dependencies]` no
// arquivo Cargo.toml. Ele nos fornece uma API conveniente para executar o CMake.
extern crate cmake;

fn main() {
    // 1. Compilar a biblioteca C++ usando o CMake.
    // --------------------------------------------------------------------------
    // O método `cmake::Config::new()` inicia um processo de configuração de build.
    // Apontamos para o diretório que contém o arquivo `CMakeLists.txt` do nosso
    // módulo nativo.
    let dst = cmake::Config::new("native_modules/git_optim")
        // Podemos adicionar flags de compilação aqui se necessário.
        // Por exemplo: .cxxflag("-std=c++17")
        .build();

    // 2. Instruir o `rustc` sobre como lincar a biblioteca.
    // --------------------------------------------------------------------------
    // O método `.build()` retorna o diretório onde a biblioteca foi compilada.
    // Nós instruímos o Cargo a adicionar este diretório ao caminho de busca de
    // bibliotecas do linker.
    // A sintaxe `cargo:rustc-link-search=native={}` é uma instrução especial
    // que o Cargo entende.
    println!("cargo:rustc-link-search=native={}", dst.display());

    // Agora, instruímos o Cargo a lincar nossa biblioteca `git_optim`.
    // O linker irá procurar por `libgit_optim.a` (em Linux/macOS) ou
    // `git_optim.lib` (em Windows) nos caminhos de busca especificados.
    // Usamos o tipo "static" para garantir que o código C++ seja embutido
    // diretamente no nosso executável final, mantendo o requisito de um
    // único binário sem dependências de .dll ou .so.
    println!("cargo:rustc-link-lib=static=git_optim");

    // 3. Garantir a Recompilação quando o Código C++ Mudar.
    // --------------------------------------------------------------------------
    // Esta é uma instrução crucial para o desenvolvimento. Ela diz ao Cargo
    // para re-executar este script de build se qualquer arquivo dentro do
    // diretório do nosso módulo C++ for modificado. Sem isso, as alterações
    // no código C++ não seriam compiladas automaticamente com `cargo build`.
    println!("cargo:rerun-if-changed=native_modules/git_optim/");
}