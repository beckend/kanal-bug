```shell
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/p-73e9ac9c-8c15-4a85-aa6a-679d60b19f96`
double free or corruption (out)
fish: Job 1, 'cargo run' terminated by signal SIGABRT (Abort)
```

```shell
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/p-73e9ac9c-8c15-4a85-aa6a-679d60b19f96`
munmap_chunk(): invalid pointer
fish: Job 1, 'cargo run' terminated by signal SIGABRT (Abort)
```

```shell
❯ cargo run --release
    Finished release [optimized] target(s) in 1.07s
     Running `target/release/p-73e9ac9c-8c15-4a85-aa6a-679d60b19f96`
free(): double free detected in tcache 2
fish: Job 1, 'cargo run --release' terminated by signal SIGABRT (Abort)
```
