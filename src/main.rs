#![allow(unused)]

use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use crossterm::style::Color;

use enigo::{Enigo, InputError, Key, Settings};
use terminal_menu::{button, label, menu, mut_menu, run, wait_for_exit, string};
use crate::cursor::Cursor;

use crate::display::{path_to_pos, Point, rect_to_points, Rectangle};
use crate::puzzle::{Puzzle, PuzzleState, solve_puzzle};

use std::{fs, io, ptr, thread};
use std::path::Path;
use std::sync::{Arc, Mutex};
use livesplit_hotkey::{Hook, Hotkey, KeyCode};

const VK_TO_CHAR: u32 = 2;

mod puzzle;
mod display;
mod cursor;

fn main() {
    let enigo = Enigo::new(&Settings::default()).unwrap();
    let mut cursor = Cursor::new(enigo);

    let menu1 = menu(vec![
        label("Use wasd or arrow keys to navigate").colorize(Color::Yellow),
        label("Press enter to select").colorize(Color::Yellow),
        label("'q' or esc or exit").colorize(Color::Yellow),
        label(""),
        label("------ SlideSolver ------").colorize(Color::Magenta),
        label("What would you like to do?"),
        label(""),
        button("Set screen coordinates"),
        button("Solve puzzle"),
        label("-------------------------").colorize(Color::Magenta),
    ]);
    run(&menu1);
    let menu_result = mut_menu(&menu1);

    // Set screen coordinates
    if menu_result.selected_item_index() == 7 {
        println!("To capture current coordinates, press 'r'");
        println!("First capture over the top left corner of the game square");
        println!("Then capture over the bottom right corner of the game square");

        let coordinates: Arc<Mutex<Vec<Point>>> = Arc::new(Mutex::new(vec![]));
        let coordinates_clone = Arc::clone(&coordinates);

        let hook = Hook::new().unwrap();
        hook.register(Hotkey::from(KeyCode::KeyR), move || {
            let mut coordinates = coordinates_clone.lock().unwrap();
            match coordinates.len() {
                len @ (0 | 1) => {
                    let coor = cursor.location().unwrap();
                    coordinates.push(Point {
                        x: coor.0,
                        y: coor.1,
                    });
                    let message = if len == 0 {
                        "First"
                    } else {
                        "Second"
                    };
                    println!("{} coordinate saved ({}, {})", message, coor.0, coor.1);
                }
                _ => {}
            }
        }).unwrap();

        while coordinates.lock().unwrap().len() < 2 {
            // Add a sleep to prevent busy-waiting
            sleep(Duration::from_millis(100));
        }

        let coordinates = coordinates.lock().unwrap();
        let mut file = File::create("./slide_solver.txt").unwrap();
        file.write_all(format!("{},{},{},{}",
                               coordinates[0].x,
                               coordinates[0].y,
                               coordinates[1].x,
                               coordinates[1].y).as_bytes()).unwrap();

        let menu2 = menu(vec![
            label("------ SlideSolver ------").colorize(Color::Magenta),
            label("Screen coordinates have been saved").colorize(Color::Green),
            button("Press enter to exit"),
            label("-------------------------").colorize(Color::Magenta),
        ]);
        run(&menu2);
        exit(0);
    }
    // Solve puzzle
    else if menu_result.selected_item_index() == 8 {
        if !Path::new("./slide_solver.txt").exists() {
            run(&menu(vec![
                label("------ SlideSolver ------").colorize(Color::Magenta),
                label("Set screen coordinates before solving").colorize(Color::Red),
                button("Press enter to exit"),
                label("-------------------------").colorize(Color::Magenta),
            ]));
            exit(0);
        }

        let raw_text = fs::read_to_string("./slide_solver.txt").unwrap();
        let split_coordinates: Vec<i32> = raw_text.split(",").collect::<Vec<&str>>().into_iter().map(|item| -> i32{ item.parse().unwrap() }).collect();

        let main_display = cursor.get_screen_size().unwrap();
        let screen_size = Point { x: main_display.0, y: main_display.1 };
        let grid_positions = Rectangle::new(Point { x: split_coordinates[0], y: split_coordinates[1] }, Point { x: split_coordinates[2], y: split_coordinates[3] }, None).grid_positions(screen_size);

        let menu = menu(vec![
            label("------ SlideSolver ------").colorize(Color::Magenta),
            label("Enter game position with the empty space being '0'"),
            string("Position", "123456780", false),
            button("Confirm"),
            label("-------------------------").colorize(Color::Magenta),
        ]);
        run(&menu);
        let result = mut_menu(&menu);

        let puzzle_nums: Puzzle = string_to_i32_vec(result.selection_value("Position").to_string());
        let puzzle = PuzzleState::new(puzzle_nums, 0);

        match solve_puzzle(puzzle) {
            Some((path, cost)) => {
                println!("Found solution with cost {}", cost);
                let points = rect_to_points(&grid_positions);

                println!("Staring in 3");
                sleep(Duration::from_secs(1));
                println!("Staring in 2");
                sleep(Duration::from_secs(1));
                println!("Staring in 1");
                sleep(Duration::from_secs(1));

                for point in path_to_pos(&path.clone(), &to_matrix(points, 3)) {
                    cursor.click_point(point, 15)
                }
            }
            None => println!("No solution found!")
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}


fn to_matrix<T>(vec: Vec<T>, row_size: usize) -> Vec<Vec<T>> where T: Clone {
    vec.chunks(row_size).map(|chunk| chunk.to_vec()).collect()
}

fn string_to_i32_vec(input: String) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut row: Vec<u8> = Vec::new();

    for (index, num_str) in input.chars().enumerate() {
        let num = num_str.to_digit(10);

        match num {
            Some(res) => {
                row.push(res as u8);
            }
            None => {
                panic!("Invalid game character: \"{}\"", num_str);
            }
        }

        if (index + 1) % 3 == 0 {
            grid.push(row);
            row = Vec::new();
        }
    }
    grid
}