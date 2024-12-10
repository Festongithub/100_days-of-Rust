use std::collections::{BTreeSet, HashSet, BTreeMap};
macro_rules! set {
    ($($item: expr), *) => {
        {
            let mut s = BTreeSet::new();
            $(s.insert($item);)*
            s
        }
    };
}

mod helpers {
    #[macro_export]
    macro_rules! key_value {
        ($cls: ty, $($key: expr => $value: expr), *) => {
            {
                let mut k = <$cls>::new();
                $(k.insert($key, $value); )*
                k
            }
        };
    }
}

fn main(){

    let usermap = key_value!(BTreeMap<&str, usize>, "hello" => 2, "world" => 1);

    let mut user = BTreeMap::new();
    user.insert("hello", 2);
    user.insert("word", 1);
    assert_eq!(usermap, user);


    let actual = set!("a", "b", "c", "a");

    let mut expected = BTreeSet::new();
    expected.insert("a");
    expected.insert("b");
    expected.insert("c");
    expected.insert("a");
    assert_eq!(actual, expected);

    for i in &expected {
        println!("{}", &i);
    }
}