use std::collections::BTreeMap;
use my_proc_macros_lib::btreemap_proc;



#[test]
fn test() {
       let mut map = BTreeMap::new();
       btreemap_proc!(map, ("a",1), ("b",2), ("c",3));


}