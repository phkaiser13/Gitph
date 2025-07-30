/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Módulo de Status do Git
//
// Implementa a funcionalidade para obter e analisar o status de um
// repositório Git.
// ==============================================================================

use anyhow::{anyhow, Context, Result};
use std::process::Command;

/// Representa o tipo de mudança detectada em um arquivo.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
    TypeChanged,
    Unmerged,
    Untracked,
}

/// Representa o status de um único arquivo no repositório.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileStatus {
    pub path: String,
    pub staged_status: Option<ChangeType>,
    pub unstaged_status: Option<ChangeType>,
}

/// Contém um resumo completo do status do repositório Git.
/// Esta estrutura é o resultado da análise da saída de `git status`.
#[derive(Debug, Default)]
pub struct GitStatus {
    pub branch_info: String,
    pub files: Vec<FileStatus>,
}

/// Executa `git status` e analisa sua saída para um formato estruturado.
///
/// Esta é a função pública do módulo. Ela invoca o Git com flags específicas
/// para uma saída estável e legível por máquina (`--porcelain=v1 --branch`)
/// e, em seguida, chama um analisador interno para construir o objeto `GitStatus`.
///
/// # Returns
/// Um `Result` contendo a estrutura `GitStatus` em caso de sucesso, ou um
/// `anyhow::Error` se o comando falhar (ex: não é um repositório Git) ou se a
/// análise da saída falhar.
pub fn get_status() -> Result<GitStatus> {
    // Executa o comando `git status` com flags para saída de máquina.
    // --porcelain=v1: Formato estável e fácil de analisar.
    // --branch: Inclui informações sobre a branch atual na saída.
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain=v1")
        .arg("--branch")
        .output()
        .context("Falha ao executar o comando 'git status'. O Git está instalado e no PATH?")?;

    // Verifica se o comando foi executado com sucesso.
    if !output.status.success() {
        // Se o comando falhou, o erro geralmente está em `stderr`.
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "O comando 'git status' falhou: {}",
            stderr.trim()
        ));
    }

    // Converte a saída `stdout` para uma string para análise.
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    parse_porcelain_output(&stdout)
}

/// Analisa a saída de `git status --porcelain=v1 --branch`.
///
/// A saída tem o seguinte formato:
/// 1. Uma linha de cabeçalho de branch: `## <branch_name>...<upstream> [<ahead/behind>]`
/// 2. Linhas de status de arquivo: `XY <path>`
///    - X: Status do "index" (staged)
///    - Y: Status da "working tree" (unstaged)
fn parse_porcelain_output(output: &str) -> Result<GitStatus> {
    let mut status = GitStatus::default();
    let mut lines = output.lines();

    // A primeira linha é sempre a informação da branch.
    if let Some(branch_line) = lines.next() {
        status.branch_info = branch_line.strip_prefix("## ").unwrap_or(branch_line).to_string();
    } else {
        // Se não houver saída, o repositório está limpo e sem branch?
        // Retornamos um status vazio, o que é um estado válido.
        return Ok(status);
    }

    // Analisa as linhas de status de arquivo restantes.
    for line in lines {
        if line.len() < 4 { continue; } // Ignora linhas malformadas.

        let (code, path_part) = line.split_at(2);
        let path = path_part.trim_start();

        let staged_char = code.chars().next().unwrap();
        let unstaged_char = code.chars().nth(1).unwrap();

        let staged_status = parse_status_char(staged_char);
        let unstaged_status = parse_status_char(unstaged_char);

        // Lida com arquivos renomeados, que têm um formato especial: "R  origem -> destino"
        if staged_char == 'R' || unstaged_char == 'R' {
            if let Some(paths) = path.split_once(" -> ") {
                status.files.push(FileStatus {
                    path: format!("{} (renomeado de {})", paths.1, paths.0),
                    staged_status,
                    unstaged_status,
                });
                continue;
            }
        }

        status.files.push(FileStatus {
            path: path.to_string(),
            staged_status,
            unstaged_status,
        });
    }

    Ok(status)
}

/// Converte um único caractere de status do Git em um `ChangeType`.
fn parse_status_char(c: char) -> Option<ChangeType> {
    match c {
        'A' => Some(ChangeType::Added),
        'M' => Some(ChangeType::Modified),
        'D' => Some(ChangeType::Deleted),
        'R' => Some(ChangeType::Renamed),
        'C' => Some(ChangeType::Copied),
        'T' => Some(ChangeType::TypeChanged),
        'U' => Some(ChangeType::Unmerged),
        '?' => Some(ChangeType::Untracked),
        ' ' => None, // Espaço indica nenhuma mudança nesta área.
        _ => None,   // Caractere desconhecido.
    }
}