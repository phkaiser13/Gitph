/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Implementação do Módulo Nativo 'git_optim'
//
// Este arquivo contém a lógica de implementação para as funções declaradas
// na interface pública em `optim.h`.
// ==============================================================================

// Incluímos nosso próprio cabeçalho para garantir que a implementação
// corresponda à declaração. Esta é uma prática padrão em C/C++.
#include "optim.h"

// Incluímos as bibliotecas padrão do C++ que usaremos.
#include <iostream> // Para std::cout (saída padrão)
#include <cstring>  // Para strlen (cálculo de comprimento de string C)

// Note que não precisamos do bloco `extern "C"` aqui. O compilador já sabe
// que estas funções devem ter ligação C porque o cabeçalho `optim.h`, que
// já contém o `extern "C"`, foi incluído.

/**
 * @brief Implementação de `hello_from_cpp`.
 */
void hello_from_cpp() {
    // Usamos std::cout para imprimir no console. O std::endl garante que a
    // saída seja "flushed" (enviada imediatamente para o terminal).
    std::cout << "[C++] Olá do mundo C++! A ligação FFI está funcionando." << std::endl;
}

/**
 * @brief Implementação de `perform_complex_calculation`.
 */
int32_t perform_complex_calculation(int32_t input) {
    // Realiza uma operação matemática simples para demonstrar a manipulação
    // de dados passados pela FFI.
    const int32_t result = (input * 2) + 10;
    return result;
}

/**
 * @brief Implementação de `get_string_length_from_cpp`.
 */
int32_t get_string_length_from_cpp(const char* text) {
    // PASSO DE SEGURANÇA CRÍTICO: Sempre verifique ponteiros que vêm de
    // uma fronteira FFI. O código Rust pode, acidentalmente ou não, passar
    // um ponteiro nulo. Acessar um ponteiro nulo resultaria em uma falha
    // de segmentação (crash).
    if (text == nullptr) {
        // Retornamos um código de erro, conforme definido em nosso contrato de API.
        return -1;
    }

    // `strlen` da biblioteca padrão `<cstring>` é a forma canônica e eficiente
    // de calcular o comprimento de uma string no estilo C. Ele conta os
    // caracteres até encontrar o caractere nulo terminador ('\0').
    size_t length = strlen(text);

    // O tipo de retorno de `strlen` é `size_t`, que é um inteiro sem sinal.
    // Nossa API define o retorno como `int32_t`. Fazemos um cast estático
    // para converter o valor. Para comprimentos de string razoáveis, isso
    // é seguro.
    return static_cast<int32_t>(length);
}