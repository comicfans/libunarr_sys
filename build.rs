use std::env;
fn main(){


    println!("cargo:rustc-flags=-L native=/usr/lib");
        
    println!("cargo:include={}", "/usr/include");

    println!("cargo:rustc-flags=-l dylib=unarr");
}
