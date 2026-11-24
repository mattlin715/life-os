param(
  [Parameter(Mandatory = $true)][string]$JavaHome,
  [Parameter(Mandatory = $true)][string]$AndroidJar,
  [Parameter(Mandatory = $true)][string[]]$TauriClasses,
  [Parameter(Mandatory = $true)][string[]]$KotlinLibraries,
  [Parameter(Mandatory = $true)][string]$OutputRoot
)
$ErrorActionPreference = 'Stop'
# Host compilation only. Never finds an SDK, device, credential, or secret on its own.
$root = Split-Path -Parent $PSScriptRoot
$out = [IO.Path]::GetFullPath($OutputRoot)
if (Test-Path -LiteralPath $out) { throw 'Host test output must be a new directory; do not clean or overwrite an earlier run.' }
$javac = Join-Path $JavaHome 'bin\javac.exe'
$java = Join-Path $JavaHome 'bin\java.exe'
foreach ($input in @($javac, $java, $AndroidJar) + $TauriClasses + $KotlinLibraries) {
  if (-not (Test-Path -LiteralPath $input)) { throw "Missing explicit compile input: $input" }
}
New-Item -ItemType Directory -Path (Join-Path $out 'classes') -Force | Out-Null
$classes = Join-Path $out 'classes'
$src = Join-Path $root 'src-tauri\android-pilot-online\java\com\lifeos\pilot\credentials'
$test = Join-Path $root 'src-tauri\tests\android_credentials\CredentialStoreHostTest.java'
& $javac -encoding UTF-8 -d $classes (Join-Path $src 'CredentialStore.java') $test 2>&1 |
  Tee-Object -FilePath (Join-Path $out 'core-compile.log')
if ($LASTEXITCODE -ne 0) { throw 'Credential core did not compile.' }
& $java -cp $classes com.lifeos.pilot.credentials.CredentialStoreHostTest 2>&1 |
  Tee-Object -FilePath (Join-Path $out 'host-core.log')
if ($LASTEXITCODE -ne 0) { throw 'Credential lifecycle host checks failed.' }
$classpath = (@($classes, $AndroidJar) + $TauriClasses + $KotlinLibraries) -join ';'
& $javac -encoding UTF-8 -cp $classpath -d $classes (Join-Path $src 'AndroidCredentialBackend.java') (Join-Path $src 'PilotCredentialsPlugin.java') 2>&1 |
  Tee-Object -FilePath (Join-Path $out 'adapter-compile.log')
if ($LASTEXITCODE -ne 0) { throw 'Android API bridge did not compile.' }
'ANDROID_CREDENTIAL_HOST=PASS; injected-backend tests and Java API compilation only; no Android Keystore/device proof.'
