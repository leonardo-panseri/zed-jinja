# Jinja Zed Extension
This zed extension provides support for Jinja templates in Zed.

Syntax highlighting is provided by [tree-sitter-jinja](https://github.com/cathaysia/tree-sitter-jinja).

Basic code actions are provided by [jinja-lsp](https://github.com/uros-5/jinja-lsp).

## Formatting
Zed does not provide a way to configure formatting directly from the extension yet.

To enable formatting with Prettier, add the following to your Zed `settings.json`:
```json
{
  "languages": {
    "Jinja": {
      "prettier": {
        "allowed": true,
        "plugins": ["prettier-plugin-jinja-template"],
        "parser": "jinja-template"
      }
    }
  }
}
```

## Language Server Configuration
You can configure `jinja-lsp` in `pyproject.toml`, `Cargo.toml`, or `jinja-lsp.toml`:
```toml
[jinja-lsp]
templates = "./templates"
backend = ["./src"]
lang = "python"
```
