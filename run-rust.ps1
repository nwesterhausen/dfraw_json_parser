# $saveFolder = "C:\Users\nwest\scoop\apps\dwarf-fortress\0.47.05\data\save\region1\raw";
$saveFolder = "C:\Users\nwesterhausen\scoop\apps\dwarf-fortress\0.47.05\data\save\region1\raw";

$raws = Get-ChildItem -Path $saveFolder -Include *.txt -Recurse | Where-Object { ! $_.PSIsContainer }

$stepCounter = 0;

Set-Content -Path .\out.json "["
foreach ($file in $raws) {
    Write-Progress -Activity "Parsing Raws" -Status (Get-Item $file ).Name -PercentComplete ((($stepCounter++) / $raws.Length) * 100)
    $parsed = target\debug\df-raw-lookup.exe $file
    if ($parsed.length -gt 0) {
        Add-Content -Path .\out.json $parsed -NoNewline
        Add-Content -Path .\out.json "," -NoNewline
    }
}
Add-Content -Path .\out.json "]" -NoNewline