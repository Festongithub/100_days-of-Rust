pub fn add(a: u32, b: u32) -> u32{
    return a + b;
}

pub fn sub(a:u32, b: u32) -> u32{
    return a - b;
}

pub fn mult(a: u32, b: u32) -> u32 {
    return a * b;
}

pub struct Rectangle {
   pub width:u32,
   pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        return self.width * self.height
    }
    pub fn perimeter(&self) -> u32 {
       return 2 * self.width + self.height;
    }
}

pub struct Student {
    pub name: String,
    pub age: u32,
    pub course: String,   
}

impl Student  {
    pub fn get_name(&self) -> &str{
        return &self.name;
    }
    pub fn get_age(&self) -> u32 {
        return self.age 
    }
    pub fn course_name(&self) -> &str{
        return &self.course
    }
}