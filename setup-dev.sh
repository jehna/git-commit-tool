#!/bin/bash
set -exuo pipefail

# To use in your own repo (no dev mode), run:
# ln -sf /path/to/prepare-commit-msg .git/hooks/prepare-commit-msg
ln -sf ../../prepare-commit-msg .git/hooks/prepare-commit-msg
