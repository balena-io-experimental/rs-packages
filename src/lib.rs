#![deny(clippy::all)]
use futures::stream::Stream;


#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub struct JsToRustStreamString;

pub struct JsToRustStream<T>(Option<T>);

impl <T> Stream for JsToRustStream<T> {
    type Item = T;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        todo!()
    }
}

#[napi]
impl JsToRustStreamString {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {}
  }

  #[napi]
  pub fn get_string(&self, name: String) -> napi::Result<String> {
    Ok(format!("Hello {}!", name))
  } 

  #[napi]
  pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => Self::fibonacci(n - 1) + Self::fibonacci(n - 2),
    }
}
}