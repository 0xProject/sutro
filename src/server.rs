use crate::prelude::*;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::{convert::Infallible, net::SocketAddr};
use tokio_compat_02::FutureExt as TokioCompat;

async fn hello_world(_req: Request<Body>) -> std::result::Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World!\n".into()))
}

// Start server in separate function so we can call it with
// `tokio_compat_02::FutureExt::compat` since it uses Tokio 0.2.
#[allow(clippy::future_not_send)]
async fn start_server<F>(socket_addr: &SocketAddr, stop_signal: F) -> Result<()>
where
    F: Future<Output = ()>,
{
    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let service =
        make_service_fn(|_connection| async { Ok::<_, Infallible>(service_fn(hello_world)) });

    Server::bind(socket_addr)
        .serve(service)
        .with_graceful_shutdown(stop_signal)
        .await
        .context("Server error")
}

#[tracing::instrument]
pub async fn async_main() -> Result<()> {
    // Catch SIGTERM so the container can shutdown without an init process.
    let stop_signal = tokio::signal::ctrl_c().map(|_| {
        info!("SIGTERM received, shutting down.");
    });

    // List on all interfaces on port 8080
    let socket_addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    TokioCompat::compat(start_server(&socket_addr, stop_signal)).await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::prelude::assert_eq;
    use hyper::{body::to_bytes, Request};

    #[tokio::test]
    async fn test_hello_world() {
        let request = Request::new(Body::empty());
        let response = hello_world(request).await.unwrap();
        let bytes = to_bytes(response.into_body()).await.unwrap();
        assert_eq!(bytes.as_ref(), b"Hello, World!\n");
    }
}
#[cfg(feature = "bench")]
pub(super) mod bench {
    #[allow(clippy::wildcard_imports)]
    use super::*;
    use crate::bench::prelude::*;
    use hyper::body::to_bytes;

    pub(in super::super) fn group(c: &mut Criterion) {
        bench_hello_world(c);
    }

    fn bench_hello_world(c: &mut Criterion) {
        c.bench_function("bench_hello_world", |b| {
            b.iter(|| {
                block_on(async {
                    let request = Request::new(Body::empty());
                    let response = hello_world(request).await.unwrap();
                    let bytes = to_bytes(response.into_body()).await.unwrap();
                    drop(black_box(bytes));
                })
            })
        });
    }
}
