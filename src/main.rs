/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Ponto de Entrada e Despachante Central da Aplicação 'gitph'
//
// Este arquivo contém a função `main`, que agora atua como o cérebro inicial
// da aplicação. Sua responsabilidade é determinar o modo de operação:
//
// 1. MODO DE COMANDO DIRETO: Se o usuário fornecer um subcomando na linha de
//    comando (ex: `gitph clone <url>`), a aplicação executa essa ação
//    imediatamente e encerra.
//
// 2. MODO INTERATIVO: Se nenhum subcomando for fornecido, a aplicação inicia
//    o painel de menu interativo, guiando o usuário pelas funcionalidades.
// ==============================================================================

// --- Declaração de Módulos ---
// A estrutura de módulos da nossa aplicação.
mod api_client;
mod cli;
mod config;
mod git_wrapper;
mod native_bindings;
mod ui;

// --- Importações (`use`) ---
use crate::cli::Cli; // Importamos a struct principal da nossa definição de CLI.
use anyhow::Result;
use clap::Parser; // Importamos o trait `Parser` para ter acesso ao método `.parse()`.

/// Função principal que é executada quando o programa inicia.
///
/// Ela analisa os argumentos da linha de comando e despacha para o manipulador
/// apropriado (CLI direta ou UI interativa).
fn main() -> Result<()> {
    // --- PASSO 1: Analisar os Argumentos da Linha de Comando ---
    // `Cli::parse()` é a mágica do `clap`. Ele lê os argumentos fornecidos pelo
    // usuário, os valida contra a estrutura que definimos em `cli.rs`, e preenche
    // a struct `Cli`. Se o usuário passar `--help` ou `--version`, `clap` lida
    // com isso e encerra a aplicação automaticamente.
    let cli_args = Cli::parse();

    // --- PASSO 2: Decidir o Fluxo de Execução ---
    // Verificamos se o campo `command` da nossa struct `Cli` contém `Some(comando)`.
    // `if let` é a maneira idiomática e limpa em Rust para fazer isso.
    if let Some(command) = cli_args.command {
        // MODO DE COMANDO DIRETO: Um subcomando foi fornecido.
        // Passamos o comando para o nosso manipulador de CLI, que executará
        // a lógica correspondente. O `?` propagará qualquer erro que ocorra.
        cli::handle_cli_command(command)?;
    } else {
        // MODO INTERATIVO: Nenhum subcomando foi fornecido.
        // Iniciamos o painel principal da aplicação, como fazíamos antes.
        ui::menus::show_main_menu()?;
    }

    // Se a execução chegar até aqui, significa que a ação (seja ela direta ou
    // interativa) foi concluída sem erros fatais. Retornamos `Ok(())` para
    // indicar um encerramento bem-sucedido para o sistema operacional.
    Ok(())
}