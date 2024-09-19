use std::{env, path::Path};

use ttf_parser::{colr::Painter, RgbaColor};

const TARGET_CHAR: char = '🌶';

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    eprintln!("File: {}", path.to_string_lossy());
    let font_raw = std::fs::read(path).expect("failed to read font file");
    eprintln!(
        "fonts_in_collection: {:?}",
        ttf_parser::fonts_in_collection(&font_raw)
    );
    let face = ttf_parser::Face::parse(&font_raw, 0).expect("failed to parse font file");
    eprintln!("# of glyphs in face: {:?}", face.number_of_glyphs());

    let urange = face.unicode_ranges();
    eprintln!(
        "{TARGET_CHAR} is in range: {}",
        urange.contains_char(TARGET_CHAR)
    );

    let tables = face.tables();
    eprintln!("glyf: {}", tables.glyf.is_some());
    eprintln!("cff: {}", tables.cff.is_some());
    eprintln!("colr: {}", tables.colr.is_some());
    eprintln!("cbdt: {}", tables.cbdt.is_some());

    let glyph_id = face.glyph_index(TARGET_CHAR).expect("No glyph");
    eprintln!("glyph id: {:?}", glyph_id);
    eprintln!("glyph name: {:?}", face.glyph_name(glyph_id));
    eprintln!("is color glyph: {:?}", face.is_color_glyph(glyph_id));

    let fg_color = RgbaColor::new(0, 0, 0, 255);
    let mut painter = DebugPainter {};
    face.paint_color_glyph(glyph_id, 1, fg_color, &mut painter);
}

struct DebugPainter {}

impl<'a> Painter<'a> for DebugPainter {
    fn outline_glyph(&mut self, glyph_id: ttf_parser::GlyphId) {
        eprintln!("[outline_glyph] {glyph_id:?}");
    }

    fn paint(&mut self, paint: ttf_parser::colr::Paint<'a>) {
        eprintln!("[paint] {paint:?}");
    }

    fn push_clip(&mut self) {
        eprintln!("[push_clip]");
    }

    fn push_clip_box(&mut self, clipbox: ttf_parser::colr::ClipBox) {
        eprintln!("[push_push_clip: {clipbox:?}]");
    }

    fn pop_clip(&mut self) {
        eprintln!("[pop_clip]");
    }

    fn push_layer(&mut self, mode: ttf_parser::colr::CompositeMode) {
        eprintln!("[push_layer] {mode:?}");
    }

    fn pop_layer(&mut self) {
        eprintln!("[pop_layer]");
    }

    fn push_transform(&mut self, transform: ttf_parser::Transform) {
        eprintln!("[push_transform] {transform:?}");
    }

    fn pop_transform(&mut self) {
        eprintln!("[pop_transform]");
    }
}
