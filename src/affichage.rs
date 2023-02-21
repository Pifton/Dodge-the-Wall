use macroquad::shapes::{draw_rectangle,draw_poly,draw_circle};
use macroquad::color::{DARKGRAY,DARKBLUE,LIGHTGRAY,RED,DARKPURPLE,YELLOW};
use macroquad::input::is_key_down;
use macroquad::input::KeyCode::{Space};
use macroquad::text::{draw_text,measure_text};
use macroquad::window::{screen_height,screen_width,clear_background};
use rand::prelude::Rng;
extern crate rand;

//static donnant le carre exclus //
static mut EXCLUS:i32 = 0;
static mut BOOST:i32 = 0;

use crate::collisions::{INVERSION, JOUEUR_Y,JOUEUR_X};
use crate::collisions::FALL;
use crate::SCORE;
use crate::GAME_OVER;

use crate::collisions::carre;
use crate::collisions::inversion;


//structure deffinissant les bords de la map
struct Bord{
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

fn bord_gauche() -> Bord{
    Bord{
        x1: screen_width() / screen_width(),
        y1: screen_height() / screen_height(),
        x2: screen_width() / 7.0,
        y2: screen_height() / 1.0,
    }
}
fn bord_droit() -> Bord{
    Bord{
        x1: screen_width() / 1.168,
        y1: screen_height() / screen_height(),
        x2: screen_width() / 7.0,
        y2: screen_height() / 1.0,
    }
}
pub fn bord(){ //affichage des bords de la map
    let bord_gauche = bord_gauche();
    let bord_droit = bord_droit();

    draw_rectangle(bord_gauche.x1, bord_gauche.y1, bord_gauche.x2, bord_gauche.y2, DARKGRAY);
    draw_rectangle(bord_droit.x1,bord_droit.y1,bord_droit.x2,bord_droit.y2, DARKGRAY);
    //on dessine donc un cercle dans un carre car je trouvais ca sympat
    //vu que JOUEUR_X s'incremente, le joueur se deplace
    unsafe {
        draw_poly(JOUEUR_X, JOUEUR_Y, 4, 25.0, 45.0, DARKPURPLE);
        draw_circle(JOUEUR_X, JOUEUR_Y, 16.0, YELLOW);
    }
}

pub fn chutte() {
    unsafe {
        //La vitesse de chutte depend du score
        if SCORE <= 15 { return FALL += 2.7; }
        if(15..=30).contains(&SCORE) { return FALL += 3.4 }
        if (30..=50).contains(&SCORE) { return FALL += 4.0 }
        if (50..=80).contains(&SCORE)  { return FALL += 5.0 }
        if (80..=120).contains(&SCORE) { return FALL += 5.5 }
        if (120..=150).contains(&SCORE) { return FALL += 6.0 }
        if FALL >= 150.0 { FALL += 7.0 }
    }
}

//choix des blocs a afficher
pub fn spawn_blocs() {
    unsafe {
        //Premiere fois que l'on donne une valeur a EXCLUS qui fonctionne uniquement lorsque l'on lance la partie
        if FALL == -60.0 {
            EXCLUS = rand::thread_rng().gen_range(1..6);
            BOOST = rand::thread_rng().gen_range(1..20);
            //println!("{}", EXCLUS);
        }
        //Assignation d'une nouvelle valeur a EXCLUS lorsque FALL depasse la position centrale du Joueur sur l'axe des Y
        //La valeur (JOUEUR_Y + 141.0) est specifie pour palier a un bug rencontre lors du developpement
        // Elle est choisi car FALL n'aura jamais cette valeur exacte
        if FALL >= JOUEUR_Y + 141.0{
            EXCLUS = rand::thread_rng().gen_range(1..6);
            FALL = -50.0;
            BOOST = rand::thread_rng().gen_range(1..15);
            println!("{}",BOOST);
            //println!("{}", EXCLUS);
        }
        //Affiche un bloc si la valeur d'EXCLUS ne lui correspond pas
        if EXCLUS != 1 {
            carre(175.0,DARKBLUE);
        }
        if EXCLUS != 2 {
            carre(265.0,DARKBLUE);
        }
        if EXCLUS != 3 {
            carre(355.0,DARKBLUE);
        }
        if EXCLUS != 4 {
            carre(445.0,DARKBLUE);
        }
        if EXCLUS != 5 {
            carre(535.0,DARKBLUE);
        }
        if EXCLUS != 6 {
            carre(625.0,DARKBLUE);
        }
        //affichage uniquement si un bloc n apparait pas
        if EXCLUS == 1 && BOOST == 1{
            inversion(175.0)
        }
        if EXCLUS == 2 && BOOST == 1{
            inversion(265.0)
        }
        if EXCLUS == 3 && BOOST == 1{
            inversion(355.0)
        }
        if EXCLUS == 4 && BOOST == 1{
            inversion(445.0)
        }
        if EXCLUS == 5 && BOOST == 1{
            inversion(535.0)
        }
        if EXCLUS == 6 && BOOST == 1 {
            inversion(625.0)
        }
    }
}
//ecran de game over
pub fn game_over() {
    unsafe {
        let text1 = "GAME OVER";
        let text2: &str = &("Votre Score est de: ".to_owned() + &SCORE.to_string());

        let text3 = "Appuyez sur [ESPACE] pour continuer";

        let taille_texte1 = measure_text(text1, None, 100, 1.0);
        let taille_texte2 = measure_text(text2, None, 30, 1.0);
        let taille_texte3 = measure_text(text3, None, 30, 1.0);
        //let text_size = measure_text(text, None, font_size as _, 1.0);

        clear_background(LIGHTGRAY);
        draw_text(text1, screen_width() / 2.0 - taille_texte1.width / 2.0, screen_height() / 2.0 - taille_texte1.height / 2.0, 100.0, RED);
        draw_text(text2, screen_width() / 2.0 - taille_texte2.width / 2.0, (screen_height() / 2.0 - taille_texte2.height / 2.0)+40.0, 30.0, DARKGRAY);
        draw_text(text3, screen_width() / 2.0 - taille_texte3.width / 2.0, (screen_height() / 2.0 - taille_texte3.height / 2.0)+80.0, 30.0, DARKGRAY);
    }
    //reinitialisation de certaines static
    if is_key_down(Space) {
        unsafe {
            GAME_OVER = false;
            FALL = -60.0;
            SCORE = 0;
            INVERSION = false;
            JOUEUR_X = 400.0;
        }
    }
}



