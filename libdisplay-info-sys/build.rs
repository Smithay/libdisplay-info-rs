fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        // don't link against unavailable native lib in doc.rs builds
        return;
    }

    let native_lib = pkg_config::Config::new()
        .range_version("0.1.0".."0.3.0")
        .probe("libdisplay-info")
        .unwrap();
    let native_version = semver::Version::parse(&native_lib.version).unwrap();
    let v2 = semver::VersionReq::parse(">=0.2").unwrap();
    if v2.matches(&native_version) {
        println!("cargo:rustc-cfg=feature=\"v0_2\"");
    }
}
