@echo off
REM Datei: ingress\scripts\run_server.cmd

REM Wechsle ins Skriptverzeichnis
cd /d %~dp0
cd ..
cd frontend

REM Node.js-Abhängigkeiten installieren
call npm install

REM Frontend builden
call npm run build

cd ..
cargo clean
cargo run
