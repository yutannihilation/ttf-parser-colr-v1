use std::{env, path::Path};

use ttf_parser::GlyphId;

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
}
