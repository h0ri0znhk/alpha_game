use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::input::{self, MouseButton};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

fn main() -> tetra::Result {
    ContextBuilder::new("Alpha game", 1440, 960)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct GameState {
    background: Texture,
    mouse: Texture,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            background: Texture::new(ctx, "./resources/Leadenhall_Street.png")?,
            mouse: Texture::new(ctx, "./resources/player.png")?,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

        graphics::draw(
            ctx,
            &self.background,
            DrawParams::new()
                .position(Vec2::new(32.0, 32.0))
                .origin(Vec2::new(8.0, 8.0))
                .scale(Vec2::new(1.0, 1.0)),
        );

        graphics::draw(
            ctx,
            &self.mouse,
            DrawParams::new()
                .position(input::get_mouse_position(ctx).round())
                .origin(Vec2::new(8.0, 8.0))
                .scale(Vec2::new(3.0, 3.0)),
        );

        Ok(())
    }
}