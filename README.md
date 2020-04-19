# Lazy Badger

> Development environment utility, a way of centralizing arbitrary scripts
> into your workflow.

## Usage

Generate a new project namespace with

```bash
lazy-badger --new <project-name>
```

This will create a new directory at `~/.lazy-badger/<project-name>/scripts`

Put your scripts inside this new generated directory and then you can
call them like that:

```bash
<project-name> <script-name-without.sh>
```

Uninstall a project with

```bash
lazy-badger --delete <project-name>
```

Happy lazy development! :)
