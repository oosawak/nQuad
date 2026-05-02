#[cfg(test)]
mod tests {
    use crate::resource::serialize::{load_package, save_package};
    use crate::resource::{ColorMode, ResourcePackage, SpriteData};

    #[test]
    fn test_sprite_creation() {
        let palette = vec![[0, 0, 0, 255], [255, 0, 0, 255]];
        let sprite = SpriteData::new(8, 8, ColorMode::Indexed256(palette));
        assert_eq!(sprite.width, 8);
        assert_eq!(sprite.height, 8);
        assert_eq!(sprite.pixels.len(), 64);
    }

    #[test]
    fn test_set_and_get_pixel_indexed() {
        let palette = vec![[0, 0, 0, 255], [255, 0, 0, 255]];
        let mut sprite = SpriteData::new(4, 4, ColorMode::Indexed256(palette));

        // Set a pixel
        sprite.set_pixel(0, 0, &[1]).expect("Failed to set pixel");

        // Get the pixel back
        let pixel = sprite.get_pixel(0, 0).expect("Failed to get pixel");
        assert_eq!(pixel, vec![1]);
    }

    #[test]
    fn test_set_and_get_pixel_fullcolor() {
        let mut sprite = SpriteData::new(4, 4, ColorMode::FullColor);

        // Set a pixel in full color
        let color = [255u8, 0, 0, 255];
        sprite.set_pixel(0, 0, &color).expect("Failed to set pixel");

        // Get the pixel back
        let pixel = sprite.get_pixel(0, 0).expect("Failed to get pixel");
        assert_eq!(pixel, color.to_vec());
    }

    #[test]
    fn test_pixel_bounds_checking() {
        let palette = vec![[0, 0, 0, 255]];
        let mut sprite = SpriteData::new(4, 4, ColorMode::Indexed256(palette));

        // Out of bounds should fail
        let result = sprite.set_pixel(5, 5, &[0]);
        assert!(result.is_err());
    }

    #[test]
    fn test_resource_package() {
        let palette = vec![[0, 0, 0, 255], [255, 0, 0, 255]];
        let sprite = SpriteData::new(8, 8, ColorMode::Indexed256(palette));

        let mut package = ResourcePackage::new();
        package.add_sprite(sprite);

        assert_eq!(package.sprites.len(), 1);
        assert_eq!(package.get_sprite(0).unwrap().width, 8);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let palette = vec![[0, 0, 0, 255], [255, 0, 0, 255]];
        let sprite = SpriteData::new(8, 8, ColorMode::Indexed256(palette.clone()));
        let mut package = ResourcePackage::new();
        package.add_sprite(sprite);

        // Serialize
        let filepath = std::path::Path::new("/tmp/test_nantar.bin");
        save_package(&package, filepath).expect("Failed to save");

        // Deserialize
        let loaded = load_package(filepath).expect("Failed to load");
        assert_eq!(loaded.sprites.len(), 1);
        assert_eq!(loaded.sprites[0].width, 8);
        assert_eq!(loaded.sprites[0].height, 8);

        // Clean up
        std::fs::remove_file(filepath).ok();
    }
}
