# smol_str

```bash
cargo build --release
./target/release/smol_str sample 2>! events.ldjson
./target/release/smol_str report events.ldjson
```
```
Read 1000 records
found 2023 events
⡁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⡏ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⡁ 82670.0
⠄                              ⡇                            ⠄
⠂                              ⡇                            ⠂
⡁                              ⡇                            ⡁
⠄                              ⡇              ⣀⣀⣀⣠⠤⠤⠤⠖⠒⠒⠚⠉⠉⠉⠅
⠂                              ⡇⣀⣀⣀⣠⠤⠤⠤⠖⠒⠒⠚⠉⠉⠉⠁             ⠂
⡁                              ⡏⠁                           ⡁
⠄                              ⡇                            ⠄
⠂                              ⡇                            ⠂
⡁                              ⡇                            ⡁
⠄              ⢸⡇              ⡇                            ⠄
⠂              ⢸⡇              ⡇                            ⠂
⡁              ⢸⡇⣀⣀⣀⣠⠤⠤⠤⠴⠒⠒⠒⠋⠉⠉⠁                            ⡁
⠄              ⢸⠉⠁                                          ⠄
⠂              ⢸                                            ⠂
⡁       ⡇      ⢸                                            ⡁
⠄       ⡧⠴⠒⠒⠒⠋⠉⠉                                            ⠄
⠂   ⣤   ⡇                                                   ⠂
⡁ ⢀ ⡟⠒⠋⠉⠁                                                   ⡁
⢄⣰⠼⠉⠁                                                       ⠄
⠉⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁ 8.0
0.0                                                      2023.0
     total events | 2023
      peak bytes  | 82.7 KB
     ----------------------------
     alloc events | 2012
     alloc bytes  | 115.7 KB
     ----------------------------
     freed events | 11
     freed bytes  | 49.0 KB
```
