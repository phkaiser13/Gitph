/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Módulo de Menus Interativos
//
// Este módulo é responsável por renderizar os menus de seleção que guiam o
// usuário através das funcionalidades do `gitph`. Ele atua como o principal
// orquestrador para o modo interativo da aplicação.
// ==============================================================================

// --- Importações ---
// Trazemos todos os módulos e tipos que usaremos para o escopo local.
use crate::api_client;
use crate::git_wrapper::{branch, clone, commit, push, remote, status::{self, ChangeType, GitStatus}, tag};
use crate::ui::prompts;
use anyhow::Result;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select};
use std::io::BufRead;

/// Exibe o menu principal da aplicação em um loop contínuo.
///
/// Esta função limpa o terminal, mostra um cabeçalho e apresenta uma lista de
/// opções. O usuário pode navegar com as setas e selecionar com Enter. O loop
/// continua até que a opção "Sair" seja selecionada.
pub fn show_main_menu() -> Result<()> {
    let term = Term::stdout();
    let options = &[
        "[1] Setar Repositório por link (srp)", // Ainda não implementado
        "[2] Adicionar, Commitar, Pushar (snd)",
        "[3] SND e Criar Tag/Release (rls)",
        "[4] Push para branch específica (psor)", // Ainda não implementado
        "[5] Ver Status (status)",
        "[6] Criar Nova Branch (cnb)",
        "[7] Mudar de Branch (cb)",
        "[8] Clonar Repositório (clone)",
        "[9] Gerenciar Workflow (cwf)", // Ainda não implementado
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
    // O `match` usa o índice do array `options` para decidir qual ação tomar.
    match index {
        1 => handle_snd_action()?,
        2 => handle_rls_action()?,
        4 => handle_status_action()?,
        5 => handle_create_branch_action()?,
        6 => handle_switch_branch_action()?,
        7 => handle_clone_action()?,
        9 => {
            println!("Obrigado por usar o gitph. Até logo!");
            return Ok(false); // Sinaliza para sair do loop.
        }
        _ => {
            println!("{}", style("Funcionalidade ainda não implementada.").yellow());
        }
    }

    println!("\nPressione Enter para voltar ao menu principal...");
    let _ = std::io::stdin().lock().read_line(&mut String::new());
    Ok(true) // Sinaliza para continuar o loop.
}

// --- Implementações dos Manipuladores de Ação ---

/// Lida com a ação "Adicionar, Commitar, Pushar".
fn handle_snd_action() -> Result<()> {
    println!("{}", style("Iniciando fluxo de trabalho: Adicionar, Commitar, Pushar").bold().cyan());
    println!("----------------------------------------------------------");
    let _ = run_snd_flow()?;
    Ok(())
}

/// Orquestra o fluxo de trabalho "SND e Criar Release".
fn handle_rls_action() -> Result<()> {
    println!("{}", style("Iniciando fluxo de trabalho: Criar Nova Release").bold().cyan());
    println!("----------------------------------------------------------");

    if !run_snd_flow()? {
        println!("\n{}", style("Fluxo de trabalho de release abortado pois a sincronização inicial não foi concluída.").yellow());
        return Ok(());
    }
    println!("----------------------------------------------------------");
    println!("✔ Sincronização inicial concluída.");

    println!("\n2. Obtendo informações do repositório remoto...");
    let (owner, repo) = match remote::get_origin_url().and_then(|url| remote::parse_github_owner_and_repo(&url)) {
        Ok(data) => data,
        Err(e) => {
            println!("{}", style("Erro:").red().bold());
            println!("{}", style(e).red());
            return Ok(());
        }
    };
    println!("✔ Repositório detectado: {}/{}", owner, repo);

    let tag_name = match prompts::get_commit_message()? {
        Some(name) if !name.trim().is_empty() => name,
        _ => {
            println!("{}", style("Nome da tag inválido ou operação cancelada.").yellow());
            return Ok(());
        }
    };

    let release_title = tag_name.clone();
    let release_notes = match prompts::get_release_notes()? {
        Some(notes) if !notes.trim().is_empty() => notes,
        _ => {
            println!("{}", style("Notas da release vazias ou operação cancelada.").yellow());
            return Ok(());
        }
    };

    println!("\n3. Criando e enviando a tag Git...");
    if let Err(e) = tag::create_annotated_tag(&tag_name, &release_title) {
        println!("{}", style("Erro ao criar a tag local:").red().bold());
        println!("{}", style(e).red());
        return Ok(());
    }
    if let Err(e) = tag::push_tag(&tag_name) {
        println!("{}", style("Erro ao enviar a tag para o remoto:").red().bold());
        println!("{}", style(e).red());
        return Ok(());
    }
    println!("✔ Tag '{}' criada e enviada com sucesso.", tag_name);

    println!("\n4. Criando a Release no GitHub...");
    match api_client::github::create_release(&owner, &repo, &tag_name, &release_title, &release_notes) {
        Ok(()) => {
            println!("{}", style("✔ Release criada com sucesso no GitHub!").green().bold());
            println!("Acesse em: https://github.com/{}/{}/releases/tag/{}", owner, repo, tag_name);
        }
        Err(e) => {
            println!("{}", style("Erro ao criar a release no GitHub:").red().bold());
            println!("{}", style(e).red());
        }
    }

    Ok(())
}

/// Lida com a ação "Ver Status".
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

/// Lida com a ação "Criar Nova Branch".
fn handle_create_branch_action() -> Result<()> {
    println!("{}", style("Criar Nova Branch").bold().cyan());
    let branch_name = match prompts::get_commit_message()? { // Reutiliza o prompt de linha única
        Some(name) if !name.trim().is_empty() => name,
        _ => {
            println!("{}", style("Nome de branch inválido ou operação cancelada.").yellow());
            return Ok(());
        }
    };

    match branch::create_branch(&branch_name) {
        Ok(()) => println!("✔ Branch '{}' criada com sucesso.", style(branch_name).cyan()),
        Err(e) => {
            println!("{}", style("Erro ao criar a branch:").red().bold());
            println!("{}", style(e).red());
        }
    }
    Ok(())
}

/// Lida com a ação "Mudar de Branch".
fn handle_switch_branch_action() -> Result<()> {
    println!("{}", style("Mudar de Branch").bold().cyan());
    let branches = match branch::list_branches() {
        Ok(b) => b,
        Err(e) => {
            println!("{}", style("Erro ao listar as branches:").red().bold());
            println!("{}", style(e).red());
            return Ok(());
        }
    };

    if branches.is_empty() {
        println!("{}", style("Nenhuma branch encontrada neste repositório.").yellow());
        return Ok(());
    }

    // Formata os nomes das branches para o menu, destacando a atual.
    let branch_names: Vec<String> = branches
        .iter()
        .map(|b| {
            if b.is_current {
                format!("* {}", b.name)
            } else {
                format!("  {}", b.name)
            }
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&branch_names)
        .with_prompt("Selecione a branch para a qual deseja mudar:")
        .default(0)
        .interact_on_opt(&Term::stdout())?;

    if let Some(index) = selection {
        let target_branch = &branches[index].name;
        match branch::switch_branch(target_branch) {
            Ok(()) => println!("✔ Mudou para a branch '{}' com sucesso.", style(target_branch).cyan()),
            Err(e) => {
                println!("{}", style("Erro ao mudar de branch:").red().bold());
                println!("{}", style(e).red());
            }
        }
    } else {
        println!("{}", style("Operação cancelada.").yellow());
    }

    Ok(())
}

/// Lida com a ação "Clonar Repositório".
fn handle_clone_action() -> Result<()> {
    println!("{}", style("Clonar Repositório Remoto").bold().cyan());
    let url = match prompts::get_commit_message()? { // Reutiliza o prompt de linha única
        Some(u) if !u.trim().is_empty() => u,
        _ => {
            println!("{}", style("URL inválida ou operação cancelada.").yellow());
            return Ok(());
        }
    };

    // A função `clone_repository` já imprime todo o feedback necessário em tempo real.
    if let Err(e) = clone::clone_repository(&url) {
        // Apenas imprimimos um erro final se a função retornar um.
        eprintln!("\n{} {}", style("Erro:").red().bold(), style(e).red());
    }
    Ok(())
}

// --- Funções Auxiliares e Lógica Reutilizável ---

/// Executa a lógica principal de Adicionar, Commitar e Pushar.
fn run_snd_flow() -> Result<bool> {
    commit::add_all().map_err(|e| {
        println!("{}", style("Erro ao adicionar arquivos:").red().bold());
        println!("{}", style(&e).red());
        e
    })?;
    println!("✔ Arquivos adicionados ao stage.");

    let status = status::get_status()?;
    if !status.files.iter().any(|f| f.staged_status.is_some()) {
        println!("{}", style("Nenhuma alteração no stage para commitar.").yellow());
        return Ok(true);
    }

    let commit_message = match prompts::get_commit_message()? {
        Some(message) if !message.trim().is_empty() => message,
        _ => {
            println!("{}", style("Commit cancelado.").yellow());
            return Ok(false);
        }
    };
    commit::commit(&commit_message).map_err(|e| {
        println!("{}", style("Erro ao criar o commit:").red().bold());
        println!("{}", style(&e).red());
        e
    })?;
    println!("✔ Commit criado com sucesso.");

    match push::push() {
        Ok(msg) => {
            println!("{}", style("✔ Push realizado com sucesso.").green());
            if !msg.is_empty() {
                println!("{}", style(msg).dim());
            }
        }
        Err(e) => {
            println!("{}", style("Erro ao realizar o push:").red().bold());
            println!("{}", style(&e).red());
            return Err(e);
        }
    }
    Ok(true)
}

/// Exibe a estrutura `GitStatus` de forma formatada e colorida.
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