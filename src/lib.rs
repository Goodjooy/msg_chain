pub use from_chain_derive::LoadFormMap;
pub use msg_chain_derive::MessageChain;
use std::collections::HashMap;

pub mod impls;

// data that contain in evry chain
#[derive(Debug, PartialEq, Clone)]
pub enum ChainMeta {
    Null,
    Str(String),
    Bool(bool),
    Num(Number),
    SubChains(Vec<ChainMeta>),
    Map(HashMap<&'static str, ChainMeta>),
    MapOwn(HashMap<String, ChainMeta>),
}

// differnt type of number for chain meta
#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    N(u64),
    T(i64),
    Float(f64),
}

/// Message Chain
pub trait MessageChain {
    fn get_type(&self) -> &'static str;
    fn get(&self, key: &str) -> Option<ChainMeta>;
    fn all_keys(&self) -> Vec<&'static str>;
    fn get_all(&self) -> Vec<(&'static str, ChainMeta)> {
        self.all_keys()
            .iter()
            .map(|f| (*f, self.get(f).unwrap()))
            .collect()
    }
}

pub trait LoadFormMap: Sized + MessageChain {
    fn load_from_map(map: &HashMap<String, ChainMeta>) -> Option<Self>;
    fn can_match(map: &HashMap<String, ChainMeta>) -> bool;
    fn type_eq(ty: &str) -> bool;
}

/// into Chain Meta
///  transfrom a type into Chain meta
pub trait IntoChainMeta {
    fn into_chain(&self) -> ChainMeta;
}

pub trait FromChainMeta: Sized {
    fn from_chain(chain: Option<&ChainMeta>) -> Option<Self>;
}

#[macro_export]
macro_rules! msg_loader_generate {
    ( $( $x:ty ),* ) => {
        pub fn message_chain_loader(map: &HashMap<String, ChainMeta>)->Option<Box<dyn MessageChain>>{
            $(
                if <$x>::can_match(map){
                    return Some(Box::new(<$x>::load_from_map(map)?));
                }
            )*

            return None
        }
    };
}

#[macro_export]
macro_rules! map_generate {
[ $($k:literal : $v:expr),* ]=> {
       {
           let mut temp = HashMap::<String,ChainMeta>::new();
            $(
                temp.insert($k.to_string(), $v.into_chain());
            )*

           temp
    }
    };
    ( $ty:ty => [$($k:ident : $v:expr),*])=>{
        {
            let mut temp = HashMap::<String,ChainMeta>::new();

            temp.insert("type".to_string(), stringify!($ty).into_chain());
            $(
                temp.insert(stringify!($k).to_string(), $v.into_chain());
            )*

            temp
        }
    };
    ($var:expr)=>{
        {
            let mut temp = HashMap::<String,ChainMeta>::new();

            temp.insert("type".to_string(), $var.get_type().into_chain());
            for data in $var.get_all(){
                temp.insert(data.0.to_string(), data.1);
            }

            temp
        }
    };
}

#[cfg(test)]
mod test {

    use super::*;
    #[derive(MessageChain, LoadFormMap, Debug, PartialEq)]
    struct Plain {
        text: Option<String>,
    }
    #[derive(MessageChain, LoadFormMap, Debug, PartialEq)]
    struct Image {
        imageId: Option<String>,
        url: Option<String>,
        path: Option<String>,
        base64: Option<String>,
    }
    #[test]
    fn test_named() {
        let msg = Plain {
            text: Some(String::from("Rust NB")),
        };

        // type
        assert_eq!(msg.get_type(), "Plain");
        // exist data
        assert_eq!(msg.get("text"), Some(ChainMeta::Str("Rust NB".to_string())));
        // not exist data
        assert_eq!(msg.get("a"), None);
        // all keys
        assert_eq!(msg.all_keys(), vec!["text"]);
        //all data
        assert_eq!(
            msg.get_all(),
            vec![("text", ChainMeta::Str("Rust NB".to_string()))]
        );
    }

    #[test]
    fn test_from_named() {
        let map = map_generate!(
            Plain=>
            [
                text: "好耶",
                id: 114145
            ]
        );
        let res = Plain::load_from_map(&map);

        assert_eq!(
            Some(Plain {
                text: Some("好耶".to_string())
            }),
            res
        )
    }
    #[test]
    fn test_from_named_fail() {
        let map: HashMap<String, ChainMeta> = HashMap::new();

        let res = Plain::load_from_map(&map);

        assert_eq!(None, res)
    }

    #[derive(MessageChain, LoadFormMap, PartialEq, Debug)]
    struct AtAll;

    #[test]
    fn test_unit() {
        let at_all = AtAll;

        // type
        assert_eq!(at_all.get_type(), "AtAll");
        // exist data
        assert_eq!(at_all.get("text"), None);
        // not exist data
        assert_eq!(at_all.get("a"), None);
        // all keys
        assert_eq!(at_all.all_keys(), Vec::<&'static str>::new());
        //all data
        assert_eq!(at_all.get_all(), vec![]);
    }

    #[test]
    fn test_from_unit() {
        let map = map_generate!(AtAll=>[]);

        let res = AtAll::load_from_map(&map);

        assert_eq!(Some(AtAll), res)
    }

    #[test]
    fn test_from_unit_fail() {
        let map: HashMap<String, ChainMeta> = HashMap::new();

        let res = AtAll::load_from_map(&map);

        assert_eq!(None, res)
    }

    msg_loader_generate!(Plain, AtAll, Image);

    #[test]
    fn test_msg_chain_picker() {
        let pla = Plain { text: None };

        let map = map_generate!(&pla);

        let res = message_chain_loader(&map).unwrap();
        let res = res.into_target::<Plain>().unwrap();

        assert_eq!(pla, res);
    }

    #[test]
    fn test_option_items() {
        let map: HashMap<String, ChainMeta> = map_generate!(
            Image=>
            [
                imageId: "{01E9451B-70ED-EAE3-B37C-101F1EEBF5B5}.mirai",
                url: Option::<String>::None
            ]
        );
        let map2=map_generate!["type":"AtAll"];

        let res: Box<dyn MessageChain> = message_chain_loader(&map).unwrap();
        //可以转换为指定类型
        let res: Image = res.into_target::<Image>().unwrap();

        assert_eq!(res,Image{
            imageId:Some("{01E9451B-70ED-EAE3-B37C-101F1EEBF5B5}.mirai".to_string()),
            url:None,
            path:None,
            base64:None
        })
    }
}
