extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::thread::sleep; 
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::time::{Duration, SystemTime};
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use std::fs::File; 
use std::io::{self,Write,Read};

extern crate rand;




const TEXTURE_SIZE: u32 = 32; 
const TETRIS_HEIGHT : usize = 40; 


#[derive(Clone, Copy)]
enum TextureColor{
    Groen,
    Blauw,
    Rood,
}


fn write_to_file(content: &str, file_name: &str) -> io::Result<()>{ //content is what will be writing into file_name 
    let mut f = File::create(file_name)?; // ? is equivalent to try!(expr) macro
    f.write_all(content.as_bytes())

}

fn read_from_file(file_name: &str) -> io::Result<String>{
    let mut f = File::open(file_name)?; 
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)

}



fn slice_to_string(slice: &[u32]) -> String {
    slice.iter().map(|highscore| highscore.to_string()).collect::<Vec<String>>().join(" ")



}


fn save_highscore_and_lines(highscores: &[u32], num_lines: &[u32]) -> bool{
    let s_highscores = slice_to_string(highscores);
    let s_num_of_lines = slice_to_string(num_lines);
    false

     //write_to_file(format!("{}\n{}\n", s_highscores, s_num_of_lines), "Scores.txt").is_ok() 
}

fn line_to_slice(line: &str) -> Vec<u32>{
    line.split(" ").filter_map(|nb| nb.parse::<u32>().ok()).collect()
}


fn load_high_scores() -> Option<(Vec<u32>, Vec<u32>)>{
    if let Ok(content) = read_from_file("Scores.txt") {
        let mut lines = content.splitn(2,"\n").map(|line| line_to_slice(line)).collect::<Vec<_>>();
        if lines.len() == 2{
            let (num_lines, highscores) = (lines.pop().unwrap(), lines.pop().unwrap());
            Some((highscores, num_lines))
        }else{
            None
        }
    }else {
        None
    }
}
fn create_texture_rect<'a>(canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, color: TextureColor, size: u32) ->Option<Texture<'a>>{

    if let Ok(mut square_texture) =
    texture_creator.create_texture_target(None,size,size){
        canvas.with_texture_canvas(&mut square_texture, |texture| {
            match color {
                TextureColor::Groen => texture.set_draw_color(Color::RGB(0,255,0)),
                TextureColor::Blauw => texture.set_draw_color(Color::RGB(0,0,225)),
                TextureColor::Rood => texture.set_draw_color(Color::RGB(255,0,0)),
            }
            texture.clear();
        }).expect("Failed to color a texture");
        Some(square_texture)
    }else{
        None
    }

}


type Piece = Vec<Vec<u8>>;
type States = Vec<Piece>;


struct Tetrimino {
    states: States, //possible states of the tetrimo 
    x: isize,
    y: usize,
    curr_state: u8,
}

trait TetriminoGenerator{
    fn new() -> Tetrimino; 
}

struct TetriminoI; 

impl TetriminoGenerator for TetriminoI { // [][][][]
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![1,1,1,1],
                                vec![0,0,0,0],
                                vec![0,0,0,0],
                                vec![0,0,0,0]],
                                vec![vec![0,1,0,0],
                                vec![0,1,0,0],
                                vec![0,1,0,0],
                                vec![0,1,0,0]]],
            x: 4,
            y: 0,
            curr_state: 0,
                                
                                
        }
    }
}

struct TetriminoJ; 

impl TetriminoGenerator for TetriminoJ{
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![ vec! [vec![2,2,2,0],
                                vec![2,0,0,0],
                                vec![0,0,0,0],
                                vec![0,0,0,0]],
                                vec![vec![2,2,0,0],vec![0,2,0,0],vec![0,2,0,0],vec![0,0,0,0]],
                                vec![vec![0,0,2,0], vec![2,2,2,0], vec![0,0,0,0], vec![0,0,0,0]],
                                vec![vec![2,0,0,0], vec![2,0,0,0], vec![2,2,0,0], vec![0,0,0,0]]],

            x: 4,
            y: 0,
            curr_state: 0,
        }
    }
}

struct TetriminoL; 

impl TetriminoGenerator for TetriminoL {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![3,3,3,0], vec![0,0,3,0], vec![0,0,0,0], vec![0,0,0,0]],
                         vec![vec![0,3,0,0], vec![0,3,0,0], vec![3,3,0,0], vec![0,0,0,0]],
                         vec![vec![3,0,0,0], vec![3,3,3,0], vec![0,0,0,0], vec![0,0,0,0]],
                         vec![vec![3,3,0,0], vec![3,0,0,0], vec![3,0,0,0], vec![0,0,0,0]]],
            x:4,
            y:0,
            curr_state: 0,
        }
    }
}

struct TetriminoO; 

impl TetriminoGenerator for TetriminoO{
    fn new() -> Tetrimino{
        Tetrimino{
            states: vec![vec![vec![4,4,0,0], vec![4,4,0,0], vec![0,0,0,0], vec![0,0,0,0]]],
            x: 5,
            y: 0,
            curr_state: 0,
        }
    }
}

struct TetriminoS; 

impl TetriminoGenerator for TetriminoS{
    fn new() -> Tetrimino{
        Tetrimino{
            states: vec![vec![vec![0,5,5,0], vec![5,5,0,0], vec![0,0,0,0], vec![0,0,0,0]],
                         vec![vec![0,5,0,0], vec![0,5,5,0], vec![0,0,5,0], vec![0,0,0,0]]],
            x: 4,
            y: 0,
            curr_state: 0,
        }
    }
}

struct TetriminoZ; 

impl TetriminoGenerator for TetriminoZ{
    fn new() -> Tetrimino{
        Tetrimino{
            states: vec![vec![vec![6,6,0,0], vec![0,6,6,0], vec![0,0,0,0], vec![0,0,0,0]],
                         vec![vec![0,0,6,0], vec![0,6,6,0], vec![0,6,0,0], vec![0,0,0,0]]],
            x: 4,
            y: 0,
            curr_state: 0,
        }
    }
}

struct TetriminoT; 

impl TetriminoGenerator for TetriminoT {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![7,7,7,0], vec![0,7,0,0], vec![0,0,0,0], vec![0,0,0,0]],
                         vec![vec![0,7,0,0], vec![7,7,0,0], vec![0,7,0,0], vec![0,0,0,0]],
                         vec![vec![0,7,0,0], vec![7,7,7,0], vec![0,0,0,0], vec![0,0,0,0]],
                         vec![vec![0,7,0,0], vec![0,7,7,0], vec![0,7,0,0], vec![0,0,0,0]]],
            x:4,
            y:0,
            curr_state: 0,
        }
    }
}

type Coordinates = Vec<u8>;
struct Tetris{
    game_map: Vec<Vec<u8>>,
    curr_level: u32,
    score: u32, 
    nb_lines: u32, 
    curr_piece: Option<Tetrimino>,
}

impl Tetris {
    fn new() -> Tetris{
        let mut game_map = Vec::new();
        for _ in 0..16 {
            
            game_map.push(vec![0,0,0,0,0,0,0,0,0,0]);
        }
        Tetris {
            game_map: game_map, 
            curr_level: 1, 
            score: 0,
            nb_lines: 0, 
            curr_piece: None, 

        }
    }

    fn create_new_tretrimino(&self) -> Tetrimino{

        static mut PREV: u8 = 7;
        
        let mut rand_nb = rand::random::<u8>() % 7; 
        if unsafe { PREV } == rand_nb {
            rand_nb = rand::random::<u8>() % 7; 
        }
        unsafe { PREV = rand_nb; } 
        match rand_nb {
            0 => TetriminoI::new(),
            1 => TetriminoJ::new(),
            2 => TetriminoL::new(),
            3 => TetriminoO::new(),
            4 => TetriminoS::new(),
            5 => TetriminoZ::new(),
            6 => TetriminoT::new(),
            _ => unreachable!(),
        }
    }
    fn check_lines(&mut self){
        let mut y = 0; 

        while y <  self.game_map.len(){
            let mut complete = true; 

            for x in &self.game_map[y] {
                if *x == 0 {
                    complete = false; 
                    break
                }
            }

            if complete == true {
                self.game_map.remove(y);
                y -= 1;

            }
            y +=1; 
        }
        while self.game_map.len() < 16 {
            self.game_map.insert(0, vec![0,0,0,0,0,0,0,0,0,0]);
        }
    }

    fn make_permanent(&mut self){
        if let Some(ref mut piece) = self.curr_piece{
            let mut shift_y = 0;

            while shift_y < piece.states[piece.curr_state as usize].len() && piece.y + shift_y < self.game_map.len(){
                let mut shift_x = 0;
                while shift_x <piece.states[piece.curr_state as usize][shift_y].len() && (piece.x + shift_x as isize) < self.game_map[piece.y + shift_y].len() as isize{
                    if piece.states[piece.curr_state as usize][shift_y][shift_x] != 0{
                        let x = piece.x + shift_x as isize;
                        self.game_map[piece.y + shift_y][x as usize] = piece.states[piece.curr_state as usize][shift_y][shift_x];
                    }
                    shift_x += 1;
                }
                shift_y += 1;
            }
        }
        self.check_lines();
        self.curr_piece = None; 
    }

    fn handle_events(tetris: &mut Tetris, quit: &mut bool, timer: &mut SystemTime, event_pump: &mut sdl2::EventPump) -> bool {
        let mut make_permanent = false;
        if let Some(ref mut piece ) = tetris.curr_piece{
            let mut tmp_x = piece.x; 
            let mut tmp_y = piece.y; 
    
            for event in event_pump.poll_iter(){
                match event {
                    Event::Quit { .. }|
                    Event::KeyDown{keycode: Some(Keycode::Escape), ..} => {*quit = true; break}
                    Event::KeyDown{keycode: Some(Keycode::Down), ..} => {*timer = SystemTime::now(); tmp_y += 1;}
                    Event::KeyDown{keycode: Some(Keycode::Right), ..} => {tmp_x += 1;}
                    Event::KeyDown{keycode: Some(Keycode::Left), ..} => {tmp_x -= 1;}
                    Event::KeyDown{keycode: Some(Keycode::Up), ..} => {piece.rotate(&tetris.game_map);}
                    Event::KeyDown{keycode: Some(Keycode::Space), ..} => {let x = piece.x;
                        let mut y = piece.y;
                        while piece.change_position(&tetris.game_map, x, y + 1) == true {
                            y +=1;
    
                        }
                        make_permanent = true;
                    
                    }
                    _ => {}
            }
        }
        if !make_permanent {
            if piece.change_position(&tetris.game_map, tmp_x, tmp_y) == false && tmp_y != piece.y {
                make_permanent = true; 
            }
        }
    
    }
    if make_permanent{
        tetris.make_permanent();
        *timer = SystemTime::now();
    }
    make_permanent
}
}







fn create_new_tretrimino() -> Tetrimino{

    static mut PREV: u8 = 7;
    
    let mut rand_nb = rand::random::<u8>() % 7; 
    if unsafe { PREV } == rand_nb {
        rand_nb = rand::random::<u8>() % 7; 
    }
    unsafe { PREV = rand_nb; } 
    match rand_nb {
        0 => TetriminoI::new(),
        1 => TetriminoJ::new(),
        2 => TetriminoL::new(),
        3 => TetriminoO::new(),
        4 => TetriminoS::new(),
        5 => TetriminoZ::new(),
        6 => TetriminoT::new(),
        _ => unreachable!(),
    }
}



impl Tetrimino {

    fn test_position(&self, game_map: &[Vec<u8>], tmp_state: usize, x:isize, y:usize  ) -> bool{
        for decal_y in 0..4{
            for decal_x in 0..4{
                let x = x + decal_x;
                if self.states[tmp_state][decal_y][decal_x as usize] != 0 && (y + decal_y >= game_map.len() || x < 0 || x as usize >= game_map[y + decal_y].len() || game_map[y + decal_y][x as usize] != 0 ) {
                    return false
                }
            }
        }
    
    
    
        true
    }

    fn rotate(&mut self, game_map: &[Vec<u8>]){
        let mut tmp_state = self.curr_state + 1;
        if tmp_state as usize >= self.states.len(){tmp_state = 0;}
        let x_pos = [0,-1,1,-2,2,-3];
        for x in x_pos.iter() {
            if self.test_position(game_map,tmp_state as usize, self.x + x, self.y ) == true {
                self.curr_state = tmp_state;
                self.x += *x;
                break
            }
        }
    }

    fn change_position(&mut self, game_map: &[Vec<u8>], new_x: isize, new_y: usize) -> bool{
        if self.test_position(game_map, self.curr_state as usize, new_x, new_y) ==true {
            self.x = new_x as isize; 
            self.y = new_y;
            true
        }else{
            false
        }

    }

    fn test_curr_position(&self, game_map: &[Vec<u8>]) -> bool{
        self.test_position(game_map, self.curr_state as usize, self.x, self.y)
    }

    
}





pub fn main(){
    let sdl_context = sdl2::init().expect("Could not initialize");
    let mut tetris = Tetris::new();
    let mut timer = SystemTime::now(); 

    let mut event_pump = sdl_context.event_pump().expect("Could not get event pump");

    loop{
        if match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs() >= 1,
            Err(_) => false
        }{
            let mut make_permanent = false;
            if let Some(ref mut piece) = tetris.curr_piece{
                let x = piece.x; 
                let y = piece.y + 1;

                make_permanent = !piece.change_position(&tetris.game_map, x, y);
            }
            if make_permanent {
                tetris.make_permanent();
            }
            timer = SystemTime::now();
        }

        if tetris.curr_piece.is_none(){
            let current_piece = tetris.create_new_tretrimino();
            if !current_piece.test_curr_position(&tetris.game_map){
                break
            }
            tetris.curr_piece = Some(current_piece);
        }
        let mut quit = false; 
        
        if quit {
            break
        }

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }



}