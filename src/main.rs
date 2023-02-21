mod affichage;
mod collisions;

use macroquad::audio::{load_sound, play_sound, play_sound_once, PlaySoundParams, stop_sound};
use macroquad::color::{GOLD,LIGHTGRAY};
use macroquad::text::draw_text;
use macroquad::window::{clear_background,next_frame};

use crate::collisions::{deplacement};
use crate::affichage::{game_over,spawn_blocs,chutte,bord};


//score du joueur, condition si jeu perdu et exclusion d un des blocs
static mut SCORE:i32 = 0;
static mut GAME_OVER:bool = false;


#[macroquad::main("BasicShapes")]
async fn main() {
    let musique_jeu = load_sound("TetrisTheme.wav").await.unwrap();
    let game_over_voice = load_sound("Game_OVER.wav").await.unwrap();

    let mut compteur = 0;
    let mut fin_musique = true;
    loop {
        unsafe {
            if !GAME_OVER {
                clear_background(LIGHTGRAY);

                spawn_blocs();
                deplacement();
                chutte();
                bord();

                //on run lorsque le bool fin_musique est vrai
                if fin_musique{
                    //on arrete le son du game_over si il run
                    stop_sound(game_over_voice);
                    //et on joue la musique du jeu
                    play_sound(musique_jeu,PlaySoundParams{
                        looped:true,  //looped est true pour jouer la musique en boucle dans le cas
                        //ou le joueur survit suffisament longtemps pour arriver a la fin de la musique
                        //ce que je ne pense pas car je pense que mon jeu est tres dur, ce qui veut dire
                        //qu il faut enormement de skill. Ce que peux de personnes ont (je n en fais pas partie)
                        volume:0.2,
                    });
                }

                //ajout d'un compteur pour calculer le score en fonction du nombre de fois que la boucle est execute
                compteur += 1;
                if compteur % 20 == 0{
                    SCORE += 1;
                }
                let string_score:&str = &("Score: ".to_owned() + &SCORE.to_string());
                draw_text(string_score, 10.0,50.0,25.0, GOLD); //affichage du score en haut a gauche de l'ecran
                //on passe le bool a false pour ne pas qu'il recommence la musique ( on la joue donc qu une seule fois)
                fin_musique = false;

            }
            else {
                game_over();
                //lorsque le bool est faux on execute la boucle
                if !fin_musique{
                    // on arrete la musique du jeu et on lance le son du game over
                    stop_sound(musique_jeu);
                    play_sound_once(game_over_voice);
                }
                //on donne la valeur true au bool pour run le son qu une seule fois
                fin_musique = true;
            }
        }
        next_frame().await
    }
}










