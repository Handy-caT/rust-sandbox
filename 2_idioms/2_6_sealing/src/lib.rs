pub mod my_error;
pub mod my_iterator_ext;


use std::any::TypeId;
use std::fmt::{Debug, Display, Formatter};
pub use self::{my_error::MyError, my_iterator_ext::MyIteratorExt};


// struct TestIerator;
//
// impl Iterator for TestIerator {
//     type Item = i32;
//     fn next(&mut self) -> Option<Self::Item> {
//         Some(1)
//     }
// }
//
// impl MyIteratorExt for TestIerator
// {}


// #[derive(Debug)]
// struct TestError;
//
// impl Display for TestError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "TestError")
//     }
// }
//
// impl MyError for TestError {
//     fn type_id(&self, _: my_error::private::Internal) -> TypeId where Self: 'static {
//         TypeId::of::<Self>()
//     }
// }


