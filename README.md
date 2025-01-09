<p align="center">
<img src="imgs/banner.png" alt="Fabelis Banner" width="100%" />
</p>
<p align="center">
<h1 align="center">FABELIS.AI Characterfile</h1>
<p align="center">
A CLI tool to Generate & Iterate your Characterfiles.
</p>

<p align="center">
<a href="https://github.com/fabelis/characterfile"><img src="https://img.shields.io/github/stars/fabelis/characterfile?style=social" alt="stars - fabelis" /></a>
&nbsp;
<a href="https://docs.fabelis.ai"><img src="https://img.shields.io/badge/🤖 docs-Fabelis-blue.svg" /></a>
&nbsp;
</p>

✨ If you like Fabelis, please consider starring the repo!

## What is Characterfile?
Fabelis Characterfile is a Rust-based CLI tool for generating and iterating character files for AI agents. It provides a flexible framework for creating detailed character profiles that can be used across different AI providers and applications.

## High-level Features

- Support for multiple AI providers (Anthropic, Cohere, Gemini, OpenAI, Perplexity, XAI)
- Local file-based character storage
- PDF/TXT extraction capabilities for importing character data

## Quick Start

### Step 1: Clone the Repository
```bash
git clone git@github.com:fabelis/characterfile.git
```

### Step 2: Configuration
Create a `config.json` in the root directory:
```json
{
    "completion_provider": "anthropic",
    "output_file_name": "bob.json"
}
```
> 💡 **MUST READ:** Output files are generated in `out/characters/*.json` if you would like to iterate on an existing character place it there with the same `"output_file_name"`.

Create an `input.json` in the root directory:
```json
{
    "name": "Bob",
    "facts": [
        "He needs to shower more often",
        "He loves to laugh",
    ],
    "files": [
        "american-psycho-script.txt",
        "the-big-short.pdf"
    ]
}
```
> 💡 **MUST READ:** Input files are stored in `in/*.txt/pdf`. All strings provided in `"files"` **must** be in the `in` folder.

### Step 3: Environment Setup
Create a `.env` file based on `.env.example` and add necessary provider credentials: (This script only uses a **completion provider**)
```env
ANTHROPIC_API_KEY="your_key_here"
ANTHROPIC_COMPLETION_MODEL="claude-3-5-sonnet-latest"
```

### Step 4: Run the CLI
```bash
cargo run
```

### Step 5: Infinitely iterate
This tool allows you to follow up Characterfile generations with edits. On every generation the script will **auto-save** to the `"output_file_name"`. After looking at this output, respond to the CLI again if you want to direct the tool to tweak the saved character again! 

## Supported Integrations (more to come...)

| Completion Providers|
|:-----------------:|
| Anthropic |
| Cohere |
| Gemini |
| OpenAI |
| Perplexity |
| XAI |

## Looking For More?
**View Our Docs [here](https://docs.fabelis.ai)**
 - **[EXAMPLES](https://docs.fabelis.ai/examples)**
 - **[SUPPORT](https://docs.fabelis.ai/support)**

---
<p align="center">Built with 🤖 and ❤️ by the Fabelis Team</p>