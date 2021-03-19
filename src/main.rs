use graphics::{
    mesh::{Mesh, ShapeStyle},
    Color, DrawParams, Rectangle,
};
use hecs::{Entity, World};
use rand::Rng;
use tetra::{graphics, math::Vec2, window::quit, Context, State};
use tetra::{
    input::{is_key_pressed, Key},
    ContextBuilder,
};

const WIN_W: f32 = 1280.0;
const WIN_H: f32 = 720.0;

const PLAYER_SIZE: f32 = 15.0;

const PIPE_WIDTH: f32 = 50.0;
const PIPE_DIST: f32 = 550.0;
const PIPE_GAP: f32 = 100.0;
const PIPE_NUM: usize = 5;

struct Player;
struct Pipe;
struct GameState {
    world: World,
    player: Entity,
    pipes: Entity,
    last_pipe_ind: usize,
}

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Debug)]
struct Size {
    w: f32,
    h: f32,
}
impl GameState {
    fn new(_ctx: &mut Context) -> tetra::Result<GameState> {
        let mut world = World::new();
        let player = world.spawn((Player, Position { x: 100.0, y: 200.0 }, 0f32));

        let mut rng = rand::thread_rng();
        let mut pipes: Vec<Position> = vec![Position {
            x: WIN_W as f32,
            y: rng.gen_range(100..WIN_H as i32 - 100) as f32,
        }];

        for i in 1..PIPE_NUM {
            pipes.push(Position {
                x: pipes[i - 1].x + PIPE_DIST,
                y: rng.gen_range(100..WIN_H as i32 - 100) as f32,
            });
        }
        let pipes = world.spawn((Pipe, pipes));
        Ok(GameState {
            world,
            player,
            pipes,
            last_pipe_ind: PIPE_NUM - 1,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        for (_, (_, pos, vel)) in self.world.query_mut::<(&Player, &mut Position, &mut f32)>() {
            *vel += 0.8;
            if is_key_pressed(ctx, Key::Space) || is_key_pressed(ctx, Key::Up) {
                *vel = -10.0;
            }
            pos.y += *vel;
        }
        for (_, (_, pipes)) in self.world.query_mut::<(&Pipe, &mut Vec<Position>)>() {
            for i in 0..pipes.len() {
                pipes[i].x -= 5.0;
                if pipes[i].x < -PIPE_WIDTH {
                    pipes[i] = Position {
                        x: pipes[self.last_pipe_ind].x + PIPE_DIST,
                        y: rand::thread_rng().gen_range(100..WIN_H as i32 - 100) as f32,
                    };
                    self.last_pipe_ind = i;
                }
            }
        }
        let player = self.world.get_mut::<Position>(self.player).unwrap();
        let player_rect = Rectangle::new(player.x, player.y, PLAYER_SIZE, PLAYER_SIZE);

        let pipes = self.world.get_mut::<Vec<Position>>(self.pipes).unwrap();

        let mut top_pipes_rect = vec![Rectangle::new(
            pipes[0].x,
            0.0,
            PIPE_WIDTH,
            pipes[0].y - PIPE_GAP / 2.0,
        )];
        let mut bottom_pipes_rect = vec![Rectangle::new(
            pipes[0].x,
            pipes[0].y + PIPE_GAP / 2.0,
            PIPE_WIDTH,
            WIN_H,
        )];
        for i in 1..pipes.len() {
            top_pipes_rect.push(Rectangle::new(
                pipes[i].x,
                0.0,
                PIPE_WIDTH,
                pipes[i].y - PIPE_GAP / 2.0,
            ));
            bottom_pipes_rect.push(Rectangle::new(
                pipes[i].x,
                pipes[i].y + PIPE_GAP / 2.0,
                PIPE_WIDTH,
                WIN_H,
            ));
        }

        for i in 0..pipes.len() {
            if player_rect.intersects(&top_pipes_rect[i])
                || player_rect.intersects(&bottom_pipes_rect[i])
            {
                quit(ctx);
            }
        }
        if player.y < 0.0 || player.y > WIN_H {
            quit(ctx);
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::WHITE);
        for (_, (_, pipes)) in self.world.query_mut::<(&Pipe, &Vec<Position>)>() {
            for pipe in pipes {
                let top_rect = Mesh::rectangle(
                    ctx,
                    ShapeStyle::Fill,
                    Rectangle::new(pipe.x, 0.0, PIPE_WIDTH, pipe.y - PIPE_GAP / 2.0),
                )?;
                top_rect.draw(ctx, DrawParams::default().color(Color::GREEN));

                let bottom_rect = Mesh::rectangle(
                    ctx,
                    ShapeStyle::Fill,
                    Rectangle::new(pipe.x, pipe.y + PIPE_GAP / 2.0, PIPE_WIDTH, WIN_H),
                )?;
                bottom_rect.draw(ctx, DrawParams::default().color(Color::GREEN));
            }
        }
        for (_, (_, pos)) in self.world.query_mut::<(&Player, &Position)>() {
            let circle =
                Mesh::circle(ctx, ShapeStyle::Fill, Vec2::new(pos.x, pos.y), PLAYER_SIZE)?;
            circle.draw(ctx, DrawParams::default().color(Color::BLACK));
        }

        Ok(())
    }
}
fn main() -> tetra::Result {
    ContextBuilder::new("Flappy Bird", WIN_W as i32, WIN_H as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
