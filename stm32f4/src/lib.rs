//! Peripheral access API for STM32F4 microcontrollers
//! (generated using [svd2rust])
//! [svd2rust]: https://github.com/japaric/svd2rust
//!
//! You can find an overview of the API here:
//! https://docs.rs/svd2rust/0.8.1/svd2rust/#peripheral-api
//!
//! For more details see the README here:
//! https://github.com/adamgreig/stm32-rs

#![feature(const_fn)]
#![feature(optin_builtin_traits)]
#![no_std]

extern crate cortex_m;
extern crate vcell;

#[cfg(feature = "stm32f401")]
pub mod stm32f401;

#[cfg(feature = "stm32f405")]
pub mod stm32f405;

#[cfg(feature = "stm32f407")]
pub mod stm32f407;

#[cfg(feature = "stm32f410")]
pub mod stm32f410;

#[cfg(feature = "stm32f411")]
pub mod stm32f411;

#[cfg(feature = "stm32f412")]
pub mod stm32f412;

#[cfg(feature = "stm32f413")]
pub mod stm32f413;

#[cfg(feature = "stm32f427")]
pub mod stm32f427;

#[cfg(feature = "stm32f429")]
pub mod stm32f429;

#[cfg(feature = "stm32f446")]
pub mod stm32f446;

#[cfg(feature = "stm32f469")]
pub mod stm32f469;
