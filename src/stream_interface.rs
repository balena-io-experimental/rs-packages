use futures::sink::Sink;
use napi::{JsObject, JsFunction, JsError, JsNumber, JsUnknown, Ref, CallContext};
use napi::bindgen_prelude::{Env, ObjectFinalize};
use std::{collections::{HashMap, VecDeque}, any::Any};

#[napi(custom_finalize)]
pub struct JsSink {
    closed: bool,
    destroyed: bool,
    ended:bool,
    cork: i16,
    highWaterMark: u32,
    buffer: Vec<JsObject>,
    maxListeners: u32,
    callbacks: HashMap<String, VecDeque<Ref<()>>>,
    callbacks_once: HashMap<String, VecDeque<Ref<()>>>,
    events:Vec<String>
}

pub trait JsEventEmitter {
    fn addListener(&mut self, eventName: String, listener: JsFunction) {
        self.on(eventName, listener);
    }

    fn removeListener(&mut self, eventName: String, listener: JsFunction)
    {
        self.off(eventName, listener);
    }

    fn listenerCount(&self, eventName: String) -> JsNumber 
    {
        self.listeners(eventName)
    }


    fn emit(&mut self, eventName: String, args: Option<Vec<JsObject>>) -> bool;
    fn eventNames(&self) -> [String];
    fn getMaxListeners(&self) -> JsNumber;
    fn listeners(&self, eventName:String) -> JsNumber;
    fn off(&mut self, eventName: String, callback: JsFunction) -> &Self;
    fn on(&mut self, eventName:String, callback: JsFunction) -> &Self;

    fn once(&mut self, eventName: String, listener: JsFunction) -> &Self;
    fn prependListener(&mut self, eventName: String, listener: JsFunction) -> &Self;
    fn prependOnceListener(&mut self, eventName: String, listener: JsFunction) -> &Self;
    fn removeAllListeners(&mut self, eventName: Option<String>) -> &Self;
    fn setMaxListeners(&mut self, n: JsNumber) -> &Self;
    fn rawListeners(&self, eventName: String) -> Vec<JsFunction>;
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
            "newListener".into(),
            "removeListener".into(),
            "close".into(),
            "drain".into(),
            "error".into(),
            "finish".into(),
            "pipe".into(),
            "unpipe".into()
        ];
       
        let mut callbacks : HashMap<String, VecDeque<Ref<()>>> = HashMap::new();
        let mut callbacks_once : HashMap<String, VecDeque<Ref<()>>> = HashMap::new();
        events.iter().for_each(|e| {
            callbacks.insert(e.clone(), VecDeque::new());
            callbacks_once.insert(e.clone(), VecDeque::new());
        });
        Self {closed: false, destroyed:false, ended:false, cork:0, highWaterMark: 10, buffer: vec![], maxListeners:10, callbacks, callbacks_once, events}
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
    pub fn write(&mut self, chunk: JsUnknown, encoding: Option<String>, callback: Option<JsFunction>) -> bool {
        println!("Writting");
        true
    }
}


#[napi]
impl JsSink {
    #[napi]
    pub fn emit(&mut self, env:Env, eventName: String, a1: Option<JsUnknown>, a2: Option<JsUnknown>, a3: Option<JsUnknown>, a4: Option<JsUnknown>, a5: Option<JsUnknown>) -> bool
    {
        println!("Emit {}", eventName);
        let args = match (a1, a2, a3, a4, a5) {
            (None, _, _, _ , _) => vec![],
            (Some(a1), None, _, _, _) => vec![a1],
            (Some(a1), Some(a2), None, _, _) => vec![a1, a2],
            (Some(a1), Some(a2), Some(a3), None, _) => vec![a1, a2, a3],
            (Some(a1), Some(a2), Some(a3), Some(a4), None) => vec![a1, a2, a3, a4],
            (Some(a1), Some(a2), Some(a3), Some(a4), Some(a5)) => vec![a1, a2, a3, a4, a5],
        };
        let mut found = false;

        if let Some(callbacks) = self.callbacks_once.get_mut(&eventName) {
            callbacks.iter_mut().for_each(|mut callback| {
                env.get_reference_value::<JsFunction>(&callback).unwrap().call(None, args.as_slice());
                callback.unref(env);
                found = true;
            });

            callbacks.clear();
        }

        if let Some(callbacks) = self.callbacks.get(&eventName) {
            callbacks.iter().for_each(|callback| {
                env.get_reference_value::<JsFunction>(&callback).unwrap().call(None, args.as_slice());
                found = true;
            });
        }

        found
    }

    #[napi]
    pub fn eventNames(&self) -> Vec<String> {
        self.events.to_vec()
    }

    #[napi]
    pub fn getMaxListeners(&self) -> u32 {
        self.maxListeners
    }

    #[napi]
    pub fn listeners(&self, eventName:String) -> u32 {
        println!("Listeners {}", eventName);
        if let Some(listeners) = self.callbacks.get(&eventName) {
            listeners.len() as u32
        }
        else { 0 }
    }

    #[napi]
    pub fn off(&mut self, env: Env, eventName: String, callback: JsFunction) -> &Self
    {
        println!("Off {}", eventName);
        if let Some(listeners) = self.callbacks.get_mut(&eventName) {
            if let Some(position) = listeners.iter().position(|c| c.type_id() == callback.type_id()) {
                listeners.remove(position);
                self.emit(env, "removeListener".into(), Some(env.create_string(eventName.as_str()).unwrap().into_unknown()), Some(callback.into_unknown()), None, None, None);
            }
        }
        self
    }

    #[napi]
    pub fn on(&mut self, env:Env, eventName:JsUnknown, callback: JsFunction) -> &Self {
        let eventName = 
           if let Ok(en) = eventName.coerce_to_string() {
            en
        } else {
            env.create_string("Unknown type").unwrap()
        }.into_utf8().unwrap();
        let eventName=eventName.as_str().unwrap();
        
        println!("On {}", eventName);
        if let Some(listeners) = self.callbacks.get_mut(eventName.into()) {
            if listeners.len() < (self.maxListeners as usize) {
                let callbackRef = env.create_reference(&callback).unwrap();
                listeners.push_back(callbackRef);
                self.emit(env, "newListener".into(), Some(env.create_string(eventName).unwrap().into_unknown()), Some(callback.into_unknown()), None, None, None);
            }
        }
        self
    }

    #[napi]
    pub fn once(&mut self, env:Env, eventName: JsUnknown, listener: JsFunction) -> &Self
    {
        let eventName = 
           if let Ok(en) = eventName.coerce_to_string() {
            en
        } else {
            env.create_string("Unknown type").unwrap()
        }.into_utf8().unwrap();
        let eventName=eventName.as_str().unwrap();

        println!("Once {}", eventName);
        if let Some(listeners) = self.callbacks_once.get_mut(eventName.into()) {
            if listeners.len() < (self.maxListeners as usize) {
                let callback = env.create_reference(&listener).unwrap();
                listeners.push_back(callback);
            }
        }
        self
    }

    #[napi]
    pub fn prependListener(&mut self, env:Env, eventName: JsUnknown, listener: JsFunction) -> &Self
    {
        let eventName = 
           if let Ok(en) = eventName.coerce_to_string() {
            en
        } else {
            env.create_string("Unknown type").unwrap()
        }.into_utf8().unwrap();
        let eventName=eventName.as_str().unwrap();

        println!("Prepend {}", eventName);
        if let Some(listeners) = self.callbacks.get_mut(eventName.into()) {
            if listeners.len() < self.maxListeners as usize {
                let callback = env.create_reference(&listener).unwrap();
                listeners.push_front(callback);
                self.emit(env, "newListener".into(), Some(env.create_string(eventName).unwrap().into_unknown()), Some(listener.into_unknown()), None, None, None);
            }
        }
        self
    }

    #[napi]
    pub fn prependOnceListener(&mut self, env:Env, eventName: JsUnknown, listener: JsFunction) -> &Self
    {

        let eventName = 
           if let Ok(en) = eventName.coerce_to_string() {
            en
        } else {
            env.create_string("Unknown type").unwrap()
        }.into_utf8().unwrap();
        let eventName=eventName.as_str().unwrap();

        println!("PrependOnce {}", eventName);
        if let Some(listeners) = self.callbacks_once.get_mut(eventName.into()) {
            if listeners.len() < self.maxListeners as usize {
                let listener = env.create_reference(&listener).unwrap();
                listeners.push_front(listener);
            }
        }
        self
    }

    #[napi]
    pub fn removeAllListeners(&mut self, eventName: Option<String>) -> &Self {
        println!("Remove All listeners");
        if let Some(eventName) = eventName {
            if let Some(listeners) = self.callbacks.get_mut(&eventName) {
                listeners.clear()
            }
        }
        else {
            self.callbacks.iter_mut().for_each(|(_,listeners)| {listeners.clear()});
        }
        self
    }

    #[napi]
    pub fn setMaxListeners(&mut self, n: JsNumber) -> &Self 
    {
        println!("Set Max listeners {}", n.get_int32().unwrap());
        if let Ok(n) = n.get_uint32() {
            self.maxListeners = n;
        }
        self
    }

    #[napi]
    pub fn rawListeners(&self, env:Env, eventName: String) -> Vec<JsFunction> {
        println!("Raw listeners {}", eventName);
        if let Some(listeners) = self.callbacks.get(&eventName) {
            let res : Vec<JsFunction> = listeners.iter().map(|e| env.get_reference_value::<JsFunction>(e).unwrap()).collect();
            res
        }
        else {
            Vec::new()
        }
    }
}

impl ObjectFinalize for JsSink{
    fn finalize(mut self, env: Env) -> napi::Result<()> {
        self.callbacks.iter_mut().for_each(|(_, cb)| {
            cb.iter_mut().for_each(|cb| {cb.unref(env);});
            cb.clear();
        });
        self.callbacks.clear();

        self.callbacks_once.iter_mut().for_each(|(_, cb)| {
            cb.iter_mut().for_each(|cb| {cb.unref(env);});
            cb.clear();
        });
        self.callbacks_once.clear();

        Ok(())
    }
}