fn main() {
    // rust-embed only tracks the files it embedded, so a frontend built after the
    // last compile (or a web/dist that did not exist then) would go unnoticed.
    // Watching the directory makes Cargo rebuild when its contents change.
    println!("cargo:rerun-if-changed=web/dist");
}
