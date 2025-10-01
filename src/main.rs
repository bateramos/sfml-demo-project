use sfml::{graphics::*, system::*, window::*};
use egui_sfml::SfEgui;

fn main() {
    let mut window = RenderWindow::new(
        (420, 680),
        "sfml-demo-project",
        Style::CLOSE,
        &Default::default(),
    ).unwrap();

    let mut sfegui = SfEgui::new(&window);

    window.set_vertical_sync_enabled(true);
    window.set_position(Vector2i { x: 200, y: 200 });

    let new_view = View::from_rect(FloatRect::new(0.,0., 420., 680.)).unwrap();

    let image_to_render = Some(Box::new(load_with_egui(sfegui.context(), "city.png")));

    loop {
        window.set_view(&new_view);
        window.clear(Color::BLACK);

        while let Some(event) = window.poll_event() {
            sfegui.add_event(&event);
            match event {
                Event::Closed | Event::KeyPressed {
                    code: Key::Escape | Key::Q, ..
                } => {
                    std::process::exit(0);
                },
                _ => {}
            };
        }

        let draw_input = sfegui.run(&mut window, |_, ctx| {
            let box_image = image_to_render.clone();
            egui_sfml::egui::Area::new("example_ui_area".into())
                .fixed_pos(egui_sfml::egui::Pos2::new(100., 100.))
                .show(ctx, |ui| {
                    if ui.button("Click here to load Texture with egui and image").clicked() {
                    }
                    if let Some(image) = box_image {
                        ui.image(&image.unwrap());
                    }
                });
        }).unwrap();

        sfegui.draw(draw_input, &mut window, None);

        window.display();
    }

    fn load_with_egui(ctx: &egui_sfml::egui::Context, name: &str) -> Option<egui_sfml::egui::TextureHandle> {
        let img = image::ImageReader::open(name)
            .expect("failed to open image file")
            .decode()
            .expect("failed to decode image")
            .into_rgb8();

        let color_image = egui_sfml::egui::ColorImage::from_rgb(
            [img.width() as usize, img.height() as usize],
            img.as_raw(),
        );

        return Some(ctx.load_texture("texture", color_image, Default::default()));
    }
}
