<div id="toc">
  <ul style="list-style: none">
    <summary>
      <p align="center">
        <img src="./assets/logo/128.png" alt="RCSS Logo" width="150">
      </p>
    </summary>
  </ul>
</div>


<div id="toc">
  <ul style="list-style: none">
    <summary>
      <h1>
      Rusty Cascading Style Sheets (RCSS)
      </h1>
    </summary>
  </ul>
</div>

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

**Rusty Cascading Style Sheets (RCSS)** is a styling language that brings Rust-inspired syntax to CSS. It combines the robustness of Rust with SASS-like features such as nesting and variables for cleaner, more maintainable styles.

```rcss
let variable: "#FFFFFF";
let breakpoint: "768px";

fn padding() {
  padding: 20px;
}

h1 {
    color: &variable;
    
    h2 {
        color: blue;
    }
}

h4 {
    color: green;
}


/* MOBILE STYLES */

@media screen and (max-width: &breakpoint) {
    h1 {
        color: red;
        
        h2 {
            color: green;
        }
    }
    
    h4 {
        font-size: 14px;
    }
}
```

---
<div id="toc">
  <ul style="list-style: none">
    <summary>
      <h2> Features </h2>
    </summary>
  </ul>
</div>


- **Rust-Inspired Syntax:** Write styles with a familiar syntax.
- **Nesting:** Organize CSS rules hierarchically.
- **Variables:** Easily manage variables and color schemes.
- **Compiles to Human-Readable or Minified CSS:** Choose between readable CSS or a optimized, minified output.

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

### 🚧 Phase 2: Enhancements (Upcoming)
- Support functions with arguments
- Implement importing
- Add RCSS formatter
- Improve output css format
- Improve error handling and debugging tools.
- Develop a VS Code extension with syntax highlighting and code completion.

### 🔮 Phase 3: Future Plans
- WASM support.

---

**Base logo** by [Dzuk](https://github.com/dzuk-mutant), licensed under [CC BY-NC-SA](https://creativecommons.org/licenses/by-nc-sa/4.0/). [Download the emoji set](https://rustacean.net/fan-art.html#fanart)

**RCSS** is licensed under the [MIT License](https://opensource.org/licenses/MIT).