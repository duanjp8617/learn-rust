
// On x86-64, rust aligns to 32-bits, 4 bytes.
struct A {
    a : u8, 
    b : u32,
    c : u16,
}

struct ABad {
    a : u8,
    a_padding : [u8; 3],
    b : u32,
    c : u16,
    c_padding : [u8; 2],
}

struct AGood {
    b : u32,
    c : u16,
    a : u8,
    a_padding : u8,
}

struct Foo<T, U> {
    count : u16,
    data1 : T,
    data2 : U,
}

enum SomeEnum {
    A(u64),
    B(u32),
}

struct MyDynamicallySizedType<T : ?Sized> {
    info : u32,
    dynamically_sized_elem : T,
}

// zero-sized type
struct Nothing;

// Empty types, you can not construct a value of this type.
enum Void {}

pub fn print_sth() {
    println!("{}", std::mem::size_of::<A>());
    println!("{}", std::mem::size_of::<ABad>());
    println!("{}", std::mem::size_of::<AGood>());
    println!("{}", std::mem::size_of::<Foo<u16, u32>>());
    println!("{}", std::mem::size_of::<Foo<u32, u16>>());
    println!("{}", std::mem::size_of::<Option<&u8>>());
    println!("{}", std::mem::size_of::<SomeEnum>());

    let var : MyDynamicallySizedType<[u8; 64]> = MyDynamicallySizedType {
        info : 1,
        dynamically_sized_elem : [0; 64],
    };

    let ref_var : &MyDynamicallySizedType<[u8]> = &var;
    println!("{}", std::mem::size_of::<MyDynamicallySizedType<[u8; 8]>>());
    println!("{}, {:?}", ref_var.info, &ref_var.dynamically_sized_elem);

    let res : Result<u32, Void> = Ok(566);
    if let Ok(val) = res {
        println!("{}", val);
    };

    let v = ();
    let nth = Nothing{};
    println!("{:?}", v);
    println!("{}", std::mem::size_of::<Nothing>());
}