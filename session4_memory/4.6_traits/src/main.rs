use std::fmt;
use std::any::Any;
use std::ops::Add;

struct Point {
    x: f32,
    y: f32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

trait Animal: std::fmt::Debug {
    fn speak(&self);
}

#[derive(Debug)]
struct Cat;


impl Animal for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}

#[derive(Debug)]
struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof");
    }
}

fn speak_twice(animal: &impl Animal) {
    animal.speak();
    animal.speak();
    println!("{animal:?}");
}

fn make_animal() -> impl Animal {
    Cat
}

trait DowncastableAnimal {
    fn speak(&self) {println!("No Idea")} 
    fn as_any(&self) -> &dyn Any;
}

struct Tortise;

impl DowncastableAnimal for Tortise {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct Horse;
 
fn main() {
    let cat = Cat;
    cat.speak();

    let dog = Dog;
    dog.speak();

    speak_twice(&cat);

    let animal = make_animal();

    // Will not work because impl trait is only allowed in function return and inherit method, not in variable binding
    // let animals: Vec<impl Animal> = vec![Cat, Dog];

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];
    animals.iter().for_each(|animal| animal.speak());

    let more_animals: Vec<Box<dyn DowncastableAnimal>> = vec![ Box::new(Tortise)];
    for animal in more_animals.iter() {
        if let Some(t) = animal.as_any().downcast_ref::<Tortise>() {
            println!("Tortise")
        }
    }

    let a = Point {x: 9.4, y: 2.0};
    let b = Point {x: 2.0, y: 4.0};
    let c = a + b;
    println!("c.x = {}, c.y = {}", c.x, c.y);

}
