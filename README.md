# Git Status

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A command-line utility written in Rust to display the status of Git repositories in folders.

# Git Helper in Rust

This project provides a set of utility functions to interact with Git using Rust. 

## Functions

### `is_git_installed() -> bool`

This function checks if Git is installed on the system by running the `git --version` command.

### `is_git_repo(path: &std::path::Path) -> bool`

This function checks if the given path is inside a Git repository by running the `git rev-parse --is-inside-work-tree` command in the given directory.

### `check_git_status(path: &std::path::Path)`

This function checks the status of the Git repository at the given path by running the `git status` command. If there are changes, it prompts the user to add, commit, and push the changes.

### `add_commit_push(path: &std::path::Path)`

This function adds all changes, commits them with the message "auto commit", and pushes them to the remote repository. It runs the `git add .`, `git commit -m "auto commit"`, and `git push` commands in the given directory.

## Usage

To use these functions, import the module and call the functions as needed. For example:

## Features

- Display the current branch name
- Show the number of modified, added, deleted, and renamed files
- Show the number of untracked files
- Display the latest commit message and author
- Show the remote repository URL

## Usage

