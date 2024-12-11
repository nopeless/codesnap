<img width="350" src="/doc/logo.png" />

<div>
  
  [![Lint CI](https://img.shields.io/github/actions/workflow/status/mistricky/CodeSnap/lint.yml?style=flat&label=Lint)](https://github.com/mistricky/CodeSnap/blob/main/.github/workflows/lint.yml)
  [![Code Style CI](https://img.shields.io/github/actions/workflow/status/mistricky/CodeSnap/lint.yml?style=flat&label=Code%20style)](https://github.com/mistricky/CodeSnap/blob/main/.github/workflows/lint.yml)
  [![Crates.io Version](https://img.shields.io/crates/v/CodeSnap?logo=rust&color=%232ecc71)](https://crates.io/crates/codesnap)
  [![Lint convention](https://img.shields.io/badge/wizardoc--commit--convention-%233498db?style=flat&logo=lintcode&logoColor=white&link=https%3A%2F%2Fgithub.com%2Fwizardoc%2Fcommitlint-wizardoc)](https://github.com/wizardoc/commitlint-wizardoc)
  
</div>


## CodeSnap
> [!WARNING]  
> This project is still in early stage, and may have some bugs

CodeSnap is a pure Rust tool for generate beautiful code snapshots, it directly use graphic engine to generate snapshots, which means the entire process is just matter of computation and rendering, without need for network or something like browser-based rendering solution.

Generally, you can directly use CLI tool provide by CodeSnap to generate code snapshots you want. Or CodeSnap also provide a library for you to integrate it into your own project, so you can generate code snapshots in your own way (See [Related projects](#) for more information).

<img src="https://github.com/user-attachments/assets/b8c9490f-ce17-4881-9d36-72e9c17bf34b" width="580px" />


## ✨ Features
- **Fast**: Pure Rust tool, generate code snapshot from graphic engine directly.
- **CLI tool**: CodeSnap provide a CLI tool for you to generate code snapshot directly from command line.
- **Library**: CodeSnap also provide a library for you to integrate it into your own project.
- **Line number**: Generate code snapshot with line number, it's really helpful if someone want to know the position of the code snippet.
- **Watermark**: Watermark can help make your code snapshot more personalized and interesting.
- **More beautiful themes**: The [Syntect](https://github.com/trishume/syntect) be treated as the syntax highlighterin CodeSnap, and it using [Sublime Text syntax definitions](https://www.sublimetext.com/docs/syntax.html#include-syntax) to highlight code, so basically you can use any theme that Sublime Text support.
- **Scale**: You can scale your code snapshot with a specific scale factor, CodeSnap will generate treble size snapshot by default to ensure the quality of the snapshot.
- **Beautiful background**: CodeSnap provide a beautiful background for your code snapshot, you can also customize the background color with solid color or gradient color.
- **Multiple snapshot format**: CodeSnap support multiple snapshot format, you can save snapshot as PNG, SVG and even HTML, or you want try ASCII code snapshot :)
- **Clipboard**: CodeSnap can copy snapshot to clipboard directly, or read code snippet from clipboard to generate snapshots.
- **Breadcrumb**: CodeSnap provide a breadcrumb for you to share your code snapshot with code path, it's really helpful if others want to know where the code snippet comes from.


## Getting started
CodeSnap provide two ways to use it, you can use it as a CLI tool or as a library in your own project.

### CLI
For CLI tool, you can install it via `cargo`:

**Cargo**
```bash
cargo install codesnap-cli
```

**Homebrew**
```bash
brew install mistricky/tap/CodeSnap
```

Use `codesnap` command to generate code snapshot:

```bash
# Run codesnap to generate code snapshot by providing code file
codesnap -f ./code_snippet.hs -o "./output.png"

# Run codesnap --help to see more information
codesnap -h
```

Read more about [codesnap cli]()

### Library
For library, add `CodeSnap` in your project using Cargo

```bash
cargo add codesnap
```

Use `CodeSnap` builder to generate code snapshot:

```rust
let snapshot = CodeSnap::default()
        .code("fn main() { println!(\"Hello, world!\"); }")
        .watermark(WatermarkBuilder::default().content("CodeSnap").build()?)
        .build()?
        .create_snapshot()?;

// Save snapshot to file
snapshot.png_data()?.save("output.png")?;

// Copy snapshot to clipboard 
snapshot.png_data()?.copy()?;
```

Read more about [codesnap library]()

## Related projects
- [codesnap](https://github.com/mistricky/CodeSnap/tree/main/core)
- [codesnap-cli](https://github.com/mistricky/CodeSnap/tree/main/cli)
- [codesnap.nvim](https://github.com/mistricky/codesnap.nvim)
- [codesnap.idea](https://github.com/RAOE/CodeSnap.idea)
- codesnap.vscode (Planning)
- codesnap.zed (Planning)


## License
MIT.
