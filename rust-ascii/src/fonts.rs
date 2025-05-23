use image::{DynamicImage, GenericImage};
// // use std::time::{SystemTime, UNIX_EPOCH};
use msdf::{ColoredShape, SDFTrait, GlyphLoader, Projection};
use rectangle_pack::{
    GroupedRectsToPlace,
    RectToInsert,
    pack_rects,
    TargetBin,
    volume_heuristic,
    contains_smallest_box
};
use ttf_parser::Face;
use std::collections::BTreeMap;
use std::collections::HashMap;
use mint::Vector2;

use image::{GenericImageView, Pixel};

pub struct LoadFontResult {
    pub lookup_table: HashMap<(char, String), usize>,
    pub atlas_map: [[f32; 4]; 512],
    pub image: DynamicImage
}

pub fn load_font(fonts: HashMap<&str, Vec<u8>>) -> LoadFontResult {
    // Load a font from ttf data.
    let font_keys = fonts.keys();
    let mut face: Face;
    let mut msdf_list = Vec::new();
    let mut rects_to_place = GroupedRectsToPlace::new();
    let mut lookup_table = HashMap::new();

    for font_key in font_keys {
        println!("Loading font: {}", font_key);
        
        let font_name = *font_key;

        let binding: Vec<u8> = fonts.get(font_name).unwrap().to_vec();

        face = Face::from_slice(binding.as_slice(), 0).unwrap();

        // Generate glyphs
        for i in 33..127 { //33..127 {
            let c = std::char::from_u32(i).unwrap();
            lookup_table.insert((c, font_name.to_string()), msdf_list.len());
            let (msdf, width, height) = gen_glyph(&face, c);
            msdf_list.push((msdf, height, width));
        }
    }

    for i in 0..msdf_list.len() {
        let (_msdf, height, width) = msdf_list.get(i).unwrap();
        rects_to_place.push_rect(
            i,
            Some(vec![0]),
            RectToInsert::new(*width, *height, 255)
        );
    }

    let mut target_bins = BTreeMap::new();

    let atlas_size = 2048;
    target_bins.insert(0, TargetBin::new(atlas_size, atlas_size, 255));

    let rectangle_placements = pack_rects(
        &rects_to_place,
        &mut target_bins,
        &volume_heuristic,
        &contains_smallest_box
    ).unwrap();

    let mut atlas_map: [[f32; 4]; 512] = [[0.0f32, 0.0f32, 0.0f32, 0.0f32]; 512];

    let mut image = DynamicImage::new_rgb8(atlas_size, atlas_size);

    for ( i, (_id, (_gid, location))) in rectangle_placements.packed_locations().iter().enumerate() {
        let (msdf, _height, _width) = msdf_list.remove(0);
        let uvx = (location.x() as f32) / (atlas_size as f32);
        let uvy = (location.y() as f32) / (atlas_size as f32);
        atlas_map[i] = [uvx, 
                        uvy, 
                        uvx + (location.width() as f32) / (atlas_size as f32), 
                        uvy + (location.height()  as f32) / (atlas_size as f32)];
        add_char_to_bitmap(&mut image, msdf, location.x(), location.y(), location.width(), location.height());
    }

    let _ = image.save("atlas.png");

    LoadFontResult {
        lookup_table,
        atlas_map,
        image
    }
}

pub fn gen_glyph(face: &Face, c: char) -> (msdf::MSDF, u32, u32) {
    let glyph_index = face.glyph_index(c).unwrap();

    let bounding_box = face.glyph_bounding_box(glyph_index).unwrap();
    let shape: msdf::Shape = face.load_shape(glyph_index).unwrap();
    
    // Not a required step for SDF and Psuedo-SDF generation. Other coloring options exist.
    let colored_shape: ColoredShape = shape.color_edges_simple(3.0);

    let scale_factor = 1.0/16.0;

    let projection = Projection {
        scale: Vector2 { x: scale_factor, y: scale_factor},
        translation: Vector2 { x: - bounding_box.x_min as f64, y: -bounding_box.y_min as f64},
    };
    
    let msdf_config = Default::default();

    let height = (bounding_box.height() as f64 * scale_factor) as u32;
    let width = (bounding_box.width() as f64 * scale_factor) as u32;

    // 64.0 is largely a magic number. Real number involves figuring out scaling factor from distance field to output screen pixels
    // Look at README: https://github.com/Chlumsky/msdfgen
    let msdf  = colored_shape.generate_msdf(width, height, 64.0, &projection, &msdf_config);

    (msdf, width, height)
}

pub fn add_char_to_bitmap(output: &mut DynamicImage, msdf: msdf::MSDF, x: u32, y: u32, w: u32, h: u32) {
    let di: DynamicImage = DynamicImage::from(msdf.to_image());

    for iy in 0..h {
        for ix in 0..w {
            let pixel = di.get_pixel(ix, iy);
            output.put_pixel(ix + x, iy + y, pixel.to_rgba());
        }
    }
}
#[cfg(test)]

mod tests {
    // use msdfont::Font;
    // use image::GenericImageView;
    use google_fonts;
    use std::collections::HashMap;

    // use super::*;
    // #[test]
    // fn test_gen_glyph() {
    //     let binding = google_fonts::lemonada_variable().unwrap();
    //     let font = Font::from_slice(binding.as_slice());
    //     let c = 'A';
    //     let (sdf, height, width) = super::gen_glyph(&font, c);
    //     assert_eq!(sdf.len(), (height * width) as usize);
    // }

    // #[test]
    // fn test_add_char_to_bitmap() {
    //     let mut image = image::DynamicImage::new_rgb8(2048, 2048);
    //     let sdf = vec![0u8; 2048 * 2048];
    //     super::add_char_to_bitmap(&mut image, &sdf, 0, 0, 2048, 2048);      
    //     assert_eq!(image.get_pixel(0, 0), image::Rgba([0, 0, 0, 255]));
    // }

    #[test]
    fn test_look_up_table() {
        let mut fonts_to_use: HashMap<&str, Vec<u8>> = HashMap::new();
        fonts_to_use.insert("default", google_fonts::lemonada_variable().unwrap());
        let res = super::load_font(fonts_to_use);
        assert_eq!(res.lookup_table.get(&('!', "default".to_string())), Some(&0));
    }
    
    // #[test]
    // fn test_atlas_map_empty() {
    //     let mut fonts_to_use: HashMap<&str, Vec<u8>> = HashMap::new();
    //     fonts_to_use.insert("default", google_fonts::lemonada_variable().unwrap());
    //     fonts_to_use.insert("libre_bodoni", google_fonts::libre_bodoni_variable().unwrap());
    //     let res = super::load_font(fonts_to_use);
    //     assert_ne!(res.atlas_map[0], [0.0f32, 0.0f32, 0.0f32, 0.0f32]);
    // }

    // #[test]
    // fn test_atlas_map_width_and_height() {
    //     let mut fonts_to_use: HashMap<&str, Vec<u8>> = HashMap::new();
    //     fonts_to_use.insert("default", google_fonts::lemonada_variable().unwrap());
    //     fonts_to_use.insert("libre_bodoni", google_fonts::libre_bodoni_variable().unwrap());
    //     let res = super::load_font(fonts_to_use);
    //     // println!("lookup.len() {}", res.lookup_table.len());
    //     for i in 0..res.lookup_table.len() {
    //         // println!("x {} y {}", res.atlas_map[i][0], res.atlas_map[i][1]);
    //         // println!("width {} height {}", res.atlas_map[i][2], res.atlas_map[i][3]);
    //         if res.atlas_map[i][2] == 0.0f32 {
    //             assert!(false);
    //         }
    //         if res.atlas_map[i][3] == 0.0f32 {
    //             assert!(false);
    //         }
    //     }
    //     assert!(true);
    // }

    // #[test]
    // fn test_image_save() {
    //     let mut fonts_to_use: HashMap<&str, Vec<u8>> = HashMap::new();
    //     fonts_to_use.insert("default", google_fonts::lemonada_variable().unwrap());
    //     fonts_to_use.insert("libre_bodoni", google_fonts::libre_bodoni_variable().unwrap());
    //     let res = super::load_font(fonts_to_use);
    //     let _ = res.image.save("atlas.png");
    //     assert!(std::path::Path::new("atlas.png").exists());
    // }
}