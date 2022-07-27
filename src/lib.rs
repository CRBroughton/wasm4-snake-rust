#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod palette;

#[no_mangle]
fn start() {
    palette::set_palette([0xfbf7f3, 0xe5b083, 0x426e5d, 0x20283d]);
}

fn update() {

}
