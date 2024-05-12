# rhd
Rust Hex Dump. Trying to outperform [xxd](https://linux.die.net/man/1/xxd). 

Disclaimer: This is not a complete reimplementation, but a naive test project meant to learn the language.
## Instalation
```
$ git clone https://github.com/MarkelCA/rhd.git
$ cd rhd
$ cargo install --path .
```
## Usage
```
$ rhd --help
Rust Hex Dump. A simple hex dump utility written in Rust.

Usage: rhd [FILE_PATH]

Arguments:
  [FILE_PATH]  Input file to read from. If not provided reads from stdin

Options:
  -h, --help     Print help
  -V, --version  Print version
```
## Examples
Text file
```
$ rhd assets/text.txt
00000000: 5468 6520 7175 6963 6b20 6272 6f77 6e20  | The quick brown
00000010: 666f 7820 6a75 6d70 7320 6f76 6572 020a  | fox jumps over .
00000020: 7468 6520 6c61 7a79 2064 6f67 0a         | the lazy dog .
```

Binary file
```
$ rhd assets/binary
00000000: 504b 0034 0140 0000 0000 e78c 5058 0000  | PK..........PX..
00000010: 0000 0000 0000 0000 0000 0040 0000 6177  | ..............aw
00000020: 732f 504b 0034 0140 0000 0000 e78c 5058  | s/PK..........PX
00000030: 0000 0000 0000 0000 0000 0000 0090 0000  | ................
00000040: 6177 732f 6469 7374 2f50 04b3 0414 0000  | aws/dist/PK.....
00000050: 0008 00a3 8b50 58db fb9b e1f6 0050 00cf  | .....PX.........
00000060: 00f0 000b 0000 0061 7773 2f69 6e73 7461  | .......aws/insta
00000070: 6c6c 8557 6d4f db48 10fe 8c7f c59c 8972  | ll.WmO.H......r
00000080: 70c2 31e4 dbd1 82ce 8594 8b2e 4d50 1cda  | p.1.........MP..
00000090: ab50 652d f63a 5961 af2d af1d c815 fefb  | .Pe-.:Ya.-......
000000a0: cdae d78e ed84 924a 0576 e7e5 9967 5e76  | .......J.v...g^v
000000b0: 72f8 9bfd c0b8 2d56 c621 5c25 e926 63cb  | r.....-V.!\%.&c.
000000c0: 055e c3d3 b3a1 85ff 0fd9 4e4c fe4b f8c0  | U.........NL.K..
000000d0: 4fe2 1318 737f 0049 062c 1740 c290 458c  | O...s.I.,.@..E.
000000e0: e454 0cc0 8922 984b 03d1 732a 68b6 a6c1  | .T...".K=.s*h...
000000f0: c038 4483 13e6 532e 0680 005f 0686 f98a  | .8D...S.h...h...
00000100: 8293 121f 7fe8 9b13 f84a 33c1 012e c3c1  | ........J3.....
00000110: 291c 0491 535f 99c7 03f8 9e14 6825 261b  | ).I.S_......h%&.
00000120: e049 0e85 a068 0829 40cf 14e8            | .I...h..@...
```
It also allows piped in input:
```
$ echo -n -e '\x48\x65\x6c\x6c\x6f\x2c\x20\x57\x6f\x72\x6c\x64\x21' | rhd
00000000: 4865 6c6c 6f2c 2057 6f72 6c64 21          Hello, World!
```

## Todos
- [ ] Optimize the memory usage
- [ ] Unit tests
