use rand::Rng;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct UserKey(pub u128);

impl UserKey {
    pub fn random() -> Self {
        let value = rand::thread_rng().gen();
        Self(value)
    }
}
