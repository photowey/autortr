@echo off
setlocal enableDelayedExpansion

echo current path: !cd!
cd /d !cd!

if "%~2"=="" (
    if "%~1"=="" (
        echo "Usage: publish.cmd <MODULE> [SUBMODULE]"
        exit /b 1
    ) else (
        call cargo publish --manifest-path %1/autortr-%1/Cargo.toml
    )
) else (
    call cargo publish --manifest-path %1/%2/Cargo.toml
)

endlocal
