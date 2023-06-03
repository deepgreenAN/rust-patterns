// -------------------------------------------------------------------------------------------------
// Room<T>について

/// プロダクトであるRoom<T>が実装すべきトレイト．Tに委譲させている．
trait Render: Clone {
    fn render(&self);
}

/// MazeGameはこのRoom<T>を作成する．
#[derive(Clone)]
struct Room<T: Render> {
    room: T,
}

impl<T: Render> Render for Room<T> {
    fn render(&self) {
        self.room.render()
    }
}

#[derive(Clone)]
struct Magic {
    title: String,
}

impl Render for Magic {
    fn render(&self) {
        println!("Magic Room: {}", self.title);
    }
}

#[derive(Clone)]
struct Ordinary {
    id: u32,
}

impl Render for Ordinary {
    fn render(&self) {
        println!("Ordinary Room: #{}", self.id);
    }
}

// -------------------------------------------------------------------------------------------------
// MazeGameについて

trait RoomProvide<T: Render> {
    fn provide_room(&self) -> Vec<Room<T>>;
}

struct MazeGame<T, S>
where
    T: Render,
    S: RoomProvide<T>,
{
    provider: S,
    _product_type: std::marker::PhantomData<T>,
}

impl<T, S> MazeGame<T, S>
where
    T: Render,
    S: RoomProvide<T>,
{
    fn from_provider(provider: S) -> Self {
        Self {
            provider,
            _product_type: std::marker::PhantomData,
        }
    }
    fn play(&self) {
        for room in self.provider.provide_room() {
            room.render();
        }
    }
}

// -------------------------------------------------------------------------------------------------
// MagicMaze

struct MagicMaze<T: Render> {
    rooms: Vec<Room<T>>,
}

impl<T: Render> MagicMaze<T> {
    fn new(rooms: Vec<Room<T>>) -> Self {
        Self { rooms }
    }
}

impl<T: Render> RoomProvide<T> for MagicMaze<T> {
    fn provide_room(&self) -> Vec<Room<T>> {
        self.rooms.clone()
    }
}

/// T=Magic, S=MagicMaze<Magic>時だけ実装
impl MazeGame<Magic, MagicMaze<Magic>> {
    fn new() -> Self {
        let magic_maze = MagicMaze::new(vec![
            Room {
                room: Magic {
                    title: "Infinite Room".to_string(),
                },
            },
            Room {
                room: Magic {
                    title: "Red Room".to_string(),
                },
            },
        ]);
        MazeGame::from_provider(magic_maze)
    }
}

// -------------------------------------------------------------------------------------------------
// OrdinaryMaze
struct OrdinaryMaze<T: Render> {
    rooms: Vec<Room<T>>,
}

impl<T: Render> OrdinaryMaze<T> {
    fn new(rooms: Vec<Room<T>>) -> Self {
        Self { rooms }
    }
}

impl<T: Render> RoomProvide<T> for OrdinaryMaze<T> {
    fn provide_room(&self) -> Vec<Room<T>> {
        let mut rooms = self.rooms.clone();
        rooms.reverse();
        rooms
    }
}

/// T=Ordinary, S=OrdinaryMaze<Ordinary>の場合のみ実装
impl MazeGame<Ordinary, OrdinaryMaze<Ordinary>> {
    fn new() -> Self {
        let ordinary_maze = OrdinaryMaze::new(vec![
            Room {
                room: Ordinary { id: 1 },
            },
            Room {
                room: Ordinary { id: 2 },
            },
        ]);
        MazeGame::from_provider(ordinary_maze)
    }
}

// -------------------------------------------------------------------------------------------------

fn game_run<S, T>(magic_game: MazeGame<T, S>)
where
    T: Render,
    S: RoomProvide<T>,
{
    println!("Loading resources...");
    println!("Starting the game...");
    magic_game.play();
}

fn main() {
    let magic_maze_game = MazeGame::<Magic, MagicMaze<Magic>>::new();

    game_run(magic_maze_game);

    let ordinary_maze_game = MazeGame::<Ordinary, OrdinaryMaze<Ordinary>>::new();

    game_run(ordinary_maze_game);
}
