use serde::{Deserialize, Serialize};
use serde_json;

#[test]
fn deserialize() {
    let msg = FooMessage::Particle(<_>::default());
    let bytes = serde_json::to_vec(&msg).unwrap();
    let test_msg: Result<FooMessage, _> = serde_json::from_slice(&bytes);
    println!("{:?}", test_msg);
    test_msg.unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct FooParticle {
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "action")]
pub enum FooMessage {
    Particle(FooParticle),
}
