//Chama os outros arquivos para utizar
mod draw;
mod game;
mod snake;
//Importa os crates
use piston_window::types::Color;
use piston_window::*;
//Importa as funções dos arquivos chamados
use crate::draw::to_coord_u32;
use crate::game::Game;
//const para pre definir valor
const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30, 30); //Define o tamanho da janela
    //Desenha a janela
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();
    //Passa os valores e começa o jogo
    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
