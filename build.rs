fn main() {
    println!("cargo:rustc-env=TARGET={}", std::env::var("TARGET").unwrap());
    if cfg!(target_arch="x86"){
        println!("cargo:rustc-env=ARCH=x86");
    }else if cfg!(target_arch="x86_64"){
        println!("cargo:rustc-env=ARCH=x86_64");
    }else if cfg!(target_arch="mips"){
        println!("cargo:rustc-env=ARCH=mips");
    }else if cfg!(target_arch="powerpc"){
        println!("cargo:rustc-env=ARCH=powerpc");
    }else if cfg!(target_arch="powerpc_64"){
        println!("cargo:rustc-env=ARCH=powerpc_64");
    }else if cfg!(target_arch="arm"){
        println!("cargo:rustc-env=ARCH=arm");
    }else if cfg!(target_arch="aarch64"){
        println!("cargo:rustc-env=ARCH=aarch64");
    }

    if cfg!(target_os="windows"){
        println!("cargo:rustc-env=OS=windows");
    }else if cfg!(target_os="macos"){
        println!("cargo:rustc-env=OS=macos");
    }else if cfg!(target_os="ios"){
        println!("cargo:rustc-env=OS=ios");
    }else if cfg!(target_os="linux"){
        println!("cargo:rustc-env=OS=linux");
    }else if cfg!(target_os="android"){
        println!("cargo:rustc-env=OS=android");
    }else if cfg!(target_os="freebsd"){
        println!("cargo:rustc-env=OS=freebsd");
    }else if cfg!(target_os="dragonfly"){
        println!("cargo:rustc-env=OS=dragonfly");
    }else if cfg!(target_os="bitrig"){
        println!("cargo:rustc-env=OS=bitrig");
    }else if cfg!(target_os="openbsd"){
        println!("cargo:rustc-env=OS=openbsd");
    }else if cfg!(target_os="netbsd"){
        println!("cargo:rustc-env=OS=netbsd");
    }

    if cfg!(unix) {
        println!("cargo:rustc-env=FAMILY=unix");
    }else if cfg!(windows) {
        println!("cargo:rustc-env=FAMILY=windows");
    }
}