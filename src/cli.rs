/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Módulo de Interface de Linha de Comando (CLI)
//
// Este módulo define e implementa a interface de linha de comando não-interativa
// da aplicação usando o crate `clap`. Ele permite que os usuários executem
// ações diretamente, como `gitph cnb nova-feature`, sem entrar no menu.
// ==============================================================================

use crate::git_wrapper::{branch, clone};
use anyhow::Result;
use clap::{Parser, Subcommand};
use console::style;

/// A estrutura principal que define a CLI.
/// `clap` usará esta struct e seus atributos para gerar o parser de argumentos,
/// mensagens de ajuda, informações de versão, etc.
#[derive(Parser, Debug)]
#[command(
    author = "Pedro H. Garcia (phkaiser13)",
    version,
    about = "gitph: Um assistente de Git moderno para otimizar seu fluxo de trabalho.",
    long_about = "Uma ferramenta de linha de comando que simplifica operações comuns do Git através de um painel interativo ou comandos diretos."
)]
pub struct Cli {
    /// O subcomando a ser executado.
    /// Se nenhum subcomando for fornecido, a aplicação iniciará o menu interativo.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Um enum que representa todos os subcomandos disponíveis na CLI.
/// Cada variante corresponde a uma ação que o usuário pode executar.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// [cnb] Cria uma nova branch local.
    Cnb {
        /// O nome da nova branch a ser criada.
        name: String,
    },
    /// [cb] Muda para uma branch existente.
    Cb {
        /// O nome da branch para a qual mudar.
        name: String,
    },
    /// [clone] Clona um repositório de uma URL.
    Clone {
        /// A URL (HTTPS ou SSH) do repositório a ser clonado.
        url: String,
    },
    // NOTA: Os comandos `snd` e `rls` são intencionalmente omitidos da CLI direta
    // por enquanto, pois seus fluxos de trabalho são inerentemente interativos
    // (exigem prompts para mensagens de commit, notas de release, etc.).
    // Eles permanecem como as principais funcionalidades do modo de painel.
}

/// Lida com a execução de um subcomando que foi analisado pela `clap`.
///
/// Esta função atua como um despachante, chamando a lógica apropriada do
/// `git_wrapper` com base no comando fornecido pelo usuário.
///
/// # Arguments
/// * `command` - O enum `Commands` que representa a ação a ser executada.
///
/// # Returns
/// `Ok(())` se a ação foi bem-sucedida, ou `Err` se ocorreu um erro.
pub fn handle_cli_command(command: Commands) -> Result<()> {
    match command {
        Commands::Cnb { name } => {
            println!("Criando nova branch '{}'...", style(&name).cyan());
            match branch::create_branch(&name) {
                Ok(()) => println!("{}", style("✔ Branch criada com sucesso.").green()),
                Err(e) => eprintln!("{} {}", style("Erro:").red().bold(), style(e).red()),
            }
        }
        Commands::Cb { name } => {
            println!("Mudando para a branch '{}'...", style(&name).cyan());
            match branch::switch_branch(&name) {
                Ok(()) => println!("{}", style("✔ Mudou para a branch com sucesso.").green()),
                Err(e) => eprintln!("{} {}", style("Erro:").red().bold(), style(e).red()),
            }
        }
        Commands::Clone { url } => {
            // A função `clone_repository` já imprime seu próprio feedback em tempo real,
            // então não precisamos de mensagens de sucesso/erro adicionais aqui.
            if let Err(e) = clone::clone_repository(&url) {
                eprintln!("\n{} {}", style("Erro:").red().bold(), style(e).red());
            }
        }
    }
    Ok(())
}