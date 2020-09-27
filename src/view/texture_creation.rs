/// Code for creating the alpha textures used in continous rendering mode
use kiss3d::{context::Context, resource::Texture};

/// Generate a 256 x 256 blob texture with only the alpha channel encoded.
fn generate_raw_texture() -> Vec<u8> {
    // Variables used to draw the blob
    let texture_size = 256;
    let (center_x, center_y) = (texture_size / 2, texture_size / 2);
    let point_size = 2f32;
    let radius = 126f32;

    let mut texture = vec![0u8; texture_size * texture_size * 4];
    for y in 0..texture_size {
        for x in 0..texture_size {
            // Get the index of the color in the array
            let index_color = x + (y * texture_size);
            // Each color contains 4 bytes, r, g, b and a. Here we only need
            // to set the alpha channel.
            let index_byte = index_color * 4 + 3;

            // Calculate the euclidean distance to the center
            let distance_to_center = {
                let dx = (x as f32) - (center_x as f32);
                let dy = (y as f32) - (center_y as f32);
                (dx * dx + dy * dy).sqrt()
            };

            // Based on the distance calculate the alpha
            let alpha = match distance_to_center {
                // If the distance it very small we just max the alpha
                distance if distance < point_size => 255u8,
                // If the distance falls out of the radius the alpha will be 0
                distance if distance > radius => 0u8,
                // Interasting case, calculate the alpha gradient based on the distance to the center.
                distance => {
                    // The normalized distance to the edge (0..1)
                    let normalized_distance = (radius - distance) / radius;
                    (normalized_distance.powf(2.5) * 256f32) as u8
                }
            };

            // Set the correct alpha value for that point
            texture[index_byte] = alpha;
        }
    }
    texture
}

/// Generate and load a texture for the blobs onto the GPU
pub fn load_texture() -> Texture {
    let ctxt = Context::get();

    // Is this correct?
    verify!(ctxt.active_texture(Context::TEXTURE1));
    verify!(ctxt.pixel_storei(Context::UNPACK_ALIGNMENT, 1));

    // Assign a index for the texture
    let texture = verify!(ctxt
        .create_texture()
        .expect("Alpha texture creation failed."));

    // All following actions need to be preformed on the texture we just created.
    verify!(ctxt.bind_texture(Context::TEXTURE_2D, Some(&texture)));

    // Load in a image containing a static blob alpha map
    let data = generate_raw_texture();
    let (width, height) = (256 as i32, 256 as i32);
    verify!(ctxt.tex_image2d(
        Context::TEXTURE_2D,
        0,
        Context::RGBA as i32,
        width,
        height,
        0,
        Context::RGBA,
        Some(&data[..])
    ));

    // Set the correct texture parameters.
    let settings = vec![
        (Context::TEXTURE_WRAP_S, Context::CLAMP_TO_EDGE as i32),
        (Context::TEXTURE_WRAP_T, Context::CLAMP_TO_EDGE as i32),
        (Context::TEXTURE_MIN_FILTER, Context::LINEAR as i32),
        (Context::TEXTURE_MAG_FILTER, Context::LINEAR as i32),
    ];
    for (pname, param) in settings {
        verify!(ctxt.tex_parameteri(Context::TEXTURE_2D, pname, param));
    }

    // Return the created texture
    texture
}
