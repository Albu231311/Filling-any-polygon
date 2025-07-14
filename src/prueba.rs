use raylib::prelude::*;

fn main() {
    let image_width = 800;
    let image_height = 600;

    // Crear imagen con fondo blanco
    let mut img = Image::gen_image_color(image_width, image_height, Color::RAYWHITE);

    // Polígonos originales (sin flip)
    let polygon1_orig = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];

    let polygon2_orig = vec![
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];

    let polygon3_orig = vec![
        (377, 249), (411, 197), (436, 249),
    ];

    let polygon4_orig = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180),
    ];

    let polygon5_orig = vec![
        (682, 175), (708, 120), (735, 148), (739, 170),
    ];

    
    let fh = image_height as i32;
    let polygon1 = flip_polygon(&polygon1_orig, fh);
    let polygon2 = flip_polygon(&polygon2_orig, fh);
    let polygon3 = flip_polygon(&polygon3_orig, fh);
    let polygon4 = flip_polygon(&polygon4_orig, fh);
    let polygon5 = flip_polygon(&polygon5_orig, fh);


    fill_polygon_image(&mut img, &polygon1, Color::LIGHTGRAY);
    draw_polygon_image(&mut img, &polygon1, Color::BLACK);

    fill_polygon_image(&mut img, &polygon2, Color::LIGHTGRAY);
    draw_polygon_image(&mut img, &polygon2, Color::BLACK);

    fill_polygon_image(&mut img, &polygon3, Color::LIGHTGRAY);
    draw_polygon_image(&mut img, &polygon3, Color::BLACK);

    fill_polygon_image(&mut img, &polygon4, Color::LIGHTGRAY);
    fill_polygon_image(&mut img, &polygon5, Color::RAYWHITE); 
    draw_polygon_image(&mut img, &polygon4, Color::BLACK);
    draw_polygon_image(&mut img, &polygon5, Color::BLACK);

    
    let output_file_name = "out.png";
    img.export_image(output_file_name);
    println!("Imagen guardada como '{}'", output_file_name);
}

fn flip_polygon(points: &[(i32, i32)], image_height: i32) -> Vec<(i32, i32)> {
    points.iter()
        .map(|&(x, y)| (x, image_height - y))
        .collect()
}

fn draw_polygon_image(img: &mut Image, points: &[(i32, i32)], color: Color) {
    for i in 0..points.len() {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % points.len()];
        draw_line_image(img, x0, y0, x1, y1, color);
    }
}

fn draw_line_image(img: &mut Image, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: Color) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        plot_image(img, x0, y0, color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn plot_image(img: &mut Image, x: i32, y: i32, color: Color) {
    let width = img.width as i32;
    let height = img.height as i32;
    if x >= 0 && y >= 0 && x < width && y < height {
        img.draw_pixel(x, y, color);
    }
}

fn fill_polygon_image(img: &mut Image, points: &[(i32, i32)], color: Color) {
    if points.is_empty() {
        return;
    }

    let min_y = points.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = points.iter().map(|&(_, y)| y).max().unwrap();

    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        for i in 0..points.len() {
            let (x0, y0) = points[i];
            let (x1, y1) = points[(i + 1) % points.len()];

            if y0 == y1 {
                continue;
            }

            if (y >= y0.min(y1)) && (y < y0.max(y1)) {
                let x = x0 + (y - y0) * (x1 - x0) / (y1 - y0);
                intersections.push(x);
            }
        }

        intersections.sort_unstable();

        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x_start = intersections[i];
                let x_end = intersections[i + 1];
                for x in x_start..=x_end {
                    plot_image(img, x, y, color);
                }
            }
        }
    }
}
