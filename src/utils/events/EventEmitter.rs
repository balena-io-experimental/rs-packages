use napi::{Env, JsObject, JsFunction, JsError, JsNumber, JsUnknown, Ref};
use napi::bindgen_prelude::{ObjectFinalize};
use std::{collections::{HashMap, VecDeque}, any::Any};

pub struct EventEmitter {
    maxListeners: u32,
    callbacks: HashMap<String, VecDeque<Ref<()>>>,
    callbacks_once: HashMap<String, VecDeque<Ref<()>>>,
    events:Vec<String>
}

pub trait JsEventEmitter {

    fn addListener(&mut self, env:Env, eventName: JsUnknown, listener: JsFunction) {
        self.on(env, eventName, listener);
    }

    fn removeListener(&mut self, env:Env, eventName: JsUnknown, listener: JsFunction)
    {
        self.off(env, eventName, listener);
    }

    fn listenerCount(&self, eventName: String) -> u32 
    {
        self.listeners(eventName)
    }


    fn emit(&mut self, env:Env, eventName: String, a1: Option<JsUnknown>, a2: Option<JsUnknown>, a3: Option<JsUnknown>, a4: Option<JsUnknown>, a5: Option<JsUnknown>) -> bool;
    fn eventNames(&self) -> Vec<String>;
    fn getMaxListeners(&self) -> u32;
    fn listeners(&self, eventName:String) -> u32;
    fn off(&mut self, env:Env, eventName: JsUnknown, callback: JsFunction) -> &Self;
    fn on(&mut self, env:Env, eventName:JsUnknown, callback: JsFunction) -> &Self;

    fn once(&mut self, env:Env, eventName: JsUnknown, listener: JsFunction) -> &Self;
    fn prependListener(&mut self, env:Env, eventName: JsUnknown, listener: JsFunction) -> &Self;
    fn prependOnceListener(&mut self, env:Env, eventName: JsUnknown, listener: JsFunction) -> &Self;
    fn removeAllListeners(&mut self, eventName: Option<String>) -> &Self;
    fn setMaxListeners(&mut self, n: JsNumber) -> &Self;
    fn rawListeners(&self, env:Env, eventName: String) -> Vec<JsFunction>;
}

impl JsEventEmitter for EventEmitter {
    fn emit(&mut self, env:Env, eventName: String, a1: Option<JsUnknown>, a2: Option<JsUnknown>, a3: Option<JsUnknown>, a4: Option<JsUnknown>, a5: Option<JsUnknown>) -> bool
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

    fn eventNames(&self) -> Vec<String> {
        self.events.to_vec()
    }

    fn getMaxListeners(&self) -> u32 {
        self.maxListeners
    }

    fn listeners(&self, eventName:String) -> u32 {
        println!("Listeners {}", eventName);
        if let Some(listeners) = self.callbacks.get(&eventName) {
            listeners.len() as u32
        }
        else { 0 }
    }

    fn off(&mut self, env: Env, eventName: JsUnknown, callback: JsFunction) -> &Self
    {
        let eventName = 
           if let Ok(en) = eventName.coerce_to_string() {
            en
        } else {
            env.create_string("Unknown type").unwrap()
        }.into_utf8().unwrap();
        let eventName=eventName.as_str().unwrap();

        println!("Off {}", eventName);
        if let Some(listeners) = self.callbacks.get_mut(eventName) {
            if let Some(position) = listeners.iter().position(|c| c.type_id() == callback.type_id()) {
                listeners.remove(position);
                self.emit(env, "removeListener".into(), Some(env.create_string(eventName).unwrap().into_unknown()), Some(callback.into_unknown()), None, None, None);
            }
        }
        self
    }

    fn on(&mut self, env:Env, eventName:JsUnknown, callback: JsFunction) -> &Self {
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

    fn once(&mut self, env:Env, eventName: JsUnknown, listener: JsFunction) -> &Self
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

    fn prependListener(&mut self, env:Env, eventName: JsUnknown, listener: JsFunction) -> &Self
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

    fn prependOnceListener(&mut self, env:Env, eventName: JsUnknown, listener: JsFunction) -> &Self
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

    fn removeAllListeners(&mut self, eventName: Option<String>) -> &Self {
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

    fn setMaxListeners(&mut self, n: JsNumber) -> &Self 
    {
        println!("Set Max listeners {}", n.get_int32().unwrap());
        if let Ok(n) = n.get_uint32() {
            self.maxListeners = n;
        }
        self
    }

    fn rawListeners(&self, env:Env, eventName: String) -> Vec<JsFunction> {
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

impl EventEmitter{
    pub fn new(maxListeners: Option<u32>) -> Self {
        let events : Vec<String> = vec![
            "newListener".into(),
            "removeListener".into(),
        ];
       
        let mut callbacks : HashMap<String, VecDeque<Ref<()>>> = HashMap::new();
        let mut callbacks_once : HashMap<String, VecDeque<Ref<()>>> = HashMap::new();
        events.iter().for_each(|e| {
            callbacks.insert(e.clone(), VecDeque::new());
            callbacks_once.insert(e.clone(), VecDeque::new());
        });
        Self { maxListeners: maxListeners.unwrap_or(10), callbacks, callbacks_once, events }
    }

    pub fn addEvent(&mut self, event: &String) {
        self.events.push(event.clone());
        self.callbacks.insert(event.clone(), VecDeque::new());
        self.callbacks_once.insert(event.clone(), VecDeque::new());
    }
}

// impl ObjectFinalize for EventEmitter{
//     fn finalize(mut self, env: Env) -> napi::Result<()> {
//         self.callbacks.iter_mut().for_each(|(_, cb)| {
//             cb.iter_mut().for_each(|cb| {cb.unref(env);});
//             cb.clear();
//         });
//         self.callbacks.clear();

//         self.callbacks_once.iter_mut().for_each(|(_, cb)| {
//             cb.iter_mut().for_each(|cb| {cb.unref(env);});
//             cb.clear();
//         });
//         self.callbacks_once.clear();

//         Ok(())
//     }
// }

