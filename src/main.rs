use probe_rs::{Lister, Permissions};

fn main() {
    // Get a list of all available debug probes.
    let lister = Lister::new();

    let probes = lister.list_all();

    // Use the first probe found.
    let probe = probes[0].open(&lister).unwrap();

    // Attach to a chip.
    let mut session = probe
        .attach("nRF52840_xxAA", Permissions::new().allow_erase_all())
        .unwrap();

    // Select a core.
    let mut core = session.core(0).unwrap();

    // Halt the attached core.
    core.halt(std::time::Duration::from_millis(10)).unwrap();
}
