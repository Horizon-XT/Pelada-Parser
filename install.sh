#!/bin/bash

PROJECT_PATH="$PWD/."

cargo build --release

if [ $? -eq 0 ]; then
		if ! grep -q "$PROJECT_PATH/target/release" "$HOME/.zshrc"; then
				echo "export PATH=\$PATH:$PROJECT_PATH/target/release" >> "$HOME/.zshrc"
				
				source "$HOME/.zshrc"
				
				echo "Build successful. pelada-parser binary added to the system's PATH."
		else
				echo "The path is already in the zshrc file. No changes made."
		fi
else
    echo "Error: Cargo build failed. Please check the build output for errors."
fi

