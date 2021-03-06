use std::collections::HashMap;

use crate::{ChainMeta, IntoChainMeta, LoadFormMap, Number};
use crate::{FromChainMeta, MessageChain};
use serde::{ser::SerializeStruct, Serialize};

mod collection;
mod json;
mod number;

impl<T> IntoChainMeta for Option<T>
where
    T: IntoChainMeta,
{
    fn into_chain(&self) -> ChainMeta {
        match self {
            Some(t) => t.into_chain(),
            None => ChainMeta::Null,
        }
    }
}

impl<T: FromChainMeta> FromChainMeta for Option<T> {
    fn from_chain(chain: Option<&ChainMeta>) -> Option<Self> {
        if let Some(_) = chain {
            if let Some(ChainMeta::Null) = chain {
                Some(None)
            } else {
                Some(Some(T::from_chain(chain)?))
            }
        } else {
            Some(None)
        }
    }
}

impl IntoChainMeta for ChainMeta {
    fn into_chain(&self) -> ChainMeta {
        self.clone()
    }
}
impl FromChainMeta for ChainMeta {
    fn from_chain(chain: Option<&ChainMeta>) -> Option<Self> {
        Some(chain?.clone())
    }
}

impl ChainMeta {
    pub fn into_target<T: FromChainMeta>(&self) -> Option<T> {
        T::from_chain(Some(self))
    }
}

impl Serialize for ChainMeta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ChainMeta::Str(s) => serializer.collect_str(s),
            ChainMeta::Bool(b) => serializer.serialize_bool(*b),
            ChainMeta::Num(n) => match n {
                Number::N(p) => serializer.serialize_u64(*p),
                Number::T(n) => serializer.serialize_i64(*n),
                Number::Float(f) => serializer.serialize_f64(*f),
            },
            ChainMeta::Null => serializer.serialize_none(),
            ChainMeta::SubChains(sc) => sc.serialize(serializer),
            ChainMeta::Map(map) => map.serialize(serializer),
            ChainMeta::MapOwn(map) => map.serialize(serializer),
        }
    }
}

impl Serialize for dyn MessageChain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let all_data = self.get_all();
        let mut data = serializer.serialize_struct("chainMeta", all_data.len() + 1)?;
        data.serialize_field("type", &self.get_type())?;

        let _t = all_data
            .iter()
            .map(|f| data.serialize_field(f.0, &f.1))
            .collect::<Vec<_>>();

        data.end()
    }
}

impl dyn MessageChain {
    pub fn into_target<T: LoadFormMap>(&self) -> Option<T> {
        if T::type_eq(self.get_type()){

            let mut map = HashMap::new();
            map.insert("type".to_string(), self.get_type().into_chain());
            
            for (k, v) in self.get_all() {
                map.insert(k.to_string(), v);
            }
            T::load_from_map(&map)
        }else {
            None
        }
    }
}


#[cfg(test)]
mod test{
    use serde_json::to_string;

    use super::*;

    #[derive(MessageChain)]

    struct Text {
        txt:String,
        show_all:Option<bool>
    }

    #[test]
    fn test_chain_meta2json() {
     let meta=ChainMeta::Str(String::from("113"));
     let res=to_string(&meta);
     
     println!("{}",res.unwrap())
    }

    #[test]
    fn test_msg_chain_to_json() {
        let text=Text{txt:String::from("111"),show_all:None};
        let bo:Box<dyn MessageChain>=Box::new(text);
        let res=to_string(&bo);

        println!("{}",res.unwrap())
    }

    #[test]
    fn test_message_chains() {
        let text=Text{txt:String::from("111"),show_all:None};
        let text2=Text{txt:String::from("11122"),show_all:Some(false)};
        let bo:Box<dyn MessageChain>=Box::new(text);
        let bo2:Box<dyn MessageChain>=Box::new(text2);

        let res=vec![bo,bo2];
        let res=to_string(&res);

        println!("{}",res.unwrap())
        
    }
}