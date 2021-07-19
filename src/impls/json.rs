use crate::{IntoChainMeta};
use serde_json::{Map, Value};

use crate::ChainMeta;

impl IntoChainMeta for Value {
 

    fn into_chain(&self) -> ChainMeta {
        match self {
            Value::Null => ChainMeta::Null,
            Value::Bool(b) => b.into_chain(),
            Value::Number(n) => {
                if n.is_u64(){
                    n.as_u64().into_chain()
                }else if n.is_i64(){
                    n.as_i64().into_chain()
                }else{
                    n.as_f64().into_chain()
                }
            },
            Value::String(s) => s.into_chain(),
            Value::Array(v) => {
               v.into_chain()
            },
            Value::Object(obj) => obj.into_chain(),
        }
    }
}

impl IntoChainMeta for Map<String,Value> {
    fn into_chain(&self) -> ChainMeta {
        let map=self.into_iter()
        .map(|f| (f.0.clone(),f.1.into_chain()))
        .collect();
        ChainMeta::MapOwn(map)
    }
}