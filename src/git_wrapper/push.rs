/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Módulo de Push do Git
//
// Este módulo é responsável por sincronizar os commits locais com o
// repositório remoto através do comando `git push`.
//
// Por ser uma operação de rede, ela pode ser lenta e suscetível a uma
// variedade de erros (falta de conexão, conflitos, permissões, etc.).
// Portanto, o feedback ao usuário e o tratamento de erros detalhado são
// as principais prioridades aqui.
// ==============================================================================

use anyhow::{anyhow, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;
use std::time::Duration;

/// Envia os commits locais para o repositório remoto configurado.
///
/// Esta função executa o comando `git push`. Ela assume que a branch atual
/// já está configurada para rastrear uma branch remota (upstream).
///
/// Durante a execução, um spinner é exibido para indicar ao usuário que uma
/// operação de rede está em andamento.
///
/// # Returns
/// Um `Result<String>`:
/// - `Ok(String)`: Em caso de sucesso, contém a mensagem de saída do Git,
///   que geralmente inclui um resumo das atualizações.
/// - `Err(anyhow::Error)`: Se o comando falhar. O erro conterá a mensagem
///   detalhada do `stderr` do Git, explicando o motivo da falha (ex: o
///   remoto contém trabalho que você não tem localmente).
pub fn push() -> Result<String> {
    // --- Preparação do Feedback Visual ---
    // Uma operação de 'push' pode ser demorada. Para evitar que o usuário
    // pense que a aplicação travou, criamos um spinner de progresso.
    let spinner = ProgressBar::new_spinner();

    // Definimos o estilo do nosso spinner. O template `{spinner:.green}` usa
    // um conjunto de caracteres giratórios (como '-', '\', '|', '/') na cor verde.
    // O `{msg}` é um placeholder para a mensagem de texto que definiremos.
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            // Se a linha acima falhar (o que é raro), o unwrap vai causar um panic,
            // o que é aceitável para uma falha de configuração interna.
            .unwrap(),
    );

    spinner.set_message("Enviando commits para o repositório remoto...");

    // Ativamos o spinner para que ele comece a animar na tela.
    // O `Duration` define a velocidade da animação.
    spinner.enable_steady_tick(Duration::from_millis(100));

    // --- Execução do Comando ---
    // Executamos `git push` e capturamos sua saída.
    let output = Command::new("git")
        .arg("push")
        .output()
        .context("Falha ao executar o comando 'git push'.")?;

    // --- Finalização do Feedback Visual ---
    // Independentemente do resultado, o spinner cumpriu sua função.
    // Nós o removemos da tela para dar lugar à mensagem de resultado.
    spinner.finish_and_clear();

    // --- Processamento do Resultado ---
    if output.status.success() {
        // SUCESSO: O push foi aceito pelo remoto.
        // É uma peculiaridade importante do `git push` que ele frequentemente
        // escreve informações de sucesso no `stderr` em vez do `stdout`.
        // Portanto, retornamos o conteúdo do `stderr` como a mensagem de sucesso.
        let success_message = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Ok(success_message)
    } else {
        // FALHA: O push foi rejeitado ou ocorreu outro erro.
        // A razão exata da falha estará no `stderr`. Capturamos essa mensagem
        // para fornecer um erro claro e acionável ao usuário.
        let error_message = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(anyhow!(
            "O comando 'git push' falhou:\n\n{}",
            error_message
        ))
    }
}