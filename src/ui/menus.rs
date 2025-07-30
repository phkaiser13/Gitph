/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// Trazemos os módulos que usaremos para o escopo local.
use crate::git_wrapper::{commit, push, status::{self, ChangeType, GitStatus}};
use crate::ui::prompts; // Importamos nosso novo módulo de prompts.
use anyhow::Result;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select};
use std::io::BufRead;

// ... (função show_main_menu existente, sem alterações) ...
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
                let continue_loop = handle_menu_action(index)?;
                if !continue_loop {
                    break;
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
fn handle_menu_action(index: usize) -> Result<bool> {
    match index {
        1 => handle_snd_action()?,
        4 => handle_status_action()?,
        9 => {
            println!("Obrigado por usar o gitph. Até logo!");
            return Ok(false);
        }
        _ => {
            println!("Funcionalidade ainda não implementada.");
        }
    }

    println!("\nPressione Enter para voltar ao menu principal...");
    let _ = std::io::stdin().lock().read_line(&mut String::new());
    Ok(true)
}

/// Orquestra o fluxo de trabalho "Adicionar, Commitar, Pushar" (SND).
fn handle_snd_action() -> Result<()> {
    println!("{}", style("Iniciando fluxo de trabalho: Adicionar, Commitar, Pushar").bold().cyan());
    println!("----------------------------------------------------------");

    // --- PASSO 1: Adicionar todos os arquivos ao Stage ---
    println!("1. Adicionando todos os arquivos ao stage (`git add .`)...");
    if let Err(e) = commit::add_all() {
        println!("{}", style("Erro ao adicionar arquivos:").red().bold());
        println!("{}", style(e).red());
        return Ok(()); // Retorna Ok para não fechar o programa, apenas parar o fluxo.
    }
    println!("{}", style("✔ Arquivos adicionados com sucesso.").green());
    println!("----------------------------------------------------------");

    // --- PASSO 2: Verificar se há algo para commitar ---
    // Decisão de engenharia: Em vez de chamar `git commit` cegamente e tratar o
    // erro "nothing to commit", nós verificamos o status proativamente.
    // Isso proporciona uma experiência de usuário muito melhor.
    let status = status::get_status()?;
    let has_staged_files = status.files.iter().any(|f| f.staged_status.is_some());

    if !has_staged_files {
        println!("{}", style("Nenhuma alteração no stage para commitar. O fluxo de trabalho foi concluído.").yellow());
        return Ok(());
    }

    // --- PASSO 3: Obter a Mensagem de Commit ---
    println!("2. Preparando para o commit...");
    let commit_message = match prompts::get_commit_message()? {
        Some(message) => {
            if message.trim().is_empty() {
                println!("{}", style("Mensagem de commit vazia. Operação cancelada.").red());
                return Ok(());
            }
            message
        },
        None => {
            // O usuário pressionou Esc para cancelar.
            println!("{}", style("Operação de commit cancelada pelo usuário.").yellow());
            return Ok(());
        }
    };

    // --- PASSO 4: Executar o Commit ---
    if let Err(e) = commit::commit(&commit_message) {
        println!("{}", style("Erro ao criar o commit:").red().bold());
        println!("{}", style(e).red());
        return Ok(());
    }
    println!("{}", style("✔ Commit criado com sucesso.").green());
    println!("----------------------------------------------------------");

    // --- PASSO 5: Executar o Push ---
    println!("3. Enviando para o repositório remoto (`git push`)...");
    match push::push() {
        Ok(success_message) => {
            println!("{}", style("✔ Push realizado com sucesso.").green());
            // Exibe a mensagem informativa retornada pelo `git push`.
            if !success_message.is_empty() {
                println!("\n-- Resposta do Servidor Remoto --\n{}", style(success_message).dim());
            }
        }
        Err(e) => {
            println!("{}", style("Erro ao realizar o push:").red().bold());
            println!("{}", style(e).red());
        }
    }

    Ok(())
}


// ... (funções handle_status_action, display_git_status, format_change_type existentes) ...
fn handle_status_action() -> Result<()> {
    println!("Obtendo status do repositório Git...\n");
    match status::get_status() {
        Ok(status) => display_git_status(&status),
        Err(e) => {
            println!("{}", style("Erro ao obter status:").red().bold());
            println!("{}", style(e).red());
        }
    };
    Ok(())
}

fn display_git_status(status: &GitStatus) {
    println!("{}", style(&status.branch_info).yellow());
    if status.files.is_empty() {
        println!("\n{}", style("Repositório limpo. Nada a commitar.").green());
        return;
    }
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