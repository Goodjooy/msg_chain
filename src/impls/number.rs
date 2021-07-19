use std::u16;

use crate::{ChainMeta, IntoChainMeta};
use crate::{FromChainMeta, Number};

impl IntoChainMeta for u8 {
    fn into_chain(&self) -> ChainMeta {
        ChainMeta::Num(Number::N(*self as u64))
    }
}

impl IntoChainMeta for u16 {
    fn into_chain(&self) -> ChainMeta {
        ChainMeta::Num(Number::N(*self as u64))
    }
}

impl IntoChainMeta for u32 {
    fn into_chain(&self) -> ChainMeta {
        ChainMeta::Num(Number::N(*self as u64))
    }
}

impl IntoChainMeta for u64 {
    fn into_chain(&self) -> ChainMeta {
        ChainMeta::Num(Number::N(*self as u64))
    }
}

impl IntoChainMeta for i8 {
    fn into_chain(&self) -> ChainMeta {
        ChainMeta::Num(Number::T(*self as i64))
    }
}
impl IntoChainMeta for i16 {
    fn into_chain(&self) -> ChainMeta {
        ChainMeta::Num(Number::T(*self as i64))
    }
}
impl IntoChainMeta for i32 {
    fn into_chain(&self) -> ChainMeta {
        ChainMeta::Num(Number::T(*self as i64))
    }
}
impl IntoChainMeta for i64 {
    fn into_chain(&self) -> ChainMeta {
        ChainMeta::Num(Number::T(*self as i64))
    }
}

impl IntoChainMeta for f32 {
    fn into_chain(&self) -> ChainMeta {
        ChainMeta::Num(Number::Float(*self as f64))
    }
}

impl IntoChainMeta for f64 {
    fn into_chain(&self) -> ChainMeta {
        ChainMeta::Num(Number::Float(*self))
    }
}

impl IntoChainMeta for bool {
    fn into_chain(&self) -> ChainMeta {
        ChainMeta::Bool(*self)
    }
}

impl FromChainMeta for bool {
    fn from_chain(chain: &ChainMeta) -> Option<Self> {
        if let ChainMeta::Bool(b)=chain{
            Some(*b)
        }else {
            None
        }
    }
}

impl FromChainMeta for u8 {
    fn from_chain(chain: &ChainMeta) -> Option<Self> {
        if let ChainMeta::Num(n) = chain {
            if let Number::N(n) = n {
                let s = n.to_string();
                Self::from_str_radix(&s, 10).ok()
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl FromChainMeta for u16 {
    fn from_chain(chain: &ChainMeta) -> Option<Self> {
        if let ChainMeta::Num(n) = chain {
            if let Number::N(n) = n {
                let s = n.to_string();
                Self::from_str_radix(&s, 10).ok()
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl FromChainMeta for u32 {
    fn from_chain(chain: &ChainMeta) -> Option<Self> {
        if let ChainMeta::Num(n) = chain {
            if let Number::N(n) = n {
                let s = n.to_string();
                Self::from_str_radix(&s, 10).ok()
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl FromChainMeta for u64 {
    fn from_chain(chain: &ChainMeta) -> Option<Self> {
        if let ChainMeta::Num(n) = chain {
            if let Number::N(n) = n {
                Some(*n)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl FromChainMeta for i8 {
    fn from_chain(chain: &ChainMeta) -> Option<Self> {
        if let ChainMeta::Num(n) = chain {
            if let Number::T(n) = n {
                let s = n.to_string();
                Self::from_str_radix(&s, 10).ok()
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl FromChainMeta for i16 {
    fn from_chain(chain: &ChainMeta) -> Option<Self> {
        if let ChainMeta::Num(n) = chain {
            if let Number::T(n) = n {
                let s = n.to_string();
                Self::from_str_radix(&s, 10).ok()
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl FromChainMeta for i32 {
    fn from_chain(chain: &ChainMeta) -> Option<Self> {
        if let ChainMeta::Num(n) = chain {
            if let Number::T(n) = n {
                let s = n.to_string();
                Self::from_str_radix(&s, 10).ok()
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl FromChainMeta for i64 {
    fn from_chain(chain: &ChainMeta) -> Option<Self> {
        if let ChainMeta::Num(n) = chain {
            if let Number::T(n) = n {
                Some(*n)
            } else {
                None
            }
        } else {
            None
        }
    }
}
impl FromChainMeta for f32 {
    fn from_chain(chain: &ChainMeta) -> Option<Self> {
        if let ChainMeta::Num(n) = chain {
            if let Number::Float(n) = n {
                let s = n.to_string();
                let n:f32=s.parse().ok()?;
                Some(n)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl FromChainMeta for f64 {
    fn from_chain(chain: &ChainMeta) -> Option<Self> {
        if let ChainMeta::Num(n) = chain {
            if let Number::Float(n) = n {
                Some(*n)
            } else {
                None
            }
        } else {
            None
        }
    }
}
