use bevy::prelude::*;

use super::{NodeBackground, WhElement};
use crate::client::assets::AssetLoader;

#[derive(Default)]
pub struct WhButton<Flags = ()>
where
    Flags: Bundle,
{
    pub flags: Flags,
    pub background: NodeBackground,
    pub width: Val,
    pub height: Val,
    pub padding: UiRect,
    pub margin: UiRect,
}

impl<Flags> WhElement for WhButton<Flags>
where
    Flags: Bundle,
{
    fn build_child(
        self: Box<Self>,
        commands: &mut Commands,
        loader: &mut AssetLoader,
        parent: Option<Entity>,
    ) {
        let background = self.background.into_button_bundle(loader);

        let mut cmd = commands.spawn((
            self.flags,
            ButtonBundle {
                style: Style {
                    width: self.width,
                    height: self.height,
                    padding: self.padding,
                    margin: self.margin,
                    ..default()
                },
                ..background
            },
        ));

        if let Some(parent) = parent {
            cmd.set_parent(parent);
        }
    }
}

impl<Flags> WhButton<Flags>
where
    Flags: Bundle,
{
    pub fn set_flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
        self
    }

    pub fn background(mut self, background: NodeBackground) -> Self {
        self.background = background;
        self
    }

    pub fn size(mut self, width: Val, height: Val) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = margin;
        self
    }
}