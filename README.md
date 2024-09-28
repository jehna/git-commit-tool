# commit-message-tool

Ah, welcome to the *commit-message-tool*. You’re here for a reason: to improve those woeful commit messages you’ve been crafting in a caffeine-fueled frenzy. This tool will guide you through writing great commit messages, because apparently, the universe wants you to avoid being a “git commit” pariah.

## Table of Contents
- [Introduction](#introduction)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Everybody knows that a well-crafted commit message is important—much like choosing a proper font for your terminal, which, let's face it, should probably be something like Arial but isn't. This tool hooks into your repository to pre-fill your commit messages based on the changes you’re making, so you can avoid subjecting the world to "fixes stuff again."

## Installation

You can set up the commit-message-tool in your repository using one of two scripts: `setup-dev.sh` for symlinking (what a time to be alive) or `setup.sh` for an actual HTTP download.

### Using `setup-dev.sh`

1. Clone this repository somewhere on your machine.
2. Navigate to the directory.
3. Run the script to set up the hook.
   ```bash
   ./setup-dev.sh
   ```

### Using `setup.sh`

1. Run the following command:
   ```bash
   ./setup.sh
   ```
2. You’ll be prompted to enter the path to your git repository. To make this easier, I suggest you already know where it is.

## Usage

After installation, you can make your commit as usual (you know, like `git commit`). The tool will automatically fill in the commit message based on the changes. Just try to remember to make meaningful changes—simply updating your README to celebrate your breakfast will not cut it.

## Examples

### Example 1: Basic Commit

1. Make some changes to your code.
2. Stage your changes:
   ```bash
   git add .
   ```
3. Commit the changes:
   ```bash
   git commit
   ```

Your commit message will be automatically generated. It might look like this:

```
Refactor user authentication

This update organizes the user authentication module
for improved readability. Fixed various issues
related to session management.
```

### Example 2: Dealing with Conflict

1. Modify some conflicting files.
2. Stage:
   ```bash
   git add .
   ```
3. Commit:
   ```bash
   git commit
   ```

This will generate a message that acknowledges what you’ve done while also reminding you that merging isn’t just for physically putting two things together in a blender.

### Example 3: Accidentally Deleted Code

1. Delete files, cause why not?
2. Stage those regrettable actions:
   ```bash
   git add .
   ```
3. Commit:
   ```bash
   git commit
   ```

The generated message might be something soul-crushing like:
```
Remove unused files

These files were determined to be unnecessary in the
current codebase, contributing to the tragic loss of
important comments.
```

## Contributing

If you’d like to contribute, perhaps guide others in crafting poignant commit messages or—dare I say—improve the horrid code, feel free to fork the repo and send a pull request. Just remember: keep it clean, or I might have to use a metaphor about a messy room.

## License

This project is licensed under the MIT License—because why not? Someone has to give you some flexibility in life, and it surely won't be your last commit message.

---

Well, there you have it: your path to a more civilized git experience. Now go forth and commit artfully! (Just kidding, it’s probably just code).