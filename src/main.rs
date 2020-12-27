mod mymod;
//add a fancy comment
//another one
fn main() {
    let some_dude = mymod::dude::new("Heralt".to_string(), 190.0, 72.0);

    let grown_dude = some_dude.grow();

    println!("Grown dude called {} is {} centimeters tall.", grown_dude.name, grown_dude.height);
}

fn fancy_function(){
    println!("I am a fancy function)");
}