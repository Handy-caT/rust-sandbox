macro_rules! btreemap {
    ($(($x:expr, $y:expr)),*) => {
        {
            let mut temp_btreemap = BTreeMap::new();
            $(
                temp_btreemap.insert($x, $y);
            )*
            temp_btreemap
        }
    };
}

use::std::collections::BTreeMap;
use my_proc_macros_lib::btreemap_proc;

fn main() {
    let map = btreemap! (
        ("a", 1),
        ("b", 2),
        ("c", 3)
    );

    println!("{:?}", map);

    btreemap_proc!(
        ("a", 1),
        ("b", 2),
        ("c", 3)
    );
}
