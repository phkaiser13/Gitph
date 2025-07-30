/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Cliente da API do GitHub
//
// Este módulo contém toda a lógica para interagir com a API REST do GitHub.
// Ele lida com a construção de requisições, autenticação via token e
// o tratamento de respostas de sucesso e de erro.
// ==============================================================================

use crate::config;
use anyhow::{anyhow, Context, Result};
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};

// Constantes para a API do GitHub.
const GITHUB_API_BASE_URL: &str = "https://api.github.com";
const APP_USER_AGENT: &str = "gitph-cli/0.1.0";

/// Define a estrutura do corpo (payload) JSON para a requisição de criação de Release.
/// A diretiva `#[derive(Serialize)]` instrui o `serde` a converter esta struct
/// em uma string JSON, que será o corpo da nossa requisição POST.
#[derive(Serialize)]
struct CreateReleasePayload<'a> {
    tag_name: &'a str,
    name: &'a str,
    body: &'a str,
    draft: bool,
    prerelease: bool,
}

/// Define a estrutura de uma resposta de erro da API do GitHub.
/// A diretiva `#[derive(Deserialize)]` nos permite analisar uma resposta JSON
/// de erro e extrair a mensagem de forma estruturada.
#[derive(Deserialize)]
struct GitHubApiError {
    message: String,
}

/// Cria uma nova Release no GitHub associada a uma tag existente.
///
/// # Arguments
/// * `owner` - O nome do dono do repositório (usuário ou organização).
/// * `repo` - O nome do repositório.
/// * `tag_name` - A tag que esta release irá marcar. A tag já deve existir no repositório.
/// * `release_name` - O título da release (ex: "Versão 1.0.0").
/// * `release_notes` - As notas da release, em formato Markdown.
///
/// # Returns
/// `Ok(())` em caso de sucesso, ou um `Err` detalhado em caso de falha.
pub fn create_release(
    owner: &str,
    repo: &str,
    tag_name: &str,
    release_name: &str,
    release_notes: &str,
) -> Result<()> {
    // --- PASSO 1: Obter o Token de Autenticação ---
    // Carregamos a configuração e verificamos se o token do GitHub está definido.
    // Sem um token, a API não nos permitirá criar uma release.
    let config = config::load()?;
    let token = match config.github_token {
        Some(t) => t,
        None => {
            // Este é um erro crítico de configuração. Fornecemos uma mensagem
            // clara e acionável para o usuário.
            return Err(anyhow!(
                "Token da API do GitHub não encontrado.\n\
                 Por favor, adicione seu token ao arquivo de configuração: {}\n\
                 Exemplo: github_token = \"seu_token_aqui\"",
                config::get_config_path()?.display()
            ));
        }
    };

    // --- PASSO 2: Construir o Payload da Requisição ---
    let payload = CreateReleasePayload {
        tag_name,
        name: release_name,
        body: release_notes,
        draft: false,      // Publica a release imediatamente.
        prerelease: false, // Marca como uma release de produção estável.
    };

    // --- PASSO 3: Construir e Enviar a Requisição HTTP ---
    let client = Client::new();
    let url = format!("{}/repos/{}/{}/releases", GITHUB_API_BASE_URL, owner, repo);

    let response = client
        .post(&url)
        // Definimos os cabeçalhos HTTP necessários.
        .header(
            AUTHORIZATION,
            format!("Bearer {}", token), // Autenticação via Bearer Token.
        )
        .header(
            ACCEPT,
            "application/vnd.github+json", // Versão recomendada da API.
        )
        .header(
            USER_AGENT,
            APP_USER_AGENT, // Muitas APIs exigem um User-Agent.
        )
        .json(&payload) // Serializa nosso `payload` para JSON e define o Content-Type.
        .send()
        .context("Falha ao enviar a requisição para a API do GitHub.")?;

    // --- PASSO 4: Processar a Resposta ---
    if response.status().is_success() {
        // Um status 201 Created indica que a release foi criada com sucesso.
        Ok(())
    } else {
        // Se a API retornou um erro, tentamos analisar a mensagem de erro
        // que o GitHub nos enviou no corpo da resposta.
        let status = response.status();
        let error_body: Result<GitHubApiError, _> = response.json();

        let error_message = match error_body {
            Ok(api_error) => api_error.message,
            Err(_) => format!("A API retornou um erro {} mas não foi possível analisar a mensagem.", status),
        };

        Err(anyhow!(
            "Falha ao criar a release no GitHub (Status: {}):\n{}",
            status,
            error_message
        ))
    }
}