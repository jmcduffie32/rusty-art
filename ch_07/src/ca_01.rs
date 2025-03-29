use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

const RULESET: [i32; 8] = [0, 1, 0, 1, 1, 0, 1, 0];

struct Model {
    cells: Vec<i32>,
    generation: i32,
}

fn initialize_middle_cell(cells: &mut Vec<i32>) {
    let middle = cells.len() / 2;
    cells[middle] = 1;
}

fn initialize_random_cells(cells: &mut Vec<i32>) {
    for i in 0..cells.len() {
        if random_f32() < 0.5 {
            cells[i] = 1;
        } else {
            cells[i] = 0;
        }
    }
}

fn model(app: &App) -> Model {
    let w: u32 = 800;
    let h: u32 = 800;

    let _window = app.new_window().size(w, h).view(view).build().unwrap();
    let mut cells = vec![0; 800 as usize];
    initialize_middle_cell(&mut cells);
    // initialize_random_cells(&mut cells);
    let generation = 0;

    Model { cells, generation }
}

fn rules(a: i32, b: i32, c: i32) -> i32 {
    // define the rules of the cellular automaton
    if a == 1 && b == 1 && c == 1 {
        RULESET[0]
    } else if a == 1 && b == 1 && c == 0 {
        RULESET[1]
    } else if a == 1 && b == 0 && c == 1 {
        RULESET[2]
    } else if a == 1 && b == 0 && c == 0 {
        RULESET[3]
    } else if a == 0 && b == 1 && c == 1 {
        RULESET[4]
    } else if a == 0 && b == 1 && c == 0 {
        RULESET[5]
    } else if a == 0 && b == 0 && c == 1 {
        RULESET[6]
    } else if a == 0 && b == 0 && c == 0 {
        RULESET[7]
    } else {
        0
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // update the cells based on the rules of the cellular automaton
    if model.generation == 0 {
        model.generation += 1;
        return;
    }
    let mut new_cells = model.cells.clone();
    for i in 1..model.cells.len() - 1 {
        new_cells[i] = rules(model.cells[i - 1], model.cells[i], model.cells[i + 1]);
    }
    model.cells = new_cells;
    model.generation += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 {
        draw.background().color(DARKSLATEGREY);
    }

    // draw a square for each cell
    let cell_count = model.cells.len();
    let cell_width = 800.0 / cell_count as f32;
    for (i, cell) in model.cells.iter().enumerate() {
        let x: f32 = ((i as f32) * cell_width) - (400.0 - cell_width / 2.0);
        let y: f32 = 400.0 - (model.generation as f32 * cell_width) - (cell_width / 2.0);
        let color = if *cell == 1 { ORANGERED } else { DARKSLATEGREY };
        if color == DARKSLATEGREY {
            continue;
        }

        for i in 0..8 {
            draw.rotate(i as f32 * PI / 4.0)
                .rect()
                .x_y(x, y)
                .w_h(cell_width, cell_width)
                .color(ORANGERED);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
