#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $val:expr,)+) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key, $val);
            )+
            map
        }
    };

    ($($key: expr => $val: expr),*) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $( 
                map.insert($key, $val); 
            )*
            map
        }
    };
}

#[test]
fn test_hashmap() {
    let map = hashmap!('a' => 3, 'b' => 11, 'z' => 32);
}