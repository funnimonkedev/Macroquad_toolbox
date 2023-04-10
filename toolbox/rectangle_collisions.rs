use macroquad::prelude::*;



#[derive(Debug, Clone)]
pub struct Rectangle {
    x: f32,
    y: f32,
    velx: f32,
    vely: f32,
    w: f32,
    h: f32,
    color: macroquad::prelude::Color,
}
#[macroquad::main("Collisions")]
async fn main() {
  /* ===example===
  
  (this example is inside the `async fn main()` after importing macroquad)
  
  let mut player = Rectangle { x: 10.0, y: 400.0, velx: 0.0, vely: 0.0, w: 13.0, h: 13.0, color: GREEN };
  let enemy = Rectangle { x: 1400.0, y: 500.0, speed: 2.0, w: 13.0, h: 13.0, color: RED };
  let mut enemy_vec: Vec<Rectangle> = vec![];
  let mut gameover = false;
  
    for _ in 0..30 {
        enemy_vec.push(enemy.clone());
    }
    
    loop {
      // ===player movement===
      
        if is_key_down(KeyCode::W) {
            player.vely += 0.4;
        }
        if is_key_down(KeyCode::A) {
            player.velx -= 0.4;
        }
        if is_key_down(KeyCode::S) {
            player.vely -= 0.4;
        }
        if is_key_down(KeyCode::D) {
            player.velx += 0.4;
        }

        player.y -= player.vely;
        player.x += player.velx;

        player.vely = player.vely / 1.1;
        player.velx = player.velx / 1.1;
        
        // ===everything else===
        
        for enemy in enemy_vec.iter_mut() {
            enemy.x -= enemy.velx;
            draw_rectangle(enemy.x, enemy.y, enemy.w, enemy.h, enemy.color);
        }
        
      draw_rectangle(player.x, player.y, player.w, player.h, player.color);
      collisions(&player, enemy_vec.clone(), &mut gameover);
      side_collisions(&player.x, &player.y, &mut gameover);
      
        if gameover == true {
            clear_background(GRAY);
            draw_text("YOU LOST", 20.0, 105.0, 150.0, BLACK);
        }
        
    }
  
  */
}
pub fn collisions(player: &Player, mut enemy_vec: Vec<Rectangle>, boolean: &mut bool) {
    for enemy in enemy_vec.iter_mut() {
        if player.x < enemy.x + enemy.w && player.x + player.w > enemy.x && player.y < enemy.y + enemy.h && player.h + player.y > enemy.y  {
            *boolean = true;
        }
    }
}
pub fn side_collisions(x: &f32, y: &f32, boolean: &mut bool) {
    if n1 >= &mut 794.0_f32 {
        *boolean = true;
    }
    if n1 <= &mut -1.5_f32 {
        *boolean = true;
    }

    if n1x >= &mut 1538.0_f32 {
        *boolean = true;
    }
    if n1x <= &mut -5.4_f32 {
        *boolean = true;
    }
}


fn rectangle_collisions(rect1: &Rectangle, rect2: &Rectangle, boolean: &mut bool) {   // I won't show an example for this because it is very easy too figure out.
    if rect1.x < rect2.x + rect2.w && rect1.x + rect1.w > rect2.x && rect1.y < rect2.y + rect2.h && rect1.h + rect1.y > rect2.y {
        *true_or_false_variable = true;
    }
    else {
        *true_or_false_variable = false;
    }
}

