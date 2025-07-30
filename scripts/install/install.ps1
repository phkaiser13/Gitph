<#
.SYNOPSIS
    Instalador para a ferramenta 'gitph' no Windows.
.DESCRIPTION
    Este script PowerShell realiza as seguintes tarefas:
    1. Verifica se o Git está instalado e acessível no PATH.
    2. Se o Git não for encontrado, solicita ao usuário que o instale.
    3. Adiciona o diretório do executável 'gitph' ao PATH do *usuário*,
       tornando o comando 'gitph' acessível de qualquer terminal.
.NOTES
    Autor: Pedro H. Garcia (phkaiser13)
    Licença: GPL-3.0
    Requisito: Deve ser executado a partir do diretório raiz do projeto 'gitph'.
#>

# ==============================================================================
# Script de Instalação do 'gitph' para Windows (PowerShell)
# ==============================================================================

# --- Configuração Inicial ---
# Define o tratamento de erros para parar a execução em caso de falha.
$ErrorActionPreference = 'Stop'

# Define cores para as mensagens de saída para melhor legibilidade.
$ColorInfo = 'Cyan'
$ColorSuccess = 'Green'
$ColorWarning = 'Yellow'
$ColorError = 'Red'

# --- Funções Auxiliares ---

# Função para verificar se o Git está disponível.
function Test-GitInstallation {
    Write-Host "1. Verificando a instalação do Git..." -ForegroundColor $ColorInfo
    # `Get-Command` tenta encontrar o executável 'git'.
    # `-ErrorAction SilentlyContinue` impede que um erro seja exibido se não for encontrado.
    $gitPath = Get-Command git -ErrorAction SilentlyContinue
    if ($null -eq $gitPath) {
        Write-Host "AVISO: O Git não foi encontrado no seu PATH." -ForegroundColor $ColorWarning
        Write-Host "O 'gitph' depende do Git para funcionar."
        $response = Read-Host "Você gostaria de abrir a página de download do Git for Windows? (s/n)"
        if ($response -eq 's') {
            # Abre o navegador padrão na página de download.
            Start-Process "https://git-scm.com/download/win"
        }
        Write-Host "Por favor, instale o Git e execute este script novamente." -ForegroundColor $ColorWarning
        # Encerra o script, pois a dependência principal está faltando.
        exit 1
    }
    else {
        Write-Host "   ✔ Git encontrado em: $($gitPath.Source)" -ForegroundColor $ColorSuccess
    }
}

# Função para adicionar o diretório do 'gitph' ao PATH do usuário.
function Add-GitphToUserPath {
    Write-Host "2. Configurando o PATH do ambiente..." -ForegroundColor $ColorInfo

    # O diretório do executável 'gitph' é assumido como 'target/release'
    # relativo à localização deste script.
    # `$PSScriptRoot` é uma variável automática que contém o diretório do script atual.
    $installDir = Join-Path -Path $PSScriptRoot -ChildPath "..\..\target\release"
    $installDir = [System.IO.Path]::GetFullPath($installDir)

    if (-not (Test-Path $installDir\gitph.exe)) {
        Write-Host "ERRO: O executável 'gitph.exe' não foi encontrado em '$installDir'." -ForegroundColor $ColorError
        Write-Host "Por favor, compile o projeto em modo de release primeiro com: cargo build --release"
        exit 1
    }

    Write-Host "   Diretório de instalação: $installDir"

    # --- Lógica de Atualização do PATH ---
    # A melhor prática é modificar o PATH do *usuário*, não o do sistema.
    # Isso não requer elevação de administrador e é mais seguro.
    $userPathScope = [System.EnvironmentVariableTarget]::User
    $currentUserPath = [System.Environment]::GetEnvironmentVariable('Path', $userPathScope)

    if ($currentUserPath -like "*$installDir*") {
        Write-Host "   ✔ O diretório do 'gitph' já está no seu PATH." -ForegroundColor $ColorSuccess
    }
    else {
        Write-Host "   Adicionando '$installDir' ao seu PATH..."
        # Concatena o PATH existente com o novo diretório.
        $newPath = "$currentUserPath;$installDir"
        [System.Environment]::SetEnvironmentVariable('Path', $newPath, $userPathScope)
        Write-Host "   ✔ PATH do usuário atualizado com sucesso." -ForegroundColor $ColorSuccess
        Write-Host "   AVISO: Você precisará reiniciar seu terminal para que as alterações entrem em vigor." -ForegroundColor $ColorWarning
    }
}


# --- Execução Principal ---
Write-Host "Iniciando a instalação do gitph..." -ForegroundColor 'White'
Write-Host "------------------------------------"

Test-GitInstallation
Add-GitphToUserPath

Write-Host "------------------------------------"
Write-Host "Instalação do gitph concluída!" -ForegroundColor $ColorSuccess