use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use std::time::Instant;

use hyper::rt::Sleep;
use hyper::rt::Timer;
use wasmtimer::tokio as tokio_time;
use pin_project::pin_project;

/// A Timer that uses the tokio runtime.
#[non_exhaustive]
#[derive(Default, Clone, Debug)]
pub(crate) struct WasmTimer;

#[pin_project]
#[derive(Debug)]
struct WasmSleep {
    #[pin]
    inner: tokio_time::Sleep,
}


impl Timer for WasmTimer {
    fn sleep(&self, duration: Duration) -> Pin<Box<dyn Sleep>> {
        Box::pin(WasmSleep {
            inner: tokio_time::sleep(duration),
        })
    }

    fn sleep_until(&self, deadline: Instant) -> Pin<Box<dyn Sleep>> {
        // Box::pin(WasmSleep {
        //     inner: tokio_time::sleep_until(deadline.into()),
        // })
        unreachable!("aaaaa");
    }

    fn reset(&self, sleep: &mut Pin<Box<dyn Sleep>>, new_deadline: Instant) {
        if let Some(sleep) = sleep.as_mut().downcast_mut_pin::<WasmSleep>() {
            sleep.reset(new_deadline)
        }
    }
}

impl WasmTimer {
    /// Create a new WasmTimer
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Future for WasmSleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().inner.poll(cx)
    }
}

impl Sleep for WasmSleep {}

impl WasmSleep {
    fn reset(self: Pin<&mut Self>, deadline: Instant) {
        // self.project().inner.as_mut().reset(deadline.into());
        unreachable!("bbbbbb");
    }
}
