# msg_chain

QQ机器人消息接收，解析trait和自动实现宏

## 提供解析[QQBot MessageChain](https://github.com/project-mirai/mirai-api-http/blob/master/docs/api/MessageType.md#%E6%B6%88%E6%81%AF%E7%B1%BB%E5%9E%8B)的工具

* 使用举例

```json
{
    "type": "Image",
    "imageId": "{01E9451B-70ED-EAE3-B37C-101F1EEBF5B5}.mirai",  //群图片格式
    //"imageId": "/f8f1ab55-bf8e-4236-b55e-955848d7069f"      //好友图片格式
    "url": "https://xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    "path": null,
    "base64": null
}
```

对应的结构体为

```rust
#[derive(MessageChain, LoadFormMap)]
struct Image{
    imageId:Option<String>,
    url:Option<String>,
    path:Option<String>,
    base64:Option<String>,
}
```

然后，将`Image`和其他实现了`MessageChain`和`LoadFormMap`一起通过`msg_loader_generate!` 注册,构造函数`message_chain_loader`

```rust
///构造函数 message_chain_loader 用于生成MessageChain
msg_loader_generate!(Image,Plain,At,AtAll);
```

在获取数据后，通过`message_chain_loader`获取当前`MessageChain`对象*无匹配对象返回 `None`*, 可以通过 `into_target`转换为特定对象*转换不可行会返回 `None`*

```rust
let map : HashMap<String, ChainMeta> = map_generate!(
            Image=>
            [
                imageId: "{01E9451B-70ED-EAE3-B37C-101F1EEBF5B5}.mirai"
                url: None,
            ]
        );
let res : Box<dyn MessageChain> = message_chain_loader(&map).unwrap();
//可以转换为指定类型
let res : Image = res.into_target::<Image>().unwrap();

```

然后就可以快乐使用了

* 为了方便快捷得构造 `HashMap` 提供了相关宏`map_generate!`帮助构造,以下为使用方法

```rust
    //以下将会构造出
    //{
    //  "type" : ChainMeta::Str("AtAll")
    //  "id" : Chain::Num(Number::N(1141451919_u64))
    //}
    let map1 : HashMap<String, ChainMeta> = map_generate![
        "type":"AtAll",
        "id":1141451919_u64
        ];

    //以下将会构造出
    //{
    //  "type" : ChainMeta::Str("Image"),
    //  "imageId" : ChainMeta::Str("{01E9451B-70ED-EAE3-B37C-101F1EEBF5B5}.mirai"),
    //  "url" : ChainMeta::Null
    //}
    let map2: HashMap<String, ChainMeta> = map_generate!(
            Image=>
            [
                imageId: "{01E9451B-70ED-EAE3-B37C-101F1EEBF5B5}.mirai",
                url: Option::<String>::None
            ]
        );
    
    let t = Image{
            imageId:Some("{01E9451B-70ED-EAE3-B37C-101F1EEBF5B5}.mirai".to_string()),
            url:None,
            path:None,
            base64:None
        };
    //以下将会构造出
    //{
    //  "type" : ChainMeta::Str("Image"),
    //  "imageId" : ChainMeta::Str("{01E9451B-70ED-EAE3-B37C-101F1EEBF5B5}.mirai"),
    //  "url" : ChainMeta::Null
    //  "path" : ChainMeta::Null
    //  "base64" : ChainMeta::Null
    //}
    let map3: HashMap<String, ChainMeta> = map_generate!( &t );
```

* `MessageChain`自动实现
  * 类型为 `namedStruct` 或者 `Unit`
  * 内部变量全部都实现了`IntoChainMeta` 和 `FromChainMeta`
  * 为了方便处理，所有实现了`MessageChain`都会实现`Serialize`

## enums

* `ChainMeta`

```rust
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
```

* `Number`

```rust
#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    N(u64),
    T(i64),
    Float(f64),
}
```

## tarits

* `IntoChainMeta` 将自身转换为`ChainMeta`对象

```rust
pub trait IntoChainMeta {
    fn into_chain(&self) -> ChainMeta;
}
```

* `FromChainMeta` 将`ChainMeta`转换为自身对象

```rust
pub trait FromChainMeta: Sized {
    fn from_chain(chain: Option<&ChainMeta>) -> Option<Self>;
}
```

* `MessageChain`

```rust
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
```

* `LoadFormMap`

```rust
pub trait LoadFormMap: Sized + MessageChain {
    fn load_from_map(map: &HashMap<String, ChainMeta>) -> Option<Self>;
    fn can_match(map: &HashMap<String, ChainMeta>) -> bool;
    fn type_eq(ty: &str) -> bool;
}
```
