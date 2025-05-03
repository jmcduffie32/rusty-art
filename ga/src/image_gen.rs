mod color_utils;

use color_utils::{delta_e, rgb_to_lab};
use nannou::image::{DynamicImage, GenericImageView, RgbaImage};
use nannou::prelude::*;
use rand::thread_rng;
use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::prelude::*;

const POPULATION_SIZE: usize = 100;
const MUTATION_RATE: f32 = 0.02;
const TARGET_IMAGE_NAME: &str = "output.png"; // Path to the target image

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    target_image: Vec<u8>,         // Store target image as raw pixel data
    target_dimensions: (u32, u32), // Store dimensions of the target image
    population: Vec<Vec<u8>>,      // Store population as raw pixel data
    generation: usize,
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    if key == Key::Space {
        // Save the best image of the current generation
        if let Some(best_image_data) = model.population.first() {
            let best_image = vec_to_rgba_image(
                best_image_data,
                model.target_dimensions.0,
                model.target_dimensions.1,
            );
            let path = format!("output_{}.png", model.generation);
            best_image.save(path).unwrap();
        }
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    // Uncap the frame rate
    app.set_loop_mode(LoopMode::rate_fps(1000.0));

    // Load the target image and convert it to raw pixel data
    let target_image =
        nannou::image::open(TARGET_IMAGE_NAME).expect("Failed to load target image");
    let target_dimensions = target_image.dimensions();
    let target_image_data = target_image.to_rgba8().into_raw();

    // Initialize the population with random images in parallel
    let population: Vec<Vec<u8>> = (0..POPULATION_SIZE)
        .into_par_iter()
        .map(|_| random_image(target_dimensions.0, target_dimensions.1))
        .collect();

    Model {
        target_image: target_image_data,
        target_dimensions,
        population,
        generation: 0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Evaluate fitness of each image in the population in parallel
    let mut fitness_scores: Vec<(usize, f32)> = model
        .population
        .par_iter()
        .enumerate()
        .map(|(i, candidate)| (i, fitness(&model.target_image, candidate)))
        .collect();

    // Sort by fitness (lower is better)
    fitness_scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // Reorder the population based on sorted fitness scores
    model.population = fitness_scores
        .iter()
        .map(|&(i, _)| model.population[i].clone())
        .collect();

    // Select parents based on fitness
    let binding = fitness_scores.iter().map(|&(_, f)| f).collect::<Vec<_>>();
    let parents = select_parents(&model.population, &binding);

    // Create the next generation through crossover and mutation
    model.population = create_next_generation(parents, model.target_dimensions);
    model.generation += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    if let Some(best_image_data) = model.population.first() {
        let best_image = vec_to_rgba_image(
            best_image_data,
            model.target_dimensions.0,
            model.target_dimensions.1,
        );
        let dynamic_image = DynamicImage::ImageRgba8(best_image);
        let texture = wgpu::Texture::from_image(app, &dynamic_image);
        draw.texture(&texture).w_h(
            model.target_dimensions.0 as f32,
            model.target_dimensions.1 as f32,
        );
    }

    draw.text(&format!("Generation: {}", model.generation))
        .x_y(
            -app.window_rect().w() / 2.0 + 100.0,
            app.window_rect().h() / 2.0 - 24.0,
        )
        .color(BLACK)
        .font_size(32);

    draw.to_frame(app, &frame).unwrap();
}

// Generate a random image as a vector of pixel data
fn random_image(width: u32, height: u32) -> Vec<u8> {
    let mut rng = StdRng::from_entropy();
    (0..(width * height * 4)) // Each pixel has 4 channels (RGBA)
        .map(|_| rng.gen())
        .collect()
}

// Convert a vector of pixel data back to an RgbaImage
fn vec_to_rgba_image(data: &[u8], width: u32, height: u32) -> RgbaImage {
    RgbaImage::from_raw(width, height, data.to_vec()).expect("Invalid image data")
}

// Calculate the fitness of a candidate image
fn fitness(target: &[u8], candidate: &[u8]) -> f32 {
    target
        .chunks(4) // Each pixel has 4 channels (RGBA)
        .zip(candidate.chunks(4))
        .map(|(t, c)| {
            let (t_r, t_g, t_b) = (t[0] as f32 / 255.0, t[1] as f32 / 255.0, t[2] as f32 / 255.0);
            let (c_r, c_g, c_b) = (c[0] as f32 / 255.0, c[1] as f32 / 255.0, c[2] as f32 / 255.0);

            // Convert RGB to Lab
            let t_lab = rgb_to_lab(t_r, t_g, t_b);
            let c_lab = rgb_to_lab(c_r, c_g, c_b);

            // Calculate Delta E
            delta_e(t_lab, c_lab)
        })
        .sum::<f32>()
        / (target.len() / 4) as f32 // Normalize by the number of pixels
}

// Select parents based on fitness (tournament selection)
fn select_parents<'a>(population: &'a [Vec<u8>], fitness_scores: &'a [f32]) -> Vec<&'a Vec<u8>> {
    let mut rng = thread_rng();
    (0..POPULATION_SIZE)
        .map(|_| {
            let tournament_size = 5;
            let selected = (0..tournament_size)
                .map(|_| {
                    let idx = rng.gen_range(0..population.len());
                    (idx, fitness_scores[idx])
                })
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();
            &population[selected.0]
        })
        .collect()
}

// Create the next generation through crossover and mutation
fn create_next_generation(parents: Vec<&Vec<u8>>, dimensions: (u32, u32)) -> Vec<Vec<u8>> {
    (0..POPULATION_SIZE)
        .into_par_iter()
        .map(|_| {
            let mut local_rng = StdRng::from_entropy();
            let parent1 = parents[local_rng.gen_range(0..parents.len())];
            let parent2 = parents[local_rng.gen_range(0..parents.len())];

            let mut child_data = vec![0; (dimensions.0 * dimensions.1 * 4) as usize];
            let split_point = (dimensions.0 * dimensions.1 * 2) as usize;

            child_data[..split_point].copy_from_slice(&parent1[..split_point]);
            child_data[split_point..].copy_from_slice(&parent2[split_point..]);

            // Perform mutation
            for byte in child_data.iter_mut() {
                if local_rng.gen::<f32>() < MUTATION_RATE {
                    *byte = local_rng.gen();
                }
            }

            child_data
        })
        .collect()
}
