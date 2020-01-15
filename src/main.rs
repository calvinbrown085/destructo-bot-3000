#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![warn(clippy::all)]

use na::Vector2;
use std::{error::Error, f32::consts::PI};

fn main() -> Result<(), Box<dyn Error>> {
    let rlbot = rlbot::init()?;

    let player_index = 0;
    rlbot.start_match(&rlbot::MatchSettings::rlbot_vs_allstar("DESTRUCTO-BOT-3000", "Allstar"))?;
    rlbot.wait_for_match_start()?;

    let mut packets = rlbot.packeteer();
    loop {
        let packet = packets.next()?;

        // check that match is started and not showing a replay.
        // `packets.next_flatbuffer()` sleeps until the next packet is
        // available, so this loop will not roast your CPU :)
        if packet.ball.is_some() {
            let input = get_input(&packet);
            rlbot.update_player_input(player_index, &input)?;
        }
    }
}

fn get_input(packet: &rlbot::GameTickPacket) -> rlbot::ControllerState {

    let car = &packet.players[0];
    let car_loc = &car.physics.location;
    let car_loc = Vector2::new(car_loc.x, car_loc.y);

    let opponent_car = &packet.players[1];
    let opponent_loc = &opponent_car.physics.location;
    let opponent_loc = Vector2::new(opponent_loc.x, opponent_loc.y);

    let offset = opponent_loc - car_loc;
    let should_boost = car.has_wheel_contact && (offset.x < 50.0);

    let should_handbrake = car.has_wheel_contact && (offset.x < 10.0);

    let desired_yaw = f32::atan2(offset.y, (offset.x - 20.0));
    let steer = desired_yaw - car.physics.rotation.yaw;

    rlbot::ControllerState {
        throttle: 1.0,
        steer: normalize_angle(steer).max(-1.0).min(1.0),
        boost: should_boost,
        handbrake: should_handbrake,
        ..Default::default()
    }
}

/// Normalize an angle to between -PI and PI.
fn normalize_angle(theta: f32) -> f32 {
    if theta < -PI {
        theta + (PI * 2.0)
    } else if theta >= PI {
        theta - (PI * 2.0)
    } else {
        theta
    }
}
