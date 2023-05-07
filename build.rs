fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindgen::builder()
        .header("co2driver/scd4x_i2c.h")
        .header("co2driver/sensirion_common.h")
        .header("co2driver/sensirion_config.h")
        .header("co2driver/sensirion_i2c.h")
        .header("co2driver/sensirion_i2c_hal.h")
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindings.rs"))
        .unwrap();
    cc::Build::new()
        .file("co2driver/scd4x_i2c.h")
        .file("co2driver/scd4x_i2c.c")
        .file("co2driver/sensirion_i2c_hal.h")
        .file("co2driver/sensirion_i2c.h")
        .file("co2driver/sensirion_i2c.c")
        .file("co2driver/sensirion_i2c_hal.c")
        .file("co2driver/sensirion_config.h")
        .file("co2driver/sensirion_common.h")
        .file("co2driver/sensirion_common.c")
        .compile("co2driver")
}
