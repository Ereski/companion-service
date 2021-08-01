//! Run services along with your executable. This is useful, for example, to
//! run and stop databases and message brokers automatically with `cargo test`.
//!
//! Automatic startup and shutdown of services relies on the [`ctor`](::ctor)
//! crate. Service registration is done with the [`linkme`] crate and the
//! [`SERVICES`](static@SERVICES) static. And each service is defined by an
//! object that implements the [`Service`] trait:
//!
//! ```rust
//! use linkme::distributed_slice;
//! use companion_service::{Service, SERVICES};
//!
//! struct Dummy;
//!
//! impl Service for Dummy {
//!   fn name(&self) -> &str {
//!     "dummy"
//!   }
//!
//!   fn start(&self) {
//!     print!("start!");
//!   }
//!
//!   fn stop(&self) {
//!     print!("stop!");
//!   }
//! }
//!
//! #[distributed_slice(SERVICES)]
//! static DUMMY: &(dyn Service + Sync) = &Dummy;
//! ```

use ctor::{ctor, dtor};
use linkme::distributed_slice;

/// The distributed slice handled by [`linkme`].
#[distributed_slice]
pub static SERVICES: [&'static (dyn Service + Sync)] = [..];

/// Trait for all services handled by this crate.
///
/// The intent is to provide a very generic interface for implementors that
/// handle the runtime of external programs (e.g. a database server).
pub trait Service {
    /// Get the name of this service. This name can be used to control the
    /// service through the toplevel functions: [`start`], [`stop`], and
    /// [`restart`].
    fn name(&self) -> &str;

    /// Starts the service. This is called once before `main`, and also as a
    /// result of the toplevel [`start`] function being called with the name of
    /// this service.
    fn start(&self);

    /// Stops the service. This is called once after `main`, and also as a
    /// result of the toplevel [`stop`] function being called with the name of
    /// this service.
    fn stop(&self);

    /// Restarts the service. This is called as a result of the toplevel
    /// [`restart`] function being called with the name of this service.
    fn restart(&self) {
        self.stop();
        self.start();
    }
}

/// Starts all services with the given name.
pub fn start(name: &str) {
    for service in SERVICES {
        if service.name() == name {
            service.start();
        }
    }
}

/// Stops all services with the given name.
pub fn stop(name: &str) {
    for service in SERVICES {
        if service.name() == name {
            service.stop();
        }
    }
}

/// Restarts all services with the given name.
pub fn restart(name: &str) {
    for service in SERVICES {
        if service.name() == name {
            service.restart();
        }
    }
}

#[ctor]
fn init() {
    for service in SERVICES {
        service.start();
    }
}

#[dtor]
fn deinit() {
    for service in SERVICES {
        service.stop();
    }
}
