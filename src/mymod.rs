
    struct some_cool_stuff{
        omg: Omgenum
    }
    pub enum Omgenum{
        choise1,
        choise2
        
    }

    pub struct dude{
        pub name: String,
        pub height: f32,
        pub weight: f32,
        pub omge: Omgenum
    }

    impl dude{
        pub fn new(name: String, height: f32, weight: f32)->Self{
            dude{name, height, weight, omge: Omgenum::choise1}
        }
        pub fn grow(self)->Self{
            dude{height: self.height+1.0, ..self}
        }
        pub fn eat(mut self)->Self{
            
            dude{weight: self.weight+1.0, ..self.grow()}
        }
    }
