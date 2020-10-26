use three_d::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug).unwrap();
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    main();
    Ok(())
}

fn main() {
    let mut window = Window::new_default("Wireframe").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let scene_center = vec3(0.0, 2.0, 0.0);
    let scene_radius = 6.0;
    let mut renderer = DeferredPipeline::new(&gl).unwrap();
    let mut camera = Camera::new_perspective(
        &gl,
        scene_center + scene_radius * vec3(0.6, 0.3, 1.0).normalize(),
        scene_center,
        vec3(0.0, 1.0, 0.0),
        degrees(90.0),
        width as f32 / height as f32,
        0.1,
        1000.0,
    );

    // Objects
    let cpu_mesh = CPUMesh::new(
        &[
            7, 6, 4, 6, 6, 4, 5, 5, 5, 4, 6, 4, 4, 6, 7, 7, 1, 1, 1, 3, 0, 3, 3, 0, 2, 2, 2, 0, 3,
            0, 0, 3, 1, 48, 60, 44, 54, 54, 34, 34, 20, 12, 89, 12, 28, 76, 76, 64, 64, 65, 62, 63,
            50, 63, 53, 65, 59, 64, 56, 62, 50, 50, 32, 32, 11, 72, 72, 77, 77, 13, 24, 73, 73, 88,
            88, 27, 40, 81, 81, 17, 17, 15, 33, 31, 31, 68, 68, 9, 32, 70, 70, 83, 83, 25, 35, 78,
            78, 75, 75, 10, 22, 69, 69, 67, 67, 30, 21, 14, 21, 37, 79, 79, 61, 61, 49, 57, 47, 47,
            86, 86, 18, 85, 85, 45, 45, 45, 55, 43, 52, 52, 41, 41, 87, 82, 82, 71, 71, 71, 23, 8,
            74, 74, 80, 80, 39, 26, 84, 84, 19, 19, 36, 82, 82, 66, 66, 66, 29, 32, 32, 16, 16, 16,
            38, 18, 18, 42, 42, 42, 51, 46, 58,
        ],
        &[
            0.25332, 0.213937, -0.645689, 0.25332, 0.381067, -0.645689, 0.25332, 0.213937,
            -0.478558, 0.25332, 0.381067, -0.478558, -0.241395, 0.213937, -0.645689, -0.241395,
            0.381067, -0.645689, -0.241395, 0.381067, -0.478558, -0.241395, 0.213937, -0.478558,
            -0.222846, -0.351697, -0.62525, -0.222846, -0.351697, -0.62525, -0.222846, -0.351697,
            -0.62525, 0.275799, -0.312242, -0.62525, 0.275799, -0.312242, -0.62525, 0.275799,
            -0.312242, -0.62525, -0.265384, 0.226391, -0.518359, -0.265384, 0.226391, -0.518359,
            -0.265384, 0.226391, -0.518359, 0.279381, 0.226391, -0.518359, 0.279381, 0.226391,
            -0.518359, 0.279381, 0.226391, -0.518359, 0.279381, 0.226391, -0.518359, -0.294713,
            -0.351631, 0.545673, -0.294713, -0.351631, 0.545673, 0.269754, -0.391086, 0.545673,
            0.269754, -0.391086, 0.545673, -0.255757, 0.380325, 0.625486, -0.255757, 0.380325,
            0.625486, 0.30871, 0.340871, 0.545673, 0.30871, 0.340871, 0.545673, -0.261802, 0.1912,
            -0.62525, -0.261802, 0.1912, -0.62525, -0.261802, 0.1912, -0.62525, 0.275799, 0.1912,
            -0.62525, 0.275799, 0.1912, -0.62525, 0.275799, 0.1912, -0.62525, -0.265787, 0.305944,
            -0.483596, -0.265787, 0.305944, -0.483596, -0.265787, 0.305944, -0.483596, -0.265787,
            0.305944, -0.483596, 0.24103, 0.345948, -0.466115, 0.24103, 0.345948, -0.466115,
            0.24103, 0.345948, -0.466115, -0.307119, -0.400465, 0.531942, -0.307119, -0.400465,
            0.531942, 0.321299, -0.400465, 0.531942, 0.321299, -0.400465, 0.531942, -0.307119,
            0.392207, 0.56653, -0.307119, 0.392207, 0.56653, 0.321299, 0.392207, 0.554657,
            0.321299, 0.392207, 0.554657, -0.294497, -0.387854, 0.610732, -0.294497, -0.387854,
            0.610732, -0.294497, -0.387854, 0.610732, 0.308677, -0.387854, 0.610732, 0.308677,
            -0.387854, 0.610732, 0.308677, -0.387854, 0.610732, -0.294497, 0.379596, 0.633447,
            -0.294497, 0.379596, 0.633447, -0.294497, 0.379596, 0.633447, 0.308677, 0.379596,
            0.633447, 0.308677, 0.379596, 0.633447, 0.308677, 0.379596, 0.633447, -0.037899,
            -0.131463, 0.65927, 0.052079, -0.131463, 0.65927, -0.037899, 0.123205, 0.65927,
            0.052079, 0.123205, 0.65927, -0.261802, -0.312242, -0.62525, -0.261802, -0.312242,
            -0.62525, -0.261802, -0.312242, -0.62525, -0.261802, -0.312242, -0.62525, 0.236843,
            -0.351697, -0.62525, 0.236843, -0.351697, -0.62525, 0.236843, -0.351697, -0.62525,
            0.236843, -0.351697, -0.62525, -0.255757, -0.391086, 0.545673, -0.255757, -0.391086,
            0.545673, 0.30871, -0.351631, 0.545673, 0.30871, -0.351631, 0.545673, -0.294713,
            0.340871, 0.625486, -0.294713, 0.340871, 0.625486, 0.269754, 0.380325, 0.625486,
            0.269754, 0.380325, 0.625486, -0.227033, 0.345948, -0.466115, -0.227033, 0.345948,
            -0.466115, -0.227033, 0.345948, -0.466115, -0.227033, 0.345948, -0.466115, 0.279784,
            0.305944, -0.483596, 0.279784, 0.305944, -0.483596, 0.279784, 0.305944, -0.483596,
            0.279784, 0.305944, -0.483596,
        ],
        &[],
    )
    .unwrap();

    let mut edges = Edges::new(&gl, &cpu_mesh.indices, &cpu_mesh.positions, 0.002);
    edges.diffuse_intensity = 0.8;
    edges.specular_intensity = 0.2;
    edges.specular_power = 5.0;
    edges.color = vec3(0.7, 0.2, 0.2);

    let mut model = cpu_mesh.to_mesh(&gl).unwrap();
    model.diffuse_intensity = 0.2;
    model.specular_intensity = 0.4;
    model.specular_power = 20.0;

    let mut plane_mesh = tri_mesh::MeshBuilder::new().plane().build().unwrap();
    plane_mesh.scale(100.0);
    let mut plane = Mesh::new(
        &gl,
        &plane_mesh.indices_buffer(),
        &plane_mesh.positions_buffer_f32(),
        &plane_mesh.normals_buffer_f32(),
    )
    .unwrap();
    plane.diffuse_intensity = 0.2;
    plane.specular_intensity = 0.4;
    plane.specular_power = 20.0;

    let ambient_light: AmbientLight = AmbientLight::new(&gl, 0.5, &vec3(1.0, 1.0, 1.0)).unwrap();

    // main loop
    let mut rotating = false;
    window
        .render_loop(move |frame_input| {
            camera.set_size(
                frame_input.screen_width as f32,
                frame_input.screen_height as f32,
            );

            for event in frame_input.events.iter() {
                match event {
                    Event::MouseClick { state, button, .. } => {
                        rotating = *button == MouseButton::Left && *state == State::Pressed;
                    }
                    Event::MouseMotion { delta } => {
                        if rotating {
                            camera.rotate(delta.0 as f32, delta.1 as f32);
                        }
                    }
                    Event::MouseWheel { delta } => {
                        camera.zoom(*delta as f32);
                    }
                    _ => {}
                }
            }

            let spotlight = SpotLight::new(
                &gl,
                1.0,
                &vec3(1.0, 1.0, 1.0),
                &vec3(
                    camera.position().x,
                    camera.position().y,
                    camera.position().z + 10.0,
                ),
                &vec3(
                    camera.target().x, // TODO: make directional light point out of camera
                    camera.target().y, //
                    camera.target().z, //
                ),
                25.0,
                0.1,
                0.001,
                0.0001,
            )
            .unwrap();

            // Geometry pass
            renderer
                .geometry_pass(width, height, &|| {
                    let transformation = Mat4::from_translation(vec3(0.0, 2.0, 0.0));
                    state::cull(&gl, state::CullType::Back);
                    model.render(&transformation, &camera);
                    edges.render(&transformation, &camera);
                    // vertices.render(&transformation, &camera);
                    plane.render(&Mat4::identity(), &camera);
                })
                .unwrap();

            // Light pass
            Screen::write(
                &gl,
                0,
                0,
                width,
                height,
                Some(&vec4(0.1, 0.1, 0.1, 1.0)),
                None,
                &|| {
                    renderer
                        .light_pass(
                            &camera,
                            Some(&ambient_light), // gi
                            &[],
                            &[&spotlight], // from camera
                            &[],
                        )
                        .unwrap();
                },
            )
            .unwrap();
        })
        .unwrap();
}
