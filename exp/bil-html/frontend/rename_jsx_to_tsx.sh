#!/bin/bash

# Navigate to the components directory
cd src/components

# Find all .jsx files and rename them to .tsx
find . -name "*.jsx" -exec sh -c '
    for file do
        mv "$file" "${file%.jsx}.tsx"
        echo "Renamed: $file -> ${file%.jsx}.tsx"
    done
' sh {} +

# Optional: Find and rename any remaining .js files to .ts
find . -name "*.js" -exec sh -c '
    for file do
        mv "$file" "${file%.js}.ts"
        echo "Renamed: $file -> ${file%.js}.ts"
    done
' sh {} +

echo "Conversion complete!"