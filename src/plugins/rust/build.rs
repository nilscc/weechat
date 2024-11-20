use cmake::Config;

fn main() {
    let target = "weechat_rust";

    let dst = Config::new("../../..")
        // disable pretty much all other features
        .define("ENABLE_RUBY", "OFF")
        .define("ENABLE_PHP", "OFF")
        .define("ENABLE_PYTHON", "OFF")
        .define("ENABLE_IRC", "OFF")
        // etc...
        // pick our target
        .build_target(&target)
        .build();

    println!(
        "cargo:rustc-link-search=native={}/build/src/plugins/rust",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static={}", &target)
}
