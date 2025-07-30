/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

use crate::git_wrapper::status::{self, ChangeType, GitStatus};
use anyhow::Result;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select};
use std::io::BufRead; // Para a pausa

/// Exibe o menu principal da aplicação em um loop contínuo.
pub fn show_main_menu() -> Result<()> {
    let term = Term::stdout();
    let options = &[
        "[1] Setar Repositório por link (srp)",
        "[2] Adicionar, Commitar, Pushar (snd)",
        "[3] SND e Criar Tag/Release (rls)",
        "[4] Push para branch específica (psor)",
        "[5] Ver Status (status)",
        "[6] Criar Nova Branch (cnb)",
        "[7] Mudar de Branch (cb)",
        "[8] Clonar Repositório (clone)",
        "[9] Gerenciar Workflow (cwf)",
        "[10] Sair",
    ];

    loop {
        term.clear_screen()?;
        println!("==============================================");
        println!("  gitph - Seu Assistente de Git Inteligente");
        println!("==============================================\n");

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(options)
            .with_prompt("Navegue com as setas e pressione Enter para selecionar uma ação:")
            .default(0)
            .interact_on_opt(&term)?;

        match selection {
            Some(index) => {
                term.clear_screen()?;
                // A ação é executada em uma função separada para manter o loop limpo.
                let continue_loop = handle_menu_action(index)?;
                if !continue_loop {
                    break; // Sai do loop se a ação retornar `false` (ex: Sair).
                }
            }
            None => {
                println!("Operação cancelada. Saindo do gitph.");
                break;
            }
        }
    }
    Ok(())
}

/// Despacha a ação selecionada no menu para a função correspondente.
/// Retorna `Ok(true)` para continuar o loop ou `Ok(false)` para sair.
fn handle_menu_action(index: usize) -> Result<bool> {
    match index {
        // ... outros casos ...
        4 => handle_status_action()?,
        9 => {
            println!("Obrigado por usar o gitph. Até logo!");
            return Ok(false); // Sinaliza para sair do loop.
        }
        _ => {
            println!("Funcionalidade ainda não implementada.");
        }
    }

    println!("\nPressione Enter para voltar ao menu principal...");
    let _ = std::io::stdin().lock().read_line(&mut String::new());
    Ok(true) // Sinaliza para continuar o loop.
}

/// Lida com a ação "Ver Status". Chama o wrapper Git e exibe o resultado.
fn handle_status_action() -> Result<()> {
    println!("Obtendo status do repositório Git...\n");
    match status::get_status() {
        Ok(status) => display_git_status(&status),
        Err(e) => {
            // Exibe o erro de forma destacada se o comando falhar.
            println!("{}", style("Erro ao obter status:").red().bold());
            println!("{}", style(e).red());
        }
    };
    Ok(())
}

/// Exibe a estrutura `GitStatus` de forma formatada e colorida.
fn display_git_status(status: &GitStatus) {
    // Exibe a informação da branch.
    println!("{}", style(&status.branch_info).yellow());

    if status.files.is_empty() {
        println!("\n{}", style("Repositório limpo. Nada a commitar.").green());
        return;
    }

    // Separa os arquivos em categorias para exibição.
    let mut staged = Vec::new();
    let mut unstaged = Vec::new();
    let mut untracked = Vec::new();

    for file in &status.files {
        if file.staged_status == Some(ChangeType::Untracked) {
            untracked.push(format!("  {}", file.path));
            continue;
        }
        if let Some(change) = &file.staged_status {
            staged.push(format!("  {}: {}", format_change_type(change), file.path));
        }
        if let Some(change) = &file.unstaged_status {
            unstaged.push(format!("  {}: {}", format_change_type(change), file.path));
        }
    }

    // Exibe cada seção apenas se ela contiver arquivos.
    if !staged.is_empty() {
        println!("\n{}", style("Alterações para Commit (Staged):").green().bold());
        println!("{}", style("(use 'git reset HEAD <arquivo>...' para remover do stage)").dim());
        println!("{}", staged.join("\n"));
    }
    if !unstaged.is_empty() {
        println!("\n{}", style("Alterações não Staged para Commit:").red().bold());
        println!("{}", style("(use 'git add <arquivo>...' para incluir no commit)").dim());
        println!("{}", unstaged.join("\n"));
    }
    if !untracked.is_empty() {
        println!("\n{}", style("Arquivos não Rastreados (Untracked):").red().bold());
        println!("{}", style("(use 'git add <arquivo>...' para rastrear)").dim());
        println!("{}", untracked.join("\n"));
    }
}

/// Formata um `ChangeType` em uma string colorida para exibição.
fn format_change_type(change: &ChangeType) -> String {
    match change {
        ChangeType::Added => style("ADICIONADO").green().to_string(),
        ChangeType::Modified => style("MODIFICADO").yellow().to_string(),
        ChangeType::Deleted => style("DELETADO  ").red().to_string(),
        ChangeType::Renamed => style("RENOMEADO ").cyan().to_string(),
        ChangeType::Copied => style("COPIADO   ").cyan().to_string(),
        ChangeType::TypeChanged => style("TIPO ALT. ").magenta().to_string(),
        ChangeType::Unmerged => style("CONFLITO  ").red().bold().to_string(),
        ChangeType::Untracked => style("NOVO ARQV.").red().to_string(),
    }
}