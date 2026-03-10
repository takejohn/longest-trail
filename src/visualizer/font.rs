use std::sync::Arc;

use eframe::egui::{FontData, FontDefinitions, FontFamily};

const NOTO_SANS_JP_REGULAR: &[u8] = include_bytes!("../../assets/Noto_Sans_JP/NotoSansJP-Regular.ttf");

pub(super) fn create_font_definitions() -> FontDefinitions {
	let mut fonts = FontDefinitions::default();
	fonts.font_data.insert(
		"NotoSansJP-Regular".to_owned(),
		Arc::new(FontData::from_static(NOTO_SANS_JP_REGULAR)),
	);
	fonts.families.entry(FontFamily::Proportional).or_default().insert(0, "NotoSansJP-Regular".to_owned());
	fonts.families.entry(FontFamily::Monospace).or_default().push("NotoSansJP-Regular".to_owned());
	return fonts;
}
