//Importa os crates
use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0; //const para tamanho do bloco

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE //Função que recebe um valor e multiplica com BLOCK_SIZE e
    //retorna float
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32 //Transforma to_coord em u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) { //Função que desenha
    //os blocos
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) { //Função que desenha os retangulos
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}
