/// The various errors we'll expect to return as a result of some invalid operation.
/// More details below:
///
#[derive(Debug)]
pub enum Errors {
    DuplicateRoom(String),
    UnknownRoom(String),
    IoError(std::io::Error),
    LineParseError { line_number: usize },
    DirectionParseError(String),
}

/// The four directions in which a room can have neighbors. You can add more traits
/// implementations to make your life easier.
///
#[derive(Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// A room in the dungeons. It is defined by name only, although in a more interesting implementation it might
/// holds items, enemies...
///
pub struct Room {
    pub name: String,
    // Other fields you need
}

/// Container for the rooms and more. We will mostly work with this structure.
///
pub struct Dungeon {
    // Fields you need
}

impl Dungeon {
    /// Constructs an empty Dungeon that does not have any rooms.
    ///
    pub fn new() -> Self {
        todo!()
    }

    /// Adding room to Dungeon named `name`. Returns `Ok(())` on success. If there is already a room with
    /// such a name, we expect you to return `Errors::DuplicateRoom` with the name.
    ///
    pub fn add_room(&mut self, name: &str) -> Result<(), Errors> {
        todo!()
    }

    /// Reading a given room -- when we call `get_room`, we expect a reference to `Room`
    /// the structure with this name.
    ///
    /// If there is no such room, we expect `Errors::UnknownRoom` with the supplied name.
    ///
    pub fn get_room(&self, room_name: &str) -> Result<&Room, Errors> {
        todo!()
    }

    /// Add a room neighbor. After calling the function, we expect the room with a name
    /// `room_name` has a connection in the `direction` direction with the room named `other_room_name`.
    ///
    /// We also expect `other_room_name` to relate to `room_name` in the *reverse* direction.
    ///
    /// A successful result is `Ok(())`. In the event that one of the two rooms does not exist, we expect an error
    /// `Errors::UnknownRoom` with the corresponding missing room name. If both are missing, then
    /// return the one you check first.
    ///
    pub fn set_link(
        &mut self,
        room_name: &str,
        direction: Direction,
        other_room_name: &str,
    ) -> Result<(), Errors> {
        todo!()
    }

    /// Reading neighbor of room named `room_name` in direction `direction`. There are several here
    /// the output variant:
    ///
    /// - If the room passed does not exist, we expect an `Errors::UnknownRoom` error
    /// - If the given room has no neighbor in that direction, Ok(None) is the meaningful result
    /// - Otherwise, we wait for a reference to the `Room` structure of the neighbor in question, wrapped in `Ok(Some(`.
    ///
    pub fn get_next_room(
        &self,
        room_name: &str,
        direction: Direction,
    ) -> Result<Option<&Room>, Errors> {
        todo!()
    }
}

impl Dungeon {
    /// We read the dungeon structure from something that implements `BufRead`. This could be it
    /// file, or, if we're testing, it could just be a collection of bytes.
    ///
    /// A successful result returns the newly created dungeon packaged in `Ok`.
    ///
    /// See below for an explanation of the errors we expect.
    ///
    pub fn from_reader<B: BufRead>(reader: B) -> Result<Self, Errors> {
        todo!()
    }
}

/// match_prefix("- ", "- Foo") //=> Some("Foo")
/// match_prefix("- ", "Bar")   //=> None
///
fn match_prefix<'a, 'b>(prefix: &'a str, input: &'b str) -> Option<&'b str> {
    todo!()
}

impl Dungeon {
    /// Searches for a path from `start_room_name` to `end_room_name` and returns it in a vector packed in
    /// `Ok(Some(` if found.
    ///
    /// If there is no path between these two rooms, returns `Ok(None)`.
    ///
    /// If reading rooms at some point returns an error, we expect you to raise the error.
    ///
    pub fn find_path(
        &self,
        start_room_name: &str,
        end_room_name: &str,
    ) -> Result<Option<Vec<&Room>>, Errors> {
        todo!()
    }
}
