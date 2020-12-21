//! Middleware to log requests and processing times.

use crate::prelude::*;
use jsonrpc_core::{
    futures::{future::Either, Future},
    middleware,
    types::request::{Call, Request},
    FutureResponse, Metadata, Middleware, Response,
};
use std::{
    sync::atomic::{self, AtomicUsize},
    time::Instant,
};

// See <https://github.com/paritytech/jsonrpc/blob/v15.1.0/core/examples/middlewares.rs>
#[derive(Default)]
pub struct Logger(AtomicUsize);

impl<M: Metadata> Middleware<M> for Logger {
    type CallFuture = middleware::NoopCallFuture;
    type Future = FutureResponse;

    fn on_request<F, X>(&self, request: Request, meta: M, next: F) -> Either<Self::Future, X>
    where
        F: FnOnce(Request, M) -> X + Send,
        X: Future<Item = Option<Response>, Error = ()> + Send + 'static,
    {
        let start = Instant::now();
        let request_number = self.0.fetch_add(1, atomic::Ordering::SeqCst);
        let request_method = format_request(&request);
        debug!("Processing request {}: {}", request_number, request_method);

        Either::A(Box::new(next(request, meta).map(move |res| {
            info!(
                "Request {}: {} took {:?}",
                request_number,
                request_method,
                start.elapsed()
            );
            res
        })))
    }
}

fn format_request(request: &Request) -> String {
    match request {
        Request::Single(call) => format_call(call),
        Request::Batch(calls) => {
            format!(
                "[{}]",
                calls.iter().map(format_call).collect::<Vec<_>>().join(", ")
            )
        }
    }
}

fn format_call(call: &Call) -> String {
    match call {
        Call::MethodCall(method) => method.method.clone(),
        Call::Notification(notification) => notification.method.clone(),
        Call::Invalid { .. } => "<invalid>".to_string(),
    }
}
