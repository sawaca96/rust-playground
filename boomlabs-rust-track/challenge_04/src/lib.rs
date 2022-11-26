use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, BufRead},
};

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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "north" => Some(Direction::North),
            "south" => Some(Direction::South),
            "east" => Some(Direction::East),
            "west" => Some(Direction::West),
            _ => None,
        }
    }
}

/// A room in the dungeons. It is defined by name only, although in a more interesting implementation it might
/// holds items, enemies...
///
pub struct Room {
    pub name: String,
    pub items: Vec<String>,
    pub enemies: Vec<String>,
}

impl Clone for Room {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            items: self.items.clone(),
            enemies: self.enemies.clone(),
        }
    }
}

/// Container for the rooms and more. We will mostly work with this structure.
///
pub struct Dungeon {
    rooms: HashMap<String, Room>,
    links: HashMap<String, HashMap<Direction, Room>>,
}

impl Dungeon {
    /// Constructs an empty Dungeon that does not have any rooms.
    ///
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            links: HashMap::new(),
        }
    }

    /// Adding room to Dungeon named `name`. Returns `Ok(())` on success. If there is already a room with
    /// such a name, we expect you to return `Errors::DuplicateRoom` with the name.
    ///
    pub fn add_room(&mut self, name: &str) -> Result<(), Errors> {
        if self.rooms.contains_key(name) {
            Err(Errors::DuplicateRoom(name.to_string()))
        } else {
            self.rooms.insert(
                name.to_string(),
                Room {
                    name: name.to_string(),
                    items: Vec::new(),
                    enemies: Vec::new(),
                },
            );
            self.links.insert(name.to_string(), HashMap::new());
            Ok(())
        }
    }

    /// Reading a given room -- when we call `get_room`, we expect a reference to `Room`
    /// the structure with this name.
    ///
    /// If there is no such room, we expect `Errors::UnknownRoom` with the supplied name.
    ///
    pub fn get_room(&self, room_name: &str) -> Result<&Room, Errors> {
        match self.rooms.get(room_name) {
            Some(room) => Ok(room),
            None => Err(Errors::UnknownRoom(room_name.to_string())),
        }
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
        if !self.rooms.contains_key(room_name) {
            return Err(Errors::UnknownRoom(room_name.to_string()));
        }
        if !self.rooms.contains_key(other_room_name) {
            return Err(Errors::UnknownRoom(other_room_name.to_string()));
        }
        let room = self.get_room(room_name).unwrap().clone();
        let other_room = self.get_room(other_room_name).unwrap().clone();
        self.links
            .entry(room_name.to_string())
            .or_default()
            .insert(direction, other_room);
        self.links
            .entry(other_room_name.to_string())
            .or_default()
            .insert(direction.opposite(), room);
        Ok(())
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
        match self.links.get(room_name) {
            Some(rooms) => match rooms.get(&direction) {
                Some(room) => Ok(Some(room)),
                None => Ok(None),
            },
            None => Err(Errors::UnknownRoom(room_name.to_string())),
        }
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
        let mut dungeon = Dungeon::new();
        let mut after_empty_line = false;
        let mut check_links_title = false;
        let mut lines = reader.lines().into_iter();

        // 첫 번째 라인에 해당하는 규칙을 체크합니다.
        match lines.next() {
            Some(line) => {
                match line {
                    Ok(line) => {
                        if line.is_empty() {
                            // 첫 번째 라인이 비어있으면 에러를 반환합니다.
                            return Err(Errors::LineParseError { line_number: (0) });
                        }
                        if match_prefix("## ", line.as_str()) != Some("Rooms") {
                            // 첫 번째 라인은 반드시 '## Rooms'가 나와야 합니다.
                            return Err(Errors::LineParseError { line_number: (1) });
                        }
                    }
                    Err(_) => return Err(Errors::IoError(io::Error::last_os_error())),
                }
            }
            None => return Err(Errors::LineParseError { line_number: (0) }),
        };

        let mut line_number = 2;
        for line in lines {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        // 첫 번째 라인 이후에 빈칸이 나오면 상태를 변경합니다.
                        after_empty_line = true;
                        check_links_title = true;
                        line_number += 1;
                        continue;
                    }
                    if check_links_title {
                        // 빈칸 다음은 '## Links'가 나와야 합니다.
                        match match_prefix("## ", line.as_str()) {
                            Some(matched) => {
                                if matched == "Links" {
                                    check_links_title = false;
                                    line_number += 1;
                                    continue;
                                } else {
                                    return Err(Errors::LineParseError {
                                        line_number: (line_number),
                                    });
                                }
                            }
                            _ => {
                                return Err(Errors::LineParseError {
                                    line_number: (line_number),
                                });
                            }
                        }
                    }

                    let match_dash = match_prefix("- ", line.as_str());
                    match match_dash {
                        Some(matched) => {
                            if !after_empty_line {
                                match dungeon.add_room(matched) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        return Err(e);
                                    }
                                }
                            } else {
                                if let [room_name, direction, other_room_name] =
                                    matched.split(" -> ").collect::<Vec<&str>>()[..]
                                {
                                    match direction {
                                        "East" | "West" | "North" | "South" => {
                                            match dungeon.set_link(
                                                room_name,
                                                Direction::from_str(direction).unwrap(),
                                                other_room_name,
                                            ) {
                                                Ok(_) => {}
                                                Err(e) => {
                                                    return Err(e);
                                                }
                                            }
                                        }
                                        _ => {
                                            return Err(Errors::DirectionParseError(
                                                direction.to_string(),
                                            ))
                                        }
                                    };
                                } else {
                                    return Err(Errors::LineParseError {
                                        line_number: (line_number),
                                    });
                                }
                            }
                        }
                        None => {
                            return Err(Errors::LineParseError {
                                line_number: line_number,
                            })
                        }
                    }
                }
                Err(_) => return Err(Errors::IoError(io::Error::last_os_error())),
            }
            line_number += 1;
        }
        Ok(dungeon)
    }
}

/// match_prefix("- ", "- Foo") //=> Some("Foo")
/// match_prefix("- ", "Bar")   //=> None
///
fn match_prefix<'a, 'b>(prefix: &'a str, input: &'b str) -> Option<&'b str> {
    if input.starts_with(prefix) {
        Some(&input[prefix.len()..])
    } else {
        None
    }
}

impl Dungeon {
    /// Searches for a path from `start_room_name` to `end_room_name` and returns it in a vector packed in
    /// `Ok(Some)` if found.
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
        let start_room = self.get_room(start_room_name)?;
        let end_room = self.get_room(end_room_name)?;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(vec![start_room]);
        while let Some(path) = queue.pop_front() {
            let last_room = path.last().unwrap();
            if last_room.name == end_room.name {
                return Ok(Some(path));
            }
            if visited.insert(last_room.name.to_string()) {
                for direction in &[
                    Direction::North,
                    Direction::South,
                    Direction::East,
                    Direction::West,
                ] {
                    if let Some(next_room) =
                        self.get_next_room(last_room.name.as_str(), *direction)?
                    {
                        let mut next_path = path.clone();
                        next_path.push(next_room);
                        queue.push_back(next_path);
                    }
                }
            }
        }
        Ok(None)
    }
}
