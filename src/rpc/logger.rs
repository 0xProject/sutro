//! Middleware to log requests and processing times.

use crate::prelude::*;
use futures::future::Either;
use humantime::Duration as HumanDuration;
use jsonrpc_core::{
    middleware,
    types::request::{Call, Request},
    FutureResponse, Metadata, Middleware, Output, Response,
};
use serde_json::to_string_pretty;
use std::{
    sync::atomic::{self, AtomicUsize},
    time::Instant,
};
use tracing::debug_span;
use tracing_futures::Instrument;

// See <https://github.com/paritytech/jsonrpc/blob/v15.1.0/core/examples/middlewares.rs>
#[derive(Default)]
pub struct Logger(AtomicUsize);

impl<M: Metadata> Middleware<M> for Logger {
    type CallFuture = middleware::NoopCallFuture;
    type Future = FutureResponse;

    fn on_request<F, X>(&self, request: Request, meta: M, next: F) -> Either<Self::Future, X>
    where
        F: FnOnce(Request, M) -> X + Send,
        X: Future<Output = Option<Response>> + Send + 'static,
    {
        let span = debug_span!("request");
        let _enter = span.enter();
        let start = Instant::now();
        let request_number = self.0.fetch_add(1, atomic::Ordering::SeqCst);
        let request_method = format_request(&request);
        debug!("Processing request {}: {}", request_number, request_method);
        trace!("Request {}", to_string_pretty(&request).unwrap_or_default());

        Either::Left(Box::pin(
            next(request, meta)
                .map(move |response| {
                    info!(
                        "Request {}: {} took {}",
                        request_number,
                        request_method,
                        HumanDuration::from(start.elapsed())
                    );
                    if let Some(Response::Single(Output::Failure(response))) = &response {
                        // TODO: Batch responses
                        warn!(
                            "Responding error {}",
                            to_string_pretty(&response).unwrap_or_default()
                        );
                    } else {
                        trace!(
                            "Response {}",
                            to_string_pretty(&response).unwrap_or_default()
                        );
                    }
                    response
                })
                .in_current_span(),
        ))
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
