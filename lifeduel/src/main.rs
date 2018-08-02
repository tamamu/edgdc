extern crate ggez;
use std::cmp;
use ggez::*;
use ggez::graphics::{Color, DrawMode, Rect};

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Blank,
    Player,
    Enemy,
    Wall,
}

const WINDOW_W: u32 = 800;
const WINDOW_H: u32 = 600;
const BOARD_SIZE: usize = 40;
const BOARD_OFFSET_X: i32 = (WINDOW_W as i32)/2 - 8*(BOARD_SIZE as i32)/2;
const BOARD_OFFSET_Y: i32 = (WINDOW_H as i32)/2 - 8*(BOARD_SIZE as i32)/2;
type Board = [[Tile; BOARD_SIZE]; BOARD_SIZE];

struct MainState {
    board: Board,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut b = [[Tile::Blank; BOARD_SIZE]; BOARD_SIZE];
        b[3][3] = Tile::Player;
        b[3][4] = Tile::Player;
        b[3][5] = Tile::Player;
        let s = MainState {
            board: b,
        };
        Ok(s)
    }
}

fn count_neighbor(board: &Board, x: usize, y: usize) -> (usize, usize) {
    let left = cmp::max(1, x) - 1;
    let right = cmp::min(x, BOARD_SIZE-2) + 1;
    let top = cmp::max(1, y) - 1;
    let bottom = cmp::min(y, BOARD_SIZE-2) + 1;
    let mut player_count = 0;
    let mut enemy_count = 0;
    for j in top..bottom+1 {
        for k in left..right+1 {
            if !(j==y && k==x) {
                match board[j][k] {
                    Tile::Player => player_count += 1,
                    Tile::Enemy => enemy_count += 1,
                    _ => {}
                }
            }
        }
    }
    (player_count, enemy_count)
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let mut new_board = self.board.clone();
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let (player_count, enemy_count) = count_neighbor(&self.board, x, y);
                let diff = player_count as i32 - enemy_count as i32;
                match self.board[y][x] {
                    Tile::Blank => {
                        if diff == 3 {
                            new_board[y][x] = Tile::Player;
                        } else if diff == -3 {
                            new_board[y][x] = Tile::Enemy;
                        }
                    },
                    Tile::Player => {
                        if diff == 2 || diff == 3 {
                            new_board[y][x] = Tile::Player;
                        } else if diff == -2 || diff == -3 {
                            new_board[y][x] = Tile::Enemy;
                        } else {
                            new_board[y][x] = Tile::Blank;
                        }
                    },
                    Tile::Enemy => {
                        if diff == 2 || diff == 3 {
                            new_board[y][x] = Tile::Player;
                        } else if diff == -2 || diff == -3 {
                            new_board[y][x] = Tile::Enemy;
                        } else {
                            new_board[y][x] = Tile::Blank;
                        }
                    }
                    _ => {}
                }
            }
        }
        self.board = new_board;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        // Draw Blank
        graphics::set_color(ctx, Color::new(0.1, 0.1, 0.1, 1.0))?;
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if self.board[y][x] == Tile::Blank {
                    let pos_x = x as i32 * 8;
                    let pos_y = y as i32 * 8;
                    graphics::rectangle(ctx,
                                        DrawMode::Fill,
                                        Rect::new_i32(BOARD_OFFSET_X+pos_x,
                                                      BOARD_OFFSET_Y+pos_y,
                                                      8,
                                                      8))?;
                }
            }
        }
        // Draw Player
        graphics::set_color(ctx, Color::new(0.1, 0.8, 0.2, 1.0))?;
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if self.board[y][x] == Tile::Player {
                    let pos_x = x as i32 * 8;
                    let pos_y = y as i32 * 8;
                    graphics::rectangle(ctx,
                                        DrawMode::Fill,
                                        Rect::new_i32(BOARD_OFFSET_X+pos_x,
                                                      BOARD_OFFSET_Y+pos_y,
                                                      8,
                                                      8))?;
                }
            }
        }
        // Draw Enemy
        graphics::set_color(ctx, Color::new(0.8, 0.2, 0.1, 1.0))?;
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if self.board[y][x] == Tile::Enemy {
                    let pos_x = x as i32 * 8;
                    let pos_y = y as i32 * 8;
                    graphics::rectangle(ctx,
                                        DrawMode::Fill,
                                        Rect::new_i32(BOARD_OFFSET_X+pos_x,
                                                      BOARD_OFFSET_Y+pos_y,
                                                      8,
                                                      8))?;
                }
            }
        }
        // Draw Wall
        graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0))?;
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if self.board[y][x] == Tile::Wall {
                    let pos_x = x as i32 * 8;
                    let pos_y = y as i32 * 8;
                    graphics::rectangle(ctx,
                                        DrawMode::Fill,
                                        Rect::new_i32(BOARD_OFFSET_X+pos_x,
                                                      BOARD_OFFSET_Y+pos_y,
                                                      8,
                                                      8))?;
                }
            }
        }

        graphics::present(ctx);
        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("lifeduel", "eddie", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    graphics::set_mode(ctx, conf::WindowMode {
        width: WINDOW_W,
        height: WINDOW_H,
        borderless: false,
        fullscreen_type: conf::FullscreenType::Off,
        vsync: false,
        min_width: 0,
        max_width: 0,
        min_height: 0,
        max_height: 0,
    }).unwrap();
    event::run(ctx, state).unwrap();
}
