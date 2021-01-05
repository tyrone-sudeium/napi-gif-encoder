#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{Env, JsObject, Result};

mod encoder;

#[cfg(all(unix, not(target_env = "musl"), not(target_arch = "aarch64")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(all(windows, target_arch = "x86_64"))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject, env: Env) -> Result<()> {
  let encoder = encoder::create_js_class(&env)?;
  exports.set_named_property("GIFEncoder", encoder)?;
  Ok(())
}
