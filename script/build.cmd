@ECHO OFF
SETLOCAL

REM for error handling
IF NOT "%selfWrapped%"=="%~0" (
  SET selfWrapped=%~0
  %ComSpec% /s /c ""%~0" %*"
  GOTO :EOF
)

REM setup
set thisdir=%~d0%~p0
pushd "%thisdir%"
cd ..
set /p version=<version.txt
set output-dir=out\win\clojure-cli

REM Compile
cargo build --release
call :check COMPILE ERROR

REM upx compression
upx target\release\clojure.exe
call :check UPX ERROR

REM download
if /I "%1"=="nodownload" goto extract
echo Downloading ClojureTools...
rd /S /Q ClojureTools
set link=https://download.clojure.org/install/clojure-tools-%version%.zip
powershell -command "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; Invoke-WebRequest -Uri '%link%'" -OutFile clojure-tools.zip
call :check DOWNLOAD ERROR

REM extract
:extract
echo Extracting files...
jar -xf clojure-tools.zip
call :check EXTRACTION ERROR

REM package
echo Packaging...
rd /S /Q out
md %output-dir%\libexec
copy ClojureTools\*.jar %output-dir%\libexec
copy ClojureTools\*.edn %output-dir%
copy script\clj.cmd %output-dir%
move /y target\release\clojure.exe %output-dir%
cd out\win

REM zip
REM we'll just assume a clojure dev has java installed
echo Compressing
jar -cfM clojure-cli-win-%version%.zip .\
call :check ZIP ERROR

:the-end
popd
exit /B

:check
if errorlevel 1 goto :error
exit /b

:error
echo !!! %* !!!
popd
exit 1
