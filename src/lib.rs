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
    MapOwn(HashMap<String,ChainMeta>)
}
#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    N(u64),
    T(i64),
    Float(f64),
}

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
}

pub trait IntoChainMeta {
    fn into_chain(&self) -> ChainMeta;
}

pub trait FromChainMeta: Sized {
    fn from_chain(chain: &ChainMeta) -> Option<Self>;
}

#[macro_export]
macro_rules! msg_loader_generate {
    ( $( $x:ty ),* ) => {
        pub fn message_chain_loader(map: &HashMap<String, ChainMeta>)->Option<Box<dyn MessageChain>>{
            $(
                if let Some(value)=<$x>::load_from_map(map){
                    return Some(Box::new(value));
                }
            )*

            return None
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
        let _t = vec![
            ("type".to_string(), "Plain".into_chain()),
            ("text".to_string(), "好耶".into_chain()),
        ];
        let mut map: HashMap<String, ChainMeta> = HashMap::new();

        let _t: Vec<_> = _t
            .iter()
            .map(|f| map.insert(f.clone().0, f.clone().1))
            .collect();

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

        assert_eq!(
            None,
            res
        )
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
        let _t = vec![("type".to_string(), "AtAll".into_chain())];
        let mut map: HashMap<String, ChainMeta> = HashMap::new();

        let _t: Vec<_> = _t
            .iter()
            .map(|f| map.insert(f.clone().0, f.clone().1))
            .collect();

        let res = AtAll::load_from_map(&map);

        assert_eq!(Some(AtAll), res)
    }

    #[test]
    fn test_from_unit_fail() {
        
        let map: HashMap<String, ChainMeta> = HashMap::new();

        let res = AtAll::load_from_map(&map);

        assert_eq!(
            None,
            res
        )
    }

    msg_loader_generate!(Plain,AtAll);

    #[test]
    fn test_msg_chain_picker() {
        let _t = vec![
            ("type".to_string(), "Plain".into_chain()),
            ("text".to_string(), "好耶".into_chain()),
        ];
        let mut map: HashMap<String, ChainMeta> = HashMap::new();

        let _t: Vec<_> = _t
            .iter()
            .map(|f| map.insert(f.clone().0, f.clone().1))
            .collect();

        let res=message_chain_loader(&map).unwrap();
        

        assert_eq!("Plain",res.get_type());
        assert_eq!("好耶".into_chain(),res.get("text").unwrap())
    }
}
