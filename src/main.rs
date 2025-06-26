use ggez;
use ggez::event::KeyCode;
use ggez::event::KeyMods;
use ggez::{conf, event, timer, Context, GameResult};
use specs::{RunNow, World, WorldExt};
use std::path;
use rand::Rng;

mod audio;
mod components;
mod constants;
mod entities;
mod events;
mod map;
mod resources;
mod systems;

use crate::audio::*;
use crate::components::*;
use crate::map::*;
use crate::resources::*;
use crate::systems::*;

struct Game {
    world: World,
    restart_requested: bool,
}

impl Game {
    // 添加重启游戏的方法
    fn restart_game(&mut self, context: &mut Context) {
        // 重置游戏世界
        self.world.delete_all();
        
        // 重置资源
        {
            let mut gameplay = self.world.write_resource::<Gameplay>();
            gameplay.state = GameplayState::Playing;
            gameplay.moves_count = 0;
        }
        
        {
            let mut input_queue = self.world.write_resource::<InputQueue>();
            input_queue.keys_pressed.clear();
        }
        
        {
            let mut event_queue = self.world.write_resource::<EventQueue>();
            event_queue.events.clear();
        }
        
        {
            let mut time = self.world.write_resource::<Time>();
            time.delta = std::time::Duration::new(0, 0);
        }
        
        // 重新初始化关卡
        initialize_level(&mut self.world);
        
        // 重新初始化声音
        initialize_sounds(&mut self.world, context);
        
        self.restart_requested = false;
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult {
        // 检查是否需要重启游戏
        if self.restart_requested {
            self.restart_game(context);
        }
        
        // 检查游戏是否结束（胜利或失败）
        {
            let gameplay = self.world.read_resource::<Gameplay>();
            if let GameplayState::Won | GameplayState::Failed = gameplay.state {
                // 游戏已结束，跳过更新逻辑
                return Ok(());
            }
        }
        
        // Run input system
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }

        // Run gameplay state system
        {
            let mut gss = GameplayStateSystem {};
            gss.run_now(&self.world);
        }

        // Get and update time resource
        {
            let mut time = self.world.write_resource::<Time>();
            time.delta += timer::delta(context);
        }

        // Run event system
        {
            let mut es = EventSystem {};
            es.run_now(&self.world);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        // Render game entities
        {
            let mut rs = RenderingSystem { context };
            rs.run_now(&self.world);
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        println!("Key pressed: {:?}", keycode);

        // 检查游戏是否结束
        let gameplay = self.world.read_resource::<Gameplay>();
        
        if let GameplayState::Won | GameplayState::Failed = gameplay.state {
            // 游戏结束状态下，只响应空格键
            if keycode == KeyCode::Space {
                println!("Restarting game...");
                self.restart_requested = true;
                return;
            }
        }
        
        // 正常游戏状态下，记录按键
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(keycode);
    }
}

// 定义10个关卡（5个有刺，5个没有刺）
const LEVELS: [&str; 10] = [
    // 关卡 1 - 无刺
    "
    N N W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . RB . . . W 
    W . P . . . . W
    W . . . . RS . W
    W . . BS . . . W
    W . . . . . . W
    W W W W W W W W
    ",
    // 关卡 2 - 无刺
    "
    N N W W W W W W
    W W W . . . . W
    W . . BB . . . W
    W . RB . . . . W 
    W . . . P . . W
    W . . . . RS . W
    W . . BS . . . W
    W . . . . . . W
    W W W W W W W W
    ",
    // 关卡 3 - 有刺
    "
    N N W W W W W W
    W W W . . . . W
    W . . . . . . W
    W . BB RB P . W 
    W . . . . . . W
    W . . . . . . W
    W . BS RS . . W
    W . S S . . . W
    W W W W W W W W
    ",
    // 关卡 4 - 无刺
    "
    N N W W W W W W
    W W W . . . . W
    W . . . . . . W
    W . . BB . . . W
    W . RB . . . . W 
    W . . . P . . W
    W . . . . RS . W
    W . . BS . . . W
    W W W W W W W W
    ",
    // 关卡 5 - 有刺
    "
    N N W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . RB . . . W 
    W . P . . . . W
    W . . . . RS . W
    W . . BS . . . W
    W . S . . . . W
    W W W W W W W W
    ",
    // 关卡 6 - 无刺
    "
    N N W W W W W W
    W W W . . . . W
    W . . . . . . W
    W . BB . . . . W
    W . . RB . . . W 
    W . P . . . . W
    W . . . . RS . W
    W . . BS . . . W
    W W W W W W W W
    ",
    // 关卡 7 - 有刺
    "
    N W W W W W W W
    W W . . . . . W
    W . BB . . . . W
    W . . RB . . . W
    W . P . . . . W 
    W . . . . RS . W
    W . . BS . . . W
    W . S S . . . W
    W W W W W W W W
    ",
    // 关卡 8 - 无刺
    "
    N N W W W W W W
    W W W . . . . W
    W . . . . . . W
    W . . . BB . . W
    W . RB . . . . W 
    W . . P . . . W
    W . . . . RS . W
    W . . BS . . . W
    W W W W W W W W
    ",
    // 关卡 9 - 有刺
    "
    N N W W W W W W
    W W . . . . . W
    W . BB . . . . W
    W . . RB . S . W
    W . P . . . . W 
    W . . . . RS . W
    W . . BS . . . W
    W . S . . . . W
    W W W W W W W W
    ",
    // 关卡 10 - 有刺
    "
    N N W W W W W W
    W W W . . . . W
    W . . . . . . W
    W . . . . . . W
    W . BB RB P . W 
    W . . . . . . W
    W . BS RS . . W
    W . S S . . . W
    W W W W W W W W
    "
];

// 随机选择关卡
pub fn initialize_level(world: &mut World) {
    let mut rng = rand::thread_rng();
    let level_index = rng.gen_range(0..LEVELS.len());
    let map = LEVELS[level_index];
    
    println!("随机选择关卡: {}", level_index + 1);
    load_map(world, map.to_string());
}

pub fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = &mut context_builder.build()?;
    initialize_sounds(&mut world, context);

    // Create the game state
    let game = &mut Game { 
        world,
        restart_requested: false,
    };
    
    // Run the main event loop
    event::run(context, event_loop, game)
}
