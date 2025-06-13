use crate::transport::channel::BoxFuture;
#[cfg(not(target_arch = "wasm32"))]
use hyper_util::rt::TokioExecutor;
use std::{future::Future, sync::Arc};

pub(crate) use hyper::rt::Executor;

#[derive(Clone)]
pub(crate) struct SharedExec {
    inner: Arc<dyn Executor<BoxFuture<'static, ()>> + Send + Sync + 'static>,
}

impl SharedExec {
    pub(crate) fn new<E>(exec: E) -> Self
    where
        E: Executor<BoxFuture<'static, ()>> + Send + Sync + 'static,
    {
        Self {
            inner: Arc::new(exec),
        }
    }

    pub(crate) fn tokio() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Self::new(TokioExecutor::new())
        }

        #[cfg(target_arch = "wasm32")]
        {
            Self::new(DummyExecutor)
        }
    }
}

impl<F> Executor<F> for SharedExec
where
    F: Future<Output = ()> + Send + 'static,
{
    fn execute(&self, fut: F) {
        self.inner.execute(Box::pin(fut))
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) struct DummyExecutor;

#[cfg(target_arch = "wasm32")]
impl<F> Executor<F> for DummyExecutor
where
    F: Future<Output = ()> + Send + 'static,
{
    fn execute(&self, _fut: F) {
        // Do nothing, this is a dummy executor.
        unreachable!("DummyExecutor should not be used in production code");
    }
}
