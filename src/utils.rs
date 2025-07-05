use egui::{ColorImage, Context, TextureHandle};
#[cfg(target_arch = "wasm32")]
use image::GenericImageView;

// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen::prelude::*;


pub fn load_atlas(ctx: &Context) -> TextureHandle {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Native: Load from filesystem

        use image::GenericImageView;
        let img = image::open("assets/PieceAtlas.png").expect("png not found");
        let size = img.dimensions();
        let rgba = img.to_rgba8();
        let raw = rgba.as_flat_samples();

        let egui_img = ColorImage::from_rgba_unmultiplied(
            [size.0 as usize, size.1 as usize],
            raw.as_slice(),
        );
        ctx.load_texture("piece_atlas", egui_img, egui::TextureOptions::default())
    }

    #[cfg(target_arch = "wasm32")]
    {
        let bytes = include_bytes!("../assets/PieceAtlas.png"); // embed at compile time
        let img = image::load_from_memory(bytes).expect("failed to load atlas image");
        let size = img.dimensions();
        let rgba = img.to_rgba8();
        let raw = rgba.as_flat_samples();

        let egui_img = ColorImage::from_rgba_unmultiplied(
            [size.0 as usize, size.1 as usize],
            raw.as_slice(),
        );
        ctx.load_texture("piece_atlas", egui_img, egui::TextureOptions::default())
    }
}