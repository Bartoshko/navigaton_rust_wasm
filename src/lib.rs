extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;
mod navigation_service;
mod maze_creator;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// wasm-bindgen for facilitating high-level interactions between wasm modules and JavaScript.
#[wasm_bindgen(js_namespace = console)]
extern "C" {
    fn log(s: &str);
}

#[wasm_bindgen()]
pub fn create_maze(path_lines_num: i32, branches_num: i32) -> Vec<i32> {
    let line_maze: maze_creator::LineMaze = maze_creator::LineMaze::new(path_lines_num, branches_num);
    line_maze.create()
}

#[wasm_bindgen()]
pub fn navigate(given_maze: Vec<i32>, starting_position: Vec<i32>, targeted_position: Vec<i32>) -> Vec<i32> {
    let new_dijkstra_result: Result<navigation_service::Dijkstra, &str> = navigation_service::Dijkstra::new(given_maze);
    let point_to_start_from = navigation_service::Point {x :starting_position[0], y: starting_position[1]};
    let point_to_calc_path_to = navigation_service::Point {x :targeted_position[0], y: targeted_position[1]};
    let dijkstra: navigation_service::Dijkstra;
    if new_dijkstra_result.is_err() {
        return vec![starting_position[0], starting_position[1]]
    }
    dijkstra = new_dijkstra_result.unwrap();
    dijkstra.calculate_shortest_path(point_to_start_from, point_to_calc_path_to)
}
