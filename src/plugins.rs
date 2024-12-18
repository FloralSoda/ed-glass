use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct GlassPlugins;
impl PluginGroup for GlassPlugins {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(crate::map::GlassMapPlugin {})
	}
}