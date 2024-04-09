use {
    std::{
        env,
        io,
    },
    winres::WindowsResource,
};

fn main() -> io::Result<()> {
    static_vcruntime::metabuild();

    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon("assets/terminal.ico")
            .compile()?;
    }
    Ok(())
}
