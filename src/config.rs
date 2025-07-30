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
//
// A configuração é armazenada em um arquivo `config.toml` em um diretório
// padrão do sistema operacional, garantindo uma integração limpa com o ambiente
// do usuário.
// ==============================================================================

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Define a estrutura dos dados de configuração da aplicação.
///
/// A diretiva `#[derive(Serialize, Deserialize)]` do `serde` gera automaticamente
/// o código necessário para converter esta struct de/para o formato TOML.
/// `#[derive(Default)]` nos permite criar uma instância padrão facilmente.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    /// Token de Acesso Pessoal (PAT) para a API do GitHub.
    pub github_token: Option<String>,
}

/// Retorna o caminho para o arquivo de configuração da aplicação.
///
/// Utiliza o crate `directories` para encontrar o local apropriado para
/// arquivos de configuração, que varia entre Windows, macOS e Linux.
/// Ex: `~/.config/gitph/config.toml` no Linux.
///
/// # Returns
/// Um `Result<PathBuf>` contendo o caminho completo para o arquivo de configuração.
/// Retorna `Err` se o diretório "home" do usuário não puder ser determinado.
fn get_config_path() -> Result<PathBuf> {
    // `ProjectDirs::from` cria um conjunto de caminhos padrão para o projeto.
    // Os qualificadores são "com", "phkaiser13", "gitph".
    if let Some(proj_dirs) = ProjectDirs::from("com", "phkaiser13", "gitph") {
        let config_dir = proj_dirs.config_dir();
        Ok(config_dir.join("config.toml"))
    } else {
        // Este erro ocorre em cenários muito raros onde o sistema operacional
        // não consegue fornecer um diretório "home".
        Err(anyhow::anyhow!("Não foi possível determinar o diretório de configuração do usuário."))
    }
}

/// Carrega a configuração do arquivo no disco.
///
/// Se o arquivo de configuração não existir, uma configuração padrão (vazia)
/// é retornada sem gerar erro. Isso permite que a aplicação funcione na
// primeira execução sem um arquivo de configuração pré-existente.
///
/// # Returns
/// Um `Result<Config>` com a configuração carregada ou padrão.
pub fn load() -> Result<Config> {
    let path = get_config_path()?;

    // Verifica se o arquivo existe.
    if path.exists() {
        // Se existe, lê seu conteúdo para uma string.
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Falha ao ler o arquivo de configuração em {:?}", path))?;

        // Analisa (parse) a string TOML para a nossa struct `Config`.
        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Falha ao analisar o arquivo de configuração TOML em {:?}", path))?;

        Ok(config)
    } else {
        // Se não existe, retorna a configuração padrão.
        Ok(Config::default())
    }
}

/// Salva a estrutura `Config` fornecida no arquivo de configuração no disco.
///
/// Esta função irá criar o diretório de configuração se ele não existir.
///
/// # Arguments
/// * `config` - Uma referência à struct `Config` a ser salva.
///
/// # Returns
/// Um `Result<()>` que indica sucesso ou falha na operação de escrita.
pub fn save(config: &Config) -> Result<()> {
    let path = get_config_path()?;

    // Garante que o diretório pai do arquivo de configuração exista.
    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir)
            .with_context(|| format!("Falha ao criar o diretório de configuração em {:?}", parent_dir))?;
    }

    // Serializa a struct `Config` de volta para uma string no formato TOML.
    let content = toml::to_string_pretty(config)
        .context("Falha ao serializar a configuração para o formato TOML.")?;

    // Escreve a string no arquivo.
    fs::write(&path, content)
        .with_context(|| format!("Falha ao escrever no arquivo de configuração em {:?}", path))?;

    Ok(())
}