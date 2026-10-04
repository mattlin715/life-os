[CmdletBinding()]
param([ValidatePattern('^[0-9a-z]{8,24}$')][string]$RunId = ([DateTime]::UtcNow.ToString('yyyyMMddHHmmss')), [switch]$ResumeFinal, [ValidateSet('a','b','c')][string]$SupplementId = 'a')
$ErrorActionPreference = 'Stop'
$repositoryRoot = Split-Path -Parent $PSScriptRoot
$sdk = Join-Path $repositoryRoot '.tools\android-sdk'
$adb = Join-Path $sdk 'platform-tools\adb.exe'
$sqlite = Join-Path $sdk 'platform-tools\sqlite3.exe'
$emulator = Join-Path $sdk 'emulator\emulator.exe'
$avdManager = Join-Path $sdk 'cmdline-tools\latest\bin\avdmanager.bat'
$avdHome = Join-Path $repositoryRoot '.artifacts\android-avd'
$artifact = Join-Path $repositoryRoot ".artifacts\android-m2c\native-review\$RunId"
$apk = Join-Path $repositoryRoot '.artifacts\android-m2c\review.apk'
$avdName = "lifeos_m2c_${RunId}_api36_x86_64"
$serial = 'emulator-5588'; $port = 5588; $cdpPort = 9228
$package = 'com.lifeos.review.m2c'
$env:ANDROID_AVD_HOME = $avdHome
$env:ANDROID_HOME = $sdk; $env:ANDROID_SDK_ROOT = $sdk
$env:JAVA_HOME = Join-Path $repositoryRoot '.tools\microsoft-jdk-21.0.12.1\jdk-21.0.12.1+1'
$database = $null; $receipt = $null; $owned = $false
$results = [Collections.Generic.List[string]]::new()
$process = $null
function Hash([string]$Path) { return (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant() }
function Adb([string[]]$Arguments, [switch]$AllowFailure) {
  $old = $ErrorActionPreference; $ErrorActionPreference = 'Continue'
  try { $output = @(& $adb -s $serial @Arguments 2>&1); $code = $LASTEXITCODE } finally { $ErrorActionPreference = $old }
  if (-not $AllowFailure -and $code -ne 0) { throw 'Exact bound adb command failed; output suppressed and fixture preserved.' }
  return $output
}
function Bind {
  $name = ((Adb @('emu','avd','name')) | Where-Object { $_ -and $_ -ne 'OK' } | Select-Object -First 1).Trim()
  if ($name -ne $avdName) { throw 'Exact emulator identity mismatch.' }
}
function Start-Emulator {
  $script:process = Start-Process -FilePath $emulator -ArgumentList @('-avd',$avdName,'-port',"$port",'-no-snapshot','-no-audio','-no-boot-anim','-no-window','-gpu','swiftshader_indirect') -WindowStyle Hidden -PassThru
  $end = [DateTime]::UtcNow.AddMinutes(4)
  do { Start-Sleep -Seconds 2; $boot = ((Adb @('shell','getprop','sys.boot_completed') -AllowFailure) -join '').Trim(); if ($boot -eq '1') { Bind; return } } while ([DateTime]::UtcNow -lt $end)
  throw 'Owned disposable emulator did not boot.'
}
function Stop-Emulator {
  Bind; Adb @('emu','kill') | Out-Null
  $end = [DateTime]::UtcNow.AddSeconds(60)
  do { Start-Sleep -Seconds 1; $devices = @(& $adb devices | Where-Object { $_ -match "^$serial\s" }); if ($devices.Count -eq 0) { return } } while ([DateTime]::UtcNow -lt $end)
  throw 'Owned emulator shutdown incomplete.'
}
function Stop-App { Bind; Adb @('shell','am','force-stop',$package) | Out-Null }
function Start-App {
  Bind; Adb @('shell','am','start','-W','-n',"$package/.MainActivity") | Out-Null
  $end = [DateTime]::UtcNow.AddSeconds(30)
  do { Start-Sleep -Milliseconds 300; $appPid = ((Adb @('shell','pidof',$package) -AllowFailure) -join '').Trim() } while (-not $appPid -and [DateTime]::UtcNow -lt $end)
  if (-not $appPid) { throw 'M2-C process did not start.' }
  Adb @('forward','--remove',"tcp:$cdpPort") -AllowFailure | Out-Null
  Adb @('forward',"tcp:$cdpPort","localabstract:webview_devtools_remote_$appPid") | Out-Null
}
function Probe([string[]]$Arguments) {
  & node scripts/android-m2c-cdp-probe.mjs --port "$cdpPort" @Arguments
  if ($LASTEXITCODE -ne 0) { throw 'Bounded native probe failed; preserve exact evidence.' }
}
function Export-File([string]$Relative, [string]$Local) {
  if ($Relative -notmatch '^\./[A-Za-z0-9_.\-/]+$' -or $Relative.Contains('..')) { throw 'Unsafe app-private source path.' }
  $info = New-Object Diagnostics.ProcessStartInfo; $info.FileName = $adb
  $info.Arguments = "-s $serial exec-out run-as $package cat $Relative"
  $info.UseShellExecute = $false; $info.RedirectStandardOutput = $true; $info.RedirectStandardError = $true
  $p = New-Object Diagnostics.Process; $p.StartInfo = $info; [void]$p.Start()
  $stream = [IO.File]::Create($Local); try { $p.StandardOutput.BaseStream.CopyTo($stream) } finally { $stream.Dispose() }
  $ignored = $p.StandardError.ReadToEnd(); $p.WaitForExit(); if ($p.ExitCode -ne 0) { throw 'App-private export failed (content suppressed).' }
}
function Screenshot([string]$Label) {
  Bind
  $info = New-Object Diagnostics.ProcessStartInfo; $info.FileName = $adb
  $info.Arguments = "-s $serial exec-out screencap -p"
  $info.UseShellExecute = $false; $info.RedirectStandardOutput = $true; $info.RedirectStandardError = $true
  $p = New-Object Diagnostics.Process; $p.StartInfo = $info; [void]$p.Start()
  $stream = [IO.File]::Create((Join-Path $artifact "$Label.png")); try { $p.StandardOutput.BaseStream.CopyTo($stream) } finally { $stream.Dispose() }
  $ignored = $p.StandardError.ReadToEnd(); $p.WaitForExit(); if ($p.ExitCode -ne 0) { throw 'Owned screenshot failed.' }
}
function Locate {
  $dbs = @((Adb @('shell','run-as',$package,'find','.','-name','android-m2c-disposable-v5.db','-print')) | Where-Object { $_ })
  $receipts = @((Adb @('shell','run-as',$package,'find','.','-name','android-m2c-direct-fresh-v5.receipt.json','-print')) | Where-Object { $_ })
  if ($dbs.Count -ne 1 -or $receipts.Count -ne 1) { throw 'Owned database/receipt path inventory mismatch.' }
  $script:database = $dbs[0].Trim(); $script:receipt = $receipts[0].Trim()
}
function Capture([string]$Label) {
  Export-File $database (Join-Path $artifact "$Label.db")
  Export-File $receipt (Join-Path $artifact "$Label.receipt.json")
}
function Sql([string]$Path, [string]$Query) {
  $value = (& $sqlite $Path $Query) -join "`n"; if ($LASTEXITCODE -ne 0) { throw 'Owned synthetic fixture SQL check failed.' }; return $value.Trim()
}
function Reset-Owned([string]$Label) {
  if (-not $owned) { throw 'Fixture ownership not established.' }
  Stop-App; Capture $Label
  Adb @('shell','pm','clear',$package) | Out-Null
  Start-App; Probe @('--action','ready'); Locate
}
function Pending-Absent {
  foreach ($name in @('.android-m2c-fresh-v5.pending.db','.android-m2c-direct-fresh-v5.pending.receipt.json')) {
    $found = ((Adb @('shell','run-as',$package,'find','.','-name',$name,'-print')) -join '').Trim()
    if ($found) { throw 'Pending artifact persisted; preserve and stop.' }
  }
}
function Install-Fixture([string]$Local) {
  if (-not $owned) { throw 'Unknown fixture ownership.' }
  $remote = "/data/local/tmp/lifeos-m2c-$RunId-fixture.db"
  Adb @('push',$Local,$remote) | Out-Null
  Adb @('shell','run-as',$package,'cp',$remote,$database) | Out-Null
  # Fixed sprint-owned temporary source only; no generic cleanup.
  Adb @('shell','rm','-f',$remote) | Out-Null
}
Push-Location $repositoryRoot
try {
  if (-not (Test-Path -LiteralPath $apk -PathType Leaf)) { throw 'Separate inspected M2-C APK is missing.' }
  if ($ResumeFinal) {
    # Read-only supplemental restart proof for a preserved, exact-owned final
    # profile. Never replay mutations, clear state, or replace a failed report.
    $owner = Get-Content -LiteralPath (Join-Path $artifact 'ownership.json') -Raw -Encoding UTF8 | ConvertFrom-Json
    if ($owner.run_id -ne $RunId -or $owner.avd -ne $avdName -or $owner.serial -ne $serial -or $owner.package -ne $package -or $owner.apk_sha256 -ne (Hash $apk)) { throw 'Supplemental ownership/APK binding mismatch.' }
    foreach ($file in @('FAILED.txt','final-fresh.db','final-fresh.receipt.json')) {
      if (-not (Test-Path -LiteralPath (Join-Path $artifact $file) -PathType Leaf)) { throw 'No preserved final-profile evidence; supplemental run refused.' }
    }
    $supplement = "supplemental-$SupplementId"
    if (@(Get-ChildItem -LiteralPath $artifact -Filter "$supplement-*").Count) { throw 'Supplemental evidence exists; do not overwrite.' }
    $devices = @(& $adb devices | Select-Object -Skip 1 | Where-Object { $_ -match '\t(device|offline|unauthorized)$' })
    if (@($devices | Where-Object { $_ -notmatch '^emulator-[0-9]+\tdevice$' }).Count) { throw 'Physical/unsafe device; supplement refused.' }
    if (-not @($devices | Where-Object { $_ -match "^$serial\tdevice$" }).Count) { Start-Emulator } else { Bind; $process = $true }
    if (-not (((Adb @('shell','pm','path',$package)) -join '').Trim())) { throw 'Owned package missing.' }
    $owned = $true; Stop-App; Start-App; Probe @('--action','ready'); Locate; Pending-Absent
    Stop-App; Capture "$supplement-before"
    if ((Hash (Join-Path $artifact "$supplement-before.db")) -ne (Hash (Join-Path $artifact 'final-fresh.db'))) { throw 'Preserved final profile differs; stop without repair.' }
    foreach ($iteration in @(1,2)) {
      Stop-Emulator; $process = $null; Start-Emulator; Start-App; Probe @('--action','ready'); Pending-Absent
      Screenshot "$supplement-ready-$iteration"; Stop-App; Capture "$supplement-restarted-$iteration"
      foreach ($extension in @('db','receipt.json')) {
        if ((Hash (Join-Path $artifact "$supplement-restarted-$iteration.$extension")) -ne (Hash (Join-Path $artifact "final-fresh.$extension"))) { throw 'Restart changed preserved final artifact bytes.' }
      }
      $results.Add("PASS full exact emulator shutdown/restart $iteration; UI ready, no pending names, byte-identical empty DB/receipt; no shell sync")
    }
    @('# Supplemental M2-C final-profile native restart evidence',"RunId: $RunId", "AVD: $avdName", "APK SHA-256: $(Hash $apk)", 'Original failed run remains preserved; this supplement only supplies missing exact final-profile restart proof and does not replace its historical exit status.', '', $results, 'Founder manual acceptance PENDING; actual power loss/physical devices untested.') | Set-Content -LiteralPath (Join-Path $artifact "$supplement-restart.md") -Encoding UTF8
    Write-Host ($results -join "`n"); exit 0
  }
  $devices = @(& $adb devices | Select-Object -Skip 1 | Where-Object { $_ -match '\t(device|offline|unauthorized)$' })
  if (@($devices | Where-Object { $_ -notmatch '^emulator-[0-9]+\tdevice$' }).Count) { throw 'Physical/offline/unauthorized device present; no operation performed.' }
  if (@($devices | Where-Object { $_ -match "^$serial\t" }).Count) { throw 'New bound serial collision; preserve and stop.' }
  if (@(Get-NetTCPConnection -LocalPort $port,($port+1),$cdpPort -ErrorAction SilentlyContinue).Count) { throw 'Bound port collision; stop.' }
  # Other existing emulators are inventory-only and never receive a command.
  $preservedDeviceInventory = $devices
  if (Test-Path -LiteralPath $artifact) { throw 'Run evidence already exists; choose a new RunId, never overwrite it.' }
  if ((Test-Path -LiteralPath (Join-Path $avdHome "$avdName.ini")) -or (Test-Path -LiteralPath (Join-Path $avdHome "$avdName.avd"))) { throw 'AVD name collision; no wipe/delete performed.' }
  New-Item -ItemType Directory -Path $artifact -Force | Out-Null
  $old = $ErrorActionPreference; $ErrorActionPreference = 'Continue'
  try {
    'no' | & $avdManager create avd --name $avdName --package 'system-images;android-36;default;x86_64' --device 'pixel_6' *> (Join-Path $artifact 'avd-create.txt')
    $avdExit = $LASTEXITCODE
  } finally { $ErrorActionPreference = $old }
  $configPath = Join-Path $avdHome "$avdName.avd\config.ini"
  if (-not (Test-Path -LiteralPath $configPath) -or -not (Test-Path -LiteralPath (Join-Path $avdHome "$avdName.ini"))) { throw "New exact-owned AVD creation failed ($avdExit)." }
  $avdConfig = Get-Content -LiteralPath $configPath -Raw
  if ($avdConfig -notmatch 'android-36' -or $avdConfig -notmatch 'x86_64') { throw 'Owned AVD configuration differs from API36 x86_64 target.' }
  @{ authorization='ANDROID-M2C-SYNTHETIC-REFLECTION-001'; run_id=$RunId; avd=$avdName; serial=$serial; package=$package; apk_sha256=(Hash $apk); ownership='Created absent AVD; install absent package; only own synthetic fixtures'; } | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $artifact 'ownership.json') -Encoding UTF8
  $preservedDeviceInventory | Set-Content -LiteralPath (Join-Path $artifact 'preserved-device-inventory.txt') -Encoding UTF8
  Start-Emulator
  if (((Adb @('shell','pm','path',$package) -AllowFailure) -join '').Trim()) { throw 'M2-C package collision; preserve and stop.' }
  Adb @('install',$apk) | Out-Null; $owned = $true
  Start-App; Probe @('--action','ready'); Locate; Pending-Absent
  Capture 'fresh'
  if ((Sql (Join-Path $artifact 'fresh.db') 'PRAGMA user_version;') -ne '5' -or (Sql (Join-Path $artifact 'fresh.db') 'SELECT COUNT(*) FROM schema_migration_receipts;') -ne '0') { throw 'Direct fresh-v5 origin contract failed.' }
  $origin = Get-Content -LiteralPath (Join-Path $artifact 'fresh.receipt.json') -Raw -Encoding UTF8 | ConvertFrom-Json
  if ($origin.origin -ne 'direct_fresh_v5' -or $origin.applicationId -ne $package) { throw 'Wrong direct-origin identity.' }
  $results.Add('PASS exact app-private direct-v5 initialization, identity and zero migration receipts')
  Probe @('--action','locale','--locale','ja'); Stop-App; Start-App; Probe @('--action','locale','--locale','ja','--verify','true'); Probe @('--action','locale','--locale','zh-TW'); Probe @('--action','locale','--locale','en')
  $results.Add('PASS English/Traditional Chinese/Japanese controls and app-private locale restoration')
  Probe @('--action','copy-review')
  foreach ($locale in @('en','zh-TW','ja')) { Probe @('--action','locale','--locale',$locale); Screenshot "copy-review-$locale" }
  $results.Add('PASS trilingual distinct per-moment/build optional headings, exact plain delete confirmation, retained erasure caveat and unchanged content on cancel')
  Reset-Owned 'copy-review-preserved'
  foreach ($locale in @('en','zh-TW','ja')) {
    Probe @('--action','status-review','--locale',$locale)
    Screenshot "status-review-$locale"
    $results.Add("PASS $locale contextual draft visibility/location and scoped create/edit/delete feedback; conflict/unconfirmed preserved")
    Reset-Owned "status-review-$locale-preserved"
  }
  $original = "  合成片刻：静かな勇気。`n第二行  "
  $updated = "  修正後：今天も静かな勇気。`n第二行`n  "
  $meta = Join-Path $artifact 'journey-metadata.json'
  Probe @('--action','create','--text',$original,'--metadata',$meta); $id = (Get-Content -LiteralPath $meta -Raw | ConvertFrom-Json).id
  Capture 'before-cancel-edit'; Probe @('--action','cancel-edit','--id',$id,'--text','unsaved synthetic edit'); Capture 'after-cancel-edit'
  if ((Hash (Join-Path $artifact 'before-cancel-edit.db')) -ne (Hash (Join-Path $artifact 'after-cancel-edit.db'))) { throw 'Cancel edit changed database bytes.' }
  Probe @('--action','edit','--id',$id,'--text',$updated); Stop-App; Start-App; Probe @('--action','text','--id',$id,'--text',$updated)
  $results.Add('PASS UI create/edit/restart exact CJK/multiline text, identity/predecessor/user authorship; cancel edit byte-identical no writes')
  Capture 'before-cancel-delete'; Probe @('--action','cancel-delete','--id',$id); Capture 'after-cancel-delete'
  if ((Hash (Join-Path $artifact 'before-cancel-delete.db')) -ne (Hash (Join-Path $artifact 'after-cancel-delete.db'))) { throw 'Cancel delete changed database bytes.' }
  Probe @('--action','delete','--id',$id); Stop-App; Start-App; Probe @('--action','absent','--id',$id); Capture 'journey-deleted'
  if ((Sql (Join-Path $artifact 'journey-deleted.db') 'SELECT COUNT(*) FROM source_revision_content;') -ne '0') { throw 'Logical source content survived deletion.' }
  if ((Sql (Join-Path $artifact 'journey-deleted.db') 'SELECT COUNT(*) FROM source_revisions;') -ne '2' -or (Sql (Join-Path $artifact 'journey-deleted.db') 'SELECT COUNT(*) FROM source_heads;') -ne '1') { throw 'Canonical content-free revision/head metadata contract failed.' }
  $results.Add('PASS confirmed UI deletion/cancel byte-identical, editor/detail purge and restart absence; canonical content-free metadata remains')
  Probe @('--action','protocol'); Stop-App; Capture 'protocol'; Start-App
  $results.Add('PASS native same-content revision, duplicate update/delete, two-editor stale conflict, edit-delete races and rollback-after-projection')
  foreach ($op in @('update','delete')) {
    foreach ($phase in @('beforeCommit','afterCommitBeforeAck')) {
      $requestMeta = Join-Path $artifact "$op-$phase-request.json"
      # A synchronous Rust debug hold can also delay the WebView/CDP response.
      # Arm in a distinct owned helper so marker observation/force-stop is not
      # delayed until the held command has already committed.
      $env:LIFE_OS_M2C_PROBE_TEXT = 'fault corrected synthetic'
      $arm = Start-Process -FilePath 'node' -ArgumentList @('scripts/android-m2c-cdp-probe.mjs','--port',"$cdpPort",'--action','arm','--operation',$op,'--phase',$phase,'--metadata',$requestMeta) -WorkingDirectory $repositoryRoot -WindowStyle Hidden -PassThru -RedirectStandardOutput (Join-Path $artifact "$op-$phase-arm.txt") -RedirectStandardError (Join-Path $artifact "$op-$phase-arm-errors.txt")
      $end = [DateTime]::UtcNow.AddSeconds(12); $marker = ''
      do { Start-Sleep -Milliseconds 200; $marker = ((Adb @('shell','run-as',$package,'find','.','-name',".m2c-$phase.hold",'-print')) -join '').Trim() } while (-not $marker -and [DateTime]::UtcNow -lt $end)
      if (-not $marker) { throw 'Synchronized native fault marker missing.' }
      Stop-App
      if (-not $arm.WaitForExit(3000)) { Stop-Process -Id $arm.Id -Force }
      Start-App
      Probe @('--action','retry','--metadata',$requestMeta,'--text','fault corrected synthetic','--committed',$(if($phase -eq 'afterCommitBeforeAck'){'true'}else{'false'}))
      $results.Add("PASS synchronized $op $phase force-stop/restart with exact frozen request reconciliation")
      Reset-Owned "$op-$phase-preserved"
    }
  }
  foreach ($failure in @('pending','newer','malformed','dependency')) {
    Stop-App; Capture "$failure-before"
    $fixture = Join-Path $artifact "$failure-fixture.db"
    Copy-Item -LiteralPath (Join-Path $artifact "$failure-before.db") -Destination $fixture
    if ($failure -eq 'pending') {
      $parent = $database.Substring(0,$database.LastIndexOf('/'))
      Adb @('shell','run-as',$package,'cp',$database,"$parent/.android-m2c-fresh-v5.pending.db") | Out-Null
    } elseif ($failure -eq 'newer') { Sql $fixture 'PRAGMA user_version=7;' | Out-Null; Install-Fixture $fixture
    } elseif ($failure -eq 'malformed') { [IO.File]::WriteAllText($fixture,'owned malformed synthetic database'); Install-Fixture $fixture
    } else {
      Sql $fixture "INSERT INTO v5_compatibility_write_guard(token,created_at) VALUES ('m2c-native-fixture-guard-00000000000000','2026-10-03T00:00:00.000Z'); INSERT INTO persisted_artifacts VALUES ('unexpected-artifact','unexpected-source','evidence','synthetic unexpected payload','2026-10-03T00:00:00.000Z','2026-10-03T00:00:00.000Z'); DELETE FROM v5_compatibility_write_guard;" | Out-Null
      Install-Fixture $fixture
    }
    Start-App; Probe @('--action','blocked'); Stop-App; Capture "$failure-preserved"
    if ($failure -eq 'pending') {
      Export-File "$parent/.android-m2c-fresh-v5.pending.db" (Join-Path $artifact 'pending-name-preserved.db')
      if ((Hash (Join-Path $artifact 'pending-name-preserved.db')) -ne (Hash (Join-Path $artifact 'pending-before.db'))) { throw 'Pending artifact bytes changed during refusal.' }
    }
    $expected = if ($failure -eq 'pending') { Join-Path $artifact "$failure-before.db" } else { $fixture }
    if ((Hash $expected) -ne (Hash (Join-Path $artifact "$failure-preserved.db"))) { throw 'Refused fixture bytes changed.' }
    $results.Add("PASS $failure fail-closed refusal and byte-identical failure evidence preservation")
    Reset-Owned "$failure-pre-reset-preserved"
  }
  foreach ($locale in @('en','zh-TW','ja')) {
    Probe @('--action','locale','--locale',$locale)
    $reflectionMeta = Join-Path $artifact "reflection-$locale-metadata.json"
    Probe @('--action','reflection-journey','--locale',$locale,'--metadata',$reflectionMeta)
    Screenshot "reflection-$locale-saved"
    Stop-App; Capture "reflection-$locale-saved"; Start-App
    Probe @('--action','reflection-reopened','--metadata',$reflectionMeta)
    Stop-App; Capture "reflection-$locale-reopened"; Start-App
    if ((Hash (Join-Path $artifact "reflection-$locale-saved.db")) -ne (Hash (Join-Path $artifact "reflection-$locale-reopened.db"))) { throw 'Read-only restart changed canonical database.' }
    $results.Add("PASS $locale explicit candidate/correction/fresh review/question/user response/save/restart; mock/user provenance and no automatic conclusions")
    Probe @('--action','reflection-source-edit-delete','--metadata',$reflectionMeta)
    Stop-App; Capture "reflection-$locale-deleted"; Start-App
    if ((Sql (Join-Path $artifact "reflection-$locale-deleted.db") 'SELECT COUNT(*) FROM artifact_revision_content;') -ne '0') { throw 'Dependent text survived parent deletion.' }
    $results.Add("PASS $locale source edit invalidation, ineligibility, cancellation and atomic parent/delete purge")
    Reset-Owned "reflection-$locale-final-preserved"
  }
  Probe @('--action','reflection-reject-skip')
  $results.Add('PASS native rejected candidate excludes question; skipped/unsaved response has no user context; cancel/no-action keeps immutable state')
  Reset-Owned 'reflection-reject-skip-preserved'
  Probe @('--action','reflection-protocol')
  $results.Add('PASS native artifact exact identity/duplicates/stale source+Evidence, foreign refs, unsupported controls and transactional rollback')
  Reset-Owned 'reflection-protocol-preserved'
  foreach ($op in @('candidate','answer')) {
    foreach ($phase in @('beforeCommit','afterCommitBeforeAck')) {
      $requestMeta = Join-Path $artifact "artifact-$op-$phase-request.json"
      $arm = Start-Process -FilePath 'node' -ArgumentList @('scripts/android-m2c-cdp-probe.mjs','--port',"$cdpPort",'--action','reflection-arm','--operation',$op,'--phase',$phase,'--metadata',$requestMeta) -WorkingDirectory $repositoryRoot -WindowStyle Hidden -PassThru -RedirectStandardOutput (Join-Path $artifact "artifact-$op-$phase-arm.txt") -RedirectStandardError (Join-Path $artifact "artifact-$op-$phase-arm-errors.txt")
      $end = [DateTime]::UtcNow.AddSeconds(15); $marker = ''
      do { Start-Sleep -Milliseconds 200; $marker = ((Adb @('shell','run-as',$package,'find','.','-name',".m2c-$phase.hold",'-print')) -join '').Trim() } while (-not $marker -and [DateTime]::UtcNow -lt $end)
      if (-not $marker) { throw 'Synchronized artifact fault marker missing; preserve.' }
      Stop-App
      if (-not $arm.WaitForExit(3000)) { Stop-Process -Id $arm.Id -Force }
      Start-App
      Probe @('--action','reflection-retry','--metadata',$requestMeta,'--committed',$(if($phase -eq 'afterCommitBeforeAck'){'true'}else{'false'}))
      $results.Add("PASS native artifact $op $phase force-stop/restart exact immutable request reconciliation; no duplicate write")
      Reset-Owned "artifact-$op-$phase-preserved"
    }
  }
  $logcat = Adb @('logcat','-d','-v','brief'); $logcat | Set-Content -LiteralPath (Join-Path $artifact 'logcat.txt') -Encoding UTF8
  if (($logcat -join "`n") -match 'M2C_CONTENT_LEAK_CANARY') { throw 'Raw canary leaked to native logcat; preserve and stop.' }
  $results.Add('PASS raw synthetic canary absent from native logcat; acknowledgement/request evidence content-free')
  Stop-App; Capture 'final-fresh'; Pending-Absent; Stop-Emulator; $process=$null
  Start-Emulator; Start-App; Probe @('--action','ready'); Pending-Absent; Capture 'final-restarted'
  Screenshot 'final-fresh-screen'
  $results.Add('PASS final fresh Founder profile survives full exact emulator shutdown/restart with no pending names; no shell sync')
  $results.Add('UNTESTED physical devices, ARM/OEM/other API, actual power loss, multi-process access and secure physical erasure; fault injection is not physical durability proof')
  $results.Add('Founder diff/manual UI acceptance PENDING; no staging/commit/push/merge/archive/reset/release/production or later slice')
  $report = @('# Android M2-C native evidence',"RunId: $RunId","AVD: $avdName","Serial: $serial","Identity: $package","APK SHA-256: $(Hash $apk)","Source HEAD: $(git rev-parse HEAD)",'') + $results
  $report | Set-Content -LiteralPath (Join-Path $artifact 'native-result.md') -Encoding UTF8
  Write-Host ($report -join "`n")
} catch {
  if ($owned) { try { Probe @('--action','state'); Screenshot $(if ($ResumeFinal) {"supplemental-$SupplementId-failed-screen"} else {'failed-screen'}) } catch { Write-Warning 'Read-only failure UI diagnostic unavailable.' } }
  if ($owned -and $database -and $receipt) {
    try { Stop-App; Capture $(if ($ResumeFinal) {"supplemental-$SupplementId-failure-preserved"} else {'failure-current-preserved'}) } catch { Write-Warning 'Failure sandbox preserved in owned AVD; snapshot export unavailable.' }
  }
  if (Test-Path -LiteralPath $artifact) { @('Native run failed; ownership/AVD/fixtures preserved. No unknown state repaired or deleted.',"Failure class: $($_.Exception.GetType().Name)") | Set-Content -LiteralPath (Join-Path $artifact $(if ($ResumeFinal) {"FAILED-supplement-$SupplementId.txt"} else {'FAILED.txt'})) -Encoding UTF8 }
  throw
} finally {
  if ($process) { try { Stop-Emulator } catch { Write-Warning 'Owned emulator may still be running; preserve and inspect exact binding.' } }
  Pop-Location
}
