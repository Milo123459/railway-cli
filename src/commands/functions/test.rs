use crate::queries::project::{ProjectProject, ProjectProjectEnvironmentsEdges};
use anyhow::bail;
use base64::prelude::*;
use similar::{ChangeTag, TextDiff};
use bollard::Docker;

use super::*;

pub async fn test(
    environment: &ProjectProjectEnvironmentsEdges,
    project: ProjectProject,
    args: Test,
) -> Result<()> {
    let d = Docker::connect_with_local_defaults().context("ensure that the docker engine is running locally")?;
    d.build_image(options, credentials, tar)
    Ok(())
}
