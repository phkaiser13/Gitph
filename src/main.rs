/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-2.0
 * This file is licensed under the GNU General Public License v2.0.
 */

// ==============================================================================
// Ponto de Entrada da Aplicação 'gitph'
//
// Este arquivo contém a função `main`, que é o início da execução do programa.
//
// RESPONSABILIDADES:
// - Declarar a estrutura de módulos da aplicação.
// - Orquestrar o fluxo inicial: parsear argumentos de linha de comando ou
//   iniciar a interface de usuário interativa.
// - Servir como o ponto mais alto para o tratamento de erros propagados.
// ==============================================================================

// --- Declaração de Módulos ---
// Informamos ao compilador Rust sobre os outros arquivos de código-fonte que
// compõem nossa aplicação. Cada `mod` corresponde a um arquivo `.rs` ou a um
// diretório com um `mod.rs` dentro.
mod api_client;
mod cli;
mod config;
mod git_wrapper;
mod native_bindings;
mod ui;

// --- Importações (`use`) ---
// Trazemos os tipos e funções que usaremos neste arquivo para o escopo local.
use anyhow::Result; // Usamos `anyhow::Result` para um tratamento de erro simplificado na aplicação.

/// Função principal que é executada quando o programa inicia.
///
/// Retorna um `anyhow::Result<()>` que permite o uso do operador `?` para
/// propagar erros de forma concisa de qualquer parte da aplicação até o topo,
/// onde serão impressos de forma amigável caso ocorram.
fn main() -> Result<()> {
    // --- Teste da Ponte FFI (Rust <-> C++) ---
    // Antes de qualquer outra coisa, vamos verificar se nossa complexa cadeia de
    // compilação e lincagem com o C++ está funcionando. Isso nos dá um feedback
    // rápido e crucial durante o desenvolvimento.
    println!("--- Verificação da Integração Nativa (C++) ---");
    native_bindings::hello(); // Chama a função que imprime "Olá do mundo C++!"
    let number: i32 = 15;
    let result = native_bindings::calculate(number);
    println!("[Rust] Chamei a função C++ com o número {} e recebi: {}", number, result);

    let test_string = "Olá, Rust e C++!";
    let length = native_bindings::string_length(test_string)?;
    println!("[Rust] A string \"{}\" tem {} bytes de acordo com o C++.", test_string, length);
    println!("----------------------------------------------\n");


    // --- Lógica Principal de Orquestração ---
    // Aqui é onde a lógica de decisão da aplicação viverá.
    // Por enquanto, vamos apenas simular o fluxo.

    // TODO: Parsear os argumentos da linha de comando usando o módulo `cli`.
    // let cli_args = cli::parse_arguments();

    // TODO: Implementar a lógica de decisão:
    // if cli_args.has_subcommand() {
    //     // Executar o comando direto (ex: `gitph SND`)
    //     cli::handle_command(cli_args)?;
    // } else {
    //     // Nenhum comando foi passado, então iniciamos o painel interativo.
    //     ui::menus::show_main_menu()?;
    // }

    // Por enquanto, vamos chamar diretamente a função do menu principal (que criaremos a seguir).
    println!("Iniciando o painel interativo de gitph...");
    // A linha abaixo está comentada porque o módulo `ui` ainda não tem a função.
    // Descomentaremos assim que criarmos o arquivo `ui/menus.rs`.
    // ui::menus::show_main_menu()?;


    // Se a execução chegar até aqui sem erros, retornamos `Ok(())` para indicar
    // um encerramento bem-sucedido. O `()` é o tipo "unit", que significa "nada".
    Ok(())
}