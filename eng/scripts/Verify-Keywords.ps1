#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackageInfoDirectory
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

$MAX_KEYWORDS = 5

$packages = Get-CargoPackages

if ($PackageInfoDirectory -and (Test-Path $PackageInfoDirectory)) {
  $packageNames = Get-PackageNamesFromPackageInfo $PackageInfoDirectory
  $packages = $packages | Where-Object { $packageNames -contains $_.name }
}

$violations = @()
foreach ($package in $packages) {
  $keywords = $package.keywords
  if ($keywords -and $keywords.Count -gt $MAX_KEYWORDS) {
    $violations += [PSCustomObject]@{
      Name     = $package.name
      Count    = $keywords.Count
      Keywords = $keywords -join ', '
    }
  }
}

if ($violations.Count -gt 0) {
  foreach ($v in $violations) {
    LogError "Crate '$($v.Name)' has $($v.Count) keywords (max $MAX_KEYWORDS): $($v.Keywords)"
  }
  exit 1
}

Write-Host "All crates have $MAX_KEYWORDS or fewer keywords."
