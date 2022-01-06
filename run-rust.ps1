### 
# This script is a wrapper around the rust program which will parse your raw files and output a
# JSON file with all the details that get parsed at this time (currently only creatures).
# The plan is to actually move this into the rust code itself and not rely on this wrapper to
# perform the process.
###

# Set $saveFolder to the folder where you want to parse the raw files. 
$saveFolder = "C:\Users\nwesterhausen\Sync\Dwarf Fortress Files\Mods\vanilla-raws\raw";

# Recursively look through the raw folder
$raws = Get-ChildItem -Path $saveFolder -Include *creature*.txt -Recurse | Where-Object { ! $_.PSIsContainer }
# We want to save it in the folder with the web client to display them in a nice way
$outPath = '.\www\out.json'
# Step counter is used to display a progress bar while this runs.
$stepCounter = 0;

# Initiate the out.json file with the beginning of an array.
Set-Content -Path $outPath "["
for ($i = 0; $i -lt $raws.Length; $i++) {
    # For each file, update the progress bar with the filename
    $file = $raws[$i];
    Write-Progress -Activity "Parsing Raws" -Status (Get-Item $file ).Name -PercentComplete ((($stepCounter++) / $raws.Length) * 100)
    # Get the output from the 
    $parsed = target\debug\df-raw-lookup.exe $file
    if ($parsed.length -gt 1) {
        # If we actually got output and didn't get broken output,
        # add that output to the json file
        Add-Content -Path $outPath $parsed -NoNewline
        if ($i + 1 -lt $raws.Length) {
            # If we are NOT in the final raw file, 
            # add a comma for proper array formatting
            Add-Content -Path $outPath "," -NoNewline
        }
    }
}
# Conclude the array in the out.json file
Add-Content -Path $outPath "]" -NoNewline