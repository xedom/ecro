use std::{time::{SystemTime, Duration}, thread};

extern crate glfw;

use glfw::{Action, Context, Key};

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                glfw::WindowEvent::Key(Key::N, _, Action::Press, _) => {
                    println!("n");
                }
                _ => {},
            }
        }
    }
}

fn game_loop() {
    println!("Hello, world!");
    let secs_per_frame: Duration = Duration::from_secs_f64(1.0 / 30.0);
    let mut previous = SystemTime::now();
    let mut steps = Duration::ZERO;

    loop {
        // handleInput();
        // updateGameState();
        let current = SystemTime::now();

        let elapsed = previous.elapsed().expect("msg di errore elapsed bho");
  
        previous = current;
        steps += elapsed; // 150 micro sec

        // println!("secs_per_frame {:?}", secs_per_frame);
        // println!("elapsed {:?}", elapsed);

        while steps >= secs_per_frame {
            update_game_state();
            steps -= secs_per_frame;
        }

        render();
        sync(current);
    }

}


fn update_game_state() {
    println!("update_game_state");
}

fn render() {
    println!("render");
}
fn sync(loop_start_time: SystemTime) {
    println!("sync {:?}", loop_start_time);
    let secs_per_frame: Duration = Duration::from_secs_f64(1.0 / 30.0);
    let loop_slot = secs_per_frame;
    let end_time = loop_start_time + loop_slot; 
    while SystemTime::now() < end_time {
        thread::sleep(Duration::from_millis(1));
    }
}