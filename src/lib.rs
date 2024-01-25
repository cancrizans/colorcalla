
// Entry point for wasm
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug).unwrap();

    use log::info;
    info!("Logging works!");

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    main();
    Ok(())
}


mod dini;

use three_d::*;
use std::f32::consts::PI;


struct DiniMaterial {
    dispx:f32,
    rot:f32,
    disps:f32,
    luma:f32,

    display_surf : bool
}

fn l_tilde(luma:f32)->f32{
    (luma +16.)/116.
}

impl Material for DiniMaterial {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        let mut src = include_str!("hypercolor.inc.glsl").to_string().to_owned();
        src.push_str(&include_str!("colordini.frag").to_string().to_owned());
        src
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            uv: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(&self, _program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {

        _program.use_uniform("dispx", self.dispx);
        _program.use_uniform("dispscale", self.disps.exp());
        _program.use_uniform("Ltilde", l_tilde(self.luma));
        _program.use_uniform("rot", self.rot);
        _program.use_uniform("displaysurf", if self.display_surf {1} else {0});
    }
    fn render_states(&self) -> RenderStates {
        RenderStates {
            depth_test: DepthTest::Less,
            write_mask: WriteMask::COLOR_AND_DEPTH,
            cull: Cull::None,
            ..Default::default()
        }
    }
    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }

    fn id(&self) -> u16 {
        0b111u16
    }
}

struct DiskMaterial {
    t : f32,
    dispx:f32,
    rot:f32,
    disps:f32,
    luma:f32
}

impl Material for DiskMaterial {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        let mut src = include_str!("hypercolor.inc.glsl").to_string().to_owned();
        src.push_str(&include_str!("disk.frag").to_string().to_owned());
        src
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            uv: true,
            ..FragmentAttributes::NONE
        }
    }
    fn use_uniforms(&self, _program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {
        _program.use_uniform("tant", self.t.tan());
        _program.use_uniform("dispx", self.dispx);
        _program.use_uniform("dispscale", self.disps.exp());
        _program.use_uniform("Ltilde", l_tilde(self.luma));
        _program.use_uniform("rot", self.rot);
    }
    fn render_states(&self) -> RenderStates {
        RenderStates {
            depth_test: DepthTest::Less,
            write_mask: WriteMask::COLOR_AND_DEPTH,
            cull: Cull::None,
            ..Default::default()
        }
    }
    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }

    fn id(&self) -> u16 {
        0b101u16
    }
}

pub fn main() {
    let mut t = 0.20f32;

    let mut dispx = 0.30f32;
    let mut rot = 0f32;
    let mut disps = 0.3f32;
    let mut luma = 61.0;
    let mut bg_color =  [0.5,0.5,0.5];
    

    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "ColorCalla".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    // Get the graphics context from the window
    let context = window.gl();

    // Create a camera
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 10.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        1.0,
        50.0,
    );

    let mut control = OrbitControl::new(
        *camera.target(), 2.0, 20.0);


    let cpu_mesh = dini::make_dini(t);

    let dinimat = DiniMaterial{dispx,rot,disps,luma,display_surf:false};
    let mut model = Gm::new(
        Mesh::new(&context, &cpu_mesh),
        dinimat
        );

    let diskmat = DiskMaterial {t,dispx,rot,disps,luma};
    let mut quad  = Gm::new(
        Mesh::new(
            &context,
            &CpuMesh::square()
        ),
        diskmat
    );

    quad.set_transformation(Mat4::from_translation(vec3(3.0, 0.0, 0.0)));

    
    model.set_animation(|time| Mat4::from_angle_y(radians(time * 0.005)));


    let mut gui = three_d::GUI::new(&context);

    window.render_loop(
        move |mut frame_input| // Begin a new frame with an updated frame input
    {
        let mut parameter_change = frame_input.first_frame;
        let mut t_change = frame_input.first_frame;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    use three_d::egui::*;
                    ui.heading("Color Calla - Parameters");
                    t_change |= ui.add(Slider::new(&mut t, 0.01f32..=(PI/2.0-0.01)).text("Blossom")).changed();
                    parameter_change |= ui.add(Checkbox::new(&mut model.material.display_surf, "Display Entire Surface")).changed();
                    parameter_change |= ui.add(Slider::new(&mut dispx, -0.8..=1.0).text("Shift")).changed();
                    parameter_change |= ui.add(Slider::new(&mut rot, 0.0..=PI*2.0).text("Rotate")).changed();
                    parameter_change |= ui.add(Slider::new(&mut disps, -5f32..=5.0).text("Slide")).changed();
                    parameter_change |= ui.add(Slider::new(&mut luma, 0.0..=100.0).text("Luma")).changed();
                    parameter_change |= ui.color_edit_button_rgb(&mut bg_color).changed();
                });
                
            },
        );
        if parameter_change {
            model.material.dispx = dispx;
            model.material.disps = disps;
            model.material.luma = luma;
            model.material.rot = rot;
            quad.material.dispx = dispx;
            quad.material.disps = disps;
            quad.material.luma = luma;
            quad.material.rot = rot;
        }
        if t_change {
            model.geometry = Mesh::new(&context, &dini::make_dini(t));
            quad.material.t = t;
            
        }

        // Ensure the viewport matches the current window viewport which changes if the window is resized
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        // Update the animation of the triangle
        //model.animate(frame_input.accumulated_time as f32);

        // Get the screen render target to be able to render something on the screen
        frame_input.screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(bg_color[0],bg_color[1],bg_color[2],1.0, 1.0))
            // Render the triangle with the color material which uses the per vertex colors defined at construction
            .render(
                &camera, model.into_iter().chain(&quad), &[]
            ).write(|| {
                gui.render();
            });

        // Returns default frame output to end the frame
        FrameOutput::default()
    },
    );
}