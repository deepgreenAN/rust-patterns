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

/// プロダクトを使ったロジックを記述．
struct MazeGame;

impl MazeGame {
    fn play<F, T>(&self, maze_factory: F)
    where
        F: Fn() -> Vec<T>,
        T: Room,
    {
        let rooms = maze_factory();
        for room in rooms.iter() {
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

fn game_run<F, T>(maze_game: &MazeGame, factory: F)
where
    F: Fn() -> Vec<T>,
    T: Room,
{
    println!("Loading resources...");
    println!("Starting the game...");
    maze_game.play(factory);
}

fn main() {
    let maze_game = MazeGame;

    game_run(&maze_game, magic_factory);

    game_run(&maze_game, ordinary_factory);
}
