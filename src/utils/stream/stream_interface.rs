use futures::sink::Sink;
use js_binders::*;
use crate::events::EventEmitter::{EventEmitter, JsEventEmitter};

#[napi]
#[derive(JsEventEmitter)]
pub struct JsSink {
    closed: bool,
    destroyed: bool,
    ended:bool,
    cork: i16,
    highWaterMark: u32,
    buffer: Vec<JsObject>,
    emitter: EventEmitter
}



#[napi]
pub struct JsEventData {
    callbacks: HashMap<String, JsFunction>
}

impl Sink<JsObject> for JsSink {
    type Error=String;

    fn poll_ready(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn start_send(self: std::pin::Pin<&mut Self>, item: JsObject) -> Result<(), Self::Error> {
        todo!()
    }

    fn poll_flush(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn poll_close(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }
}

#[napi]
impl JsSink {
    #[napi(constructor)]
    pub fn new() -> Self {
        let events : Vec<String> = vec![
            "close".into(),
            "drain".into(),
            "error".into(),
            "finish".into(),
            "pipe".into(),
            "unpipe".into()
        ];
       
        let mut emitter = EventEmitter::new(Some(10));
        events.iter().for_each(|e| {emitter.addEvent(e);});
        Self {closed: false, destroyed:false, ended:false, cork:0, highWaterMark: 10, buffer: vec![], emitter}
    }

    // Request to buffer data instead of processing them directly
    #[napi]
    pub fn cork(&mut self) {
        self.cork += 1;
    }

    #[napi]
    pub fn destroy(&mut self, _error: Option<JsUnknown>) -> &Self {
        self
    }

    #[napi]
    pub fn closed(&self) -> bool {
        self.closed
    }

    #[napi]
    pub fn destroyed(&self) -> bool {
        self.destroyed
    }

    #[napi]
    pub fn end(&mut self, env:Env, chunk: Option<JsUnknown>, encoding: Option<String>, callback: Option<JsFunction>) -> &Self {
        if let Some(chunk) = chunk {
            self.write(env, chunk, encoding, None);
        }
        println!("Ended");
        
        self.ended = true;
        if let Some(cb) = callback {
            println!("Passed one callback");
            cb.call::<JsUnknown>(None, &[]).unwrap();
        }
        self.emit(env, "finish".into(), None, None, None, None, None);
        self
    }

    #[napi]
    pub fn setDefaultEncoding(&mut self, encoding: String) -> &Self {
        self
    }

    #[napi]
    pub fn uncork(&mut self) {
        if self.cork > 0 {
            self.cork -= 1;
        }
    }

    #[napi]
    pub fn writable(&self) -> bool {
        !(self.destroyed || self.closed)
    }

    #[napi]
    pub fn writableEnded(&self) -> bool {
        self.ended
    }

    #[napi]
    pub fn writableCorked(&self) -> i16 {
        self.cork
    }

    #[napi]
    pub fn errored(&self) -> Option<JsError> {
        None
    }

    #[napi]
    pub fn writableFinished(&self) -> bool {
        self.ended
    }

    #[napi]
    pub fn writableHighWaterMark(&self) -> u32 {
        self.highWaterMark
    }

    #[napi]
    pub fn writableLength(&self) -> u32 {
        self.buffer.len() as u32
    }

    #[napi]
    pub fn writableNeedDrain(&self) -> bool {
        self.buffer.len() > (self.highWaterMark as usize)
    }

    #[napi]
    pub fn writableObjectMode(&self) -> bool {
        true
    }

    #[napi]
    pub fn write(&mut self, env: Env, chunk: JsUnknown, encoding: Option<String>, callback: Option<JsFunction>) -> bool {
        if chunk.is_typedarray().unwrap()
        {
            let data = chunk;
        }
        true
    }

}


