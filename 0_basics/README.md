Step 0: Become familiar with Rust basics
========================================

__Estimated time__: 3 days

Read through [Rust Book], [Rust FAQ], and become familiar with basic [Rust] concepts, syntax, memory model, type and module systems.

Polish your familiarity by completing [Rust By Example] and [rustlings].

Read through [Cargo Book] and become familiar with [Cargo] and its workspaces.

After completing these steps, you should be able to answer (and understand why) the following questions:
- What memory model [Rust] has? Is it single-threaded or multiple-threaded? Is it synchronous or asynchronous?
#### Rust does not yet have a defined memory model. Various academics and industry professionals are working on various proposals, but for now, this is an under-defined place in the language.
#### Rust is a multi-threaded language. Rust has no runtime, so it is not synchronous or asynchronous. 
#### Read only memory for constants. 
#### Язык многопоточен если его рантайм по умолчанию исполняется на многих потоках, она встроенная и неявная. Раст не многопоточен, но позволяет писать многопоточные программы.
#### Язык по умолчанию синхронен, но позволяет писать асинхронные программы.
- What runtime [Rust] has? Does it use a GC (garbage collector)?
#### Rust has no runtime. Rust does not use a GC.
#### Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.
#### Not in the typical sense used by languages such as Java, but parts of the Rust standard library can be considered a “runtime”, providing a heap, backtraces, unwinding, and stack guards. There is a small amount of initialization code that runs before the user’s main function.
#### Garbage collectors могут собирать данные использующие циклические ссылки.
#### Garbage collector может оптимизировать работу с памятью
- What statically typing means? What is a benefit of using it?
#### Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it.
- What are generics and parametric polymorphism? Which problems do they solve?
#### Every programming language has tools for effectively handling the duplication of concepts. In Rust, one such tool is generics: abstract stand-ins for concrete types or other properties.
#### Genetics helps us to avoid code duplication. Parametric polymorphism is used to write code that works on any type that can satisfy the constraints.
#### Early and late binding. Rust uses early binding.
- What are traits? How are they used? How do they compare to interfaces? What are an auto trait and a blanket impl? What is a marker trait?
#### A trait defines functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.
#### We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library.
#### Auto traits are marker traits that are automatically implemented for every type, unless the type, or a type it contains, has explicitly opted out via a negative impl.
#### Свои Трейты можно релизовать для любого типа, интерфесы же нет.
#### За единой сигнатурой будет находится разный код.
#### Marker traits are used to mark types as having some property. They usually don't have any associated types and methods and are used to tell the compiler about some property of a type.
#### A blanket implementation is an implementation of a trait on a generic parameter: it applies to any type that satisfies the trait bounds.
#### In Rust, "trait coherence" (or simply "coherence") is the property that there is at most one implementation of a trait for any given type.
#### Rust enforces trait coherence through two sets of rules.
#### Автотрейты это элемент структурной типизации в расте
- What are static and dynamic dispatches? Which should I use, and when?
#### Диспетчеризация это вызов функции в условиях полиморфизма.
#### Статическая типизация это когда компилятор знает типы всех переменных во время компиляции.
#### While Rust favors static dispatch, it also supports dynamic dispatch through a mechanism called 'trait objects.'
#### This has some upsides: static dispatching of any method calls, allowing for inlining and hence usually higher performance. It also has some downsides: causing code bloat due to many copies of the same function existing in the binary, one for each type
#### Static dispatch is when the compiler knows which method you’re calling at compile time. Dynamic dispatch is when the compiler can’t know which method you’re calling until runtime.
#### While using dynamic dispatch, the compiler can’t verify that the values used with the trait have implemented the trait. Only the code that is running at runtime can ensure that. Dynamic dispatch also adds runtime overhead because the compiler adds code to find the correct trait method at runtime instead of doing that work at compile time. 
#### Dynamic dispatch is using vtables to call methods.
- What is a crate and what is a module in Rust? How do they differ? How are the used?
#### A crate is the smallest amount of code that the Rust compiler considers at a time. A crate can come in one of two forms: a binary crate or a library crate.
#### Modules let us organize code within a crate for readability and easy reuse. Modules also allow us to control the privacy of items, because code within a module is private by default.
- What are move semantics? What are borrowing rules? What is the benefit of using them?
#### Move semantics is a feature of Rust that allows us to reuse data in place, without copying it. This is done by transferring ownership of the data from one variable to another.
#### Borrowing is a feature of Rust that allows us to reuse data in place, without moving it. This is done by passing a reference to the data to another variable or function.
#### Borrowing rules are: 1. At any given time, you can have either one mutable reference or any number of immutable references. 2. References must always be valid.
#### Partial move
- What is immutability? What is the benefit of using it?
#### The benefit of immutability is that it makes your code easier to reason about. If you know a value can’t change, you never have to spend time worrying about whether its value has changed in some other part of your program.
#### Один из плюссов это эргономика. 
- What is cloning? What is copying? How do they compare?
#### Clone is used for heap allocated data, Copy is used for stack allocated data.
#### Copy refers to stack allocated data. Copy is implicitly used when a variable is assigned to another variable. Copy is also used when passing a variable to a function.
#### You can't implement Copy for types that implement Drop.
- What is RAII? How is it implemented in [Rust]? What is the benefit of using it?
#### Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you’ve used RAII patterns.
#### RAII is a terrible name. OBRM (Ownership-Based Resource Management) is used in Rust sometimes, and is a much better name.
- What is an iterator? What is a collection? How do they differ? How are they used?
#### Rust’s standard library includes a number of very useful data structures called collections. Most other data types represent one specific value, but collections can contain multiple values. Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.
#### The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. When you use iterators, you don’t have to reimplement that logic yourself.
#### In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.
- What are macros? Which problems do they solve? What is the difference between declarative and procedural macro?
#### Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming. Metaprogramming is useful for reducing the amount of code you have to write and maintain, which is one of the reasons macros are useful to use.
#### A function signature must declare the number and type of parameters the function has. Macros, on the other hand, can take a variable number of parameters.
#### The second form of macros is the procedural macro, which acts more like a function (and is a type of procedure). Procedural macros accept some code as an input, operate on that code, and produce some code as an output rather than matching against patterns and replacing the code with other code as declarative macros do.
#### The three kinds of procedural macros are custom derive, attribute-like, and function-like, and all work in a similar fashion.
#### Расширение языка.
#### Декларативные языки гарантируют гигену. Гигиена это то как макрос при раскрытии меняет код снаружи. Код раскрываясь может неявно вызывать ошибки. В декларативных макросах компилятор делет дополнителные действия для предотвращения.
- How code is tested in [Rust]? Where should you put tests and why?
#### Tests are Rust functions that verify that the non-test code is functioning in the expected manner. They usually perform 3 actions: Set up any needed data or state, Run the code you want to test, Assert the results are what you expect.
#### Tests are put in the same file as the code they are testing. The convention is to create a module named tests in each file to contain the test functions and to annotate the module with cfg(test).
#### All tests are annotated with the test attribute.
#### Integration tests are stored in a tests directory at the top level of the project directory. Each file in the tests directory is a separate crate.
- Why [Rust] has `&str` and `String` types? How do they differ? When should you use them?
#### `String` is an owned buffer of UTF-8 bytes allocated on the heap. Mutable `String`'s can be modified, growing their capacity as needed.
#### `&str` is a fixed-capacity “view” into a `String` allocated elsewhere, commonly on the heap, in the case of slices dereferenced from `String`'s, or in static memory, in the case of string literals.
#### `&str` is a primitive type implemented by the Rust language, while `String` is implemented in the standard library.
#### `&static str` is a special case of `&str` that can be used to refer to string literals. It can be used in all the same places as `&str`, but it has a `'static` lifetime.
#### Also, there is a `&a str` type, which is a string slice with a lifetime of `'a`. It can be used to refer to a string slice with a lifetime of `'a`.
- What are lifetimes? Which problems do they solve? Which benefits do they give?
#### Lifetimes are another kind of generic that we’ve already been using. Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.
#### They are solving the problem of dangling references. The main aim is to prevent using a reference after the resource has been dropped.
- Is [Rust] OOP language? Is it possible to use SOLID/GRASP? Does it have an inheritance?
#### It is multi-paradigm. Many things you can do in OO languages you can do in Rust, but not everything, and not always using the same abstraction you’re accustomed to.
#### Rust doesn't have inheritance, but it has traits, which are similar to interfaces and can be used to prevent code duplication.

After you're done notify your lead in an appropriate PR (pull request), and he will exam what you have learned.

_Additional_ articles, which may help to understand the above topic better:
- [Chris Morgan: Rust ownership, the hard way][1]
- [Adolfo Ochagavía: You are holding it wrong][12]
- [Vikram Fugro: Beyond Pointers: How Rust outshines C++ with its Borrow Checker][15]
- [Sabrina Jewson: Why the “Null” Lifetime Does Not Exist][16]
- [HashRust: A guide to closures in Rust][13]
- [Ludwig Stecher: Rusts Module System Explained][2]
- [Tristan Hume: Models of Generics and Metaprogramming: Go, Rust, Swift, D and More][3]
- [Jeff Anderson: Generics Demystified Part 1][4]
- [Jeff Anderson: Generics Demystified Part 2][5]
- [Bradford Hovinen: Demystifying trait generics in Rust][14]
- [Brandon Smith: Three Kinds of Polymorphism in Rust][6]
- [Jeremy Steward: C++ & Rust: Generics and Specialization][7]
- [cooscoos: &stress about &Strings][8]
- [Jimmy Hartzell: RAII: Compile-Time Memory Management in C++ and Rust][9]
- [Georgios Antonopoulos: Rust vs Common C++ Bugs][10]
- [Yurii Shymon: True Observer Pattern with Unsubscribe mechanism using Rust][11]




[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Rust]: https://www.rust-lang.org
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust FAQ]: https://prev.rust-lang.org/faq.html
[rustlings]: https://rustlings.cool

[1]: https://chrismorgan.info/blog/rust-ownership-the-hard-way
[2]: https://aloso.github.io/2021/03/28/module-system.html
[3]: https://thume.ca/2019/07/14/a-tour-of-metaprogramming-models-for-generics
[4]: https://web.archive.org/web/20220525213911/http://jeffa.io/rust_guide_generics_demystified_part_1
[5]: https://web.archive.org/web/20220328114028/https://jeffa.io/rust_guide_generics_demystified_part_2
[6]: https://www.brandons.me/blog/polymorphism-in-rust
[7]: https://www.tangramvision.com/blog/c-rust-generics-and-specialization#substitution-ordering--failures
[8]: https://cooscoos.github.io/blog/stress-about-strings
[9]: https://www.thecodedmessage.com/posts/raii
[10]: https://geo-ant.github.io/blog/2022/common-cpp-errors-vs-rust
[11]: https://web.archive.org/web/20230319015854/https://ybnesm.github.io/blah/articles/true-observer-pattern-rust
[12]: https://ochagavia.nl/blog/you-are-holding-it-wrong
[13]: https://hashrust.com/blog/a-guide-to-closures-in-rust
[14]: https://gruebelinchen.wordpress.com/2023/06/06/demystifying-trait-generics-in-rust
[15]: https://dev.to/vikram2784/beyond-pointers-how-rust-outshines-c-with-its-borrow-checker-1mad
[16]: https://sabrinajewson.org/blog/null-lifetime
