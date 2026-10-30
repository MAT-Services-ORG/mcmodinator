@echo off
@set ver="1 WBAT"
@echo MAT Modinator TUI %ver% for Windows BATCH
@echo.
@if EXIST mcmodinator-cli*.exe goto exists
ECHO [ERROR] MCModinaotr (mcmodinator-cli*.exe) not found in current folder.
pause
exit
:exists
FOR /F "delims=" %%A IN ('dir mcmodinator-cli*.exe /B') DO (SET "ModinatorFile=%%A")
echo Choose an option: 
echo Build mod              -^> m
echo Build CPack            -^> c
echo Build mod (LEGACY)     -^> ml
echo Build CPack (LEGACY)   -^> cl
echo Print version          -^> v
echo Print doccumentation   -^> d
echo Cancel                 -^> x
set "INPUT="
set /p "INPUT=Choose an option => "
if not defined INPUT goto exists
if %INPUT% == x exit
if %INPUT% == m goto build
if %INPUT% == c goto build
if %INPUT% == ml goto build
if %INPUT% == cl goto build
if %INPUT% == v goto v
if %INPUT% == d goto d
echo Invalid input. Retry.
echo.
goto exists
:v
echo MCModinator TUI V.%ver%
"%ModinatorFile%" -V
exit
:build
set "InputPath="
set /p "InputPath=Project path (Empty for current folder)  => "
if not defined InputPath set "InputPath=."
echo %InputPath%
set "OutputPath="
set /p "OutputPath=Output path (Empty for current folder) => "
if not defined OutputPath set "OutputPath=."
echo %OutputPath%
if %INPUT%==m (set "args=-mtrue") else if %INPUT%==c (set "args=-ctrue") else if %INPUT%==ml (set "args=-mtrue -l") else if %INPUT%==cl (set "args=-ctrue -l")

"%ModinatorFile%" %args% -p "%InputPath%" -o "%OutputPath%"

pause