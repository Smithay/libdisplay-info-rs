use libdisplay_info::{cta::CTA, edid::ExtensionTag, info::Info};

fn main() {
    let edid = std::fs::read("/sys/class/drm/card1-DP-3/edid").unwrap();
    let info = Info::parse(&edid).unwrap();
    let edid = info.edid().unwrap();
    let cea_ext = edid
        .extensions()
        .iter()
        .find(|ext| ext.tag() == ExtensionTag::CEA)
        .unwrap();
    let cta = CTA::from_extension(cea_ext).unwrap();
    dbg!(cta.flags());
}
