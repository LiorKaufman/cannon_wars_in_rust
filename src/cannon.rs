use bevy::prelude::*;

pub struct CannonPlugin;
impl Plugin for CannonPlugin {
	fn build(&self, app: &mut App) {
		prinprintln!("como estas");
		app.add_startup_system(cannon_spawn_system)
		// .add_system(player_fire_system);
		// }
	}

	// #[derive(Component)]
	// pub struct TurretRotation {
	// 	pub rotation_speed_x: f32,
	// 	pub rotation_speed_y: f32,
	// }

	// #[derive(Component)]
	// pub struct Velocity {
	// 	pub x: f32,
	// 	pub y: f32,
	// }

	// pub fn cannon_spawn_system(mut commands: &mut Commands) {
	// 	commands.spawn(SpriteBundle {
	// 		sprite: Sprite {
	// 			color: Color::rgb(0.25, 0.25, 0.75),
	// 			custom_size: Some(Vec2::new(55.0, 100.0)),
	// 			..default()
	// 		},
	// 		..default()
	// 	});
}
