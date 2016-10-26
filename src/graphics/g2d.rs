trait Batch {
    fn begin(&self);
    fn end(&self);
    fn draw(&self);//TODO: all params in one struct with default values
}

struct Sprite {

}

struct TextureRegion {

}

struct Texture {

}

struct Pixel {
    x: i64,
    y: i64,
    color: Color,
}

struct Color {

}

///BitMap, PixelMap
struct Canvas {

}
