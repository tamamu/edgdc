extern crate ggez;
use ggez::*;
use ggez::graphics::{Color, DrawMode, Point2};

struct MainState {
    p: f32,
    r: f32,
    b: f32,
    x: f32,
    y: f32,
    z: f32,
    t: f32,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            p: 10.0,
            r: 28.0,
            b: 8.0/3.0,
            x: 0.10,
            y: 0.00,
            z: 0.00,
            t: 0.00,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let dt = 0.01;
        let x = self.x + (-self.p * self.x + self.p * self.y)*dt;
        let y = self.y + (-self.x * self.z + self.r * self.x - self.y)*dt;
        let z = self.z + (self.x * self.y - self.b * self.z)*dt;
        self.x = x;
        self.y = y;
        self.z = z;
        self.t += 0.1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, Color::new(self.t.asin(), self.t.sin(), self.t.cos(), 1.0))?;
        graphics::circle(ctx,
                         DrawMode::Fill,
                         Point2::new(400.+self.x, 300.+self.y),
                         8.,
                         1.)?;
        graphics::present(ctx);
        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("lifeduel", "eddie", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
