mod game;
mod drawing; 

use piston_window::*; 
use piston_window::types::Color; 
use crate::drawing::to_gui_coord_u32; 
use crate::game::Game;

const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0]; 

fn main() {
    let (width, height) = (20, 20); 
    let mut window: PistonWindow = WindowSettings::new("snake game", 
        [to_gui_coord_u32(width), to_gui_coord_u32(height)])
        .exit_on_esc(true)
        .build()
        .unwrap(); 
    let mut game = Game::new(width, height); 
    while let Some(event) = window.next(){
        if let Some(Button::Keyboard(key)) = event.press_args(){
            game.key_pressed(key); 
        }
        window.draw_2d(&event, |c, g|{
            clear(BACK_COLOR, g); 
            game.draw(&c, g); 
        });
        event.update(|arg|{
            game.update(arg.dt);
        });
    }
}
