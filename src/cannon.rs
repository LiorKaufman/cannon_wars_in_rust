use crate::components::{Cannon, FromPlayer, Laser, Movable, Player, SpriteSize, Velocity};
use crate::{
	GameTextures, Angle, WinSize, PLAYER_LASER_SIZE, PLAYER_RESPAWN_DELAY, PLAYER_SIZE,
	SPRITE_SCALE,
};
use bevy::prelude::*;
use bevy::time::FixedTimestep;

pub struct CannonPlugin;

impl Plugin for CannonPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(Angle::default()).add_system_set(
			SystemSet::new()
				.with_run_criteria(FixedTimestep::step(0.5))
				.with_system(player_spawn_system),
		);
		// .add_system(player_keyboard_event_system)
		// .add_system(player_fire_system);
	}
}

fn player_spawn_system(
	mut commands: Commands,
	mut cannon_angle: ResMut<Angle>,
	time: Res<Time>,
	game_textures: Res<GameTextures>,
	win_size: Res<WinSize>,
) {
	let now = time.elapsed_seconds_f64();
    let angle = cannon_angle.degrees as f32;

	// add player
	let bottom = -win_size.h / 2.;
	let left = -win_size.w / 2.;
	commands
		.spawn(SpriteBundle {
			texture: game_textures.player.clone(),
			transform: Transform {
				translation: Vec3::new(
					left + PLAYER_SIZE.1 / 2. * SPRITE_SCALE,
					bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5.,
					10.,
				),
				scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                rotation: Quat::from_rotation_z(angle),
				..Default::default()
			},
			..Default::default()
		})
		.insert(Cannon{rotation_speed: 80. })
		.insert(SpriteSize::from(PLAYER_SIZE))
		.insert(Movable { auto_despawn: false })
		.insert(Velocity { x: 0., y: 0. });

	// player_state.spawned();
}
