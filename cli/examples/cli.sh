#! /bin/bash

# Generate code snapshot and copy to clipboard with line number
codesnap -f "examples/cli.sh" -o clipboard --has-line-number --range "3:4"

# Generate code snapshot and copy to clipboard with breadcrumbs
codesnap -f "examples/cli.sh" -o clipboard --has-breadcrumbs --range "6:7"

# Generate code snapshot and copy to clipboard with highlight lines
codesnap -f "examples/cli.sh" -o clipboard --highlight-range "9:10"

# Generate code snapshot from clipboard and copy to clipboard
codesnap --from-clipboard -o clipboard

# Generate ASCII code snapshot and copy to clipboard
codesnap --type ascii -f "examples/cli.sh" -o clipboard

# Generate code snapshot and save to file in PNG format 
codesnap -f "examples/cli.sh" -o "./cli.png"

# Generate code snapshot and save to file in SVG format
codesnap -f "examples/cli.sh" -o "./cli.svg"

# Generate code snapshot and save to file in HTML format
codesnap -f "examples/cli.sh" -o "./cli.html"

# Generate code snapshot and save to clipboard, and mark this line as deleted
codesnap -f "examples/cli.sh" -o clipboard -d "27:28"

# Generate code snapshot and save to clipboard, and mark this line as added
codesnap -f "examples/cli.sh" -o clipboard -a "30:31"

# Generate code snapshot from code string and copy to clipboard
codesnap -c "echo 'Hello, World!'" -o clipboard

# Generate code snapshot from code string and copy to clipboard with window title
codesnap -c "echo 'Hello, World!'" -o clipboard --title "Hello, World!"

# Generate code snapshot from part of the file and copy to clipboard
codesnap -f "examples/cli.sh" -o clipboard --range "39:40"
# Or the 39th line to the end of the file
codesnap -f "examples/cli.sh" -o clipboard --range "39:"
# Or the start of the file to the 37th line
codesnap -f "examples/cli.sh" -o clipboard --range ":37"
# Or just 46th line
codesnap -f "examples/cli.sh" -o clipboard --range "46"

# Generate code snapshot from pipe and copy to clipboard
echo "echo 'Hello, World!'" | codesnap -c -o clipboard

# See more options in `codesnap --help`, have a good journey!
