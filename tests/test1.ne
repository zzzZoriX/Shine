priv struct Person {
    constructor(var int age, var str name) -> Person {
        this::age = age;
        this::name = name;
    }

    pub var str name;
    pub var int age;
}

decl pub fun print_person(var st Person!) -> ...;

pub fun main() -> int {
    var st Person! vasya = (12, "Vasya");

    print_person(vasya);
}

impl pub fn print_person(var st Person! people) -> ...
    => print(people::name, people::age);