/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Módulo de Prompts Interativos
//
// Este módulo contém funções para solicitar entradas específicas do usuário,
// como texto, senhas, e confirmações (sim/não). Ele abstrai a complexidade
// da biblioteca `dialoguer` para fornecer uma API simples e focada em tarefas.
// ==============================================================================

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input};

/// Solicita ao usuário uma mensagem de commit.
///
/// Apresenta um prompt de entrada de texto. A função lida com vários casos:
/// - O usuário digita uma mensagem e pressiona Enter.
/// - O usuário não digita nada e pressiona Enter (permitido, mas pode ser validado).
/// - O usuário cancela a operação (pressionando Esc).
///
/// # Returns
/// - `Ok(Some(String))` se o usuário fornecer uma mensagem.
/// - `Ok(None)` se o usuário cancelar a operação.
/// - `Err` se houver um problema ao interagir com o terminal.
pub fn get_commit_message() -> Result<Option<String>> {
    // Cria um prompt de entrada de texto com um tema colorido.
    let input = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Digite a mensagem de commit (pressione ESC para cancelar)")
        // Permite que o usuário submeta uma entrada vazia. A lógica de negócio
        // decidirá se um commit vazio é permitido (geralmente não é, mas a
        // captura da intenção é separada da validação).
        .allow_empty(true)
        // Exibe o prompt e aguarda a interação do usuário.
        .interact_text_on_opt(&Term::stdout())?;

    // `interact_text_on_opt` retorna `Option<String>`, que se alinha perfeitamente
    // com nossa assinatura de função. `None` indica que o usuário cancelou.
    Ok(input)
}

/// Solicita uma confirmação (sim/não) do usuário.
///
/// # Arguments
/// * `prompt` - A pergunta a ser feita ao usuário.
///
* `default_val` - O valor padrão se o usuário apenas pressionar Enter.
///
/// # Returns
/// - `Ok(true)` se o usuário confirmar.
/// - `Ok(false)` se o usuário negar ou cancelar.
/// - `Err` se houver um problema com o terminal.
pub fn confirm(prompt: &str, default_val: bool) -> Result<bool> {
    let confirmation = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default_val)
        .interact_on_opt(&Term::stdout())?
        .unwrap_or(false); // Se o usuário cancelar (Esc), consideramos como 'não'.

    Ok(confirmation)
}