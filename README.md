# 🖋️ encre-compile

A simple Rust CLI tool that **compiles class strings into CSS** using [`encre-css`](https://crates.io/crates/encre-css).
It’s useful for quickly previewing or generating CSS output from class utilities, similar to Tailwind-style class expansion.

---

## 🚀 Features

- 🧩 Converts simple class strings into CSS.
- ⚙️ Optional `--full` flag to include the full CSS configuration.
- 💡 Ideal for testing or debugging `encre-css` configurations.

---

## 🧰 Installation

You can build and install the CLI locally with Cargo:

```bash
cargo install --path .
```

---

## 🧑‍💻 Usage

```bash
encre-compile [OPTIONS] [CLASSES]...
```

### Arguments

| Argument | Description |
|-----------|-------------|
| `CLASSES` | Space-separated list of class names to compile. |

### Options

| Flag | Description |
|------|--------------|
| `-f`, `--full` | Build full CSS, including the basic configuration. |
| `-V`, `--version` | Print version info. |
| `-h`, `--help` | Print help information. |

---

## 🧾 Examples

### Generate CSS for specific classes

```bash
encre-compile "text-red-500" "bg-gray-100"
```

**Output:**
```css
.text-red-500 { color: #ef4444; }
.bg-gray-100 { background-color: #f3f4f6; }
```

### Include the full configuration

```bash
encre-compile --full "text-blue-600"
```

**Output:**
Full CSS including the base configuration plus your requested classes.

---

## ⚙️ How It Works

Under the hood, the CLI:
1. Parses command-line arguments using [`clap`](https://docs.rs/clap/latest/clap/).
2. Uses `encre-css` to generate CSS from a mock HTML snippet containing the given class list.
3. If `--full` is not provided, it diffs the generated CSS against a baseline config to output only relevant styles.

---

## 🧪 Example Workflow

```bash
# Basic class compilation
encre-compile "p-4" "text-center"

# Full output
encre-compile --full "bg-red-500" "hover:underline"
```

---

## 🧩 Dependencies

- [clap](https://crates.io/crates/clap) — for command-line argument parsing
- [encre-css](https://crates.io/crates/encre-css) — for CSS generation

---

## 📄 License

MIT © 2025 — Built with ❤️ using Rust.

---

## 🧠 Inspiration

This CLI was designed to be a lightweight companion to the [`encre-css`](https://crates.io/crates/encre-css) library, making it easier to test and visualize class-to-CSS compilation from the terminal.
