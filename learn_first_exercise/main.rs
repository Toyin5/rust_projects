// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    // todo!("Fix enum definition so code compiles");
    Manual = 1,
    SemiAuto = 2,
    Automatic = 3
}
#[derive(PartialEq, Debug)]
enum Age{
    New,
    Old
}

// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool, miles:u32) -> Car {

    // Use the values of the input arguments
    // All new cars always have zero mileage
    let car: Car = Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        age: car_quality(miles),
    };
    return car;
}

fn main(){
     // Create car color array
     let colors = ["Blue", "Green", "Red", "Silver"];

     // Declare the car type and initial values
     let mut car: Car;
     let mut engine: Transmission = Transmission::Manual;
     
     // Order 3 cars, one car for each type of transmission

    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[1]), engine, true, 0);
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.convertible, car.transmission, car.color, car.age.1);

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[0]), engine, false, 100);
    println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.convertible, car.transmission, car.color, car.age.1);

    // Car order #3: Used, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.convertible, car.transmission, car.color, car.age.1);
}

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Create a tuple for the car quality with the Age ("New" or "Used") and mileage
// Return a tuple with the arrow `->` syntax
fn car_quality (miles: u32) -> (Age, u32) {

    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    let quality: (Age, u32) = (Age::New, miles);

    // Return the completed tuple to the caller
    return quality;
}