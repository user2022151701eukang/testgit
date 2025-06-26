use specs::{Join, ReadStorage, System, Write};
use std::collections::HashMap;

use crate::{
    components::{Box, BoxSpot, Position, Player, Spike}, // 添加Player和Spike
    resources::{Gameplay, GameplayState},
};

pub struct GameplayStateSystem {}

impl<'a> System<'a> for GameplayStateSystem {
    // Data
    type SystemData = (
        Write<'a, Gameplay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
        ReadStorage<'a, Player>, // 添加玩家组件
        ReadStorage<'a, Spike>,  // 添加刺组件
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gameplay_state, positions, boxes, box_spots, players, spikes) = data;

        // 如果已经失败或胜利，不再检测
        if let GameplayState::Won | GameplayState::Failed = gameplay_state.state {
            return;
        }

        // 获取所有刺的位置
        let spike_positions: Vec<(u8, u8)> = (&positions, &spikes)
            .join()
            .map(|(pos, _)| (pos.x, pos.y))
            .collect();

        // 检查玩家是否在刺上
        for (player_pos, _) in (&positions, &players).join() {
            if spike_positions.contains(&(player_pos.x, player_pos.y)) {
                gameplay_state.state = GameplayState::Failed;
                return;
            }
        }

        // 检查箱子是否在刺上
        for (box_pos, _) in (&positions, &boxes).join() {
            if spike_positions.contains(&(box_pos.x, box_pos.y)) {
                gameplay_state.state = GameplayState::Failed;
                return;
            }
        }

        // 原有的胜利条件检测
        let boxes_by_position: HashMap<(u8, u8), &Box> = (&positions, &boxes)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect::<HashMap<_, _>>();

        for (box_spot, position) in (&box_spots, &positions).join() {
            if let Some(the_box) = boxes_by_position.get(&(position.x, position.y)) {
                if the_box.colour == box_spot.colour {
                    // continue
                } else {
                    // return, haven't won yet
                    gameplay_state.state = GameplayState::Playing;
                    return;
                }
            } else {
                gameplay_state.state = GameplayState::Playing;
                return;
            }
        }

        // 如果所有箱子都放置正确，胜利
        gameplay_state.state = GameplayState::Won;
    }
}