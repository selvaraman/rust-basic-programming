#[derive(Debug, PartialEq)]
enum Category {
    Minor,
    Major,
}

#[derive(Debug)]
struct Employee {
    name: String,
    age: Option<Category>,
}

impl Employee {
    fn is_minor(&self) -> bool {
        self.age == Some(Category::Minor)
    }

    fn is_major(&self) -> bool {
        self.age == Some(Category::Major)
    }
}

impl Employee {
    // Associated function to list employees
    fn list_employees(list: &Vec<Employee>) {
        let mut age_list: Vec<Option<Category>> = Vec::new();
        for l in list {
            age_list.push(l.age);
        }
    }
}

fn main() {
    let list: Vec<Employee> = vec! [
        Employee {
            name: "John".to_string(),
            age: Some(Category::Minor),
        },
        Employee {
            name: "Little John".to_string(),
            age: Some(Category::Major),
        },
    ];
    Employee::list_employees(&list);
    println!("TT: {:?}", list);
}

