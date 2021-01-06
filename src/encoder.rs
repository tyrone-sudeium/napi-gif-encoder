use std::fs::File;
use std::{
    convert::{TryFrom, TryInto},
    mem::swap,
};

use engiffen::{engiffen, Image, Quantizer};
use napi::*;
use napi_derive::js_function;
pub struct Encoder {
    width: u16,
    height: u16,
    framerate: u16,
    output_path: String,
    images: Vec<Ref<JsBufferValue>>,
}

impl Encoder {
    pub fn new(width: u16, height: u16, output_path: &str) -> Self {
        Encoder {
            width,
            height,
            framerate: 25,
            output_path: output_path.to_string(),
            images: Vec::new(),
        }
    }
}

pub fn create_js_class(env: &Env) -> Result<JsFunction> {
    env.define_class(
        "GIFEncoder",
        encoder_constructor,
        &[
            Property::new(&env, "addFrame")?.with_method(add_frame),
            Property::new(&env, "setFrameRate")?.with_method(set_framerate),
            Property::new(&env, "finish")?.with_method(finish),
        ],
    )
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("The provided argument was too large")]
    ArgumentTooLarge(#[from] std::num::TryFromIntError),
    #[error("GIF encoder encountered an error")]
    EncoderError(#[from] engiffen::Error),
}

impl From<Error> for napi::Error {
    fn from(value: Error) -> Self {
        Self {
            status: napi::Status::GenericFailure,
            reason: value.to_string(),
        }
    }
}

// JS function: constructor(width: number, height: number, file: string)
#[js_function(3)]
fn encoder_constructor(ctx: CallContext) -> Result<JsUndefined> {
    let width32: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
    let height32: u32 = ctx.get::<JsNumber>(1)?.try_into()?;
    let width = u16::try_from(width32).map_err(Error::ArgumentTooLarge)?;
    let height = u16::try_from(height32).map_err(Error::ArgumentTooLarge)?;
    let file_path = ctx.get::<JsString>(2)?.into_utf8()?;
    let encoder = Encoder::new(width, height, file_path.as_str()?);
    let mut this = ctx.this_unchecked::<JsObject>();
    ctx.env.wrap(&mut this, encoder)?;
    ctx.env.get_undefined()
}

// JS function: addFrame(frame: Buffer)
#[js_function(1)]
fn add_frame(ctx: CallContext) -> Result<JsUndefined> {
    let this = ctx.this_unchecked::<JsObject>();
    let encoder = ctx.env.unwrap::<Encoder>(&this)?;
    let data = ctx.get::<JsBuffer>(0)?.into_ref()?;
    encoder.images.push(data);

    ctx.env.get_undefined()
}

// JS function: setFrameRate(framerate: number)
#[js_function(1)]
fn set_framerate(ctx: CallContext) -> Result<JsUndefined> {
    let this = ctx.this_unchecked::<JsObject>();
    let encoder = ctx.env.unwrap::<Encoder>(&this)?;
    let fps32: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
    let fps = u16::try_from(fps32).map_err(Error::ArgumentTooLarge)?;
    encoder.framerate = fps;

    ctx.env.get_undefined()
}

struct RenderTask {
    width: u16,
    height: u16,
    framerate: u16,
    output_path: String,
    images: Vec<Ref<JsBufferValue>>,
}

impl RenderTask {
    fn release_refs(self, env: Env) -> Result<()> {
        for imgref in self.images {
            imgref.unref(env)?;
        }
        Ok(())
    }
}

impl Task for RenderTask {
    type Output = ();
    type JsValue = JsUndefined;

    fn compute(&mut self) -> Result<Self::Output> {
        let mut imgs: Vec<Image> = Vec::new();
        for data in &self.images {
            let pixels = data
                .chunks_exact(4)
                .map(|pixel| <[u8; 4]>::try_from(pixel).unwrap())
                .collect();
            let img = Image {
                pixels: pixels,
                width: u32::from(self.width),
                height: u32::from(self.height),
            };
            imgs.push(img);
        }
        // let start = std::time::SystemTime::now();
        let gif = engiffen(&imgs, self.framerate.into(), Quantizer::NeuQuant(10))
            .map_err(Error::EncoderError)?;
        let mut file = File::create(&self.output_path)?;
        gif.write(&mut file).map_err(Error::EncoderError)?;
        // let end = std::time::SystemTime::now();
        // println!("{:?}", end.duration_since(start));
        Ok(())
    }

    fn resolve(self, env: Env, _output: Self::Output) -> Result<Self::JsValue> {
        self.release_refs(env)?;
        env.get_undefined()
    }

    fn reject(self, env: Env, err: napi::Error) -> Result<Self::JsValue> {
        self.release_refs(env)?;
        Err(err)
    }
}

// JS function: finish(): Promise<void>
#[js_function(0)]
fn finish(ctx: CallContext) -> Result<JsObject> {
    let this = ctx.this_unchecked::<JsObject>();
    let encoder = ctx.env.unwrap::<Encoder>(&this)?;
    let mut images: Vec<Ref<JsBufferValue>> = Vec::new();
    // let file_str = encoder.output_path.as_str()?;
    swap(&mut images, &mut encoder.images);
    let task = RenderTask {
        width: encoder.width,
        height: encoder.height,
        framerate: encoder.framerate,
        output_path: encoder.output_path.clone(),
        images,
    };
    ctx.env
        .spawn(task)
        .map(|async_task| async_task.promise_object())
}
