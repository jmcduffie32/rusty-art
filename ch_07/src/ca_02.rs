use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    cells: Vec<Vec<i32>>,
    generation: i32,
}

fn initialize_random_cells(cells: &mut Vec<Vec<i32>>) {
    for i in 0..cells.len() {
        for j in 0..cells[i].len() {
            cells[i][j] = if random_f32() < 0.5 { 1 } else { 0 };
        }
    }
}

fn model(app: &App) -> Model {
    let w: u32 = 800;
    let h: u32 = 800;

    let _window = app.new_window().size(w, h).view(view).build().unwrap();
    let mut cells: Vec<Vec<i32>> = vec![vec![0; 100]; 100];
    initialize_random_cells(&mut cells);
    let generation = 0;

    Model { cells, generation }
}

fn rules(value: i32, neighbor_sum: i32) -> i32 {
    // define the rules of the cellular automaton
    let new_value: i32;
    if value == 1 && (neighbor_sum <= 1 || neighbor_sum >= 4) {
        new_value = 0;
    } else if value == 0 && (neighbor_sum == 3) {
        new_value = 1;
    } else {
        new_value = value;
    }
    new_value
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // update the cells based on the rules of the cellular automaton
    if model.generation == 0 {
        model.generation += 1;
        return;
    }
    let mut new_cells = model.cells.clone();
    let row_count = model.cells.len();
    let col_count = model.cells[0].len();
    for i in 0..col_count {
        for j in 0..row_count {
            let mut neighbor_sum = 0;
            for x in 0..3i32 {
                for y in 0..3i32 {
                    let cols = col_count as i32;
                    let rows = row_count as i32;
                    let board_x = ((i as i32) + (x - 1) + cols) % cols;
                    let board_y = ((j as i32) + (y - 1) + rows) % rows;
                    neighbor_sum += model.cells[board_x as usize][board_y as usize];
                }
            }
            let current_value = model.cells[i as usize][j as usize];
            neighbor_sum -= current_value;
            new_cells[i][j] = rules(current_value, neighbor_sum);
        }
    }
    model.cells = new_cells;
    model.generation += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(DARKSLATEGREY);

    // draw a square for each cell
    let col_count = model.cells.len();
    let cell_width = 800.0 / col_count as f32;
    for (row_index, row) in model.cells.iter().enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            let x: f32 = ((col_index as f32) * cell_width) - (400.0 - cell_width / 2.0);
            let y: f32 = 400.0 - (row_index as f32 * cell_width) - (cell_width / 2.0);
            let color = if *cell == 1 { ORANGERED } else { DARKSLATEGREY };
            draw.rect()
                .x_y(x, y)
                .w_h(cell_width, cell_width)
                .color(color);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
