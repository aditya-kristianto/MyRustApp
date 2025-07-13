def custom_sort:
  (split("-")[0] | split(".") | map(tonumber)) as $a |
  (split("-")[1] | split(".") | map(tonumber)) as $b |
  $a[0] as $major_a |
  $a[1] as $minor_a |
  $a[2] as $patch_a |
  $b[0] as $major_b |
  $b[1] as $minor_b |
  $b[2] as $patch_b |
  if $major_a != $major_b then $major_a - $major_b
  elif $minor_a != $minor_b then $minor_a - $minor_b
  else $patch_a - $patch_b
  end;

.[]
| sort_by(.; custom_sort)
| .[]

