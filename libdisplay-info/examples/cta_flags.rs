use libdisplay_info::{cta::CTA, edid::ExtensionTag, info::Info};

fn main() {
    let edid = std::fs::read("/home/cmeissl/Documents/EDIDs/AOC-2260").unwrap();
    let info = Info::parse(&edid).expect("failed to parse edid");
    if let Some(failure_message) = info.failure_msg() {
        eprintln!("{:?}", failure_message);
    }
    let edid = info.edid().expect("no edid");

    let Some(cea_ext) = edid
        .extensions()
        .iter()
        .find(|ext| ext.tag() == ExtensionTag::CEA)
    else {
        eprintln!("No cea extension block, assuming no audio");
        return;
    };
    let cta = CTA::from_extension(cea_ext).unwrap();
    if cta.flags().basic_audio {
        eprintln!("basic audio support");
    } else {
        eprintln!("no basic audio support");
    }
}
