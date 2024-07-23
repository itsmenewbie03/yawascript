# YawaScript ⚡ [![author/maintainer](https://img.shields.io/badge/by-itsmenewbie03-016eea.svg?logo=github&labelColor=181717&longCache=true&style=flat-square)](https://itsmenewbie03.github.io)

YawaScript an esoteric programming language inspired by [COW programming language](https://esolangs.org/wiki/COW) which is a variant of [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck).

## Commands

These keywords are 1 to 1 mapping of the Brainfuck commands.

| Keyword | Meaning                                                                                                                                                                              |
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `yawA`  | Increment the data pointer by one (to point to the next cell to the right).                                                                                                          |
| `yawa`  | decrement the data pointer by one (to point to the next cell to the left).                                                                                                           |
| `yawa`  | Increment the byte at the data pointer by one.                                                                                                                                       |
| `YAwa`  | Decrement the byte at the data pointer by one.                                                                                                                                       |
| `yawa`  | Output the byte at the data pointer.                                                                                                                                                 |
| `YAWA`  | Accept one byte of input, storing its value in the byte at the data pointer.                                                                                                         |
| `YAWa`  | If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.    |
| `yAWA`  | If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.[a] |
