/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Módulo de Configuração da Aplicação
//
// Este módulo gerencia todas as configurações persistentes do `gitph`.
// Ele é responsável por carregar, salvar e fornecer acesso a configurações
// como tokens de API, preferências do usuário, etc.
// ==============================================================================

use anyhow::{anyhow, Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Define a estrutura dos dados de configuração da aplicação.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    /// Token de Acesso Pessoal (PAT) para a API do GitHub.
    pub github_token: Option<String>,
}

/// Retorna o caminho para o arquivo de configuração da aplicação.
///
/// Utiliza o crate `directories` para encontrar o local apropriado para
/// arquivos de configuração, que varia entre Windows, macOS e Linux.
///
/// # Returns
/// Um `Result<PathBuf>` contendo o caminho completo para o arquivo de configuração.
/// Retorna `Err` se o diretório "home" do usuário não puder ser determinado.
//
// CORREÇÃO: Adicionamos `pub` para tornar esta função acessível a outros módulos.
pub fn get_config_path() -> Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "phkaiser13", "gitph") {
        let config_dir = proj_dirs.config_dir();
        Ok(config_dir.join("config.toml"))
    } else {
        Err(anyhow::anyhow!("Não foi possível determinar o diretório de configuração do usuário."))
    }
}

/// Carrega a configuração do arquivo no disco.
///
/// Se o arquivo de configuração não existir, uma configuração padrão (vazia)
/// é retornada sem gerar erro.
pub fn load() -> Result<Config> {
    let path = get_config_path()?;

    if path.exists() {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Falha ao ler o arquivo de configuração em {:?}", path))?;

        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Falha ao analisar o arquivo de configuração TOML em {:?}", path))?;

        Ok(config)
    } else {
        Ok(Config::default())
    }
}

/// Salva a estrutura `Config` fornecida no arquivo de configuração no disco.
///
/// Esta função irá criar o diretório de configuração se ele não existir.
pub fn save(config: &Config) -> Result<()> {
    let path = get_config_path()?;

    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir)
            .with_context(|| format!("Falha ao criar o diretório de configuração em {:?}", parent_dir))?;
    }

    let content = toml::to_string_pretty(config)
        .context("Falha ao serializar a configuração para o formato TOML.")?;

    fs::write(&path, content)
        .with_context(|| format!("Falha ao escrever no arquivo de configuração em {:?}", path))?;

    Ok(())
}