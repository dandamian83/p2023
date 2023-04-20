pub fn greet() {
    println!("Greetings!");
}

pub fn exercise_c_simple_types() {
    let coords: (f32, f32) = (6.3, 15.0);
    // 1. Pass parts of `coords` to the `print_difference` function. This should show the difference
    // between the two numbers in coords when you do `cargo run`.  Use tuple indexing.
    //
    // The `print_difference` function is defined below the `main` function. It may help if you look
    // at how it is defined.
    //
    print_difference(coords.0, coords.1);   // Uncomment and finish this line


    // 2. We want to use the `print_array` function to print coords...but coords isn't an array!
    // Create an array of type [f32; 2] and initialize it to contain the
    // information from coords.  Uncomment the print_array line and run the code.
    //
    let coords_arr: [f32; 2] = [coords.0, coords.1];               // create an array literal out of parts of `coord` here
    print_array(coords_arr);        // and pass it in here (this line doesn't need to change)


    let series = [1, 1, 2, 3, 5, 8, 13];
    // 3. Make the `ding` function happy by passing it the value 13 out of the `series` array.
    // Use array indexing.  Done correctly, `cargo run` will produce the additional output
    // "Ding, you found 13!"
    //
    ding(series[series.len() - 1]);


    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    // 4. Pass the `on_off` function the value `true` from the variable `mess`.  Done correctly,
    // `cargo run` will produce the additional output "Lights are on!" I'll get you started:
    //
    on_off(mess.2[1].0);

    // 5.  What a mess -- functions in a binary! Let's get organized!
    //
    // - Make a library file (src/lib.rs)
    // - Move all the functions (except main) into the library
    // - Make all the functions public with `pub`
    // - Bring all the functions into scope using use statements. Remember, the name of the library
    //   is defined in Cargo.toml.  You'll need to know that to `use` it.
    //
    // `cargo run` should produce the same output, only now the code is more organized. 🎉

    // Challenge: Uncomment the line below, run the code, and examine the
    // output. Then go refactor the print_distance() function according to the
    // instructions in the comments inside that function.
    print_distance(coords);
}

fn print_difference(x: f32, y: f32) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

fn print_array(a: [f32; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
}

fn print_distance(z: (f32, f32)) {
    // Using z.0 and z.1 is not nearly as nice as using x and y.  Lucky for
    // us, Rust supports destructuring function arguments.  Try replacing "z" in
    // the parameter list above with "(x, y)" and then adjust the function
    // body to use x and y.
    println!(
        "Distance to the origin is {}",
        ( z.0.powf(2.0) + z.1.powf(2.0) ).sqrt());
}

pub fn exercise_e_ownership_and_references() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    // 1. Write a function `inspect` that takes a reference to a String, returns nothing, but
    // prints whether the contents of the String is plural or singular. Then uncomment and run this
    // code with `cargo run apple` and `cargo run apples'.  Hint: use `.ends_with("s")` on the
    // String reference
    //
    inspect(&arg);

    // 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
    // the String if it doesn't already end with "s". Then uncomment and run the code below with
    // `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
    //
    change(&mut arg);
    println!("I have many {}", arg);

    // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
    // indicating whether or not the String both starts with a "b" AND contains an "a".
    // Hint 1: use `.starts_with("b")` and `.contains("a")`
    // Hint 2: `&&` is the boolean "AND" operator
    
    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "bedazzle" that takes a mutable reference to a String and
    // ignores what is in the string and replaces the contents of the string with the String
    // "sparkly". Then uncomment the code below.
    //
    // Hint: You will need to dereference the mutable reference in order to assign it a
    // new value.
    //
    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}

fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("Plural");
    } else {
        println!("Singular");
    }
}

fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s");
    }
}

fn eat(s: String) -> bool {
    s.starts_with("b") && s.contains("a")
}

fn bedazzle(s: &mut String) {
    *s = "concrete".to_string()
}

pub fn exercise_f_structs_traits() {
// 1. Define a trait named `Bite`
//
// Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// want to bite something.  Once this trait is defined, you should be able to run the program with
// `cargo run` without any errors.
//
//  trait Bite...


// 2. Now create a struct named Grapes with a field that tracks how many grapes are left.  If you
// need a hint, look at how it was done for Carrot at the bottom of this file (you should probably
// use a different field, though).
//
// #[derive(Debug)] // include this line right before your struct definition
// struct Grapes...


// 3. Implement Bite for Grapes.  When you bite a Grapes, subtract 1 from how many grapes are left.
// If you need a hint, look at how it was done for Carrot at the bottom of this file.
//
// impl Bite for...

 // Once you finish #1 above, this part should work.
 let mut carrot = Carrot { percent_left: 100.0 };
 carrot.bite();
 println!("I take a bite: {:?}", carrot);

 // 4. Uncomment and adjust the code below to match how you defined your
 // Grapes struct.
 //
 let mut grapes = Grapes { grapes_left: 100 };
 grapes.bite();
 println!("Eat a grape: {:?}", grapes);

 // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
 // function that:
 // - takes a mutable reference to any type that implements Bite
 // - calls `.bite()` several times
 // Hint: Define the generic type between the function name and open paren:
 //       fn function_name<T: Bite>(...)
 //
 bunny_nibbles(&mut carrot);
 println!("Bunny nibbles for awhile: {:?}", carrot);
}

trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Grapes {
    grapes_left: i32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.grapes_left -= 1;
    }
}

fn bunny_nibbles<T: Bite>(something: &mut T) {
    something.bite();
}