use anyhow::Result;
use rustbox::{Color, RustBox};
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;
mod cpu;
use cpu::Cpu;

const TILE_CODES: [char; 5] = [' ', '#', '▢', '▀', '●'];

struct Game {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
    score: usize,
    board: HashMap<(usize, usize), char>,
    blocks_total: usize,
    sender: Sender<i128>,
    receiver: Receiver<i128>,
}

impl Game {
    fn new(sender: Sender<i128>, receiver: Receiver<i128>) -> Result<Game> {
        let board = HashMap::new();

        Ok(Game {
            max_x: 0,
            min_x: 0,
            max_y: 0,
            min_y: 0,
            sender,
            receiver,
            board,
            score: 0,
            blocks_total: 0,
        })
    }

    fn joystick_position(&mut self) -> Result<()> {
        let ball_xy = self
            .board
            .iter()
            .find(|((_x, _y), v)| *v == &TILE_CODES[4])
            .unwrap_or((&(0, 0), &' '))
            .0;
        let paddle_xy = self
            .board
            .iter()
            .find(|((_x, _y), v)| *v == &TILE_CODES[3])
            .unwrap_or((&(0, 0), &' '))
            .0;
        let position = match true {
            true if ball_xy.0 < paddle_xy.0 => -1,
            true if ball_xy.0 > paddle_xy.0 => 1,
            _ => 0,
        };
        if self.sender.send(position).is_err() {
            anyhow::bail!("");
        }

        Ok(())
    }

    fn receive_tiles(&mut self) -> Result<()> {
        loop {
            if let Ok(x) = self.receiver.recv_timeout(Duration::from_millis(5)) {
                match (x, self.receiver.recv(), self.receiver.recv()) {
                    (-1, Ok(0), Ok(score)) => {
                        self.score = score as usize;
                    }
                    (x, Ok(y), Ok(tile)) => {
                        self.board
                            .insert((x as usize, y as usize), TILE_CODES[tile as usize]);
                    }
                    _ => anyhow::bail!("received error"),
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    fn run(&mut self, rb: &RustBox) -> Result<()> {
        if self.min_y == 0 {
            self.receive_tiles()?;
            self.max_x = self.board.keys().max_by_key(|(x, _)| x).unwrap().0;
            self.min_x = self.board.keys().min_by_key(|(x, _)| x).unwrap().0;
            self.max_y = self.board.keys().max_by_key(|(_, y)| y).unwrap().1;
            self.min_y = self.board.keys().min_by_key(|(_, y)| y).unwrap().1;
            self.blocks_total = self.board.values().filter(|t| *t == &TILE_CODES[2]).count();
        }
        loop {
            self.receive_tiles()?;
            self.print_board(rb)?;

            if self.joystick_position().is_err() {
                break;
            }
        }

        rb.print(
            0usize,
            (self.max_y + 2) as usize,
            rustbox::RB_NORMAL,
            Color::Blue,
            Color::Default,
            &format!("Part 1: {}", self.blocks_total),
        );
        rb.print(
            0usize,
            (self.max_y + 3) as usize,
            rustbox::RB_NORMAL,
            Color::Blue,
            Color::Default,
            &format!("Part 2: {}", self.score),
        );
        rb.print(
            0,
            self.max_y + 4,
            rustbox::RB_BOLD,
            Color::Red,
            Color::Default,
            "Press any to exit.",
        );
        rb.present();

        loop {
            match rb.poll_event(false) {
                Ok(rustbox::Event::KeyEvent(_)) => break,
                Ok(_) => {}
                Err(e) => anyhow::bail!(e),
            }
        }
        Ok(())
    }

    fn print_board(&self, rb: &RustBox) -> Result<()> {
        rb.clear();

        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                match self
                    .board
                    .get(&(x, y))
                    .ok_or_else(|| anyhow::anyhow!("not found"))
                {
                    Ok(ch) => {
                        rb.print_char(
                            x as usize,
                            y as usize,
                            rustbox::RB_NORMAL,
                            Color::Default,
                            Color::Default,
                            *ch,
                        );
                    }
                    Err(err) => {
                        anyhow::bail!(err);
                    }
                };
            }
        }

        let blocks = self.board.values().filter(|t| *t == &TILE_CODES[2]).count();

        rb.print(
            0usize,
            (self.max_y + 1) as usize,
            rustbox::RB_NORMAL,
            Color::Default,
            Color::Default,
            &format!(
                "Score: {}  Blocks: {}/{}",
                self.score, blocks, self.blocks_total
            ),
        );

        rb.present();

        Ok(())
    }
}

fn main() -> Result<()> {
    let mut prog = cpu::parse_input("resources/day13-input.txt")?;
    prog[0] = 2;

    let (sender, rx): (Sender<i128>, Receiver<i128>) = channel();
    let (tx2, receiver): (Sender<i128>, Receiver<i128>) = channel();
    let mut cpu = Cpu::new(&prog, tx2, rx);
    let th = thread::spawn(move || cpu.execute());

    let mut game = Game::new(sender, receiver)?;

    let rustbox = RustBox::init(Default::default())?;

    game.run(&rustbox)?;

    th.join().unwrap().unwrap();

    Ok(())
}
