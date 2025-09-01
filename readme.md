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

./json-grabber <config> <pwd> <args>
```
