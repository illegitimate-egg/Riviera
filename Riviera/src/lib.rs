use quick_noise::{Fbm, Grid, Perlin};

use std::slice;

#[no_mangle]
pub extern "C" fn BufferTest(_buffer: *mut f32, _len: usize) -> usize {
    /*
     *  use this only for buffer test
     */
    let buffer: *mut f32 = _buffer;
    let length: usize = _len;

    if buffer.is_null() {
        return 0;
    }

    let out = unsafe { slice::from_raw_parts_mut(buffer, length) };

    for (i, v) in out.iter_mut().enumerate() {
        *v = i as f32;
    }

    length
}

#[no_mangle]
pub extern "C" fn Get2DNoiseHeightMap(
    _buffer: *mut f32,
    _size_x: usize,
    _size_y: usize,
    _seed: u32,
    _scale: f64,
    _origin_x: i32,
    _origin_y: i32,
    _octaves: usize,
    _persistence: f64,
    _lacunarity: f64,
) -> usize {
    /*
     *  returns heght map with the defined values for noise
     *  use origing for seamless generation
     */
    let (buffer, length, scale) = (_buffer, (_size_x * _size_y), _scale);

    if buffer.is_null() {
        return 0;
    }

    let grid = Grid::<2>::new(_size_x, _size_y)
        .grid_position(
            _origin_x as f64 / _size_x as f64,
            _origin_y as f64 / _size_y as f64,
        )
        .seed(_seed.into());

    let out = unsafe { slice::from_raw_parts_mut(buffer, length) };

    grid.builder::<Fbm, Perlin>()
        .octaves(_octaves)
        .lacunarity(_lacunarity as f32)
        .persistence(_persistence as f32)
        .scaling(scale as f32, scale as f32)
        .initialize(true)
        .finalize(true)
        .fill(out);

    length
}
