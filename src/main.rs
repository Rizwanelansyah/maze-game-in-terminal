mod maps;

use std::{io, i32, usize};
use k_board::{keyboard::Keyboard, keys::Keys};
use figlet_rs::FIGfont;
use maps::get_maps;

use rand::Rng;

enum Direction {
    Up, Down, Left, Right, Stay
}

struct Point {
    x: i32, y: i32,
}

impl Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn clone(&self) -> Point {
        Point { x: self.x, y: self.y }
    }
}

impl Direction {
    fn from(ch: char) -> Direction {
        match ch {
            'w' => Direction::Up,
            's' => Direction::Down,
            'a' => Direction::Left,
            'd' => Direction::Right,
            _ => Direction::Stay,
        }
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn render_map(map: &Vec<Vec<i32>>) {
    clear_screen();
    render_map_noclear(map);
}

fn render_map_noclear(map: &Vec<Vec<i32>>) {
    for row in map {
        let mut line = String::new();
        for code in row {
            line += match code {
                1 => "\x1b[48;2;250;250;255m  \x1b[0m",
                5 => "\x1b[48;2;0;0;250m  \x1b[0m",
                9 => "\x1b[48;2;250;250;0m  \x1b[0m",
                _ => "  ",
            };
        }
        println!("{line}");
    }
}

fn find_in_map(map: &Vec<Vec<i32>>, num: i32) -> Point {
    let mut pos = Point { x: 0, y: 0 };
    
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == num {
                pos = Point { x: x as i32, y: y as i32 };
            }
        }
    }

    pos
}

fn get_with_point(map: &Vec<Vec<i32>>, point: &Point) -> i32 {
    map[point.y as usize][point.x as usize]
}

fn read_move() -> Direction {
    println!("Move with w(up) a(left) s(down) d(right)");
    for key in Keyboard::new() {
        match key {
            Keys::Char(ch) => {
                return Direction::from(ch);
            },
            _ => {
                return Direction::Stay;
            }
        }
    }
    return Direction::Stay;
}

fn draw_title(title: &str) {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert(title);
    println!("{}", figure.unwrap());
}

fn main() {
    let maps = get_maps();
    let solid_obj_codes = [1];
    let index = rand::thread_rng().gen_range(0..maps.len());
    let mut turns = 0;
    let mut selected_map = maps[index].clone();
    let mut player_pos = find_in_map(&selected_map, 9);
    let win_pos = find_in_map(&selected_map, 5);

    clear_screen();
    draw_title("Welcome to The MAZE");
    draw_title("Youre the Yellow Box");
    draw_title("You must go to Blue Box");
    println!("Press Enter to continue");
    render_map_noclear(&selected_map);

    let mut o = String::new();
    let _ = io::stdin().read_line(&mut o);

    println!("Turns: {}", turns);
    render_map(&selected_map);

    while !player_pos.eq(&win_pos) {
        
        let last_player_pos = player_pos.clone();
        match read_move() {
            Direction::Up => {
                if player_pos.y > 0 {
                    player_pos.y -= 1;
                    if solid_obj_codes.contains(&get_with_point(&selected_map, &player_pos)) {
                        player_pos.y += 1;
                    }
                }
            },
            Direction::Down => {
                if (player_pos.y as usize) < (selected_map.len() - 1) {
                    player_pos.y += 1;
                    if solid_obj_codes.contains(&get_with_point(&selected_map, &player_pos)) {
                        player_pos.y -= 1;
                    }
                }
            },
            Direction::Left => {
                if player_pos.x > 0 {
                    player_pos.x -= 1;
                    if solid_obj_codes.contains(&get_with_point(&selected_map, &player_pos)) {
                        player_pos.x += 1;
                    }
                }
            },
            Direction::Right => {
                if (player_pos.x as usize) < (selected_map[0].len() - 1) {
                    player_pos.x += 1;
                    if solid_obj_codes.contains(&get_with_point(&selected_map, &player_pos)) {
                        player_pos.x -= 1;
                    }
                }
            },
            Direction::Stay => (),
        }

        if !last_player_pos.eq(&player_pos) {
            selected_map[last_player_pos.y as usize][last_player_pos.x as usize] = 0;
            selected_map[player_pos.y as usize][player_pos.x as usize] = 9;
            turns += 1;
        }

        render_map(&selected_map);
        println!("Turns: {}", turns);
    }

    draw_title("You Win!");
}
