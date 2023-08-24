use std::collections::HashMap;
use rand::Rng;



pub struct Fact<T> {
    phantom: std::marker::PhantomData<T>,
    facts: HashMap<String, Vec<&'static str>>
}

impl<T> Fact<T> {
    pub fn new() -> Self {
        let mut fact = Fact {
            phantom: std::marker::PhantomData,
            facts: HashMap::new()
        };

        let vec_facts = vec![
            "Vec is a growable array type",
            "Vec is a heap-allocated array",
            "Vec may reallocate when it grows",
        ];

        let string_facts = vec![
            "String is a owned string type",
            "String is used to store this fact",
            "String is not a primitive type",
        ];

        let u32_facts = vec![
            "u32 is a really primitive type",
            "u32 is a 32-bit unsigned integer",
            "u32 max value is 4,294,967,295",
        ];

        fact.facts.insert("Vec".to_string(), vec_facts);
        fact.facts.insert("String".to_string(), string_facts);
        fact.facts.insert("u32".to_string(), u32_facts);

        fact
    }

    pub fn fact(&self) -> String {
        let format_strings = vec![
            "{} is a phantom type in fact",
            "{} may be a primitive type",
            "{} could be any type"
        ];
        let type_name = std::any::type_name::<T>().split("::").last().unwrap();
        let without_generics = type_name.split('<').next();
        let type_name = match without_generics {
            Some(name) => name,
            None => type_name
        };

        if self.facts.get(type_name).is_none() {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..format_strings.len());

            format_strings[index].replace("{}", type_name)
        } else {
            let facts = self.facts.get(type_name).unwrap();
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..facts.len());

            facts[index].to_string()
        }


    }
}
