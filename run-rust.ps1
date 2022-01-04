# $saveFolder = "C:\Users\nwest\scoop\apps\dwarf-fortress\0.47.05\data\save\region1\raw";
$saveFolder = "C:\Users\nwesterhausen\Sync\Dwarf Fortress Files\DFSaves\region8\raw";

$raws = Get-ChildItem -Path $saveFolder -Include *creature*.txt -Recurse | Where-Object { ! $_.PSIsContainer }

$stepCounter = 0;

Set-Content -Path .\out.json "["
for ($i = 0; $i -lt $raws.Length; $i++) {
    $file = $raws[$i];
    Write-Progress -Activity "Parsing Raws" -Status (Get-Item $file ).Name -PercentComplete ((($stepCounter++) / $raws.Length) * 100)
    $parsed = target\debug\df-raw-lookup.exe $file
    if ($parsed.length -gt 1) {
        Add-Content -Path .\out.json $parsed -NoNewline
        if ($i + 1 -lt $raws.Length) {
            Add-Content -Path .\out.json "," -NoNewline
        }
    }
}
Add-Content -Path .\out.json "]" -NoNewline