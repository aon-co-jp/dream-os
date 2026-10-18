<#
.SYNOPSIS
  DreamOS インストーラーUSBメモリを作る / インストーラーISOをCDに焼く (Windows)

.DESCRIPTION
  dream-os-disk make-iso で作った dreamos.iso の中身を、USBメモリ(UEFIブート可能なFAT32)へ複写します。
  同じ dreamos.iso は CD/DVD にもそのまま焼けます(-BurnCd)。

  安全のため:
   * 既定は「確認だけ」(何も消しません)。実行は -Execute と -ConfirmText "ERASE <ディスク番号>" の両方が必要です。
   * 対象は BusType=USB かつ システム/ブートディスクでないものだけです。内蔵HDD/SSDは選べません。
   * 管理者権限が必要です。

.EXAMPLE
  .\make-usb.ps1 -List
  .\make-usb.ps1 -IsoPath .\dreamos.iso -DiskNumber 2                       # 確認だけ
  .\make-usb.ps1 -IsoPath .\dreamos.iso -DiskNumber 2 -Execute -ConfirmText "ERASE 2"
  .\make-usb.ps1 -IsoPath .\dreamos.iso -BurnCd                             # Windows標準のディスク書き込みツールを開く
#>
[CmdletBinding()]
param(
  [string]$IsoPath,
  [int]$DiskNumber = -1,
  [switch]$List,
  [switch]$Execute,
  [string]$ConfirmText = "",
  [switch]$BurnCd,
  [string]$Label = "DREAMOS"
)
$ErrorActionPreference = 'Stop'

function Get-UsbCandidates {
  Get-Disk | Where-Object { $_.BusType -eq 'USB' -and -not $_.IsSystem -and -not $_.IsBoot } |
    ForEach-Object {
      [pscustomobject]@{
        Number = $_.Number
        Name   = $_.FriendlyName
        SizeGB = [math]::Round($_.Size / 1GB, 1)
        Style  = $_.PartitionStyle
      }
    }
}

if ($List) {
  Write-Host "書き込み候補のUSBメモリ (内蔵ディスクは表示しません):"
  $c = @(Get-UsbCandidates)
  if ($c.Count -eq 0) { Write-Host "  (見つかりません)" } else { $c | Format-Table -AutoSize }
  return
}

if (-not $IsoPath -or -not (Test-Path -LiteralPath $IsoPath)) { throw "-IsoPath に dreamos.iso を指定してください" }
$IsoPath = (Resolve-Path -LiteralPath $IsoPath).Path

if ($BurnCd) {
  Write-Host "CD/DVDにISOを焼きます。空のディスクを入れて、開く画面で [書き込み] を押してください。"
  Start-Process -FilePath "$env:SystemRoot\System32\isoburn.exe" -ArgumentList "`"$IsoPath`""
  return
}

if ($DiskNumber -lt 0) { throw "-DiskNumber を指定してください (候補は -List で確認)" }
$cand = @(Get-UsbCandidates) | Where-Object { $_.Number -eq $DiskNumber }
if (-not $cand) { throw "ディスク $DiskNumber はUSBメモリの候補ではありません (内蔵/システムディスクは拒否します)" }

Write-Host ("対象: ディスク {0} {1} {2}GB  ← 中身は全て消えます" -f $cand.Number, $cand.Name, $cand.SizeGB)
Write-Host ("元: {0}" -f $IsoPath)
if (-not $Execute) {
  Write-Host "確認だけで終了しました。実行するには -Execute -ConfirmText `"ERASE $DiskNumber`" を付けてください。"
  return
}
if ($ConfirmText -ne "ERASE $DiskNumber") { throw "-ConfirmText `"ERASE $DiskNumber`" が一致しません" }
$admin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $admin) { throw "管理者として実行してください" }

# 直前にもう一度、対象が安全か確認する
$d = Get-Disk -Number $DiskNumber
if ($d.BusType -ne 'USB' -or $d.IsSystem -or $d.IsBoot) { throw "対象ディスクが安全条件を満たしません" }

Write-Host "パーティションを作成しています..."
Clear-Disk -Number $DiskNumber -RemoveData -RemoveOEM -Confirm:$false
$style = (Get-Disk -Number $DiskNumber).PartitionStyle
if ($style -eq 'RAW') { Initialize-Disk -Number $DiskNumber -PartitionStyle GPT }
elseif ($style -ne 'GPT') {
  # リムーバブルでは Set-Disk が非対応のことがあるため diskpart で変換する(対象は上で検証済みのUSBディスクのみ)
  $dp = Join-Path $env:TEMP "dreamos-diskpart.txt"
  "select disk $DiskNumber`r`nclean`r`nconvert gpt`r`nexit`r`n" | Set-Content -Path $dp -Encoding ASCII
  diskpart /s $dp | Out-Host
  Remove-Item $dp -ErrorAction SilentlyContinue
  Update-HostStorageCache
}
# FAT32 の書式設定は Windows では 32GB までのため、USBが大きくても先頭 32GB のみ使う(UEFIはFAT32を読む)
$size = [math]::Min($d.Size - 8MB, 32GB - 1MB)
$part = New-Partition -DiskNumber $DiskNumber -Size $size -AssignDriveLetter
Format-Volume -Partition $part -FileSystem FAT32 -NewFileSystemLabel $Label -Confirm:$false | Out-Null
$dst = ($part | Get-Volume).DriveLetter + ":\"

Write-Host "ISOの中身を複写しています..."
$img = Mount-DiskImage -ImagePath $IsoPath -PassThru
try {
  $src = (($img | Get-Volume).DriveLetter) + ":\"
  robocopy $src $dst /E /R:1 /W:1 /NFL /NDL /NP | Out-Null
  if ($LASTEXITCODE -ge 8) { throw "robocopy が失敗しました (コード $LASTEXITCODE)" }
} finally {
  Dismount-DiskImage -ImagePath $IsoPath | Out-Null
}
$boot = Test-Path -LiteralPath (Join-Path $dst "EFI\BOOT\BOOTX64.EFI")
Write-Host ("完了: {0}  UEFIブートローダー: {1}" -f $dst, $(if ($boot) { "あり" } else { "なし(dream-os-disk make-iso に --bootloader を付けて作り直してください)" }))
