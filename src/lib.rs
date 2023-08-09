extern crate rust_web_server;

use rust_web_server::plugins::{PluginDesc, PluginType};

#[allow(dead_code)]
#[no_mangle]
extern "C" fn get_desc() -> PluginDesc {
    PluginDesc {
        r#type: PluginType::ResponsePlugin,
        name: "rws-plugin",
        version: "0.1.0",
        description: "Sample plugin",
    }
}

#[allow(dead_code)]
#[no_mangle]
extern "C" fn modify_response(mut res: Vec<u8>) -> Vec<u8> {
    res
}