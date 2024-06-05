use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct UiFonts {
    pub normal: Handle<Font>,
    pub bold: Handle<Font>,
    pub italic: Handle<Font>,
    pub bold_italic: Handle<Font>,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct TextFlags: u8 {
        const NORMAL        = 0b00;
        const ITALIC        = 0b01;
        const BOLD          = 0b10;
        const BOLD_ITALIC   = 0b11;
    }
}
