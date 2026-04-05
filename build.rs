fn main() {
    // tell cargo where memory.x is
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}", manifest_dir);
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper/");

    cc::Build::new()
        .compiler("arm-none-eabi-gcc")
        .file("wrapper/gpio_wrap.c")
        .file("wrapper/cmu_wrap.c")
        .file("wrapper/system_clock_stubs.c")
        .file("wrapper/emlib/src/em_gpio.c")
        .file("wrapper/emlib/src/em_cmu.c")
        .file("wrapper/emlib/src/em_core.c")
        .file("wrapper/emlib/src/em_emu.c")
        .include("wrapper")
        .include("wrapper/emlib/inc")
        .include("wrapper/CMSIS/Core/Include")
        .include("wrapper/device/EFR32MG24/Include")
        .include("common/inc")
        .define("EFR32MG24B220F1536IM48", None)
        .flag("-mthumb")
        .flag("-mfpu=fpv5-sp-d16")
        .flag("-mfloat-abi=hard")
        .flag("-std=c99")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .flag("-fno-exceptions")
        .flag("-fno-unwind-tables")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-sign-compare")
        .flag_if_supported("-Wno-unused-function")
        .compile("emlib_wrap");
}
