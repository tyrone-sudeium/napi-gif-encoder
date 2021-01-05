use std::{
  convert::{TryFrom, TryInto},
  num::TryFromIntError,
};
use std::{fs::File, io::Write};

use gif::{EncodingError, Repeat};
use napi::*;
pub struct Encoder<W: Write> {
  gif_encoder: gif::Encoder<W>,
  width: u16,
  height: u16,
  delay: u16,
}

impl<W: Write> Encoder<W> {
  pub fn new(w: W, width: u16, height: u16) -> std::result::Result<Self, EncodingError> {
    gif::Encoder::new(w, width, height, &[]).map(|enc| Encoder {
      gif_encoder: enc,
      width,
      height,
      delay: 4,
    })
  }
}

pub fn create_js_class(env: &Env) -> Result<JsFunction> {
  env.define_class("GIFEncoder", encoder_constructor, &vec![])
}

fn map_to_js_range_error(err: TryFromIntError) -> napi::Error {
  napi::Error {
    status: Status::GenericFailure,
    reason: format!("{}", err),
  }
}

fn encoding_err_to_js_err(err: EncodingError) -> napi::Error {
  napi::Error {
    status: Status::GenericFailure,
    reason: format!("{}", err),
  }
}

// JS function: constructor(width: number, height: number, file: string)
#[js_function(3)]
fn encoder_constructor(ctx: CallContext) -> Result<JsUndefined> {
  let width32: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
  let height32: u32 = ctx.get::<JsNumber>(1)?.try_into()?;
  let width = u16::try_from(width32).map_err(map_to_js_range_error)?;
  let height = u16::try_from(height32).map_err(map_to_js_range_error)?;
  let file_path = ctx.get::<JsString>(2)?.into_utf8()?;
  let image = File::create(file_path.as_str()?)?;
  let mut encoder = Encoder::new(image, width, height).map_err(encoding_err_to_js_err)?;

  // TODO: allow changing this repeat from JS
  encoder
    .gif_encoder
    .set_repeat(Repeat::Infinite)
    .map_err(encoding_err_to_js_err)?;

  let mut this = ctx.this_unchecked::<JsObject>();
  ctx.env.wrap(&mut this, encoder)?;
  ctx.env.get_undefined()
}

// JS function: addFrame(frame: Buffer)
#[js_function(1)]
fn add_frame(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  // TODO: How do you deal with the type argument? Right now it's always File, but
  // ideally it wouldn't always be ...
  let encoder = ctx.env.unwrap::<Encoder<File>>(&this)?;
  let data = ctx.get::<JsBuffer>(0)?.into_ref()?;

  let mut frame = gif::Frame::from_rgb(encoder.width, encoder.height, data.as_ref());
  frame.delay = 1;
  encoder
    .gif_encoder
    .write_frame(&frame)
    .map_err(encoding_err_to_js_err)?;
  ctx.env.get_undefined()
}

// JS function: setDelay(delay: number)
#[js_function(1)]
fn set_delay(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let encoder = ctx.env.unwrap::<Encoder<File>>(&this)?;
  let delay32: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
  let delay = u16::try_from(delay32).map_err(map_to_js_range_error)?;
  encoder.delay = delay;
  ctx.env.get_undefined()
}
