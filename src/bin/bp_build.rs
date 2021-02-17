use inline_buildpack::script::Script;
use libcnb::build::{cnb_runtime_build, GenericBuildContext};
use std::fs;

const PROJECT_TOML_PATH: &str = "project.toml";

fn main() {
    cnb_runtime_build(build);
}

fn build(ctx: GenericBuildContext) -> anyhow::Result<()> {
    let project_toml = fs::read_to_string(PROJECT_TOML_PATH)?.parse::<toml::Value>()?;
    let table = project_toml.as_table().unwrap();
    let buildpacks = table.get("build").unwrap().as_array().unwrap();

    if let Some(inline) = buildpacks.into_iter().find(|buildpack| {
        let bp_table = buildpack.as_table().unwrap();
        bp_table.contains_key("script")
    }) {
        let inline_table = inline.as_table().unwrap();
        let script_table = inline_table.get("script").unwrap();
        let script = script_table.clone().try_into::<Script>()?;

        let script_layer = ctx.layer("script")?;
        script.run(script_layer.as_path().join("script.sh"))?;
    }

    Ok(())
}
