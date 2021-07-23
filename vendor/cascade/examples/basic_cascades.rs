#[macro_use]
extern crate cascade;

#[derive(Clone, Debug)]
struct Person {
    name: String,
    age: u32,
    height: u32,
}

impl Person {
    pub fn blank() -> Person {
        Person {
            name: "".to_string(),
            age: 0,
            height: 0,
        }
    }
}

#[derive(Clone, Debug)]
struct Chain {
    links: Vec<u32>,
}

impl Chain {
    fn blank() -> Chain {
        Chain { links: vec![] }
    }
    fn add(mut self, link: u32) -> Self {
        self.links.push(link);
        self
    }
}

#[allow(unused)]
fn main() {
    // Cascades can be used recursively!
    let people = cascade! {
       Vec::new();
       ..push(cascade! {
           Person::blank();
           ..name = "John Smith".to_string();
           ..height = 72; // 6 feet, or 72 inches tall
           ..age = 34;
       });
       // This is what an expanded cascade looks like.
       ..push({
          let mut __tmp = Person::blank();
          __tmp.name = "Jason Smith".to_string();
          __tmp.height = 64;
          __tmp.age = 34;
          __tmp
       });
    };
    // Any expression works as the first statement of a cascade.
    let other_person = cascade! {
       people[0].clone();
       ..name = "Bob Smith".to_string();
       ..height = 66;
    };
    // You can also use +=, -=, *=, /= for operators
    let another_person = cascade! {
       other_person.clone();
       ..name = "Joan Smith".to_string();
       ..age += 3;
       ..height -= 4;
    };
    // You can put regular statements inside of a cascade macro
    let yet_another_person = cascade! {
       people[0].clone();
       ..name = "James Smith".to_string();
       ..age = 27;
       println!("Cascades in Rust are cool!");
       ..height -= 3;
    };
    // You can bind the initial value of the cascade to an identifier, which reflects the current state of the cascaded value.
    let one_more_person = cascade! {
      let person = people[0].clone();
      println!("'person' was equal to: {:?}", person);
      ..name = "James Smith".to_string();
      ..height = ((person.height as f32) * 0.8) as u32;
      println!("'person' is now equal to: {:?}", person);
    };
    // As of version 0.1.2, you can also chain methods together. Observe:
    let method_chain_example = cascade! {
      let ch = Chain::blank();
      ..add(5).add(6).add(7); // In this case, ch is consumed here. So we have to shadow ch to avoid an error. Obviously, this isn't the most useful way to method-chain.
      let ch = ();
    };

    // You can have nested blocks within the cascade
    let block_example = cascade! {
      Vec::new();
      ..push(1);
      ..push(2);
    };

    let has_more_than_three_elements = cascade! {
        let v = vec![1,2,3];
        ..push(4);
        v.len() > 3
    };

    println!("{}", cascade! {
        vec![1,2,3];
        ..push(4);
        ..into_iter().fold(0, |acc, x| acc + x)
    });

    cascade! {
        let _: Vec<u32> = vec![1,2,3].into_iter().map(|x| x + 1).collect();
        ..push(1);
    };

    option_cascade_test().unwrap().unwrap();
}

// As of version 0.1.3, you can use the ? operator after a .. statement.
fn option_cascade_test() -> Result<Result<(), ()>, ()> {
    let question_mark_operator_example: Result<Result<(), ()>, ()> = cascade! {
      Ok(Ok(()));
      ..unwrap()?;
    };
    return question_mark_operator_example;
}
