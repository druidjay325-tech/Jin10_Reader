param([string]$NpmCli = '', [switch]$Offline)

$ErrorActionPreference = 'Stop'
$readerRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$readerOldEncoded = $env:CARGO_ENCODED_RUSTFLAGS
$readerOldFlags = $env:RUSTFLAGS
$readerOldAwsFlags = $env:AWS_LC_SYS_CFLAGS
$readerOldShellFlags = $env:CC_SHELL_ESCAPED_FLAGS
$readerFlags = @()
if ($readerOldEncoded) {
    $readerFlags += $readerOldEncoded.Split([char]31)
} elseif ($readerOldFlags) {
    throw 'Use CARGO_ENCODED_RUSTFLAGS for additional flags, so paths containing spaces remain intact.'
}
# Remap home and project paths in compiler diagnostics embedded in the binary.
if ($env:USERPROFILE) { $readerFlags += "--remap-path-prefix=$env:USERPROFILE=/build-user" }
$readerFlags += "--remap-path-prefix=$readerRoot=/jin10-reader"
$env:CARGO_ENCODED_RUSTFLAGS = $readerFlags -join [char]31
# AWS-LC contains C __FILE__ strings, outside Rust's path remapping.
if ($env:USERPROFILE) {
    $readerProfilePath = $env:USERPROFILE.Replace('\', '/')
    $env:AWS_LC_SYS_CFLAGS = ($readerOldAwsFlags + ' /experimental:deterministic /pathmap:"' + $readerProfilePath + '=/build-user"').Trim()
    $env:CC_SHELL_ESCAPED_FLAGS = '1'
}
$readerArgs = @('build', '--bundles', 'nsis', '--', '--locked')
if ($Offline) { $readerArgs += '--offline' }
Push-Location $readerRoot
try {
    if ($NpmCli) { & node $NpmCli @readerArgs }
    else { & cargo tauri @readerArgs }
    if ($LASTEXITCODE -ne 0) { throw "Tauri build failed ($LASTEXITCODE)." }
    & node tools/check-binary.mjs target/release/jin10-reader.exe
    if ($LASTEXITCODE -ne 0) { throw 'Release executable did not pass the binary boundary check.' }
    foreach ($readerNativeDll in Get-ChildItem -LiteralPath 'resources/acrylic' -Filter '*.dll' -File) {
        & node tools/check-binary.mjs $readerNativeDll.FullName
        if ($LASTEXITCODE -ne 0) { throw 'Native material resource did not pass the binary boundary check.' }
    }
    foreach ($readerInstaller in Get-ChildItem -LiteralPath 'target/release/bundle/nsis' -Filter '*.exe' -File) {
        & node tools/check-binary.mjs $readerInstaller.FullName
        if ($LASTEXITCODE -ne 0) { throw 'Installer did not pass the binary boundary check.' }
    }
} finally {
    Pop-Location
    $env:CARGO_ENCODED_RUSTFLAGS = $readerOldEncoded
    $env:RUSTFLAGS = $readerOldFlags
    $env:AWS_LC_SYS_CFLAGS = $readerOldAwsFlags
    $env:CC_SHELL_ESCAPED_FLAGS = $readerOldShellFlags
}
