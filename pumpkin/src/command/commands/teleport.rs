use async_trait::async_trait;
use pumpkin_util::math::vector3::Vector3;
use pumpkin_util::text::TextComponent;

use crate::command::args::entities::EntitiesArgumentConsumer;
use crate::command::args::entity::EntityArgumentConsumer;
use crate::command::args::position_3d::Position3DArgumentConsumer;
use crate::command::args::rotation::RotationArgumentConsumer;
use crate::command::args::ConsumedArgs;
use crate::command::args::FindArg;
use crate::command::tree::builder::{argument, literal};
use crate::command::tree::CommandTree;
use crate::command::CommandError;
use crate::command::{CommandExecutor, CommandSender};

const NAMES: [&str; 2] = ["teleport", "tp"];
const DESCRIPTION: &str = "Teleports entities, including players."; // todo

/// position
const ARG_LOCATION: &str = "location";

/// single entity
const ARG_DESTINATION: &str = "destination";

/// multiple entities
const ARG_TARGETS: &str = "targets";

/// rotation: yaw/pitch
const ARG_ROTATION: &str = "rotation";

/// single entity
const ARG_FACING_ENTITY: &str = "facingEntity";

/// position
const ARG_FACING_LOCATION: &str = "facingLocation";

fn yaw_pitch_facing_position(
    looking_from: &Vector3<f64>,
    looking_towards: &Vector3<f64>,
) -> (f32, f32) {
    let direction_vector = (looking_towards.sub(looking_from)).normalize();

    let yaw_radians = -direction_vector.x.atan2(direction_vector.z);
    let pitch_radians = (-direction_vector.y).asin();

    let yaw_degrees = yaw_radians.to_degrees();
    let pitch_degrees = pitch_radians.to_degrees();

    (yaw_degrees as f32, pitch_degrees as f32)
}

struct TpEntitiesToEntityExecutor;

#[async_trait]
impl CommandExecutor for TpEntitiesToEntityExecutor {
    async fn execute<'a>(
        &self,
        _sender: &mut CommandSender<'a>,
        _server: &crate::server::Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let targets = EntitiesArgumentConsumer::find_arg(args, ARG_TARGETS)?;

        let destination = EntityArgumentConsumer::find_arg(args, ARG_DESTINATION)?;
        let pos = destination.living_entity.entity.pos.load();

        for target in targets {
            let yaw = target.living_entity.entity.yaw.load();
            let pitch = target.living_entity.entity.pitch.load();
            target.living_entity.entity.teleport(pos, yaw, pitch).await;
        }

        Ok(())
    }
}

struct TpEntitiesToPosFacingPosExecutor;

#[async_trait]
impl CommandExecutor for TpEntitiesToPosFacingPosExecutor {
    async fn execute<'a>(
        &self,
        _sender: &mut CommandSender<'a>,
        _server: &crate::server::Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let targets = EntitiesArgumentConsumer::find_arg(args, ARG_TARGETS)?;

        let pos = Position3DArgumentConsumer::find_arg(args, ARG_LOCATION)?;

        let facing_pos = Position3DArgumentConsumer::find_arg(args, ARG_FACING_LOCATION)?;
        let (yaw, pitch) = yaw_pitch_facing_position(&pos, &facing_pos);

        for target in targets {
            target.living_entity.entity.teleport(pos, yaw, pitch).await;
        }

        Ok(())
    }
}

struct TpEntitiesToPosFacingEntityExecutor;

#[async_trait]
impl CommandExecutor for TpEntitiesToPosFacingEntityExecutor {
    async fn execute<'a>(
        &self,
        _sender: &mut CommandSender<'a>,
        _server: &crate::server::Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let targets = EntitiesArgumentConsumer::find_arg(args, ARG_TARGETS)?;

        let pos = Position3DArgumentConsumer::find_arg(args, ARG_LOCATION)?;

        let facing_entity = &EntityArgumentConsumer::find_arg(args, ARG_FACING_ENTITY)?
            .living_entity
            .entity;
        let (yaw, pitch) = yaw_pitch_facing_position(&pos, &facing_entity.pos.load());

        for target in targets {
            target.living_entity.entity.teleport(pos, yaw, pitch).await;
        }

        Ok(())
    }
}

struct TpEntitiesToPosWithRotationExecutor;

#[async_trait]
impl CommandExecutor for TpEntitiesToPosWithRotationExecutor {
    async fn execute<'a>(
        &self,
        _sender: &mut CommandSender<'a>,
        _server: &crate::server::Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let targets = EntitiesArgumentConsumer::find_arg(args, ARG_TARGETS)?;

        let pos = Position3DArgumentConsumer::find_arg(args, ARG_LOCATION)?;

        let (yaw, pitch) = RotationArgumentConsumer::find_arg(args, ARG_ROTATION)?;

        for target in targets {
            target.living_entity.entity.teleport(pos, yaw, pitch).await;
        }

        Ok(())
    }
}

struct TpEntitiesToPosExecutor;

#[async_trait]
impl CommandExecutor for TpEntitiesToPosExecutor {
    async fn execute<'a>(
        &self,
        _sender: &mut CommandSender<'a>,
        _server: &crate::server::Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let targets = EntitiesArgumentConsumer::find_arg(args, ARG_TARGETS)?;

        let pos = Position3DArgumentConsumer::find_arg(args, ARG_LOCATION)?;

        for target in targets {
            let yaw = target.living_entity.entity.yaw.load();
            let pitch = target.living_entity.entity.pitch.load();
            target.living_entity.entity.teleport(pos, yaw, pitch).await;
        }

        Ok(())
    }
}

struct TpSelfToEntityExecutor;

#[async_trait]
impl CommandExecutor for TpSelfToEntityExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender<'a>,
        _server: &crate::server::Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let destination = EntityArgumentConsumer::find_arg(args, ARG_DESTINATION)?;
        let pos = destination.living_entity.entity.pos.load();

        match sender {
            CommandSender::Player(player) => {
                let yaw = player.living_entity.entity.yaw.load();
                let pitch = player.living_entity.entity.pitch.load();
                player.living_entity.entity.teleport(pos, yaw, pitch).await;
            }
            _ => {
                sender
                    .send_message(TextComponent::translate("permissions.requires.player", []))
                    .await;
            }
        };

        Ok(())
    }
}

struct TpSelfToPosExecutor;

#[async_trait]
impl CommandExecutor for TpSelfToPosExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender<'a>,
        _server: &crate::server::Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        match sender {
            CommandSender::Player(player) => {
                let pos = Position3DArgumentConsumer::find_arg(args, ARG_LOCATION)?;
                let yaw = player.living_entity.entity.yaw.load();
                let pitch = player.living_entity.entity.pitch.load();
                player.living_entity.entity.teleport(pos, yaw, pitch).await;
            }
            _ => {
                sender
                    .send_message(TextComponent::translate("permissions.requires.player", []))
                    .await;
            }
        };

        Ok(())
    }
}

pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(argument(ARG_LOCATION, Position3DArgumentConsumer).execute(TpSelfToPosExecutor))
        .then(argument(ARG_DESTINATION, EntityArgumentConsumer).execute(TpSelfToEntityExecutor))
        .then(
            argument(ARG_TARGETS, EntitiesArgumentConsumer)
                .then(
                    argument(ARG_LOCATION, Position3DArgumentConsumer)
                        .execute(TpEntitiesToPosExecutor)
                        .then(
                            argument(ARG_ROTATION, RotationArgumentConsumer)
                                .execute(TpEntitiesToPosWithRotationExecutor),
                        )
                        .then(
                            literal("facing")
                                .then(
                                    literal("entity").then(
                                        argument(ARG_FACING_ENTITY, EntityArgumentConsumer)
                                            .execute(TpEntitiesToPosFacingEntityExecutor),
                                    ),
                                )
                                .then(
                                    argument(ARG_FACING_LOCATION, Position3DArgumentConsumer)
                                        .execute(TpEntitiesToPosFacingPosExecutor),
                                ),
                        ),
                )
                .then(
                    argument(ARG_DESTINATION, EntityArgumentConsumer)
                        .execute(TpEntitiesToEntityExecutor),
                ),
        )
}
