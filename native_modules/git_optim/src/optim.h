/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-2.0
 * This file is licensed under the GNU General Public License v2.0.
 */

 // ==============================================================================
 // Interface Pública para o Módulo Nativo 'git_optim'
 //
 // Este arquivo de cabeçalho define a API (Application Programming Interface)
 // do nosso módulo C++. As funções declaradas aqui serão implementadas em
 // `optim.cpp` e expostas para serem chamadas a partir do código Rust.
 // ==============================================================================

 // `#pragma once` é uma diretiva de pré-processador que garante que este arquivo
 // seja incluído apenas uma vez durante a compilação, evitando erros de
 // redefinição. É uma alternativa moderna e mais simples aos include guards
 // tradicionais (#ifndef/#define/#endif).
 #pragma once

 // Para garantir a interoperabilidade com outras linguagens (como Rust),
 // precisamos usar tipos de dados que tenham um tamanho e representação
 // bem definidos. `cstdint` nos dá acesso a tipos como `int32_t`.
 #include <cstdint>

 // O bloco `extern "C"` é ESSENCIAL para a FFI (Foreign Function Interface).
 // Ele instrui o compilador C++ a exportar estas funções com uma convenção de
 // chamada C, que possui um ABI (Application Binary Interface) estável e
 // previsível. Sem isso, o C++ aplicaria "name mangling" aos nomes das funções,
 // tornando impossível para o Rust encontrá-las durante a lincagem.
 extern "C" {

 /**
  * @brief Uma função de teste simples para verificar se a ligação FFI funciona.
  *
  * Esta função não recebe argumentos e não retorna valor. Seu único propósito
  * é imprimir uma mensagem no console a partir do código C++, confirmando que
  * a chamada do Rust para o C++ foi bem-sucedida.
  */
 void hello_from_cpp();

 /**
  * @brief Realiza um cálculo de exemplo para demonstrar a passagem de dados.
  *
  * Esta função demonstra a passagem de um tipo primitivo (um inteiro) do Rust
  * para o C++, a execução de uma operação e o retorno do resultado para o Rust.
  *
  * @param input Um inteiro de 32 bits assinado vindo do chamador (Rust).
  * @return O valor de entrada multiplicado por 2 e somado com 10.
  */
 int32_t perform_complex_calculation(int32_t input);

 /**
  * @brief Calcula o comprimento de uma string codificada em UTF-8.
  *
  * Demonstra a passagem de um ponteiro de dados (uma string no estilo C) do
  * Rust para o C++.
  *
  * @param text Um ponteiro para o início de uma string de caracteres terminada
  *             com nulo (null-terminated C-style string).
  *
  * @return O número de bytes na string (excluindo o terminador nulo). Retorna
  *         -1 se o ponteiro de entrada for nulo.
  *
  * @note FFI Safety: O chamador (Rust) é responsável por garantir que `text`
  *       seja um ponteiro válido para uma string null-terminated. Passar um
  *       ponteiro inválido ou não-terminado resultará em comportamento indefinido.
  */
 int32_t get_string_length_from_cpp(const char* text);

 } // extern "C"