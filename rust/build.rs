use std::io;

fn main() -> io::Result<()> {
    #[cfg(all(windows, target_env = "msvc"))]
    static_vcruntime::metabuild();

    Ok(())
}
