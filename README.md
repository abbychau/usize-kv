# usize-kv
The fastest, 64-bit data-chunk only, one-to-many key-value storage.

## use-case
It is specialized for one-to-many relation for NOSQL / key-less structure of data-storage.

## features

1. Compact and hashmap-level performance.
2. Theoriotically most compact data-store.
```
  Offset: 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F 	
00000000: 00 00 00 00 00 00 00 01 00 00 00 00 00 00 00 02    ................
00000010: 00 00 00 00 00 01 86 A0 00 00 00 00 00 03 0D 40    ...............@
00000020: 00 00 00 00 00 01 86 A0 00 00 00 00 00 03 0D 40    ...............@
00000030: FF FF FF FF FF FF FF FF 00 00 00 00 00 03 0D 40    ...............@
00000040: FF FF FF FF FF FF FF FF 00 00 00 00 00 03 0D 40    ...............@
00000050: FF FF FF FF FF FF FF FF 00 00 00 00 00 03 0D 40    ...............@
00000060: FF FF FF FF FF FF FF FF 00 00 00 00 00 03 0D 40    ...............@
00000070: FF FF FF FF FF FF FF FF 00 00 00 00 00 03 0D 40    ...............@
00000080: FF FF FF FF FF FF FF FF 00 00 00 00 00 03 0D 40    ...............@
00000090: FF FF FF FF FF FF FF FF 00 00 00 00 00 03 0D 40    ...............@
000000a0: FF FF FF FF FF FF FF FF 00 00 00 00 00 03 0D 40    ...............@
000000b0: FF FF FF FF FF FF FF FF 00 00 00 00 00 03 0D 40    ...............@
000000c0: FF FF FF FF FF FF FF FF 00 00 00 00 00 03 0D 40    ...............@
000000d0: FF FF FF FF FF FF FF FF 00 00 00 00 00 03 0D 40    ...............@
000000e0: FF FF FF FF FF FF FF FF 00 00 00 00 00 03 0D 40    ...............@
000000f0: FF FF FF FF FF FF FF FF 00 00 00 00 00 03 0D 40    ...............@
00000100: FF FF FF FF FF FF FF FF 00 00 00 00 00 03 0D 40    ...............@
00000110: FF FF FF FF FF FF FF FF 00 00 00 00 00 03 0D 40    ...............@
00000120: 00 00 00 01 00 00 00 00 00 00 00 00 00 00 00 00    ................
```
3. Read Lock-less and Read-Write isolation. That writing will never produce locks to harm reading performance.
![Read throughput](https://github.com/jonhoo/rust-evmap/raw/master/benchmark/read-throughput.png)
4. Total binary transaction from internal to TCP communication.
e.g. `00 00 00 00 00 00 00 00 FF FF FF FF FF FF FF FF 00 00 00 00 00 00 00 00` to get data from key `INT_MAX`.

## warning
This project is under development. It may not be even usable.