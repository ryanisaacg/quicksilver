use super::{new_wasm_error, web_try};
use futures_util::future::{poll_fn, ready, TryFutureExt};
use std::{
    future::Future,
    io::Error as IOError,
    task::{Context, Poll},
};
use stdweb::{
    traits::*,
    unstable::TryInto,
    web::{
        event::{ProgressAbortEvent, ProgressLoadEvent},
        ArrayBuffer, TypedArray, XhrReadyState, XhrResponseType, XmlHttpRequest,
    },
    Reference,
};

pub fn make_request(path: &str) -> impl Future<Output = Result<Vec<u8>, IOError>> {
    ready(create_request(path)).and_then(|xhr| {
        let mut have_set_handlers = false;
        poll_fn(move |ctx| poll_request(&xhr, ctx, &mut have_set_handlers))
    })
}

fn create_request(path: &str) -> Result<XmlHttpRequest, IOError> {
    let xhr = XmlHttpRequest::new();
    web_try(xhr.open("GET", path), "Failed to create a GET request")?;
    web_try(
        xhr.set_response_type(XhrResponseType::ArrayBuffer),
        "Failed to set the response type",
    )?;
    web_try(xhr.send(), "Failed to send a GET request")?;
    Ok(xhr)
}

fn poll_request(
    xhr: &XmlHttpRequest,
    ctx: &mut Context,
    have_set_handlers: &mut bool,
) -> Poll<Result<Vec<u8>, IOError>> {
    if !*have_set_handlers {
        *have_set_handlers = true;
        let waker = ctx.waker().clone();
        xhr.add_event_listener(move |_: ProgressLoadEvent| waker.wake_by_ref());
        let waker = ctx.waker().clone();
        xhr.add_event_listener(move |_: ProgressAbortEvent| waker.wake_by_ref());
    }
    let status = xhr.status();
    let ready_state = xhr.ready_state();
    match (status / 100, ready_state) {
        (2, XhrReadyState::Done) => {
            let reference: Reference = xhr
                .raw_response()
                .try_into()
                .expect("The response will always be a JS object");
            Poll::Ready(
                reference
                    .downcast::<ArrayBuffer>()
                    .map(|arr| TypedArray::<u8>::from(arr).to_vec())
                    .ok_or_else(|| new_wasm_error("Failed to cast file into bytes")),
            )
        }
        (2, _) => Poll::Pending,
        (0, _) => Poll::Pending,
        _ => Poll::Ready(Err(new_wasm_error("Non-200 status code returned"))),
    }
}
