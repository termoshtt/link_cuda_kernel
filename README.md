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

    /* Link CUDA Runtime (libcudart.so) */

    // Add link directory
    // - This path depends on where you install CUDA (i.e. depends on your Linux distribution)
    // - This should be set by `$LIBRARY_PATH`
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
    println!("cargo:rustc-link-lib=cudart");

    /* Optional: Link CUDA Driver API (libcuda.so) */

    // println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64/stub");
    // println!("cargo:rustc-link-lib=cuda");
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

```cuda
/** CUDA Kernel Device code */
__global__ void vectorAdd(const float *A, const float *B, float *C, int numElements) {
    int i = blockDim.x * blockIdx.x + threadIdx.x;
    if (i < numElements)
    {
        C[i] = A[i] + B[i];
    }
}

/** Host main routine */
extern "C" {  // To avoid demangle
int vectorAdd_main (void) {
    /* call kernel in CUDA/C++ way */
}
} // extern C
```
