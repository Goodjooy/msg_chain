use std::collections::HashMap;

use crate::{ChainMeta, FromChainMeta, IntoChainMeta};

impl IntoChainMeta for String {
    fn into_chain(&self) -> ChainMeta {
        ChainMeta::Str(self.clone())
    }
}

impl FromChainMeta for String {
    fn from_chain(chain: &ChainMeta) -> Option<Self> {
        if let ChainMeta::Str(s) = chain {
            Some(s.clone())
        } else {
            None
        }
    }
}

impl IntoChainMeta for str {
    fn into_chain(&self) -> ChainMeta {
        ChainMeta::Str(self.to_string())
    }
}



impl<T> IntoChainMeta for Vec<T>
where
    T: IntoChainMeta,
{
    fn into_chain(&self) -> ChainMeta {
        ChainMeta::SubChains(self.iter().map(|t| t.into_chain()).collect())
    }
}

impl<T: FromChainMeta> FromChainMeta for Vec<T> {
    fn from_chain(chain: &ChainMeta) -> Option<Self> {
        if let ChainMeta::SubChains(v) = chain {
            let v = v
                .iter()
                .map(|f| T::from_chain(f))
                .filter(|f| match f {
                    Some(_) => true,
                    None => false,
                })
                .map(|f| f.unwrap())
                .collect::<Vec<T>>();
            Some(v)
        } else {
            None
        }
    }
}

impl<T: IntoChainMeta> IntoChainMeta for HashMap<&'static str, T> {
    fn into_chain(&self) -> ChainMeta {
        ChainMeta::Map(self.iter().map(|f| (*f.0, f.1.into_chain())).collect())
    }
}
impl<T: FromChainMeta> FromChainMeta for HashMap<&'static str, T> {
    fn from_chain(chain: &ChainMeta) -> Option<Self> {
        if let ChainMeta::Map(map) = chain {
            let map = map
                .iter()
                .map(|f| (*f.0, T::from_chain(f.1)))
                .filter(|f| match f.1 {
                    Some(_) => true,
                    None => false,
                })
                .map(|f| (f.0, f.1.unwrap()))
                .collect();

            Some(map)
        } else {
            None
        }
    }
}

impl<T: IntoChainMeta> IntoChainMeta for (T,) {
    fn into_chain(&self) -> ChainMeta {
        ChainMeta::SubChains(vec![self.0.into_chain()])
    }
}

impl<T: FromChainMeta> FromChainMeta for (T,) {
    fn from_chain(chain: &ChainMeta) -> Option<Self> {
        if let ChainMeta::SubChains(vc) = chain {
            if vc.len() == 1 {
                let v = T::from_chain(vc.get(0).unwrap())?;
                Some((v,))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<T: IntoChainMeta> IntoChainMeta for [T] {
    fn into_chain(&self) -> ChainMeta {
        ChainMeta::SubChains(self.iter().map(|f|f.into_chain()).collect())
    }
}

