#!/bin/bash

# Directory containing images to process
DIR=$1
MINIFY_JS_TOOL=$2 # UGLIFYJS, TERSER
S3_BUCKET=$3
TEMPLATE_TYPE=$4

# Check if the directory is provided and exists
if [ -z "$DIR" ] || [ ! -d "$DIR" ]; then
  echo "Usage: $0 <directory>"
  exit 1
fi

# Loop through all .png and .jpg files in the directory and its subdirectories
# for img in $(find "$DIR" -type f \( -name "*.png" -o -name "*.jpg" -o -name "*.jpeg" \)); do
#   echo "Processing $img"
#   # Add your processing commands here
#   cwebp $img 
# done

# Loop through all .webp files in the directory and its subdirectories
# find "$DIR" -type f \( -name "*.webp" \) | while read -r img; do
#   # Remove the original file after conversion if desired
#   rm -rf $img
# done

# Loop through all .png and .jpg files in the directory and its subdirectories
# find "$DIR" -type f \( -name "*.png" -o -name "*.jpg" \) | while read -r img; do
#   # Get the base name without extension
#   base="${img%.*}"
#   # Set the new file name with .webp extension
#   new_file="${base}.webp"
  
#   # Check if the target .webp file exists
#   if [ ! -f "$new_file" ]; then
#     echo "Converting $img to $new_file"
#     cwebp $img -o $new_file
#   else
#     echo "Skipping $img as $new_file already exists"
#   fi

#   # Add your conversion command here, for example:
#   # cwebp "$img" -o "$new_file"

#   # Remove the original file after conversion if desired
#   # rm "$img"
# done

# Counter for progress
count=0
total=$(find "$DIR" -type f -name "*.js" ! -name "*.min.js" | wc -l)

# Ensure the total is greater than 0 to avoid division by zero
# if [[ $total -eq 0 ]]; then
#   echo "No files to minify in $DIR."
#   exit 0
# fi

# Process .html files, excluding .min.html files
find "$DIR/demo1" -type f \( -name "*.html" ! -name "*.min.html" \) | while read -r html; do
  # Increment the counter
  count=$((count + 1))

  # Get the base name without extension
  base="${html%.html}"
  # Set the new file name with .min.js extension
  new_file="${base}.min.html"

  # Check if the target .min.html file exists
  echo "Check: $new_file"
  
  if [[ -e $new_file ]]; then
    rm -rf $new_file
  fi

  if [[ ! -e $new_file ]]; then
    # Minify the file and save to the output directory
    html-minifier-terser \
      --collapse-whitespace \
      --remove-comments \
      --remove-optional-tags \
      --minify-css true \
      --minify-js true \
      "$html" -o "$new_file"

      echo "html : $html"
      echo "new_file: $new_file"

    # Show progress
    echo "[$count/$total] Minified: $html -> $new_file"
  else
    echo "[$count/$total] Skipped (already exists): $new_file"
  fi
done

# # Process .js files, excluding .min.js files
# find "$DIR" -type f \( -name "*.js" ! -name "*.min.js" \) | while read -r js; do
#   # Increment the counter
#   count=$((count + 1))

#   # Get the base name without extension
#   base="${js%.js}"
#   # Set the new file name with .min.js extension
#   new_file="${base}.min.js"

#   # Check if the target .min.js file exists
#   if [[ ! -e $new_file ]]; then
#     if [[ $MINIFY_JS_TOOL == "TERSER" ]]; then
#         # Output the terser command (or you can uncomment to execute rm)
#         terser "$js" -o "$new_file"
#     elif [[ $MINIFY_JS_TOOL == "UGLIFYJS" ]]; then
#         uglifyjs "$js" -o "$new_file"
#     else
#       echo "Unknown minify tool: $MINIFY_JS_TOOL"
#       exit 1
#     fi

#     # Show progress
#     echo "[$count/$total] Minified: $js -> $new_file"
#   else
#     echo "[$count/$total] Skipped (already exists): $new_file"
#   fi
# done

# # Counter for progress
# count=0
# total=$(find "$DIR" -type f -name "*.css" ! -name "*.min.css" | wc -l)

# # Ensure the total is greater than 0 to avoid division by zero
# if [[ $total -eq 0 ]]; then
#   echo "No files to minify in $DIR."
#   exit 0
# fi

# # Process .js files, excluding .min.js files
# find "$DIR" -type f \( -name "*.css" ! -name "*.min.css" \) | while read -r css; do
#   # Increment the counter
#   count=$((count + 1))

#   # Get the base name without extension
#   base="${css%.css}"
#   # Set the new file name with .min.js extension
#   new_file="${base}.min.css"

#   # Check if the target .min.js file exists
#   if [[ ! -e $new_file ]]; then
#     uglifycss "$css" > "$new_file"

#     # Show progress
#     echo "[$count/$total] Minified: $js -> $new_file"
#   else
#     echo "[$count/$total] Skipped (already exists): $new_file"
#   fi
# done

# Sync the directory with S3
# if aws s3 sync "$DIR/$TEMPLATE_TYPE/assets" "$S3_BUCKET" --exclude "*" --include "*.webp" --include "*.svg"; then
#   echo "Sync completed successfully with S3 bucket: $S3_BUCKET"
# else
#   echo "Error occurred while syncing with S3 bucket: $S3_BUCKET"
#   exit 1
# fi

# if aws s3 cp "$DIR/$TEMPLATE_TYPE/assets" "$S3_BUCKET" --recursive --metadata-directive REPLACE --exclude "*" --include "*.js" --include "*.min.js" --content-type "application/javascript"; then
#   echo "Sync completed successfully with S3 bucket: $S3_BUCKET"
# else
#   echo "Error occurred while syncing with S3 bucket: $S3_BUCKET"
#   exit 1
# fi

# if aws s3 cp "$DIR/$TEMPLATE_TYPE/assets" "$S3_BUCKET" --recursive --metadata-directive REPLACE --exclude "*" --include "*.svg" --content-type "image/svg+xml"; then
#   echo "Sync completed successfully with S3 bucket: $S3_BUCKET"
# else
#   echo "Error occurred while syncing with S3 bucket: $S3_BUCKET"
#   exit 1
# fi

# if aws s3 cp "$DIR/$TEMPLATE_TYPE/assets" "$S3_BUCKET" --recursive --metadata-directive REPLACE --exclude "*" --include "*.css" --include "*.min.css" --content-type "text/css"; then
#   echo "Sync completed successfully with S3 bucket: $S3_BUCKET"

#   aws cloudfront create-invalidation --distribution-id E3PB2N1L4PK343 --paths "/*"
# else
#   echo "Error occurred while syncing with S3 bucket: $S3_BUCKET"
#   exit 1
# fi

# Add Meta Data in S3 Assets Bucket Files
# aws s3 cp $S3_BUCKET $S3_BUCKET --recursive --metadata-directive REPLACE --cache-control "max-age=31536000"