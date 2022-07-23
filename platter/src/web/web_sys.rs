use super::{new_wasm_error, web_try};
use futures_util::future::{poll_fn, ready, TryFutureExt};
use js_sys::Uint8Array;
use std::{
    future::Future,
    io::Error as IOError,
    task::{Context, Poll},
};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{XmlHttpRequest, XmlHttpRequestResponseType};

pub fn make_request(path: &str) -> impl Future<Output = Result<Vec<u8>, IOError>> {
    ready(create_request(path)).and_then(|xhr| {
        let mut have_set_handlers = false;
        poll_fn(move |ctx| poll_request(&xhr, ctx, &mut have_set_handlers))
    })
}

fn create_request(path: &str) -> Result<XmlHttpRequest, IOError> {
    let xhr = web_try(XmlHttpRequest::new(), "Failed to create an HTTP request")?;
    web_try(xhr.open("GET", path), "Failed to create a GET request")?;
    xhr.set_response_type(XmlHttpRequestResponseType::Arraybuffer);
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
        let wake_up = Closure::wrap(Box::new(move || waker.wake_by_ref()) as Box<dyn FnMut()>);
        let wake_up = wake_up.as_ref().unchecked_ref();
        xhr.set_onload(Some(&wake_up));
        xhr.set_onerror(Some(&wake_up));
    }
    let status = xhr
        .status()
        .expect("Failed to get the XmlHttpRequest status");
    let ready_state = xhr.ready_state();
    match (status / 100, ready_state) {
        (2, 4) => {
            // Done
            Poll::Ready(
                web_try(xhr.response(), "Failed to get HTTP response").map(|resp| {
                    let array = Uint8Array::new(&resp);
                    let mut buffer = vec![0; array.length() as usize];
                    array.copy_to(&mut buffer[..]);

                    buffer
                }),
            )
        }
        (2, _) => Poll::Pending,
        (0, _) => Poll::Pending,
        _ => Poll::Ready(Err(new_wasm_error("Non-200 status code returned"))),
    }
}
