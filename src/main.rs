use bevy::{math::Vec3Swizzles, prelude::*, time::FixedTimestep};

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);
const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component)]
pub struct GroundVelocity {
	x_pos: f32,
	y_pos: f32,
}
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Player {
	/// linear speed in meters per second
	movement_speed: f32,
	/// rotation speed in radians per second
	rotation_speed: f32,
}

#[derive(Component)]
enum Direction {
	Up,
	Down,
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system(sprite_movement)
		// .add_plugin(CannonPlugin)
		.run();
}
fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>, // mut mesmes: ResMut<Assets<Mesh>>,
	                                // mut materials: ResMut<Assets<ColorMaterial>>,
) {
	commands.spawn(Camera2dBundle::default());
	commands.spawn((
		SpriteBundle {
			texture: asset_server.load("cannon.png"),
			transform: Transform::from_xyz(0.0 - 300., 0.0, 0.0),
			..default()
		},
		Direction::Up,
		Player {
			movement_speed: 500.,
			rotation_speed: 300.,
		},
		GroundVelocity { x_pos: 0., y_pos: 0. },
	));
}
// fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
// 	for (mut transform, velocity) in &mut query {
// 		transform.translation.x += velocity.x * TIME_STEP;
// 		transform.translation.y += velocity.y * TIME_STEP;
// 	}
// }
fn sprite_movement(
	keyboard_input: Res<Input<KeyCode>>,
	time: Res<Time>,
	mut sprite_position: Query<(
		&mut Direction,
		&mut Player,
		&mut GroundVelocity,
		&mut Transform,
	)>,
) {
	for (mut logo, mut player, mut velocity, mut transform) in &mut sprite_position {
		let mut rotation_factor = 0.0;
		let mut movement_factor = 0.0;
		let mut x_move = velocity.x_pos;
		let mut y_move = velocity.y_pos;
		match *logo {
			Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
			Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
		}

		if transform.translation.y > 200. {
			*logo = Direction::Down;
		} else if transform.translation.y < -200. {
			*logo = Direction::Up;
		}

		if keyboard_input.pressed(KeyCode::Left) {
			// rotation_factor += 1.0;
			x_move -= 3.
		}

		if keyboard_input.pressed(KeyCode::Right) {
			// rotation_factor -= 1.0;
			x_move += 3.;
		}
		// update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
		transform.rotate_z(rotation_factor * player.rotation_speed * TIME_STEP);

		// get the ship's forward vector by applying the current rotation to the ships initial facing vector
		let movement_direction = transform.rotation * Vec3::Y;
		// get the distance the ship will move based on direction, the ship's movement speed and delta time
		let movement_distance = movement_factor * player.movement_speed * TIME_STEP;
		// create the change in translation using the new movement direction and distance
		let translation_delta = movement_direction * movement_distance;
		// update the ship translation with our new translation delta
		transform.translation += translation_delta;

		// bound the ship within the invisible level bounds
		let extents = Vec3::from((BOUNDS / 2.0, 0.0));
		transform.translation = transform.translation.min(extents).max(-extents);
		transform.translation.x += x_move;
		// transform.translation.y += y_move * time.delta_seconds();
	}
}
