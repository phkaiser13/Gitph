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
use console::Term;
use dialoguer::{theme::ColorfulTheme, Editor, Input, Confirm}; // Adicionamos o `Editor` e `Confirm`

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
    let input = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Digite a mensagem de commit (pressione ESC para cancelar)")
        .allow_empty(true)
        .interact_text_on_opt(&Term::stdout())?;
    Ok(input)
}

/// Solicita uma confirmação (sim/não) do usuário.
///
/// # Arguments
/// * `prompt` - A pergunta a ser feita ao usuário.
/// * `default_val` - O valor padrão se o usuário apenas pressionar Enter.
///
/// # Returns
/// - `Ok(true)` se o usuário confirmar.
/// - `Ok(false)` se o usuário negar ou cancelar.
/// - `Err` se houver um problema com o terminal.
pub fn confirm(prompt: &str, default_val: bool) -> Result<bool> {
    let confirmation = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default_val)
        .interact_on_opt(&Term::stdout())?
        .unwrap_or(false); // Se o usuário cancelar (Esc), consideramos como 'não'.

    Ok(confirmation)
}

/// Abre o editor de texto padrão do sistema para obter uma entrada multi-linha.
///
/// Esta abordagem é ideal para textos longos, como notas de release, pois
/// oferece uma experiência de edição muito superior a um prompt de linha única.
///
/// # Returns
/// - `Ok(Some(String))` se o usuário salvar o conteúdo no editor.
/// - `Ok(None)` se o usuário sair do editor sem salvar (ou se o arquivo ficar vazio).
/// - `Err` se o editor não puder ser aberto.
pub fn get_release_notes() -> Result<Option<String>> {
    println!("{}", console::style("Abrindo seu editor de texto padrão para as notas da release...").dim());
    println!("{}", console::style("Dica: Salve e feche o arquivo para continuar, ou feche sem salvar para cancelar.").dim());

    // `Editor::new()` cria uma instância do prompt do editor.
    let response = Editor::new()
        // O texto a seguir será pré-preenchido no arquivo temporário que o editor abrir.
        // Isso serve como um template útil para o usuário.
        .edit("## Novidades\n\n\n## Correções\n\n\n## Melhorias\n\n")?
        ;

    // `edit()` retorna `Ok(Option<String>)`. `None` significa que o usuário
    // não salvou nada, o que tratamos como um cancelamento.
    Ok(response)
}