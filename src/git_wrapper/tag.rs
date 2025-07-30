/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Módulo de Tags do Git
//
// Este módulo implementa a funcionalidade para criar e sincronizar tags Git.
// Tags são usadas para marcar pontos específicos e importantes na história do
// projeto, como o lançamento de uma nova versão.
// ==============================================================================

use anyhow::{anyhow, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;
use std::time::Duration;

/// Cria uma tag Git anotada localmente.
///
/// Tags anotadas são preferíveis para releases, pois são objetos completos no
/// banco de dados do Git que armazenam o autor, email, data e uma mensagem.
///
/// # Arguments
/// * `tag_name` - O nome da tag (ex: "v1.0.0").
/// * `message` - A mensagem de anotação para a tag.
///
/// # Returns
/// `Ok(())` em caso de sucesso, ou um `Err` se a tag já existir ou outro
/// erro do Git ocorrer.
pub fn create_annotated_tag(tag_name: &str, message: &str) -> Result<()> {
    // Realizamos uma validação de entrada para garantir que não estamos
    // tentando criar uma tag com nome ou mensagem vazios.
    if tag_name.trim().is_empty() {
        return Err(anyhow!("O nome da tag não pode ser vazio."));
    }
    if message.trim().is_empty() {
        return Err(anyhow!("A mensagem de anotação da tag não pode ser vazia."));
    }

    // Construímos o comando `git tag -a <nome> -m <mensagem>`.
    // O flag `-a` especifica que queremos uma tag anotada.
    let output = Command::new("git")
        .arg("tag")
        .arg("-a")
        .arg(tag_name)
        .arg("-m")
        .arg(message)
        .output()
        .context("Falha ao executar o comando 'git tag'.")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "Falha ao criar a tag '{}': {}",
            tag_name,
            stderr.trim()
        ));
    }

    Ok(())
}

/// Envia uma tag específica para o repositório remoto 'origin'.
///
/// O envio de tags é uma operação separada do `git push` normal.
///
/// # Arguments
/// * `tag_name` - O nome da tag a ser enviada.
///
/// # Returns
/// `Ok(String)` com a mensagem de sucesso do servidor, ou `Err` se o push
/// da tag falhar.
pub fn push_tag(tag_name: &str) -> Result<String> {
    if tag_name.trim().is_empty() {
        return Err(anyhow!("O nome da tag a ser enviada não pode ser vazio."));
    }

    // Mantemos a consistência da experiência do usuário. Como esta é uma
    // operação de rede, exibimos um spinner.
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message(format!("Enviando tag '{}' para o remoto...", tag_name));
    spinner.enable_steady_tick(Duration::from_millis(100));

    // O comando para enviar uma única tag é `git push origin <nome_da_tag>`.
    let output = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg(tag_name)
        .output()
        .context("Falha ao executar o comando 'git push' para a tag.")?;

    spinner.finish_and_clear();

    if output.status.success() {
        // Assim como no push normal, a mensagem de sucesso geralmente está no stderr.
        let success_message = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Ok(success_message)
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(anyhow!(
            "Falha ao enviar a tag '{}' para o remoto:\n\n{}",
            tag_name,
            error_message
        ))
    }
}