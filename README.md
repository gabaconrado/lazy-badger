# Lazy Badger

> Development environment utility, a way of centralizing arbitrary scripts
> into your workflow.

## Usage

Generate a new project namespace with

```bash
lazy-badger --new --path <project-workdir> <project-name>
```

This will create a new directory at `~/.lazy-badger/<project-name>/scripts`

Put your scripts inside this new generated directory and then you can
call them like that:

```bash
<project-name> <script-name-without.sh>
```

If your project have a `docker-compose.yml`, you can use the project name as alias

```bash
$ <project-name> up -d
...
$ <project-name> down
```

Uninstall a project with

```bash
lazy-badger --delete <project-name>
```

Happy lazy development! :)
