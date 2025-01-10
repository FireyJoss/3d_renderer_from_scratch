mod mesh;
mod camera;

use camera::Camera;
use mesh::Mesh3D;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};
use nalgebra::{Vector2, Vector3, Vector4};
use sdl2::keyboard::Scancode;



fn calc_det(p: Vector2<f32>, p1: Vector2<f32>, p2: Vector2<f32>) -> f32
{
    let v1: Vector2<f32> = p2 - p1;
    let v2: Vector2<f32> = p - p1;
    v1.x * v2.y - v1.y * v2.x
}

fn inside_triangle(p: Vector2<f32>, p0: Vector2<f32>, p1: Vector2<f32>, p2: Vector2<f32>) -> bool
{
    let sum = calc_det(p, p0, p1).signum() + calc_det(p, p1, p2).signum() + calc_det(p, p2, p0).signum();
    let sumi = sum as i32;
    if sumi.abs() == 3 {return true;}
    false
}


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("3d renderer", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;

    //create Mesh
    let positions = vec![
        // Front face (z = -2.5)
        Vector4::new(-0.5, -0.5, -2.5, 1.0), // Vertex 0
        Vector4::new(0.5, -0.5, -2.5, 1.0),  // Vertex 1
        Vector4::new(0.5, 0.5, -2.5, 1.0),   // Vertex 2
        Vector4::new(-0.5, 0.5, -2.5, 1.0),  // Vertex 3

        // Back face (z = -3.5)
        Vector4::new(-0.5, -0.5, -3.5, 1.0), // Vertex 4
        Vector4::new(0.5, -0.5, -3.5, 1.0),  // Vertex 5
        Vector4::new(0.5, 0.5, -3.5, 1.0),   // Vertex 6
        Vector4::new(-0.5, 0.5, -3.5, 1.0),  // Vertex 7*/
    ];

    //define triangles
    let triangles: Vec<Vector3<usize>> = vec![
        // Front face
        Vector3::new(0, 1, 2),
        Vector3::new(0, 2, 3),

        // Back face
        Vector3::new(4, 5, 6),
        Vector3::new(4, 6, 7),

        // Left face
        Vector3::new(0, 3, 7),
        Vector3::new(0, 7, 4),

        // Right face
        Vector3::new(1, 5, 6),
        Vector3::new(1, 6, 2),

        // Top face
        Vector3::new(3, 2, 6),
        Vector3::new(3, 6, 7),

        // Bottom face
        Vector3::new(0, 1, 5),
        Vector3::new(0, 5, 4),
    ];


    // Creating a mesh
    let mesh3d = Mesh3D::new(positions, triangles);
    
    let (w , h) = canvas.output_size().unwrap();

    let fov: f32 = std::f32::consts::PI / 4.0; // 45 degrees in radians
    let f = 20.0; // Far plane
    let n = 1.0;  // Near plane
    let world_position: Vector4<f32> = Vector4::new(0.0,0.0,0.0,1.0);

    let mut camera = Camera::new(world_position, 0.0, w as f32, h as f32, fov,n, f);

    let mut last_frame_time = Instant::now();
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        // Calculate delta time
        let current_frame_time = Instant::now();
        let delta_time = current_frame_time.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = current_frame_time;

    
        

        // Update
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let (w, h) = canvas.output_size().unwrap();
        let (w, h): (i32, i32) = (
            w.try_into().expect("Width is out of range for i32"),
            h.try_into().expect("Height is out of range for i32"),
        );

        //handle cam movement

        let mut dir_vec: Vector4<f32> = Vector4::new(0.0, 0.0, 0.0, 1.0);
        let mut y_rotation: f32 = 0.0;
        
        // Check keyboard state for continuous input detection
        let keyboard_state = event_pump.keyboard_state();
    
        if keyboard_state.is_scancode_pressed(Scancode::W) {
            dir_vec.z -= 1.0;
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            dir_vec.z += 1.0;
        }
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            dir_vec.x -= 1.0;
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            dir_vec.x += 1.0;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Q) {
            y_rotation -= 1.0;
        }
        if keyboard_state.is_scancode_pressed(Scancode::E) {
            y_rotation += 1.0;
        }



        let delta_position: Vector4<f32> = dir_vec * delta_time;
        let delta_y_rotation = y_rotation * delta_time;
        camera.move_cam(delta_position, delta_y_rotation);

        let mesh2d = camera.get_2d_mesh(&mesh3d);
        
        let half_width = (w as f32) / 2.0;
        let half_height = (h as f32) / 2.0;

        for triangle in mesh2d.triangles {
            let [i0, i1, i2] = [triangle.x, triangle.y, triangle.z];
        
            // Get the positions of the vertices
            let p0 = Vector2::new(
                mesh2d.get_vertex(i0).expect("Invalid vertex index")[0],
                mesh2d.get_vertex(i0).expect("Invalid vertex index")[1],
            );
            let p1 = Vector2::new(
                mesh2d.get_vertex(i1).expect("Invalid vertex index")[0],
                mesh2d.get_vertex(i1).expect("Invalid vertex index")[1],
            );
            let p2 = Vector2::new(
                mesh2d.get_vertex(i2).expect("Invalid vertex index")[0],
                mesh2d.get_vertex(i2).expect("Invalid vertex index")[1],
            );
            

            
            
            let min_x = p0[0].min(p1[0]).min(p2[0]).max(-w as f32/ 2.0) as i32;
            let max_x = p0[0].max(p1[0]).max(p2[0]).min(w as f32/ 2.0) as i32;
            let min_y = p0[1].min(p1[1]).min(p2[1]).max(-h as f32/ 2.0) as i32;
            let max_y = p0[1].max(p1[1]).max(p2[1]).min(h as f32/ 2.0) as i32;

            for y in min_y..max_y {
                for x in min_x..max_x {
                    let p: Vector2<f32> = Vector2::new(x as f32, y as f32);

                    if inside_triangle(p, p0, p1, p2) {
                        canvas.set_draw_color(Color::RGB(255, 255, 255));

                        // Add half_width and half_height to center the camera on screen
                        canvas.draw_point(sdl2::rect::Point::new(
                            x + half_width as i32,
                            y + half_height as i32,
                        ))
                        .unwrap();
                    }
                }
            }

        }
        

        canvas.present();

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}