use macroquad::shapes::{draw_poly,draw_circle};
use macroquad::color::{Color, GREEN};
use macroquad::input::is_key_down;
use macroquad::input::KeyCode::{Right,Left};
use macroquad::prelude::Vec2;

//positions x et y du joueur
pub static mut JOUEUR_X:f32 = 400.0;
pub static JOUEUR_Y:f32 = 500.0;
//position dans l'axe des y du centre des blocs en chutte libre
pub static mut FALL:f32 = -60.0;
//pouvoir d'inversion
pub static mut INVERSION:bool = false;
use crate::GAME_OVER;

//definition des structures du joueur et des blocs contenant 4 points car ce sont
//tous des carre.
struct Polygon{
    p1:Vec2,
    p2:Vec2,
    p3:Vec2,
    p4:Vec2,
}

//deplacement du personnage et collision avec les murs sur le cote
//le personnage ne peut pas se deplacer en dehors de ces coordones
pub fn deplacement(){
    unsafe {
        if !INVERSION{
            if JOUEUR_X > 130.0 && is_key_down(Left) {
                //incrementate l'axe X du joueur
                return JOUEUR_X -= 5.0;
                }
            if JOUEUR_X < 670.0 && is_key_down(Right) {
                return JOUEUR_X += 5.0;
            }
        }

        //lorsque le pouvoir d inversion est true, les controles changent
        if INVERSION && JOUEUR_X > 130.0 {
            if is_key_down(Right) {
                //incrementate l'axe X du joueur
                return JOUEUR_X -= 5.0;
            }
            if JOUEUR_X < 670.0 && is_key_down(Left) {
                JOUEUR_X += 5.0;
            }
        }
    }
}

//affichage des carres en plus de la fonction determinant le game over
pub fn carre(x:f32,color:Color){

    unsafe {
        let joueur = Polygon {
            p1: Vec2::new(JOUEUR_X - 17.5, JOUEUR_Y - 17.5),
            p2: Vec2::new(JOUEUR_X - 17.5, JOUEUR_Y + 17.5),
            p3: Vec2::new(JOUEUR_X + 17.5, JOUEUR_Y - 17.5),
            p4: Vec2::new(JOUEUR_X + 17.5, JOUEUR_Y + 17.5),
        };

        let carre = Polygon {
            p1: Vec2::new(x - 44.5, FALL - 44.5),
            p2: Vec2::new(x - 44.5, FALL + 44.5),
            p3: Vec2::new(x + 44.5, FALL - 44.5),
            p4: Vec2::new(x + 44.5, FALL + 44.5),
        };

        //j ai utilise le teoreme de pythagore pour determiner la taille du rayon.
        //Car je voulais un carre de 89px de taille
        draw_poly(x, FALL, 4, 63.0, 45.0, color);

        //explication de ces collisions dans le fichier "Explications des collisions.pdf"
        // Dans le pdf 44.5 est appele taille
        //Car honnettement je n'ai pas trop envie d ecrire 80 lignes de commentaires
        //qui ne vont pas etre clair car je suis nul en explication
        if (FALL + 44.5) >= joueur.p1.y && (FALL - 44.5) <= joueur.p2.y {
            if joueur.p1 <= carre.p3 && joueur.p3 >= carre.p1 {
                GAME_OVER = true;
            }
        }
    }
}

pub fn inversion(x:f32){
    unsafe {
        let joueur = Polygon {
            p1: Vec2::new(JOUEUR_X - 17.5, JOUEUR_Y - 17.5),
            p2: Vec2::new(JOUEUR_X - 17.5, JOUEUR_Y + 17.5),
            p3: Vec2::new(JOUEUR_X + 17.5, JOUEUR_Y - 17.5),
            p4: Vec2::new(JOUEUR_X + 17.5, JOUEUR_Y + 17.5),
        };

        let _inversion = Polygon {
            p1: Vec2::new(x - 10.0, FALL - 10.0),
            p2: Vec2::new(x - 10.0, FALL + 10.0),
            p3: Vec2::new(x + 10.0, FALL - 10.0),
            p4: Vec2::new(x + 10.0, FALL + 10.0),
        };


        if (FALL + 10.0) <= joueur.p1.y {
            draw_circle(x, FALL, 10.0, GREEN);
        }

        //fait passer le pouvoir d inversion a true
        if FALL >= joueur.p1.y + 139.0 && !INVERSION {
                INVERSION = true;
        }
    }
}



