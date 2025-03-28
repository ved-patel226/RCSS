<div id="toc">
  <ul style="list-style: none">
    <summary>
      <p align="center">
        <img src="./assets/logo/128.png" alt="RCSS Logo" width="150">
      </p>
    </summary>
  </ul>
</div>

<div align="center">
  <div id="toc">
    <ul style="list-style: none">
      <summary>
        <h1>Rusty Cascading Style Sheets (RCSS)</h1>
      </summary>
    </ul>
  </div>
</div>


---

<p align="center">
  <a aria-label="License" href="https://github.com/ved-patel226/rcss/blob/master/LICENSE"><img alt="" src="https://img.shields.io/npm/l/turbo.svg?style=for-the-badge&labelColor=000000&color="></a>
  <a aria-label="Join the community on GitHub" href="https://github.com/ved-patel226/rcss/discussions"><img alt="" src="https://img.shields.io/badge/Join%20the%20community-blueviolet.svg?style=for-the-badge&logo=turborepo&labelColor=000000&logoWidth=20&logoColor=white"></a>
  <a aria-label="Crates.io Package" href="https://crates.io/crates/rcss-css"><img alt="" src="https://img.shields.io/crates/v/rcss-css?style=for-the-badge"></a>
</p>

> [!TIP]
> Download the **[VSCode extension](https://marketplace.visualstudio.com/items?itemName=rcss-syntax-highlighting.rcss)** for syntax highlighting!

**Rusty Cascading Style Sheets (RCSS)** is a styling language that brings Rust-inspired syntax to CSS. It combines the robustness of Rust with SASS-like features such as nesting and variables for cleaner, more maintainable styles.

```rcss
let variable: "#FFFFFF";
let breakpoint: "768px";

fn padding() {
  padding: 20px;
}

.container {
    padding();

    h2 {
        color: blue;
    }
}

h4 {
    width: 50%;
    color: green;
}


/* MOBILE STYLES */

@media screen and (max-width: &breakpoint) {
  .container {
      width: 100%;
  }

  h4 {
      width: 100%;
  }
}
```

> [!NOTE]
> The above RCSS code compiles to CSS in around **572.40µs**!

---

<div id="toc">
  <ul style="list-style: none">
    <summary>
      <h2> Installation </h2>
    </summary>
  </ul>
</div>

First, if you don't have Cargo (Rust's package manager) installed, you can install it by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Then, install:

```bash
cargo install rcss-css
```

> [!WARNING]
> If you encounter the following warning:
>
> ```bash
> warning: be sure to add `/home/<YourUsername>/.cargo/bin` to your PATH to be able to run the installed binaries
> ```
>
> ### **For Linux Users**
>
> Add the following line to your shell configuration file (e.g., `.bashrc`, `.zshrc`, etc.):
>
> ```bash
> export PATH="$HOME/.cargo/bin:$PATH"
> ```
>
> Reload your shell configuration to apply the changes:
>
> ```bash
> source ~/.bashrc
> ```
>
> ### **For Windows Users**
>
> 1. Open the Start Menu and search for "Environment Variables."
> 2. Click on "Edit the system environment variables."
> 3. In the **System Properties** window, click the **Environment Variables** button.
> 4. Under **System variables**, locate the `Path` variable and click **Edit**.
> 5. Add the following path to the list:
>
>    ```
>    C:\Users\<YourUsername>\.cargo\bin
>    ```
>
> 6. Click **OK** to save your changes.
>
> Restart your terminal or command prompt to ensure the updated PATH is recognized.

---

<div id="toc">
  <ul style="list-style: none">
    <summary>
      <h2> Usage </h2>
    </summary>
  </ul>
</div>

RCSS expects a directory argument to watch. On file save, RCSS will compile automatically to `../css`

```bash
rcss-css styles/rcss
```

This command will compile `.rcss` files in `styles/rcss` into standard CSS files at `styles/css`.

---

<div id="toc">
  <ul style="list-style: none">
    <summary>
      <h2> Roadmap </h2>
    </summary>
  </ul>
</div>

### ✅ Phase 1: Core Features (Current)

- Implement Rust-like syntax parsing.
- Support variables and nesting.
- Support functions with no arguments
- Develop a VS Code extension with syntax highlighting.

### 🚧 Phase 2: Enhancements (Upcoming)

- Support functions with arguments
- Implement importing
- Add RCSS formatter
- Improve output css format
- Improve error handling and debugging tools.

### 🔮 Phase 3: Future Plans

- WASM support.

---

**Base logo** by [Dzuk](https://github.com/dzuk-mutant), licensed under [CC BY-NC-SA](https://creativecommons.org/licenses/by-nc-sa/4.0/). [Download the emoji set](https://rustacean.net/fan-art.html#fanart)

**RCSS** is licensed under the [MIT License](https://opensource.org/licenses/MIT).
