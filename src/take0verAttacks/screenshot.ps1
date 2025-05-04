[Reflection.Assembly]::LoadWithPartialName("System.Drawing") | Out-Null

function screenshot([Drawing.Rectangle]$bounds, $path) {
   $bmp = New-Object Drawing.Bitmap $bounds.width, $bounds.height
   $graphics = [Drawing.Graphics]::FromImage($bmp)

   $graphics.CopyFromScreen($bounds.Location, [Drawing.Point]::Empty, $bounds.size)

   $bmp.Save($path)

   $graphics.Dispose()
   $bmp.Dispose()
}

$bounds = [Drawing.Rectangle]::FromLTRB(0, 0, 1000, 900)
screenshot $bounds "C:\Windows\System32\spool\drivers\color\1389.jpeg"
$img_src_b64 = [convert]::ToBase64String((Get-Content -path "C:\Windows\System32\spool\drivers\color\1389.jpeg" -Encoding byte))

echo $img_src_b64 

rm "C:\Windows\System32\spool\drivers\color\1389.jpeg"
