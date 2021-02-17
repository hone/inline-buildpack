use anyhow::{anyhow, bail};
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
    let buildpacks = table
        .get("build")
        .ok_or_else(|| anyhow!(r#"project.toml did not have a "build" key"#))?
        .as_table()
        .ok_or_else(|| anyhow!(r#"the "build" key is not a table"#))?
        .get("buildpacks")
        .ok_or_else(|| anyhow!(r#"project.toml did not have a "build.buildpacks" key"#))?
        .as_array()
        .ok_or_else(|| anyhow!(r#""build.buildpacks" is not an array"#))?
        .into_iter()
        .map(|buildpack| {
            Ok(buildpack.as_table().ok_or_else(|| {
                anyhow!(r#"build.buildpacks element is not a table: {}"#, buildpack)
            })?)
        })
        .collect::<Result<Vec<&toml::map::Map<String, toml::Value>>, anyhow::Error>>()?;

    if let Some(script) = buildpacks
        .into_iter()
        .find(|buildpack| buildpack.contains_key("script"))
    {
        // just did this check above
        let script_table = script.get("script").unwrap();
        let script = script_table.clone().try_into::<Script>()?;

        let script_layer = ctx.layer("script")?;
        script.run(script_layer.as_path().join("script.sh"))?;
    } else {
        bail!(r#"project.toml did not have a "build.buildpacks.script""#);
    }

    Ok(())
}
