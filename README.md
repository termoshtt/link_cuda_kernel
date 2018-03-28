HowTo: Compile CUDA with nvcc, and link to Rust through FFI
------------------------------------------------------------

Build `kernel.cu` (copied from CUDA sample) into `libvector_add.a` in `build.rs`:

```rust
extern crate cc;

fn main() {
    cc::Build::new()
        .cuda(true)
        .flag("-cudart=shared")
        .flag("-gencode")
        .flag("arch=compute_61,code=sm_61")
        .file("kernel.cu")
        .compile("libvector_add.a");
    println!("cargo:rustc-link-lib=cuda");
    println!("cargo:rustc-link-lib=cudart");
}
```

and link this host code into Rust executable:

```rust
#[link(name = "vector_add", kind = "static")]
extern "C" {
    fn vectorAdd_main();
}

fn main() {
    unsafe {
        vectorAdd_main();
    }
}
```
