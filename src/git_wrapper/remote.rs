/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Módulo de Remotos do Git
//
// Este módulo fornece utilitários para interagir com a configuração de
// repositórios remotos do Git, como obter a URL do 'origin'.
// ==============================================================================

use anyhow::{anyhow, Context, Result};
use std::process::Command;

/// Obtém a URL do repositório remoto 'origin'.
///
/// Executa `git config --get remote.origin.url` para ler a URL configurada.
///
/// # Returns
/// `Ok(String)` com a URL, ou `Err` se o comando falhar ou o remoto não estiver configurado.
pub fn get_origin_url() -> Result<String> {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .output()
        .context("Falha ao executar 'git config' para obter a URL do remoto.")?;

    if !output.status.success() {
        return Err(anyhow!(
            "Não foi possível encontrar a URL do remoto 'origin'. O repositório está configurado para um remoto?"
        ));
    }

    let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(url)
}

/// Analisa uma URL de repositório Git e extrai o proprietário e o nome do repositório.
///
/// Esta função é projetada para lidar com os dois formatos mais comuns de URL do GitHub:
/// - HTTPS: `https://github.com/owner/repo.git`
/// - SSH:   `git@github.com:owner/repo.git`
///
/// # Arguments
/// * `url` - A URL do Git a ser analisada.
///
/// # Returns
/// `Ok((String, String))` contendo `(owner, repo)`, ou `Err` se a URL não
/// corresponder a um formato reconhecido.
pub fn parse_github_owner_and_repo(url: &str) -> Result<(String, String)> {
    // Tenta analisar o formato SSH primeiro.
    if let Some(ssh_path) = url.strip_prefix("git@github.com:") {
        if let Some(path) = ssh_path.strip_suffix(".git") {
            if let Some((owner, repo)) = path.split_once('/') {
                return Ok((owner.to_string(), repo.to_string()));
            }
        }
    }

    // Se não for SSH, tenta analisar o formato HTTPS.
    if let Some(https_path) = url.strip_prefix("https://github.com/") {
        if let Some(path) = https_path.strip_suffix(".git") {
            if let Some((owner, repo)) = path.split_once('/') {
                return Ok((owner.to_string(), repo.to_string()));
            }
        }
    }

    // Se nenhum dos formatos corresponder, retornamos um erro claro.
    Err(anyhow!(
        "Formato de URL do GitHub não reconhecido: '{}'.\n\
         Formatos esperados: 'https://github.com/owner/repo.git' ou 'git@github.com:owner/repo.git'",
        url
    ))
}