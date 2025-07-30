/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Módulo de Branches do Git
//
// Este módulo implementa a funcionalidade para listar, criar e mudar de
// branches em um repositório Git. A manipulação de branches é uma operação
// central no fluxo de trabalho do Git.
// ==============================================================================

use anyhow::{anyhow, Context, Result};
use std::process::Command;

/// Representa as informações sobre uma única branch.
///
/// Em vez de retornar uma simples string, usamos uma struct para fornecer
/// dados ricos e estruturados para a camada de UI, que pode então usar
/// a flag `is_current` para destacar a branch ativa.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchInfo {
    pub name: String,
    pub is_current: bool,
}

/// Lista todas as branches locais no repositório.
///
/// Executa `git branch` e analisa a saída para identificar a branch atual
/// (marcada com um `*`) e os nomes de todas as outras branches.
///
/// # Returns
/// Um `Result` contendo um vetor de `BranchInfo`, ou um `Err` se o comando falhar.
pub fn list_branches() -> Result<Vec<BranchInfo>> {
    let output = Command::new("git")
        .arg("branch")
        .output()
        .context("Falha ao executar o comando 'git branch'.")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Falha ao listar as branches: {}", stderr.trim()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut branches = Vec::new();

    // Analisamos cada linha da saída do comando.
    for line in stdout.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }

        // O Git marca a branch atual com um asterisco.
        let is_current = trimmed_line.starts_with('*');
        // Removemos o prefixo `* ` para obter o nome limpo da branch.
        let name = if is_current {
            trimmed_line.strip_prefix("* ").unwrap_or(trimmed_line).to_string()
        } else {
            trimmed_line.to_string()
        };

        branches.push(BranchInfo { name, is_current });
    }

    Ok(branches)
}

/// Cria uma nova branch local.
///
/// Executa `git branch <name>`.
///
/// # Arguments
/// * `name` - O nome da nova branch a ser criada.
///
/// # Returns
/// `Ok(())` em caso de sucesso, ou `Err` se a branch já existir ou o nome for inválido.
pub fn create_branch(name: &str) -> Result<()> {
    let trimmed_name = name.trim();
    if trimmed_name.is_empty() {
        return Err(anyhow!("O nome da branch não pode ser vazio."));
    }

    let output = Command::new("git")
        .arg("branch")
        .arg(trimmed_name)
        .output()
        .context("Falha ao executar o comando 'git branch' para criar a branch.")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "Falha ao criar a branch '{}': {}",
            trimmed_name,
            stderr.trim()
        ));
    }

    Ok(())
}

/// Muda para uma branch existente.
///
/// Executa `git checkout <name>`.
///
/// # Arguments
/// * `name` - O nome da branch para a qual mudar.
///
/// # Returns
/// `Ok(())` em caso de sucesso, ou `Err` se a branch não existir ou se houver
/// alterações não commitadas que impediriam a mudança.
pub fn switch_branch(name: &str) -> Result<()> {
    let trimmed_name = name.trim();
    if trimmed_name.is_empty() {
        return Err(anyhow!("O nome da branch não pode ser vazio."));
    }

    let output = Command::new("git")
        .arg("checkout")
        .arg(trimmed_name)
        .output()
        .context("Falha ao executar o comando 'git checkout'.")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "Falha ao mudar para a branch '{}': {}",
            trimmed_name,
            stderr.trim()
        ));
    }

    Ok(())
}