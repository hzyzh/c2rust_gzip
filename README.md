# c2rust_gzip
Applying c2rust to transpile source codes of gzip to unsafe rust version.

Under c_dir/ are original source code files of gzip in C.

Under rust_dir/ are transpiled Rust code files generated by directly applying C2Rust.

This Rust version gzip is compilable and runnable, which is supposed to preserve the behaviour of C version.
