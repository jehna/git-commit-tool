# commit-message-tool

Welcome to **commit-message-tool**—because if you're going to commit to a change, you might as well commit to a message too. This tool exists solely to ensure that your commit messages are as fancy as your code (which is likely not very). Let's dive in, shall we?

## Overview

`commit-message-tool` automatically generates beautifully crafted commit messages based on the diffs in your Git staged files. So, instead of your usual "fixed stuff" or "updated things," you'll have messages that could even engage the most uninterested of developers (which is a small victory, considering the state of our profession).

## Files

- **prepare-commit-msg**: The actual brain behind the operation, written in Python 3. It reads your diff, generates a commit message using OpenAI's API, and then sprinkles some magic over your existing message if you happen to have one.

- **setup-dev.sh**: A script that sets up the `prepare-commit-msg` hook. Just one more thing for you to ignore until it's absolutely necessary, just like your dentist reminding you to floss.

## Installation

To install `commit-message-tool`, simply perform the following steps. Don’t worry, it’s more straightforward than trying to explain to your cat why you started talking to your computer.

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/commit-message-tool.git
   cd commit-message-tool
   ```

2. Set up the `prepare-commit-msg` hook by executing:
   ```bash
   bash setup-dev.sh
   ```

   This creates a symbolic link from the script to the Git hooks directory. Because typing out long paths every time you want to set something up is what we live for.

3. Ensure that you have your OpenAI API key set as an environment variable. This step is crucial unless you want your tool to become as useful as a chocolate teapot:
   ```bash
   export OPENAI_API_KEY='your-key-here'
   ```

## Usage

To use **commit-message-tool**, simply make your changes and stage them. When you run `git commit`, the hook will automatically invoke the `prepare-commit-msg` script and generate a commit message based on your staged changes.

Let's go through an example, shall we?

### Example

1. Make some changes in your code. Nothing too drastic; we don’t need to reinvent the wheel here.

2. Stage your changes:
   ```bash
   git add .
   ```

3. Commit your changes:
   ```bash
   git commit
   ```

4. Take a moment to appreciate the formatted commit message. Hopefully, it describes everything better than your last attempt at explaining why your code occasionally behaves like a grumpy cat.

### Sample Output

Suppose your diff included the following:
```diff
- fixed typo in documentation
- added new feature that does nothing
```

Your automagic commit message might look like:
```
Fix typo in documentation

Added a new feature that is utterly useless.
```

Clearly, we’re stepping up from your legendary “fixed stuff.” Progress!

## Important Notes

- Make sure you have Python 3 installed. But if you’re still using Python 2, it might be time to have a serious talk with yourself.

- The commit message generator relies on OpenAI's API, so you might want to ensure it’s up and running unless you enjoy staring at wall after wall of blank nothingness.

- If you run into problems or bugs, just remember: those are not bugs, they’re just features waiting to be discovered. At least, that's what we tell ourselves.

## License

This project is open-source, which means you can do whatever you want with it—as long as you don’t go and break things worse than they already are.

Now go forth and commit messagely!