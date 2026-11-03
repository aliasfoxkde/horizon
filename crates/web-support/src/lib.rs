//! Web (wasm32) plumbing. See Cargo.toml — the crate's job is feature
//! unification: depending on getrandom with `js` here turns the browser
//! entropy backend on for every workspace member in a wasm build.
