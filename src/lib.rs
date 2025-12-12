#![feature(async_fn_track_caller)]
#![allow(async_fn_in_trait)]
use std::{error::Error, future::Future, panic::Location, thread::sleep, time::Duration};

const INTERVAL_MS: Duration = Duration::from_millis(50);

pub trait RetryableResultFn<T> {
    fn unwrap_blocking(self) -> T;
}

impl<T, E: Error + Clone + PartialEq, F: FnMut() -> Result<T, E>> RetryableResultFn<T> for F {
    #[track_caller]
    fn unwrap_blocking(mut self) -> T {
        let caller = Location::caller();
        let mut res = self();
        let mut err = None;

        let handle_err = |err: &mut Option<E>, e: &E| {
            if err.as_ref() != Some(e) {
                println!(
                    "Error at {}:{}:{}: {e:#?}, will block till success...",
                    caller.file(),
                    caller.line(),
                    caller.column()
                );
                *err = Some(e.clone());
            }
        };

        loop {
            match res {
                Ok(o) => return o,
                Err(ref e) => {
                    handle_err(
                        &mut err, e,
                    );
                    res = self();
                }
            }
        }
    }
}

pub trait RetryableResultAsyncFn<T> {
    async fn unwrap_res(self, wait: Option<Duration>) -> T;
}

impl<T, E: Error + Clone + PartialEq, Fut: Future<Output = Result<T, E>>, F: FnMut() -> Fut> RetryableResultAsyncFn<T> for F {
    #[track_caller]
    async fn unwrap_res(mut self, wait: Option<Duration>) -> T {
        let caller = Location::caller();
        let mut res = self().await;
        let mut err = None;

        let handle_err = |err: &mut Option<E>, e: &E| {
            if err.as_ref() != Some(e) {
                println!(
                    "Error at {}:{}:{}: {e:#?}, will block till success...",
                    caller.file(),
                    caller.line(),
                    caller.column()
                );
                *err = Some(e.clone());
            }
        };

        loop {
            match res {
                Ok(o) => return o,
                Err(ref e) => {
                    handle_err(
                        &mut err, e,
                    );
                    res = self().await;
                }
            }
            sleep(wait.unwrap_or(INTERVAL_MS));
        }
    }
}
