#![cfg_attr(
    feature = "track-caller",
    feature(async_fn_track_caller)
)]
#![allow(async_fn_in_trait)]

use std::{error::Error, future::Future, panic::Location, thread::sleep, time::Duration};

const INTERVAL_MS: Duration = Duration::from_millis(50);

pub trait RetryableResultFn<T> {
    fn unwrap_blocking(self) -> T;
}

impl<T, E: Error, F: FnMut() -> Result<T, E>> RetryableResultFn<T> for F {
    #[cfg_attr(
        feature = "track-caller",
        track_caller
    )]
    fn unwrap_blocking(mut self) -> T {
        let caller = Location::caller();
        let mut res = self();
        let mut err = None;

        loop {
            match res {
                Ok(o) => return o,
                Err(ref e) => {
                    let e = format!("{e:#?}");
                    if err.as_ref() != Some(&e) {
                        if cfg!(feature = "track-caller") {
                            println!(
                                "Error at {}:{}:{}: {e}, will block till success...",
                                caller.file(),
                                caller.line(),
                                caller.column()
                            );
                        } else {
                            println!("Error: {e}, will block till success...");
                        }
                        err = Some(e);
                    }
                    res = self();
                }
            }
        }
    }
}

pub trait RetryableResultAsyncFn<T> {
    async fn unwrap_res(self, wait: Option<Duration>) -> T;
}

impl<T, E: Error, Fut: Future<Output = Result<T, E>>, F: FnMut() -> Fut> RetryableResultAsyncFn<T> for F {
    #[cfg_attr(
        feature = "track-caller",
        track_caller
    )]
    async fn unwrap_res(mut self, wait: Option<Duration>) -> T {
        let caller = Location::caller();
        let mut res = self().await;
        let mut err: Option<String> = None;

        loop {
            match res {
                Ok(o) => return o,
                Err(ref e) => {
                    let e = format!("{e:#?}");
                    if err.as_ref() != Some(&e) {
                        if cfg!(feature = "track-caller") {
                            println!(
                                "Error at {}:{}:{}: {e}, will block till success...",
                                caller.file(),
                                caller.line(),
                                caller.column()
                            );
                        } else {
                            println!("Error: {e}, will block till success...");
                        }
                        err = Some(e);
                    }
                    res = self().await;
                }
            }
            sleep(wait.unwrap_or(INTERVAL_MS));
        }
    }
}

pub trait RetryableOptionFn<T> {
    fn unwrap_blocking(self) -> T;
}

impl<T, F: FnMut() -> Option<T>> RetryableOptionFn<T> for F {
    #[cfg_attr(
        feature = "track-caller",
        track_caller
    )]
    fn unwrap_blocking(mut self) -> T {
        let caller = Location::caller();
        let mut printed = false;

        loop {
            match self() {
                Some(v) => return v,
                None => {
                    if !printed {
                        if cfg!(feature = "track-caller") {
                            println!(
                                "None at {}:{}:{}, will block till Some...",
                                caller.file(),
                                caller.line(),
                                caller.column()
                            );
                        } else {
                            println!("None, will block till Some...");
                        }
                        printed = true;
                    }
                }
            }
        }
    }
}

pub trait RetryableOptionAsyncFn<T> {
    async fn unwrap_opt(self, wait: Option<Duration>) -> T;
}

impl<T, Fut: Future<Output = Option<T>>, F: FnMut() -> Fut> RetryableOptionAsyncFn<T> for F {
    #[cfg_attr(
        feature = "track-caller",
        track_caller
    )]
    async fn unwrap_opt(mut self, wait: Option<Duration>) -> T {
        let caller = Location::caller();
        let mut printed = false;

        loop {
            match self().await {
                Some(v) => return v,
                None => {
                    if !printed {
                        if cfg!(feature = "track-caller") {
                            println!(
                                "None at {}:{}:{}, will block till Some...",
                                caller.file(),
                                caller.line(),
                                caller.column()
                            );
                        } else {
                            println!("None, will block till Some...");
                        }
                        printed = true;
                    }
                }
            }
            sleep(wait.unwrap_or(INTERVAL_MS));
        }
    }
}
