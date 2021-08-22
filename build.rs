extern crate vcpkg;
#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    vcpkg::find_package("libmysql").unwrap();
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico");
    res.compile().unwrap();
}

#[cfg(linux)]
fn main() {
    vcpkg::find_package("libmysql").unwrap();
}

#[cfg(macos)]
fn main() {
    vcpkg::find_package("libmysql").unwrap();
}

