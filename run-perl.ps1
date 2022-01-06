$saveFolder = "C:\Users\nwest\scoop\apps\dwarf-fortress\0.47.05\data\save\region1\raw";

$raws = Get-ChildItem -Path $saveFolder -Include *.txt -Recurse | Where-Object { ! $_.PSIsContainer }
$outPath = '.\www\out.json'

$stepCounter = 0;

Set-Content -Path $outPath "["
foreach ($file in $raws) {
    Write-Progress -Activity "Parsing Raws" -Status (Get-Item $file ).Name -PercentComplete ((($stepCounter++) / $raws.Length) * 100)
    $parsed = perl ./parse.pl $file
    if ($parsed.length -gt 0) {
        Add-Content -Path $outPath $parsed -NoNewline
        Add-Content -Path $outPath "," -NoNewline
    }
}
Add-Content -Path $outPath "]" -NoNewline