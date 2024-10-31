use dotenv::dotenv;
use ic_cdk_bindgen::{Builder, Config};
use std::env;
use std::path::PathBuf;

fn main() {
    dotenv().ok();

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Cannot find manifest dir"));

    let xrc_did_path = manifest_dir.join("../xrc/declarations/xrc.did");
    let xrc_did_str = xrc_did_path.to_str().expect("Path invalid");
    unsafe { env::set_var("CANISTER_CANDID_PATH_XRC", xrc_did_str) };
    let mut builder = Builder::new();
    let mut xrc = Config::new("xrc");
    xrc.binding
        .set_type_attributes("#[derive(Debug, CandidType, Deserialize)]".into());
    builder.add(xrc);
    builder.build(Some(manifest_dir.join("src/canisters")));
}
