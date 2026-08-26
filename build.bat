@echo off
REM Batch script wrapper for PowerShell build script
REM Usage: build.bat [command] [contract-name]

powershell -ExecutionPolicy Bypass -File "%~dp0build.ps1" %*
