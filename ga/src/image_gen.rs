mod color_utils;

use color_utils::{delta_e, rgb_to_lab};
use nannou::image::{DynamicImage, GenericImageView, RgbaImage};
use nannou::prelude::*;
use plotters::prelude::*;
use nannou::color::named::{WHITE, BLACK}; // Explicitly import WHITE from nannou
use rand::thread_rng;
use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::prelude::*;

const POPULATION_SIZE: usize = 1000;
const MUTATION_RATE: f32 = 0.001;
const TARGET_IMAGE_NAME: &str = "scaled_girl.jpg"; // Path to the target image

fn main() {
    nannou::app(model).update(update).run();
}

// Calculate the variance of a pixel (example implementation)
// fn pixel_variance(pixel: &[u8]) -> f32 {
//     let mean = pixel.iter().map(|&channel| channel as f32).sum::<f32>() / pixel.len() as f32;
//     pixel
//         .iter()
//         .map(|&channel| (channel as f32 - mean).powi(2))
//         .sum::<f32>()
//         / pixel.len() as f32
// }

struct Model {
    target_image: Vec<u8>,         // Store target image as raw pixel data
    target_dimensions: (u32, u32), // Store dimensions of the target image
    population: Vec<Vec<u8>>,      // Store population as raw pixel data
    generation: usize,
    fitness_history: Vec<f32>,     // Track best fitness over generations
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
        fitness_history: Vec::new(), // Initialize an empty fitness history
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

    // Track the best fitness value
    let best_fitness = fitness_scores.first().unwrap().1;
    model.fitness_history.push(best_fitness);

    // Select parents based on fitness
    let binding = fitness_scores.iter().map(|&(_, f)| f).collect::<Vec<_>>();
    let parents = select_parents(&model.population, &binding);

    // Create the next generation through crossover and mutation
    model.population = create_next_generation(parents, model.target_dimensions, &fitness_scores, &model.population);
    model.generation += 1;

    // Plot the fitness history every 10 generations
    if model.generation % 10 == 0 {
        plot_fitness_history(&model.fitness_history, model.generation);
    }
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
    
    // display the fitness of the best image
    if let Some(best_fitness) = model.population.first() {
        let fitness_value = fitness(&model.target_image, best_fitness);
        draw.text(&format!("Best Fitness: {:.2}", fitness_value))
            .x_y(
                -app.window_rect().w() / 2.0 + 100.0,
                app.window_rect().h() / 2.0 - 90.0,
            )
            .color(BLACK)
            .font_size(32);
    }

    draw.to_frame(app, &frame).unwrap();
}

// Generate a random image as a vector of pixel data
fn random_image(width: u32, height: u32) -> Vec<u8> {
    let mut rng = thread_rng(); // Use thread-local RNG for better randomness
    let block_size = 2; // Define the size of each square block (e.g., 16x16 pixels)
    let mut image_data = vec![0; (width * height * 4) as usize]; // Resize to the required size

    for y in (0..height).step_by(block_size as usize) {
        for x in (0..width).step_by(block_size as usize) {
            // Generate a random RGBA value for the block
            let r = rng.gen::<u8>();
            let g = rng.gen::<u8>();
            let b = rng.gen::<u8>();
            let a = 255; // Fully opaque

            // Fill the block with the same RGBA value
            for block_y in 0..block_size {
                for block_x in 0..block_size {
                    let pixel_x = x + block_x;
                    let pixel_y = y + block_y;

                    // Ensure we don't go out of bounds
                    if pixel_x < width && pixel_y < height {
                        let index = ((pixel_y * width + pixel_x) * 4) as usize;
                        image_data[index..index + 4].copy_from_slice(&[r, g, b, a]);
                    }
                }
            }
        }
    }

    image_data
}

// Convert a vector of pixel data back to an RgbaImage
fn vec_to_rgba_image(data: &[u8], width: u32, height: u32) -> RgbaImage {
    RgbaImage::from_raw(width, height, data.to_vec()).expect("Invalid image data")
}

// Calculate the fitness of a candidate image
fn fitness(target: &[u8], candidate: &[u8]) -> f32 {
    const MAX_DELTA_E: f32 = 100.0; // Maximum possible Delta E value for normalization

    target
        .chunks(4) // Each pixel has 4 channels (RGBA)
        .zip(candidate.chunks(4))
        .par_bridge() // Use parallel iterator for processing
        .map(|(t, c)| {
            let (t_r, t_g, t_b) = (t[0] as f32 / 255.0, t[1] as f32 / 255.0, t[2] as f32 / 255.0);
            let (c_r, c_g, c_b) = (c[0] as f32 / 255.0, c[1] as f32 / 255.0, c[2] as f32 / 255.0);
            
            // Convert RGB to Lab
            let t_lab = rgb_to_lab(t_r, t_g, t_b);
            let c_lab = rgb_to_lab(c_r, c_g, c_b);

            // Calculate normalized Delta E
            let delta = delta_e(t_lab, c_lab) / MAX_DELTA_E;

            // Amplify differences by squaring the normalized Delta E
            delta.powi(2)
        })
        .sum::<f32>()
        / (target.len() / 4) as f32 // Normalize by the number of pixels
        // + candidate.chunks(4).map(|pixel| pixel_variance(pixel)).sum::<f32>()
}

// Select parents based on fitness (tournament selection)
fn select_parents<'a>(population: &'a [Vec<u8>], fitness_scores: &'a [f32]) -> Vec<&'a Vec<u8>> {
    let tournament_size = 100;

    (0..POPULATION_SIZE)
        .into_par_iter() // Parallelize the parent selection
        .map(|_| {
            let selected = (0..tournament_size)
                .map(|_| {
                    let idx = thread_rng().gen_range(0..population.len());
                    (idx, fitness_scores[idx])
                })
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();
            &population[selected.0]
        })
        .collect()
}

// Create the next generation through crossover and mutation
fn create_next_generation(parents: Vec<&Vec<u8>>, dimensions: (u32, u32), fitness_scores: &[(usize, f32)], population: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let elite_count = (POPULATION_SIZE as f32 * 0.05) as usize; // Top 5% of the population
    let mut next_generation = Vec::with_capacity(POPULATION_SIZE);

    // Add elites (best individuals) to the next generation
    let elites: Vec<Vec<u8>> = fitness_scores
        .iter()
        .take(elite_count)
        .map(|&(idx, _)| population[idx].clone())
        .collect();
    next_generation.extend(elites);

    // Fill the rest of the population with offspring
    let offspring_count = POPULATION_SIZE - elite_count;
    let offspring: Vec<Vec<u8>> = (0..offspring_count)
        .into_par_iter()
        .map(|_| {
            let mut local_rng = StdRng::from_entropy();
            let parent1 = parents[local_rng.gen_range(0..parents.len())];
            let parent2 = parents[local_rng.gen_range(0..parents.len())];

            let mut child_data = vec![0; (dimensions.0 * dimensions.1 * 4) as usize];
            let split_point = (dimensions.0 * dimensions.1 * local_rng.gen_range(0..4)) as usize;

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
        .collect();

    next_generation.extend(offspring);
    next_generation
}

// Function to plot the fitness history
fn plot_fitness_history(fitness_history: &[f32], generation: usize) {
    let root_area = BitMapBackend::new("fitness_plot.png", (800, 600))
        .into_drawing_area();
    root_area.fill(&plotters::style::WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Fitness Over Generations", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..generation as f64, 0.0..1.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            fitness_history.iter().enumerate().map(|(i, &f)| (i as f64, f as f64)),
            &plotters::style::RED,
        ))
        .unwrap();

    chart.configure_series_labels().background_style(&plotters::style::WHITE).draw().unwrap();
}
