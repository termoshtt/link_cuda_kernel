#[link(name = "vector_add", kind = "static")]
extern "C" {
    fn vectorAdd_main();
}

fn main() {
    unsafe {
        vectorAdd_main();
    }
}
