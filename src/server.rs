use super::{Command, Options};
use crate::{
    prelude::*,
    rpc::{self, types::BlockNumber},
};

pub(super) async fn async_main(options: Options) -> AnyResult<()> {
    use crate::chain::ChainState;

    // Create a client
    let url = match options.command {
        Some(Command::Chain { fork }) => fork,
        _ => return Err(anyhow!("Unexpected subcommand")),
    };
    let client = rpc::client(&url).await?;
    let result = client
        .get_block_by_number(BlockNumber::Latest, true)
        .await
        .map_err(|err| anyhow!("Error: {}", err))
        .context("Fetching latest block")?;
    info!("Latest block: {:#?}", result);

    // Create a forked chain
    let chain = crate::chain::fork(&url).await.context("Forking chain")?;
    let block = chain.block();
    info!("Block info: {:#?}", block);

    // Create an RPC server
    let rpc_handler = rpc::RpcHandler {
        client_version: "sutro/0.0.0".into(),
        chain_id:       1337,
        gas_price:      U256::zero(),
    };
    let server = rpc::serve(rpc_handler)?;
    let server_stop = server.close_handle();
    let mut server_task = tokio::task::spawn_blocking(move || {
        info!("RPC server starting");
        server.wait();
    });

    // Catch SIGTERM so the container can shutdown without an init process.
    let stop_signal = tokio::signal::ctrl_c().map(|_| {
        info!("SIGTERM received, shutting down.");
    });

    tokio::select! {
        _ = &mut server_task => {},
        _ = stop_signal => {},
    };
    server_stop.close();
    server_task.await?;

    Ok(())
}
