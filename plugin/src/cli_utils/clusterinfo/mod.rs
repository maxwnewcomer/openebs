use crate::cli_utils::CliArgs;
use plugin::ExecuteOperation;
use snafu::Snafu;

/// Cluster-info operations.
#[derive(clap::Subcommand, Debug)]
pub enum Operations {
    /// Gets information of all installed engines.
    Get(ClusterInfoArg),
}

#[async_trait::async_trait(?Send)]
impl ExecuteOperation for Operations {
    type Args = CliArgs;
    type Error = Error;

    async fn execute(&self, _cli_args: &CliArgs) -> Result<(), Error> {
        match self {
            Operations::Get(_cluster_info_arg) => {
                let _ = dummy_construct();
                todo!("Implementation pending for this command")
            }
        }
    }
}

#[derive(Debug, Clone, clap::Args)]
pub struct ClusterInfoArg {}

/// Temporary function to fix warning as snafu variant is not getting constructed.
fn dummy_construct() -> Result<(), Error> {
    Err(Error::Generic {
        source: anyhow::anyhow!("dummy"),
    })
}

/// Error for clusterinfo stem.
#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Generic error: {}", source))]
    Generic { source: anyhow::Error },
}
