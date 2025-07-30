/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// Trazemos todos os módulos que usaremos para o escopo local.
use crate::api_client;
use crate::git_wrapper::{commit, push, remote, status::{self, ChangeType, GitStatus}, tag};
use crate::ui::prompts;
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
        2 => handle_rls_action()?, // Nova ação
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

/// Lida com a ação "Adicionar, Commitar, Pushar".
/// Esta função agora é um simples wrapper em torno da lógica reutilizável.
fn handle_snd_action() -> Result<()> {
    println!("{}", style("Iniciando fluxo de trabalho: Adicionar, Commitar, Pushar").bold().cyan());
    println!("----------------------------------------------------------");
    // A lógica real foi movida para `run_snd_flow` para ser reutilizada.
    let _ = run_snd_flow()?;
    Ok(())
}

/// Orquestra o fluxo de trabalho "SND e Criar Release".
fn handle_rls_action() -> Result<()> {
    println!("{}", style("Iniciando fluxo de trabalho: Criar Nova Release").bold().cyan());
    println!("----------------------------------------------------------");

    // --- PASSO 1: Executar o fluxo SND (Add, Commit, Push) ---
    // Reutilizamos a lógica `snd` para garantir que o repositório remoto
    // esteja sincronizado antes de criarmos a tag e a release.
    if !run_snd_flow()? {
        // Se `run_snd_flow` retornar `false`, significa que o fluxo foi
        // interrompido (ex: nada para commitar, ou cancelado pelo usuário).
        // Nesse caso, abortamos o fluxo de release também.
        println!("\n{}", style("Fluxo de trabalho de release abortado pois a sincronização inicial não foi concluída.").yellow());
        return Ok(());
    }
    println!("----------------------------------------------------------");
    println!("✔ Sincronização inicial concluída.");

    // --- PASSO 2: Obter informações do repositório para a API ---
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

    // --- PASSO 3: Obter detalhes da Release do usuário ---
    let tag_name = match prompts::get_commit_message()? { // Reutilizando o prompt de commit para o nome da tag
        Some(name) if !name.trim().is_empty() => name,
        _ => {
            println!("{}", style("Nome da tag inválido ou operação cancelada.").yellow());
            return Ok(());
        }
    };

    let release_title = tag_name.clone(); // Sugerimos o nome da tag como título da release.

    let release_notes = match prompts::get_release_notes()? {
        Some(notes) if !notes.trim().is_empty() => notes,
        _ => {
            println!("{}", style("Notas da release vazias ou operação cancelada.").yellow());
            return Ok(());
        }
    };

    // --- PASSO 4: Criar e Enviar a Tag Git ---
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

    // --- PASSO 5: Criar a Release no GitHub ---
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

/// Executa a lógica principal de Adicionar, Commitar e Pushar.
/// Esta função foi refatorada para ser reutilizável.
/// Retorna `Ok(true)` se o fluxo foi concluído, `Ok(false)` se foi interrompido.
fn run_snd_flow() -> Result<bool> {
    // Adicionar
    commit::add_all().map_err(|e| {
        println!("{}", style("Erro ao adicionar arquivos:").red().bold());
        println!("{}", style(e).red());
        e
    })?;
    println!("✔ Arquivos adicionados ao stage.");

    // Verificar se há algo para commitar
    let status = status::get_status()?;
    if !status.files.iter().any(|f| f.staged_status.is_some()) {
        println!("{}", style("Nenhuma alteração no stage para commitar.").yellow());
        return Ok(true); // Consideramos sucesso, pois não há nada a fazer.
    }

    // Obter mensagem e Commitar
    let commit_message = match prompts::get_commit_message()? {
        Some(message) if !message.trim().is_empty() => message,
        _ => {
            println!("{}", style("Commit cancelado.").yellow());
            return Ok(false); // Fluxo interrompido.
        }
    };
    commit::commit(&commit_message).map_err(|e| {
        println!("{}", style("Erro ao criar o commit:").red().bold());
        println!("{}", style(e).red());
        e
    })?;
    println!("✔ Commit criado com sucesso.");

    // Pushar
    match push::push() {
        Ok(msg) => {
            println!("{}", style("✔ Push realizado com sucesso.").green());
            if !msg.is_empty() {
                println!("{}", style(msg).dim());
            }
        }
        Err(e) => {
            println!("{}", style("Erro ao realizar o push:").red().bold());
            println!("{}", style(e).red());
            return Err(e);
        }
    }
    Ok(true) // Fluxo concluído com sucesso.
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