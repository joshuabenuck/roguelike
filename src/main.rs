mod components;
mod map;
mod player;
mod rect;

use components::{Player, Position, Renderable};
use map::{draw_map, new_map_rooms_and_cooridors, xy_idx, TileType};
use player::player_input;
use rect::Rect;
use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() {
    use rltk::RltkBuilder;
    let context = match RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()
    {
        Ok(c) => c,
        Err(err) => panic!(err),
    };
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    gs.ecs.insert(new_map_rooms_and_cooridors());

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    if let Err(err) = rltk::main_loop(context, gs) {
        panic!(err);
    };
}
