//! Image serializator for Grid. Each single step can be saved to png file.
//! 
use crate::grid::{Cell, Grid};
use raqote::*;

pub struct ImageExportConfig {
    grid_box_size: f32,
    grid_line_width: f32,
    bg_color: [u8; 4],
    grid_color: [u8; 4],
    box_color: [u8; 4],
}

impl Default for ImageExportConfig {
    fn default() -> Self {
        ImageExportConfig {
            grid_box_size: 10.0,
            grid_line_width: 1.0,
            bg_color: [0xff, 0xff, 0xff, 0xff],
            grid_color: [0x0, 0x0, 0x80, 0x80],
            box_color: [0x0, 0x0, 0xff, 0xff],
        }
    }
}

fn draw_grid(
    dt: &mut DrawTarget,
    src: &Source,
    grid_count: u32,
    grid_box_size: f32,
    grid_line_width: f32,
) {
    let d: f32 = grid_count as f32 * (grid_box_size + grid_line_width);
    let mut pb = PathBuilder::new();
    for n in 0..(grid_count + 1) {
        let n_box = (n as f32) * grid_box_size;
        pb.move_to(0., n_box);
        pb.line_to(d, n_box);
        pb.move_to(n_box, 0.);
        pb.line_to(n_box, d);
    }
    pb.close();
    let path = pb.finish();

    dt.stroke(&path, &src, &StrokeStyle::default(), &DrawOptions::new());
}

fn draw_rect(dt: &mut DrawTarget, src: &Source, r: [f32; 4]) {
    let mut pb = PathBuilder::new();
    pb.rect(r[0], r[1], r[2], r[3]);
    dt.fill(&pb.finish(), &src, &DrawOptions::new());
}

fn source_from(item: [u8; 4]) -> SolidSource {
    SolidSource {
        r: item[0],
        g: item[1],
        b: item[2],
        a: item[3],
    }
}

impl Grid {
    pub fn save(&self, file_name: &str, cfg: &ImageExportConfig) -> Result<(), std::io::Error> {
        let grid_size = self.size;
        let image_size = grid_size as f32 * cfg.grid_box_size + cfg.grid_line_width;

        let mut dt = DrawTarget::new(image_size as i32, image_size as i32);

        //background
        draw_rect(
            &mut dt,
            &Source::Solid(source_from(cfg.bg_color)),
            [0., 0., image_size, image_size],
        );

        //grid
        draw_grid(
            &mut dt,
            &Source::Solid(source_from(cfg.grid_color)),
            grid_size as u32,
            cfg.grid_box_size,
            cfg.grid_line_width,
        );

        for ndx in 0..grid_size * grid_size {
            let (x, y) = (ndx % grid_size, ndx / grid_size);

            let xx = (x as f32) * (cfg.grid_box_size as f32);
            let yy = (y as f32) * (cfg.grid_box_size as f32);

            if let Cell::Alive = self.grid[ndx] {
                draw_rect(
                    &mut dt,
                    &Source::Solid(source_from(cfg.box_color)),
                    [xx, yy, cfg.grid_box_size, cfg.grid_box_size],
                );
            }
        }

        dt.write_png(file_name)?;
        Ok(())
    }
}
