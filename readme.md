# JSON GRABBER
Nice utility to remember important commands, make scripts, straight from your CLI !!
</br>
(you may want to use jq for formatting it nicely)
> **_NOTE:_** This project was made with the knowlage gather in [this course](https://frontendmasters.com/courses/typescript-go-rust/) and is also in early development, expect bugs and weird quirks
## DEMO
![Image](https://github.com/user-attachments/assets/5c6b3288-d43c-4d05-b0a3-e2dc2b395050)

## Features
- Look up variables based on your current path and subpaths
- Search specific variables based on your current path and subpaths
- Save variables from a path
- Edit variables based on your current path and subpaths
- Remove variables based on your current path and subpaths

### Requirements
- Rust (1.80 or later)
- Cargo
- Terminal or your choice

## How to run
```bash
git clone https://github.com/LuisDavid01/json-grabber

cd json-grabber

cargo build --release

cd target/release
#Depends on your os (use .exe extension if on windows)
.\json-grabber <config> <pwd> <args>
```
## Options
### `--config`, `-c`
**Description:** Create or specify a custom configuration file

**Usage:**
```bash
./json-grabber --config /path/to/config.json
./json-grabber -c my-config.json
```

**Example:**
```bash
# Create a new config file
./json-grabber -c project-commands.json

# Use an existing config file
./json-grabber --config ~/.json-grabber/work-config.json
```

### `--pwd`
**Description:** Specify the current working directory context

**Usage:**
```bash
./json-grabber --pwd /path/to/directory [ACTION]
```

**Example:**
```bash
# Set context to a specific directory
./json-grabber --pwd /home/user/my-project add "npm start" "dev"

# Use current directory if not specified
./json-grabber add "cargo build" "build"
```

## Actions

### `add`
**Description:** Creates a new entry or edits an existing value

**Syntax:**
```bash
./json-grabber add <key> <value>
./json-grabber add "<command>" "<alias>"
```

**Examples:**
```bash
# Add a simple command
./json-grabber add "git status" "status"

# Add a complex command with flags
./json-grabber add "docker run -it --rm ubuntu:latest" "ubuntu-shell"

# Add a build command for current project
./json-grabber add "npm run build && npm run test" "build-and-test"
```

### `rm`
**Description:** Removes a value by its key/alias

**Syntax:**
```bash
./json-grabber rm <key>
./json-grabber rm <alias>
```

**Examples:**
```bash
# Remove a command by alias
./json-grabber rm "status"

# Remove a specific entry
./json-grabber rm "build-and-test"
```

### `get` 
**Description:** Retrieve a stored command by its alias

**Syntax:**
```bash
./json-grabber <key>
```




