use specs::{Component, NullStorage, VecStorage, World, WorldExt, storage::DenseVecStorage};
use std::fmt;
use std::fmt::Display;

// Components
#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Immovable;

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Renderable {
    paths: Vec<String>,
    kind: RenderableKind,
}

impl Renderable {
    pub fn new_static(path: String) -> Self {
        Renderable {
            paths: vec![path],
            kind: RenderableKind::Static,
        }
    }

    pub fn new_animated(paths: Vec<String>) -> Self {
        Renderable {
            paths,
            kind: RenderableKind::Animated,
        }
    }

    pub fn kind(&self) -> RenderableKind {
        self.kind.clone()  // 改为克隆而不是移动
    }

    pub fn path(&self, path_index: usize) -> String {
        // If we get asked for a path that is larger than the
        // number of paths we actually have, we simply mod the index
        // with the length to get an index that is in range.
        self.paths[path_index % self.paths.len()].clone()
    }
}

#[derive(Debug, Clone)]  // 添加 Clone trait
pub enum RenderableKind {
    Static,
    Animated,
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Player;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Wall;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum BoxColour {
    Red,
    Blue,
}

impl Display for BoxColour {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            BoxColour::Red => "red",
            BoxColour::Blue => "blue",
        })?;
        Ok(())
    }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Box {
    pub colour: BoxColour,
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct BoxSpot {
    pub colour: BoxColour,
}

// 添加刺组件
#[derive(Component, Debug, Default)]  // 添加 Default trait
#[storage(NullStorage)]
pub struct Spike;

// 注册所有组件
pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Movable>();
    world.register::<Immovable>();
    world.register::<Wall>();
    world.register::<Box>();
    world.register::<BoxSpot>();
    world.register::<Spike>(); // 注册刺组件
}
