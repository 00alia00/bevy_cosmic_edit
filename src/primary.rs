use std::path::PathBuf;

use bevy::ecs::world::DeferredWorld;

use crate::prelude::*;

/// Plugin struct that adds systems and initializes resources related to cosmic edit functionality.
#[derive(Default)]
pub struct CosmicEditPlugin {
    pub font_config: CosmicFontConfig,
}

impl Plugin for CosmicEditPlugin {
    fn build(&self, app: &mut App) {
        trace!("Loading cosmic edit plugin");
        let font_system = create_cosmic_font_system(self.font_config.clone());

        app.add_plugins((
            crate::cosmic_edit::plugin,
            crate::editor_buffer::plugin,
            crate::render::plugin,
            crate::input::plugin,
            crate::focus::plugin,
            crate::placeholder::plugin,
            crate::password::plugin,
            crate::user_select::plugin,
            crate::double_click::plugin,
        ))
        // TODO: Use the builtin bevy CosmicFontSystem
        .insert_resource(crate::cosmic_edit::CosmicFontSystem(font_system));

        app.register_type::<CosmicRenderOutput>();

        #[cfg(feature = "internal-debugging")]
        app.add_plugins(crate::debug::plugin);
    }
}

/// Resource struct that holds configuration options for cosmic fonts.
#[derive(Resource, Clone)]
pub struct CosmicFontConfig {
    pub fonts_dir_path: Option<PathBuf>,
    pub font_bytes: Option<Vec<&'static [u8]>>,
    /// If [false], some characters (esspecially Unicode emoji) might not load properly
    /// Caution: this can be relatively slow
    pub load_system_fonts: bool,
}

impl Default for CosmicFontConfig {
    fn default() -> Self {
        let fallback_font = include_bytes!("./font/FiraMono-Regular-subset.ttf");
        Self {
            load_system_fonts: true,
            font_bytes: Some(vec![fallback_font]),
            fonts_dir_path: None,
        }
    }
}

/// Used to ferry data from a [`CosmicEditBuffer`]
#[derive(Component, Default, Reflect, Debug, Deref)]
#[component(on_add = new_image_from_default)]
pub(crate) struct CosmicRenderOutput(pub(crate) Handle<Image>);

/// Without this, multiple buffers will show the same image
/// as the focused editor.
fn new_image_from_default(
    mut world: DeferredWorld,
    bevy::ecs::lifecycle::HookContext { entity, .. }: bevy::ecs::lifecycle::HookContext,
) {
    let mut images = world.resource_mut::<Assets<Image>>();
    let default_image = images.add(Image::default());
    *world
        .entity_mut(entity)
        .get_mut::<CosmicRenderOutput>()
        .unwrap() = CosmicRenderOutput(default_image);
}

fn create_cosmic_font_system(cosmic_font_config: CosmicFontConfig) -> cosmic_text::FontSystem {
    let locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));
    let mut db = cosmic_text::fontdb::Database::new();
    if let Some(dir_path) = cosmic_font_config.fonts_dir_path.clone() {
        db.load_fonts_dir(dir_path);
    }
    if let Some(custom_font_data) = &cosmic_font_config.font_bytes {
        for elem in custom_font_data {
            db.load_font_data(elem.to_vec());
        }
    }
    if cosmic_font_config.load_system_fonts {
        db.load_system_fonts();
    }
    cosmic_text::FontSystem::new_with_locale_and_db(locale, db)
}

#[cfg(test)]
mod tests {
 
    use super::*;

    fn test_spawn_cosmic_edit_system(
        mut commands: Commands,
        mut font_system: ResMut<CosmicFontSystem>,
    ) {
        let attrs = cosmic_text::Attrs::new();
        commands.spawn(
            CosmicEditBuffer::new(&mut font_system, cosmic_text::Metrics::new(20., 20.))
                .with_rich_text(&mut font_system, vec![("Blah", attrs.clone())], attrs),
        );
    }

}
