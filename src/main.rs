trait MyTrait {
    fn mymthod(&self) -> i32;
}

struct A {
    x: i32,
}

struct B {
    y: i32,
}

struct C {
    z: i32,
}

impl A {
    fn print_info(&self) {
        println!("A.print_info called.");
    } 
}

impl B {
    fn print_info(&self) {
        println!("B.print_info called.");
    } 
}

impl C {
    fn print_info(&self) {
        println!("C.print_info called.");
    } 
}

impl MyTrait for A{
    fn mymthod(&self) -> i32 {
        println!("A.mymthod called.");
        self.x
    }
}

impl MyTrait for B{
    fn mymthod(&self) -> i32 {
        println!("B.mymthod called.");
        self.y
    }
}

impl MyTrait for C{
    fn mymthod(&self) -> i32 {
        println!("C.mymthod called.");
        self.z
    }
}



fn main() {
    let a = A { x: 4 };
    let b = B { y: 5 };
    let c = C { z: 6 };

    #[derive(Debug)]
    enum ABC {
        a,
        b,
        c,
    }

    // 使用枚举包裹三个不同的类型，并放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
    let vec1 = vec![ABC::a, ABC::b, ABC::c]; 
    for item in vec1 {
        println!("Now we run {:?}.", &item);
        match item {
            ABC::a =>  a.print_info(),
            ABC::b =>  b.print_info(),
            ABC::c =>  c.print_info(),
        }
    }

    // 定义三个不同的类型，使用Trait Object，将其放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
    let mytrait1: &dyn MyTrait = & a;
    // mytrait1.mymthod();

    let mytrait2: &dyn MyTrait = & b;
    // mytrait2.mymthod();

    let mytrait3: &dyn MyTrait = & c;
    // mytrait3.mymthod();
    let vec2 = vec![mytrait1, mytrait2, mytrait3];
    for item in vec2 {
        println!("return num is:{}.", item.mymthod());
    }
       
}