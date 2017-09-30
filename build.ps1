Add-Type -AssemblyName System.IO.Compression.FileSystem
function Unzip
{
    param([string]$zipfile, [string]$outpath)

    [System.IO.Compression.ZipFile]::ExtractToDirectory($zipfile, $outpath)
}

function MakeDir
{
    param([string]$path)

    New-Item -ItemType directory -Path $path
}

function GetLibrary
{
    param([string]$url)

    $path = "dlls.zip"

    $WebClient = New-Object System.Net.WebClient
    $WebClient.DownloadFile( $url, $path )

    Unzip "dlls.zip" "."

    Remove-Item "dlls.zip"
}

MakeDir "msvc"
MakeDir "msvc/dll"
MakeDir "msvc/dll/32"
MakeDir "msvc/dll/64"
MakeDir "msvc/lib"
MakeDir "msvc/lib/32"
MakeDir "msvc/lib/64"
MakeDir "gnu-mingw"
MakeDir "gnu-mingw/dll"
MakeDir "gnu-mingw/dll/32"
MakeDir "gnu-mingw/dll/64"
MakeDir "gnu-mingw/lib"
MakeDir "gnu-mingw/lib/32"
MakeDir "gnu-mingw/lib/64"

GetLibrary "http://www.libsdl.org/release/SDL2-devel-2.0.6-VC.zip"

Move-Item "SDL2-2.0.6\lib\x86\*.dll" "msvc\dll\32"
Move-Item "SDL2-2.0.6\lib\x64\*.dll" "msvc\dll\64"
Move-Item "SDL2-2.0.6\lib\x86\*.lib" "msvc\lib\32"
Move-Item "SDL2-2.0.6\lib\x64\*.lib" "msvc\lib\64"

Remove-Item "SDL2-2.0.6" -Force -Recurse 
