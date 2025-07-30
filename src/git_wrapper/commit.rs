/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Módulo de Commit do Git
//
// Implementa a funcionalidade para adicionar arquivos ao stage e para criar
// commits no repositório local.
// ==============================================================================

use anyhow::{anyhow, Context, Result};
use std::process::Command;

/// Adiciona todas as alterações no diretório de trabalho ao stage do Git.
///
/// Executa o comando `git add .`. Este comando prepara todas as alterações
/// (arquivos novos, modificados e deletados) para serem incluídos no
/// próximo commit.
///
/// # Returns
/// Um `Result<()>` que é `Ok` se o comando for bem-sucedido, ou `Err` se
/// o comando `git add` falhar.
pub fn add_all() -> Result<()> {
    let output = Command::new("git")
        .arg("add")
        .arg(".") // O ponto representa "tudo no diretório atual e subdiretórios"
        .output()
        .context("Falha ao executar o comando 'git add'.")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "O comando 'git add .' falhou: {}",
            stderr.trim()
        ));
    }

    Ok(())
}

/// Cria um novo commit com a mensagem fornecida.
///
/// Executa o comando `git commit -m "<mensagem>"`.
///
/// # Arguments
/// * `message` - A mensagem de commit a ser usada.
///
/// # Returns
/// Um `Result<()>` que é `Ok` se o commit for criado com sucesso. Retorna `Err`
/// se o comando `git commit` falhar, por exemplo, se não houver nada no stage
/// para commitar, ou se a configuração do Git (user.name, user.email)
/// não estiver definida.
pub fn commit(message: &str) -> Result<()> {
    // Validação de entrada: uma mensagem de commit não pode ser vazia.
    if message.trim().is_empty() {
        return Err(anyhow!("A mensagem de commit não pode ser vazia."));
    }

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .output()
        .context("Falha ao executar o comando 'git commit'.")?;

    if !output.status.success() {
        // Captura tanto stdout quanto stderr, pois `git commit` pode escrever
        // mensagens informativas de erro em ambos os canais.
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let error_message = format!("{}\n{}", stdout.trim(), stderr.trim()).trim().to_string();

        return Err(anyhow!(
            "O comando 'git commit' falhou: {}",
            error_message
        ));
    }

    Ok(())
}