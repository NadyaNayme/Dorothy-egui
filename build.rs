// #[cfg(not(target_arch = "wasm32"))]
use std::io;
// use winres::WindowsResource;
fn main() -> io::Result<()> {
    //     #[cfg(windows)]
    //     {
    //         WindowsResource::new()
    //             .set_icon("./src/images/dorothy.ico")
    //             .compile()?;
    //     }
    Ok(())
}
