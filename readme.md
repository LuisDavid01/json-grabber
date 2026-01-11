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
# Run on windows 
#.\jg <config> <pwd> <args>
# Run on Linux/MacOs
./jg <config> <pwd> <args>

```
## Options
### `--config`, `-c`
**Description:** Create or specify a custom configuration file

**Usage:**
```bash
./jg --config /path/to/config.json
./jg -c my-config.json
```

**Example:**
```bash
# Create a new config file
./jg -c project-commands.json

# Use an existing config file
./jg --config ~/.json-grabber/work-config.json
```

### `--pwd`
**Description:** Specify the current working directory context

**Usage:**
```bash
./jg --pwd /path/to/directory [ACTION]
```

**Example:**
```bash
# Set context to a specific directory
./jg --pwd /home/user/my-project add "npm start" "dev"

# Use current directory if not specified
./jg add "cargo build" "build"
```

## Actions

### `add`
**Description:** Creates a new entry or edits an existing value

**Syntax:**
```bash
./jg add <key> <value>
./jg add "<command>" "<alias>"
```

**Examples:**
```bash
# Add a simple command
./jg add "git status" "status"

# Add a complex command with flags
./jg add "docker run -it --rm ubuntu:latest" "ubuntu-shell"

# Add a build command for current project
./jg add "npm run build && npm run test" "build-and-test"
```

### `rm`
**Description:** Removes a value by its key/alias

**Syntax:**
```bash
./jg rm <key>
./jg rm <alias>
```

**Examples:**
```bash
# Remove a command by alias
./jg rm "status"

# Remove a specific entry
./jg rm "build-and-test"
```

### `get` 
**Description:** Retrieve a stored command by its alias

**Syntax:**
```bash
./jg <key>
```




