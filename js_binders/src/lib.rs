use syn;
use quote::quote;
use proc_macro::TokenStream;

fn impl_js_event_emitter_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        use napi::{Env, JsObject, JsFunction, JsError, JsNumber, JsUnknown, Ref};
        use napi::bindgen_prelude::{ObjectFinalize};
        use std::{collections::{HashMap, VecDeque}, any::Any};

        #[napi]
        impl #name {
            #[napi]
            pub fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
        
        #[napi]
        impl #name {

            #[napi]
            pub fn emit(&mut self, env:Env, eventName: String, a1: Option<JsUnknown>, a2: Option<JsUnknown>, a3: Option<JsUnknown>, a4: Option<JsUnknown>, a5: Option<JsUnknown>) -> bool
            {
                self.emitter.emit(env, eventName, a1, a2, a3, a4, a5)
            }

            #[napi]
            pub fn eventNames(&self) -> Vec<String>
            {
                self.emitter.eventNames()
            }

            #[napi]
            pub fn getMaxListeners(&self) -> u32 
            {
                self.emitter.getMaxListeners()
            }

            #[napi]
            pub fn listeners(&self, eventName:String) -> u32
            {
                self.emitter.listeners(eventName)
            }

            #[napi]
            pub fn off(&mut self, env:Env, eventName: JsUnknown, callback: JsFunction) -> &Self
            {
                self.emitter.off(env, eventName, callback);
                self
            }

            #[napi]
            pub fn on(&mut self, env:Env, eventName:JsUnknown, callback: JsFunction) -> &Self
            {
                self.emitter.on(env, eventName, callback);
                self
            }

            #[napi]
            pub fn once(&mut self, env:Env, eventName: JsUnknown, listener: JsFunction) -> &Self
            {
                self.emitter.once(env, eventName, listener);
                self
            }

            #[napi]
            pub fn prependListener(&mut self, env:Env, eventName: JsUnknown, listener: JsFunction) -> &Self
            {
                self.emitter.prependListener(env, eventName, listener);
                self
            }

            #[napi]
            pub fn prependOnceListener(&mut self, env:Env, eventName: JsUnknown, listener: JsFunction) -> &Self
            {
                self.emitter.prependOnceListener(env, eventName, listener);
                self
            }

            #[napi]
            pub fn removeAllListeners(&mut self, eventName: Option<String>) -> &Self
            {
                self.emitter.removeAllListeners(eventName);
                self
            }

            #[napi]
            pub fn setMaxListeners(&mut self, n: JsNumber) -> &Self
            {
                self.emitter.setMaxListeners(n);
                self
            }

            #[napi]
            pub fn rawListeners(&self, env:Env, eventName: String) -> Vec<JsFunction>
            {
                self.emitter.rawListeners(env, eventName)
            }
        }
    };
    gen.into()
}



#[proc_macro_derive(JsEventEmitter)]
pub fn js_event_emitter_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_js_event_emitter_derive(&ast)
}