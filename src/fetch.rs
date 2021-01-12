/// Fetch raw blocks using RPC
use super::{Command, Options};
use crate::{prelude::*, rpc, rpc::EthereumRpcClient};
use bytesize::ByteSize;
use std::{
    io::Write,
    path::PathBuf,
    time::{Duration, Instant},
};
use tokio::fs::File;

/// On a sample of 1000 recent blocks this provided maximum compression while
/// not harming decompression performance.
///  0:  24016184 ->  13236459 (1.814), 150.2 MB/s ,2299.2 MB/s
///  1:  24016184 ->  14294272 (1.680), 357.1 MB/s ,2567.5 MB/s
///  2:  24016184 ->  13646204 (1.760), 274.1 MB/s ,2465.9 MB/s
///  3:  24016184 ->  13236459 (1.814), 155.7 MB/s ,2325.4 MB/s
///  4:  24016184 ->  12972168 (1.851), 139.7 MB/s ,2230.6 MB/s
///  5:  24016184 ->  12825522 (1.873),  50.4 MB/s ,2182.8 MB/s
///  6:  24016184 ->  12792421 (1.877),  42.5 MB/s ,2262.0 MB/s
///  7:  24016184 ->  12738316 (1.885),  38.9 MB/s ,2475.6 MB/s
///  8:  24016184 ->  12722610 (1.888),  33.2 MB/s ,2466.4 MB/s
///  9:  24016184 ->  12702180 (1.891),  27.2 MB/s ,2469.2 MB/s
/// 10:  24016184 ->  12557057 (1.913),  15.7 MB/s ,2330.7 MB/s
/// 11:  24016184 ->  12542556 (1.915),  14.9 MB/s ,2299.0 MB/s
/// 12:  24016184 ->  12530460 (1.917),  13.1 MB/s ,2274.7 MB/s
/// 13:  24016184 ->  12522726 (1.918),  12.9 MB/s ,2407.5 MB/s
/// 14:  24016184 ->  12504961 (1.921),  12.5 MB/s ,2315.9 MB/s
/// 15:  24016184 ->  12493264 (1.922), 10.10 MB/s ,2232.7 MB/s
/// 16:  24016184 ->  12467835 (1.926),  6.40 MB/s ,2218.2 MB/s  <----
/// 17:  24016184 ->  12320293 (1.949),  4.92 MB/s ,2014.0 MB/s
/// 18:  24016184 ->  12301849 (1.952),  4.48 MB/s ,1820.0 MB/s
/// 19:  24016184 ->  12278427 (1.956),  3.21 MB/s ,1843.9 MB/s
/// 20:  24016184 ->  12230695 (1.964),  3.05 MB/s ,1633.5 MB/s
/// 21:  24016184 ->  12229613 (1.964),  2.78 MB/s ,1605.8 MB/s
///
/// Better compression is likely possible using a more efficient format than RLP
/// (skipping transaction/uncle roots and block number. Delta encoding
/// timestamp, etc.)
const ZSTD_LEVEL: i32 = 16;

async fn fetch_batch(
    client: &EthereumRpcClient,
    file: PathBuf,
    start: u64,
    end: u64,
) -> AnyResult<()> {
    let tstart = Instant::now();
    let concurrent_requests = 12;
    let blocks = stream::iter(start..end)
        .map(|block_number| {
            let client = &client;
            async move {
                let block_rlp = client
                    .get_block_rlp(block_number)
                    .await
                    .map_err(|err| anyhow!("Error: {}", err))
                    .context("Fetching block rlp")?;
                let size = block_rlp.0.len() as u64;
                info!("Fetched block {} {}", block_number, ByteSize(size));
                AnyResult::<_>::Ok(block_rlp.0)
            }
        })
        .buffered(concurrent_requests)
        .try_collect::<Vec<_>>()
        .await?;
    let tfetched = Instant::now();
    info!(
        "Fetching {} blocks took {} ({} bps)",
        end - start,
        humantime::Duration::from(tfetched - tstart),
        (end - start) as f64 / (tfetched - tstart).as_secs_f64()
    );
    let mut file = std::fs::File::create(file)?;
    let mut encoder = zstd::stream::Encoder::new(file, ZSTD_LEVEL).unwrap();
    for block in blocks.iter() {
        encoder.write(block)?;
    }
    encoder.finish()?;
    let tcompress = Instant::now();
    info!(
        "Compressing {} blocks took {} ({} bps)",
        end - start,
        humantime::Duration::from(tcompress - tfetched),
        (end - start) as f64 / (tcompress - tfetched).as_secs_f64()
    );

    Ok(())
}

pub async fn fetch(url: String, file: PathBuf) -> AnyResult<()> {
    let client = rpc::client(&url)
        .await
        .context("Creating RPC client to fork from")?;

    let base = PathBuf::from("./blocks");

    // TODO: Re-org in batches of 100 000. Or maybe batches of 10MB?
    const BATCH_SIZE: u64 = 1000;
    for start in 10_000.. {
        let start = start * BATCH_SIZE;
        let end = start + BATCH_SIZE;
        let file = base.join(format!("{}.rlp.zstd", start));
        info!("Fetching blocks {}..{} to {}", start, end, file.display());
        fetch_batch(&client, file, start, end).await?;
    }
    Ok(())
}
