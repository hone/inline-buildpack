use libcnb::{
    data::build_plan::BuildPlan,
    detect::{cnb_runtime_detect, DetectOutcome, GenericDetectContext},
};

fn main() {
    cnb_runtime_detect(detect)
}

fn detect(ctx: GenericDetectContext) -> anyhow::Result<DetectOutcome> {
    let buildplan = BuildPlan::new();

    let outcome = if ctx.app_dir().join("project.toml").exists() {
        DetectOutcome::Pass(buildplan)
    } else {
        DetectOutcome::Fail
    };

    Ok(outcome)
}
