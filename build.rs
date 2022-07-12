use std::io;
#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
use winres::WindowsResource;

fn main() -> io::Result<()> {
    {
        if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") {
            WindowsResource::new()
                .set_icon("./src/images/dorothy.ico")
                .compile()?;
        }
    }
    Ok(())
}
