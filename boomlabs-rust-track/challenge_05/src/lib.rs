pub struct Room {
    pub name: String,
    pub north: String,
    pub south: String,
    pub east: String,
    pub west: String,
}
let room = Room {
    name: String::from("Living Room"),
    north: String::from("Balcony"),
    south: String::from("Bathroom"),
    east: String::from("Kitchen"),
    west: String::from("Entrance"),
};

println!("{}", room);