#!/bin/bash
# setup-hooks.sh

# Create git hooks directory if it doesn't exist
mkdir -p .git/hooks

# Create symbolic link from .github/hooks to .git/hooks
ln -sf "$(pwd)/.github/hooks/pre-commit" .git/hooks/pre-commit

# Make the hook executable
chmod +x .github/hooks/pre-commit

echo "Git hooks installed successfully!"