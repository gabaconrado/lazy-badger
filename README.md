# Lazy Badger

> Simple utility to run scripts in the project level to boost your workflow

## Overview

Lazy badger is a tool to call scripts in a pre-configured directory to make the
development workflow smooth in a consistent way across different projects.

## Features

- Calls bash scripts from the working directory;
- Forwards received parameters to the inner script;

## Is this a **make** with files instead of commands?

Almost, the main differences are:

- Commands are identified by their file names;
- You can send parameters to them directly;
- No dependencies between scripts;

The project was actually motivated by `make`, but without the building pragmatism
present there. It aims to be a simplified, more focused on general workflow management.

## Basic usage

Add some scripts to a directory (`./scripts` by default) in your working directory
and use the tool to call them.

```bash
# For the following directory structure
#
# .
# ./scripts/
# ./scripts/script_one.sh
# ./scripts/scoped/script_two.sh
# ./scripts/scoped/more_scoped/with_args.sh

# Call the scripts directly
lazy-badger script_one
lazy-badger script_two

# Pass arguments to them
lazy-badger with_args foo bar baz

# Change default script root
lazy-badger --scripts-root my-root-dir my-script

# List all available scripts
lazy-badger

# Help message
lazy-badger --help
```

## Requirements/Limitations

- Only works for systems that use `bash` as their underlying command executor;
- Scripts require to be readable and executable by the user running the application;
- Not really sure how to make scripts that calls `$EDITOR` to work;

## Future improvements

- Add logging and better debugging capabilities;
- Support scripts with the same name in different sub-directories;
