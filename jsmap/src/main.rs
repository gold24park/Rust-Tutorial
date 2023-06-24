use std::fmt::Display;
trait JsKey: PartialEq + Clone {}
impl JsKey for i32 {}
impl JsKey for &str {} // [?] owned 타입을 안쓰면..
impl JsKey for String {}

trait JsMap: IntoIterator<Item = (Self::Key, Self::Value)> {
    type Key: JsKey;
    type Value;

    fn entries(self) -> Self::IntoIter;
    fn keys(&self) -> Vec<Self::Key>; // [?] keys는 어떻게 구현하실 것같나요??
    fn get_value(&self, key: Self::Key) -> Option<&Self::Value>;
}

impl<K, V> JsMap for Vec<(K, V)>
where
    K: JsKey,
    V: Display,
{
    type Key = K;
    type Value = V;

    fn get_value(&self, key: Self::Key) -> Option<&Self::Value> {
        match self.iter().find(|(k, _)| *k == key) {
            Some(e) => Some(&e.1),
            None => None,
        }
    }

    fn entries(self) -> Self::IntoIter {
        self.into_iter()
    }

    fn keys(&self) -> Vec<Self::Key> {
        self.into_iter().map(|(k, _)| k).cloned().collect()
    }
}

fn main() {
    let sv1 = vec![("1", 123123124), ("12", 1123124)];
    let sv2 = vec![(5, "valval"), (2, "valvalval")];

    for (k, v) in sv2.entries() {
        println!("K: {}, V: {}", k, v);
    }

    println!("{:?}", sv1.get_value(&String::from("12")));
    println!("{:?}", sv1.get_value("12"));
    println!("{:?}", sv1.keys());
    println!("{:?}", sv1);
}
