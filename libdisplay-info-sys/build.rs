fn main() {
    let deps = system_deps::Config::new().probe().unwrap();
    let native_lib = deps.get_by_name("libdisplay-info").unwrap();
    let native_version = semver::Version::parse(&native_lib.version).unwrap();
    let has_v2 = semver::VersionReq::parse(">=0.2")
        .unwrap()
        .matches(&native_version);
    if has_v2 {
        println!("cargo:rustc-cfg=feature=\"v0_2\"");
    }
}
