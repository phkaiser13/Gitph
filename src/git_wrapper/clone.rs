/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Módulo de Clone do Git
//
// Este módulo implementa a funcionalidade para clonar um repositório remoto.
// A decisão de engenharia chave aqui é fornecer feedback em tempo real ao
// usuário, em vez de um spinner genérico. Para isso, nós "escutamos" a saída
// do processo `git clone` enquanto ele está em execução.
// ==============================================================================

use anyhow::{anyhow, Context, Result};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

/// Clona um repositório a partir de uma URL.
///
/// Esta função executa `git clone <url>` e, crucialmente, captura a saída
/// de progresso em tempo real e a exibe no console. O Git escreve suas
/// informações de progresso para o `stderr`, então é este o fluxo que
/// monitoramos.
///
/// # Arguments
/// * `url` - A URL (HTTPS ou SSH) do repositório a ser clonado.
///
/// # Returns
/// `Ok(())` em caso de sucesso. Se o clone falhar, as mensagens de erro
/// do Git já terão sido impressas na tela, e a função retornará um `Err`
/// genérico indicando a falha.
pub fn clone_repository(url: &str) -> Result<()> {
    let trimmed_url = url.trim();
    if trimmed_url.is_empty() {
        return Err(anyhow!("A URL do repositório não pode ser vazia."));
    }

    println!("Clonando de '{}'...", trimmed_url);

    // --- Configuração do Comando para Streaming ---
    // Em vez de usar `.output()`, que bloqueia até o fim, usamos `.spawn()`.
    // Para capturar a saída em tempo real, precisamos redirecionar o fluxo
    // de `stderr` para um "pipe", que podemos ler em nosso programa.
    let mut child = Command::new("git")
        .arg("clone")
        .arg(trimmed_url)
        .stderr(Stdio::piped()) // Redireciona o stderr para que possamos lê-lo.
        .spawn()
        .context("Falha ao iniciar o processo 'git clone'.")?;

    // --- Leitura em Tempo Real do Stderr ---
    // `child.stderr.take()` nos dá um handle para o fluxo de erro do processo filho.
    // Envolvemos este handle em um `BufReader` para ler a saída linha por linha
    // de forma eficiente.
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            match line {
                // Imprimimos cada linha de progresso diretamente no console.
                Ok(line_content) => println!("{}", line_content),
                // Se houver um erro ao ler a linha (raro), o propagamos.
                Err(e) => return Err(anyhow!(e).context("Falha ao ler a saída do git clone.")),
            }
        }
    }

    // --- Verificação do Status Final ---
    // Após a leitura de toda a saída, esperamos o processo terminar para obter
    // seu código de saída final.
    let status = child
        .wait()
        .context("Falha ao aguardar o término do processo 'git clone'.")?;

    if !status.success() {
        // Se o processo terminou com um código de erro, nós retornamos um erro.
        // A mensagem de erro específica do Git já foi impressa na tela
        // durante o loop de leitura, então um erro genérico aqui é suficiente.
        return Err(anyhow!(
            "O comando 'git clone' falhou. Verifique a saída acima para detalhes."
        ));
    }

    println!("\nRepositório clonado com sucesso.");
    Ok(())
}