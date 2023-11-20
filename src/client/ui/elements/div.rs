use bevy::prelude::*;

use super::{BoxedElement, ElementDirection, ElementJustify, NodeBackground, WhElement};
use crate::client::assets::AssetLoader;

#[derive(Default)]
pub struct WhDiv<Flags>
where
    Flags: Bundle,
{
    pub flags: Flags,
    pub background: NodeBackground,
    pub width: Val,
    pub height: Val,
    pub direction: ElementDirection,
    pub justify: ElementJustify,
    pub gap: Val,
    pub padding: UiRect,
    pub margin: UiRect,
    pub flex_grow: f32,
    pub children: Vec<BoxedElement>,
}

impl<Flags> WhElement for WhDiv<Flags>
where
    Flags: Bundle,
{
    fn build_child(
        self: Box<Self>,
        commands: &mut Commands,
        loader: &mut AssetLoader,
        parent: Option<Entity>,
    ) {
        let background = self.background.into_image_bundle(loader);

        let flex_direction = match self.direction {
            ElementDirection::Row => FlexDirection::Row,
            ElementDirection::Column => FlexDirection::Column,
        };

        let justify_content = match self.justify {
            ElementJustify::Start => JustifyContent::FlexStart,
            ElementJustify::Center => JustifyContent::Center,
            ElementJustify::End => JustifyContent::FlexEnd,
        };

        let mut cmd = commands.spawn((
            self.flags,
            ImageBundle {
                style: Style {
                    flex_direction,
                    justify_content,
                    align_items: AlignItems::Center,
                    flex_grow: self.flex_grow,
                    row_gap: self.gap,
                    column_gap: self.gap,
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

        let id = cmd.id();
        for child in self.children.into_iter() {
            child.build_child(commands, loader, Some(id));
        }
    }
}

impl<Flags> WhDiv<Flags>
where
    Flags: Bundle,
{
    pub fn set_flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
        self
    }

    pub fn add_children(mut self, mut children: Vec<BoxedElement>) -> Self {
        self.children.append(&mut children);
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

    pub fn direction(mut self, direction: ElementDirection, gap: Val) -> Self {
        self.direction = direction;
        self.gap = gap;
        self
    }

    pub fn justify(mut self, justify: ElementJustify) -> Self {
        self.justify = justify;
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

    pub fn growing(mut self) -> Self {
        self.flex_grow = 1.0;
        self
    }
}
