use std::collections::BTreeMap;
use my_proc_macros_lib::btreemap_proc;



#[test]
fn test() {
       let map = btreemap_proc!(("a",1), ("b",2), ("c",3));

       println!("{:?}", map);


}