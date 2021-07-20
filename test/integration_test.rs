use crate::*;

msg_loader_generate!(Plain);

#[derive(MessageChain, LoadFormMap, Default, Debug, PartialEq)]
struct Plain {
    text: String,
}

#[test]
fn test_plain() {
    let map = map_generate!("Plain"("text":"好耶"));

    let res=message_chain_loader(&map).unwrap();
    let p=res.into_target::<Plain>().unwrap();

    assert_eq!(p,Plain{text:"好耶".to_string()})

}
