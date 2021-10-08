extern crate winres;

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("src\\SBMI.ico");
    res.compile().unwrap();
}