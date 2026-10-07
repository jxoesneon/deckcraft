# DeckCraft Studio

An open-source, sovereign slide presentation and visual deck authoring application built in pure Rust, powered by the **Martensite** GPU-accelerated retained-mode GUI engine.

![DeckCraft Studio on Martensite](brag/demo.gif)

## Architecture

- **`crates/ui-martensite`**: Sovereign retained-mode presentation UI with slide sorter strip, master templates, and presenter notes.
- **`crates/engine`**: Lossless PPTX reading/writing, widescreen vector layouts, and high-DPI GPU presentation mode.

## Legal & Compliance Notice

DeckCraft is an independent open-source presentation software. It is not affiliated with Microsoft Corporation. Microsoft, PowerPoint, and Office are trademarks of Microsoft Corporation. Slide sorter light tables and presentation mechanics are standard public domain paradigms.

## License

Dual-licensed under MIT OR Apache-2.0.
