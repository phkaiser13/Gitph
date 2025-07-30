/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Interface Pública para o Módulo Nativo 'git_optim'
//
// Este arquivo de cabeçalho define o "contrato" da nossa biblioteca C++.
// As funções aqui declaradas são a face pública do módulo, expostas para
// serem consumidas por outras partes do sistema, como nosso código Rust.
// ==============================================================================

#pragma once

#include <cstdint> // Para tipos de inteiros de tamanho fixo como `int32_t`.

// O bloco `extern "C"` é a pedra angular da interoperabilidade (FFI).
// Ele instrui o compilador C++ a não aplicar o "name mangling" do C++
// aos nomes das funções, em vez disso, exportá-las usando a convenção de
// chamada C, que é estável e compreendida por outras linguagens como o Rust.
extern "C" {

/**
 * @brief Imprime uma mensagem de diagnóstico para verificar a ligação FFI.
 *
 * Esta função serve como um "ping" para confirmar que o código Rust pode
 * chamar com sucesso uma função na biblioteca C++ compilada.
 */
void hello_from_cpp();

/**
 * @brief Executa um cálculo simples para demonstrar a passagem de dados.
 *
 * @param input Um inteiro de 32 bits assinado, passado pelo chamador (Rust).
 * @return O resultado de uma operação matemática sobre a entrada.
 */
int32_t perform_complex_calculation(int32_t input);

/**
 * @brief Calcula o comprimento de uma string no estilo C (terminada em nulo).
 *
 * @param text Um ponteiro para o primeiro caractere de uma string UTF-8
 *             terminada com o caractere nulo ('\0').
 *
 * @return O número de bytes na string (sem incluir o terminador nulo).
 *         Retorna -1 se o ponteiro de entrada for um ponteiro nulo.
 *
 * @safety O chamador (Rust) é 100% responsável por garantir que `text`
 *         aponte para um bloco de memória válido e que a string seja
 *         corretamente terminada em nulo. Passar um ponteiro inválido
 *         resultará em comportamento indefinido (provavelmente um crash).
 */
int32_t get_string_length_from_cpp(const char* text);

} // Fim do bloco extern "C"