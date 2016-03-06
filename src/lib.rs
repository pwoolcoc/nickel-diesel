#![feature(reflect_marker)]

extern crate nickel;
extern crate r2d2;
extern crate diesel;
extern crate r2d2_diesel;
extern crate plugin;
extern crate typemap;

pub use middleware::{ DieselMiddleware, DieselRequestExtensions };

mod middleware;
