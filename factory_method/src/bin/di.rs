// -------------------------------------------------------------------------------------------------
// Roomトレイト

/// プロダクトが実装すべきトレイト．
trait Room: Clone {
    fn render(&self);
}

#[derive(Clone)]
struct MagicRoom {
    title: String,
}

impl Room for MagicRoom {
    fn render(&self) {
        println!("Magic Room: {}", self.title);
    }
}

#[derive(Clone)]
struct OrdinaryRoom {
    id: u32,
}

impl Room for OrdinaryRoom {
    fn render(&self) {
        println!("Ordinary Room: #{}", self.id);
    }
}

// -------------------------------------------------------------------------------------------------
// MazeGame

struct MazeGame<T: Room> {
    rooms: Vec<T>,
}

impl<T: Room> MazeGame<T> {
    fn new<F>(maze_factory: F) -> Self
    where
        F: Fn() -> Vec<T>,
    {
        Self {
            rooms: maze_factory(),
        }
    }
    fn play(&self) {
        for room in self.rooms.iter() {
            room.render();
        }
    }
}

// -------------------------------------------------------------------------------------------------
// 各種ファクトリ

fn magic_factory() -> Vec<MagicRoom> {
    vec![
        MagicRoom {
            title: "Infinite Room".to_string(),
        },
        MagicRoom {
            title: "Red Room".to_string(),
        },
    ]
}

fn ordinary_factory() -> Vec<OrdinaryRoom> {
    let mut rooms = vec![OrdinaryRoom { id: 1 }, OrdinaryRoom { id: 2 }];
    rooms.reverse();
    rooms
}

// -------------------------------------------------------------------------------------------------

fn game_run<T: Room>(maze_game: MazeGame<T>) {
    println!("Loading resources...");
    println!("Starting the game...");
    maze_game.play();
}

fn main() {
    let magic_maze_game = MazeGame::new(magic_factory);
    game_run(magic_maze_game);

    let ordinary_maze_game = MazeGame::new(ordinary_factory);
    game_run(ordinary_maze_game);
}
