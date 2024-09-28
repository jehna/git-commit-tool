#!/bin/bash

read -rp "Enter the path to your repo: " repo_path

curl -sSfL https://github.com/jehna/git-commit-tool/raw/refs/heads/main/prepare-commit-msg -o "$repo_path/.git/hooks/prepare-commit-msg"
