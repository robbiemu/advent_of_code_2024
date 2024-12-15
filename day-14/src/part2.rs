use bevy::input::keyboard::{Key, KeyCode, KeyboardInput};
use bevy::input::ButtonState;
use bevy::prelude::*;

use crate::object_zoo::prelude::*;
use crate::object_zoo::ProblemDefinition;


fn compute_normalized_variances(
  sum_x: f64,
  sum_y: f64,
  sum_x_squared: f64,
  sum_y_squared: f64,
  num_robots: usize,
  width: f64,
  height: f64,
) -> (f64, f64) {
  if num_robots == 0 {
    return (0.0, 0.0);
  }

  let n = num_robots as f64;

  // Variance in x
  let mean_x = sum_x / n;
  let variance_x = (sum_x_squared / n) - (mean_x * mean_x);

  // Variance in y
  let mean_y = sum_y / n;
  let variance_y = (sum_y_squared / n) - (mean_y * mean_y);

  // Maximum possible variances
  let max_variance_x = width * width / 4.0;
  let max_variance_y = height * height / 4.0;

  // Normalized variances
  let normalized_variance_x = variance_x / max_variance_x;
  let normalized_variance_y = variance_y / max_variance_y;

  (normalized_variance_x, normalized_variance_y)
}

fn get_window(dims: &Point<usize>) -> (f32, f32) {
  // Original dimensions in pixels
  let original_width = dims.x as f32 * 10.0;
  let original_height = dims.y as f32 * 10.0 + 100.0; // Additional space for UI

  // Maximum allowed height
  let max_height = 600.0;

  // Calculate the scaling factor
  let scale = if original_height > max_height {
    max_height / original_height
  } else {
    1.0
  };

  // Apply the scaling factor
  (original_width * scale, original_height * scale)
}

pub fn transform_part2_bevy(data: ProblemDefinition, dims: Point<usize>) {
  let (window_width, window_height) = get_window(&dims);

  App::new()
    .insert_resource(SimulationConfig {
      data,
      width: dims.x,
      height: dims.y,
      current_time: 0,
      step: 0,
      paused: false,
      speed: 20,
      clustering_threshold: 0.3,
    })
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "Simulation".to_string(),
        resolution: (window_width, window_height).into(),
        ..default()
      }),
      ..default()
    }))
    .add_event::<UpdateSimulationEvent>()
    .add_systems(Startup, (setup_camera, setup_text))
    .add_systems(
      Update,
      (update_simulation, handle_input, update_time_display),
    )
    .run();
}

#[derive(Resource)]
struct SimulationConfig {
  data: ProblemDefinition,
  width: usize,
  height: usize,
  current_time: usize,
  step: usize,
  paused: bool,
  speed: usize, // 1x speed by default
  clustering_threshold: f64,
}

#[derive(Component)]
struct RobotMarker;

#[derive(Component)]
struct TimeDisplay;

#[derive(Event)]
struct UpdateSimulationEvent;

fn setup_camera(mut commands: Commands) {
  commands.spawn(Camera2d);
}

fn setup_text(mut commands: Commands) {
  commands.spawn((
    Text2d::new("Time Step: 0"),
    TextFont { font_size: 50.0, ..default() },
    TextLayout::default(),
    TextColor(Color::WHITE),
    TimeDisplay,
  ));
}

fn update_simulation(
  mut commands: Commands,
  query: Query<Entity, With<RobotMarker>>,
  mut config: ResMut<SimulationConfig>,
  event_reader: EventReader<UpdateSimulationEvent>,
) {
  if config.paused && event_reader.is_empty() {
    return; // Pause the simulation unless triggered by an event
  }

  // Update the simulation based on the speed
  if !config.paused {
    config.current_time += 1;
    if config.current_time % config.speed != 0 {
      return;
    }
    config.step += 1;
  } else {
    config.current_time = config.step * config.speed;
  }

  // Clear existing robots
  for entity in query.iter() {
    commands.entity(entity).despawn();
  }

  let width = config.width as isize;
  let height = config.height as isize;

  let mut sum_x = 0_f64;
  let mut sum_y = 0_f64;
  let mut sum_x_2 = 0_f64;
  let mut sum_y_2 = 0_f64;
  for robot in config.data.robots.iter() {
    let x = (robot.position.x as isize
      + config.step as isize * robot.velocity.x as isize)
      .rem_euclid(width) as f32;
    let y = (robot.position.y as isize
      + config.step as isize * robot.velocity.y as isize)
      .rem_euclid(height) as f32;

    let adjusted_x = x - width as f32 / 2.0;
    let adjusted_y = height as f32 / 2.0 - y;

    sum_x += adjusted_x as f64;
    sum_y += adjusted_y as f64;
    sum_x_2 += adjusted_x.powf(2.) as f64;
    sum_y_2 += adjusted_y.powf(2.) as f64;

    commands.spawn((
      Sprite {
        custom_size: Some(Vec2::new(10.0, 10.0)),
        color: Color::srgb(0.0, 1.0, 0.0),
        ..Default::default()
      },
      Transform::from_translation(Vec3::new(
        adjusted_x * 10.0,
        adjusted_y * 10.0,
        0.0,
      )),
      RobotMarker,
    ));
  }
  let (variance_x, variance_y) = compute_normalized_variances(
    sum_x,
    sum_y,
    sum_x_2,
    sum_y_2,
    config.data.robots.len(),
    width as f64,
    height as f64,
  );

  // println!("Variance X: {}", variance_x);
  // println!("Variance Y: {}", variance_y);
  if variance_x < config.clustering_threshold
    && variance_y < config.clustering_threshold
  {
    config.paused = true;
    println!("Clustering detected. Simulation paused.");
  }
}

fn handle_input(
  mut config: ResMut<SimulationConfig>,
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut update_simulation_event: EventWriter<UpdateSimulationEvent>,
  mut keyboard_input_events: EventReader<KeyboardInput>,
) {
  // Handle character inputs for '+' and '-'
  for event in keyboard_input_events.read() {
    let key = &event.logical_key;
    match key {
      Key::Character(c) if c == "+" => {
        if event.state == ButtonState::Pressed {
          config.clustering_threshold *= 1.1;
          println!(
            "Increased min_magnitude to {}",
            config.clustering_threshold
          );
        }
      }
      Key::Character(c) if c == "-" => {
        if event.state == ButtonState::Pressed {
          config.clustering_threshold /= 1.1;
          println!(
            "Decreased min_magnitude to {}",
            config.clustering_threshold
          );
        }
      }
      _ => {}
    }
  }

  if keyboard_input.just_pressed(KeyCode::Space) {
    config.paused = !config.paused;
  }

  if keyboard_input.just_pressed(KeyCode::Escape)
    || keyboard_input.just_pressed(KeyCode::KeyQ)
  {
    std::process::exit(0);
  }

  if keyboard_input.just_pressed(KeyCode::ArrowUp) && config.speed > 1 {
    config.speed = (config.speed - 1).max(1);
    println!("Simulation speed increased to {}x", config.speed);
  }

  if keyboard_input.just_pressed(KeyCode::ArrowDown) {
    config.speed += 1;
    println!("Simulation speed decreased to {}x", config.speed);
  }

  if keyboard_input.just_pressed(KeyCode::ArrowRight) {
    config.paused = true;
    config.step += 1;
    update_simulation_event.send(UpdateSimulationEvent);
  }

  if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
    config.paused = true;
    if config.step > 0 {
      config.step -= 1;
      update_simulation_event.send(UpdateSimulationEvent);
    }
  }
}

fn update_time_display(
  config: Res<SimulationConfig>,
  mut query: Query<&mut Text2d, With<TimeDisplay>>,
) {
  if config.current_time % config.speed == 0 {
    for mut text in query.iter_mut() {
      text.0 = format!("Time Step: {}", config.step);
    }
  }
}
