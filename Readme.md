# Simple estimate file space usage - RIIR

A simple tool for size analyze files in your file system like `du` command.

# Actual Status

This is a initial implementation, for now we  have only a simple list with size of directories and the unique acceptabled param is directory path.

# Todo

1. Params rules; Like du https://linuxcommand.org/lc3_man_pages/du1.html
2. Is a good idea make async callers to improve perform?;
3. Exceptions tratatives;
4. Tests.

# Setup

Clone this repository and enter to new folder:

```
git clone https://github.com/sr2ds/du-rs && cd du-rs
```

Run with Cargo:
```
cargo watch -x run
```
 
----

Writing in Rust <3