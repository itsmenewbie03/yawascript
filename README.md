# YawaScript ⚡ [![author/maintainer](https://img.shields.io/badge/by-itsmenewbie03-016eea.svg?logo=github&labelColor=181717&longCache=true&style=flat-square)](https://itsmenewbie03.github.io)

**YawaScript** is an esoteric programming language inspired by [COW programming language](https://esolangs.org/wiki/COW) which is a variant of [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck).

Like this project? **Leave a star**! ⭐⭐⭐⭐⭐

## ⚙️ Commands

These keywords are 1 to 1 mapping of the Brainfuck commands.

| Keyword | Meaning                                                                                                                                                                                |
| ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `yawA`  | Increment the data pointer by one (to point to the next cell to the right).                                                                                                            |
| `Yawa`  | Decrement the data pointer by one (to point to the next cell to the left).                                                                                                             |
| `yaWA`  | Increment the byte at the data pointer by one.                                                                                                                                         |
| `YAwa`  | Decrement the byte at the data pointer by one.                                                                                                                                         |
| `yawa`  | Output the byte at the data pointer.                                                                                                                                                   |
| `YAWA`  | Accept one byte of input, storing its value in the byte at the data pointer.                                                                                                           |
| `YAWa`  | If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching `yAWA` command. |
| `yAWA`  | If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching `YAWa` command. |

## 🚀 Examples

A simple "Hello, World!" program is available at [src/examples](https://github.com/itsmenewbie03/yawascript/tree/main/src/examples).
