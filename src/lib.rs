pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

pub trait GetName {
    fn get_name(&self) -> &String;
}

pub trait GetAge {
    fn get_age(&self) -> u32;
}

struct PeopleMatchInformation<T, U> {
    master: T,
    employee: U,
}

// 11-15 行也可以写成：impl<T: GetName + GetAge, U: GetName + GetAge> PeopleMatchInformation<T, U>
impl<T, U> PeopleMatchInformation<T, U>
where
    T: GetName + GetAge, // T 和 U 都必须实现 GetName 和 GetAge trait
    U: GetName + GetAge,
{
    fn print_all_information(&self) {
        println!("teacher name = {}", self.master.get_name());
        println!("teacher age = {}", self.master.get_age());
        println!("student name = {}", self.employee.get_name());
        println!("student age = {}", self.employee.get_age());
    }
}

//使用
pub struct Teacher {
    pub name: String,
    pub age: u32,
}

impl GetName for Teacher {
    fn get_name(&self) -> &String {
        &(self.name)
    }
}

impl GetAge for Teacher {
    fn get_age(&self) -> u32 {
        self.age
    }
}

pub struct Student {
    pub name: String,
    pub age: u32,
}

impl GetName for Student {
    fn get_name(&self) -> &String {
        &(self.name)
    }
}

impl GetAge for Student {
    fn get_age(&self) -> u32 {
        self.age
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    //第一个参数为函数指针
    f(arg) + f(arg)
}

pub fn subtract(left: usize, right: usize) -> usize {
    left - right
}

trait Vehicle {
    fn run(&self);
}

struct Car(u32);

impl Vehicle for Car {
    fn run(&self) {
        println!("Car {:?} run ... ", self.0);
    }
}

struct Truck(u32);

impl Vehicle for Truck {
    fn run(&self) {
        println!("Truck {:?} run ... ", self.0);
    }
}

//vehicle_run 方法的参数要求是一个 Vehicle trait 对象
fn vehicle_run(vehicle: Box<dyn Vehicle>) {
    vehicle.run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        let result = subtract(4, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works_6() {
        let car = Car(1001);
        let truck = Truck(1002);

        let v1 = Box::new(car);
        vehicle_run(v1);
        let v2 = Box::new(truck);
        vehicle_run(v2);
        println!("The answer is: {}", chrono::Utc::now());
        println!("The answer is: {}", chrono::Local::now());
    }

    #[test]
    fn it_works_5() {
        let handle = thread::spawn(|| {
            // 创建一个线程
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        handle.join().unwrap(); // 等待子线程结束
        let answer = do_twice(add_one, 5);
        println!("The answer is: {}", answer);
    }

    #[test]
    fn it_works_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_2() {
        let a: [u32; 5] = [1, 2, 3, 4, 5];
        let b = &a[1..3];
        println!("b: {:?}", b); // 2, 3

        let v: Vec<u32> = vec![1, 2, 3, 4, 5];
        let b = &v[1..3];
        println!("b: {:?}", b); // 2, 3
    }

    #[test]
    fn it_works_3() {
        let result = divide(4.0, 2.0);

        if let Ok(value) = result {
            println!("The result is {}", value);
        }

        match result {
            Ok(value) => println!("The result is {}", value),
            Err(error) => println!("Error: {}", error),
        }

        let result = divide(10.0, 2.0);
        assert_eq!(result, Ok(5.0));

        let result = divide(4.0, 0.0);

        if let Err(error) = result {
            println!("Error: {}", error);
        }
    }

    #[test]
    fn it_works_4() {
        let teacher = Teacher { name: String::from("teacher"), age: 30 };

        let student = Student { name: String::from("student"), age: 20 };

        let people_match_information = PeopleMatchInformation { master: teacher, employee: student };

        people_match_information.print_all_information();
        println!("{:?}", wifiscanner::scan());
    }
}
