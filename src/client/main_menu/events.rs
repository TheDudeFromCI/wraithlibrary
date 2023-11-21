use bevy::prelude::*;

use crate::common::uuid::Uuid;

#[derive(Debug, Event)]
pub struct OpenTitleScreenEvent;

#[derive(Debug, Event)]
pub struct OpenSinglePlayerScreenEvent;

#[derive(Debug, Event)]
pub struct OpenMultiplayerScreenEvent;

#[derive(Debug, Event)]
pub struct OpenSettingsScreenEvent;

#[derive(Debug, Event)]
pub struct OpenCreditsScreenEvent;

#[derive(Debug, Event)]
pub struct OpenEditServerScreenEvent;

#[derive(Debug, Event)]
pub struct AddServerEntry {
    pub uuid: Uuid,
    pub name: String,
    pub address: String,
}
