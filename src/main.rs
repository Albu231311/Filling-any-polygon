use raylib::prelude::*;

struct FrameBuffer {
    image: Image,
    width: i32,
    height: i32,
    current_color: Color,
}

impl FrameBuffer {
    fn new(width: i32, height: i32, background_color: Color) -> Self {
        let image = Image::gen_image_color(width, height, background_color);
        Self {
            image,
            width,
            height,
            current_color: background_color,
        }
    }


    fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    fn draw_pixel(&mut self, x: i32, y: i32) {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            self.image.draw_pixel(x, y, self.current_color);
        }
    }

    fn render_to_file(&self, filename: &str) {
        self.image.export_image(filename);
    }
}


fn draw_polygon_border(fb: &mut FrameBuffer, points: &[(i32, i32)]) {
    for i in 0..points.len() {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % points.len()];
        draw_line(fb, x0, y0, x1, y1);
    }
}

// Rellena el polígono usando Scanline fill
fn fill_polygon(fb: &mut FrameBuffer, vertices: &[(i32, i32)]) {
    let min_y = vertices.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = vertices.iter().map(|&(_, y)| y).max().unwrap();

    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        for i in 0..vertices.len() {
            let (x0, y0) = vertices[i];
            let (x1, y1) = vertices[(i + 1) % vertices.len()];

            if y0 == y1 { continue; }

            if (y >= y0.min(y1)) && (y < y0.max(y1)) {
                let x = x0 + (y - y0) * (x1 - x0) / (y1 - y0);
                intersections.push(x);
            }
        }

        intersections.sort_unstable();

        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                for x in intersections[i]..=intersections[i + 1] {
                    fb.draw_pixel(x, y);
                }
            }
        }
    }
}

// Algoritmo de Bresenham
fn draw_line(fb: &mut FrameBuffer, mut x0: i32, mut y0: i32, x1: i32, y1: i32) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        fb.draw_pixel(x0, y0);
        if x0 == x1 && y0 == y1 { break; }
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

fn flip_vertical(points: &[(i32, i32)], height: i32) -> Vec<(i32, i32)> {
    points.iter().map(|&(x, y)| (x, height - y)).collect()
}

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = FrameBuffer::new(width, height, Color::RAYWHITE);

    // Polígono 1
    let polygon1 = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];

    // Polígono 2
    let polygon2 = vec![
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];

    // Polígono 3
    let polygon3 = vec![
        (377, 249), (411, 197), (436, 249),
    ];

    // Polígono 4
    let polygon4 = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180),
    ];

    // Polígono 5 (agujero en polígono 4)
    let hole = vec![
        (682, 175), (708, 120), (735, 148), (739, 170),
    ];

    
    let p1 = flip_vertical(&polygon1, height);
    let p2 = flip_vertical(&polygon2, height);
    let p3 = flip_vertical(&polygon3, height);
    let p4 = flip_vertical(&polygon4, height);
    let h1 = flip_vertical(&hole, height);

    // Rellenar polígonos con colores diferentes
    framebuffer.set_current_color(Color::LIGHTGRAY);
    fill_polygon(&mut framebuffer, &p1);
    framebuffer.set_current_color(Color::BLACK);
    draw_polygon_border(&mut framebuffer, &p1);

    framebuffer.set_current_color(Color::LIGHTGRAY);
    fill_polygon(&mut framebuffer, &p2);
    framebuffer.set_current_color(Color::BLACK);
    draw_polygon_border(&mut framebuffer, &p2);

    framebuffer.set_current_color(Color::LIGHTGRAY);
    fill_polygon(&mut framebuffer, &p3);
    framebuffer.set_current_color(Color::BLACK);
    draw_polygon_border(&mut framebuffer, &p3);

    framebuffer.set_current_color(Color::ORANGE);
    fill_polygon(&mut framebuffer, &p4);

    // Rellenar el agujero con el color de fondo
    framebuffer.set_current_color(Color::RAYWHITE);
    fill_polygon(&mut framebuffer, &h1);

    framebuffer.set_current_color(Color::BLACK);
    draw_polygon_border(&mut framebuffer, &p4);
    draw_polygon_border(&mut framebuffer, &h1);

    // Exportar imagen final
    framebuffer.render_to_file("out.png");
    println!("Imagen guardada como 'out.png'");
}
