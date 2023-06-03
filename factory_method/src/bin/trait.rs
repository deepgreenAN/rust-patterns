/// Productの実装すべきトレイト
trait Room {
    fn render(&self);
}

/// Creatorの実装すべきトレイト．プロダクトの型を静的に定めるため型パラメータ―か関連型を利用する．
/// 今回はこのトレイトを利用する型とRoomを実装する型が一対一で定まるため関連型を利用する．
trait MazeGame {
    type RoomType: Room;
    // ファクトリメソッド．MazeGameで実装が要求される．
    fn rooms(&self) -> Vec<Self::RoomType>;

    // Roomトレイトに対して実装されたロジック
    fn play(&self) {
        for room in self.rooms() {
            room.render();
        }
    }
}

// -------------------------------------------------------------------------------------------------
// MagicMaze

#[derive(Clone)]
pub struct MagicRoom {
    title: String,
}

impl MagicRoom {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}

impl Room for MagicRoom {
    fn render(&self) {
        println!("Magic Room: {}", self.title);
    }
}

pub struct MagicMaze {
    rooms: Vec<MagicRoom>,
}

impl MagicMaze {
    pub fn new() -> Self {
        Self {
            rooms: vec![
                MagicRoom::new("Infinite Room".into()),
                MagicRoom::new("Red Room".into()),
            ],
        }
    }
}

impl MazeGame for MagicMaze {
    type RoomType = MagicRoom;

    fn rooms(&self) -> Vec<Self::RoomType> {
        self.rooms.clone()
    }
}

// -------------------------------------------------------------------------------------------------
// OrdinaryRoom

#[derive(Clone)]
pub struct OrdinaryRoom {
    id: u32,
}

impl OrdinaryRoom {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

impl Room for OrdinaryRoom {
    fn render(&self) {
        println!("Ordinary Room: #{}", self.id);
    }
}

pub struct OrdinaryMaze {
    rooms: Vec<OrdinaryRoom>,
}

impl OrdinaryMaze {
    pub fn new() -> Self {
        Self {
            rooms: vec![OrdinaryRoom::new(1), OrdinaryRoom::new(2)],
        }
    }
}

impl MazeGame for OrdinaryMaze {
    type RoomType = OrdinaryRoom;

    fn rooms(&self) -> Vec<Self::RoomType> {
        let mut rooms = self.rooms.clone();
        rooms.reverse();

        rooms
    }
}

// -------------------------------------------------------------------------------------------------

fn game_run(maze_game: impl MazeGame) {
    println!("Loading resources...");
    println!("Starting the game...");

    maze_game.play();
}
fn main() {
    let ordinary_maze = OrdinaryMaze::new();

    game_run(ordinary_maze);

    let magic_maze = MagicMaze::new();

    game_run(magic_maze);
}
