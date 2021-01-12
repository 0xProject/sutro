use super::{fetch::fetch, Command, Options};
use crate::{prelude::*, rpc};

pub(super) async fn async_main(options: Options) -> AnyResult<()> {
    match options.command {
        Some(Command::Fetch { node, file }) => fetch(node, file).await,
        Some(Command::Chain { fork }) => chain(fork).await,
        None => unimplemented!(),
    }
}

async fn chain(url: String) -> AnyResult<()> {
    use crate::chain::ChainState;

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
    let addr = "0.0.0.0:8545".parse()?;
    let server = rpc::serve(&addr, rpc_handler)?;
    let server_stop = server.close_handle();
    let mut server_task = tokio::task::spawn_blocking(move || {
        info!("RPC server started on {}", addr);
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
