# emoji-translator-rs 🐱‍💻🌍

[![npm version](https://img.shields.io/npm/v/emoji-translator-rs)](https://www.npmjs.com/package/emoji-translator-rs) [![license](https://img.shields.io/npm/l/emoji-translator-rs)](https://github.com/mrablfz05/emoji-translator/blob/main/LICENSE)

A fast and efficient **Emoji Translator** written in **Rust** and compiled to **WebAssembly (WASM)**. This package provides an easy way to convert emoji symbols into human-readable text and vice versa!

---

## 🚀 Installation

You can install the package via npm:

```sh
npm install emoji-translator-rs
```

## 💻 Usage
You can use the package by importing it and calling the translate function:

```sh
import { emoji_translator } from "emoji-translator-rs";

// Translate emoji to text
const result = emoji_translator("Hello World!");
console.log(result);  // Output: "Hello 🌍!"
```

## 🔧 API
### `emoji_translator(text: string): string`
### Parameters:

- `text` (string): A string containing emoji(s) to be translated. \
Returns:

- A `string` with emojis translated into human-readable text.

## 🌐 WebAssembly Support
This package uses `Rust`, `WebAssembly` and `wasm-pack` to provide a fast and lightweight translation of emojis. It’s designed to work both in the browser and in Node.js.

## 💡 Contributing
Contributions are welcome! If you’d like to contribute, feel free to fork the repository, create a branch, and submit a pull request. Please make sure to follow the code style and write tests for new features.

## 📝 License
This project is licensed under the MIT License - see the LICENSE file for details.

## 📄 Changelog
For a detailed history of changes, check the [CHANGELOG](https://github.com/mrablfz05/emoji-translator/blob/main/CHANGELOG.md).