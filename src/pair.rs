pub struct Pair<K, V> {
    key: K,
    value: V,
}

impl<K, V> Pair<K, V> {

    pub fn new(key: K, value: V) -> Pair<K, V> {
        Pair { key, value }
    }

    pub fn key(&self) -> &K {
        &self.key
    }
    pub fn value(&self) -> &V {
        &self.value
    }

}