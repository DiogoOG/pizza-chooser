
mod menu;

use crate::menu::*;

fn main() {
    let i1 : Ingredient = Ingredient { name : "Tomat".to_owned() , meat : false, animal_origin : false};
    let i2 : Ingredient = Ingredient { name : "Muzz".to_owned() , meat : false, animal_origin : true};
    let i3 : Ingredient = Ingredient { name : "Oregano".to_owned() , meat : false, animal_origin : false};
    let i4 : Ingredient = Ingredient { name : "Peperonnee".to_owned() , meat : true, animal_origin : true};

    let p : Pizza = Pizza { name : "Pepy".to_owned() , ingredients : vec![ i1 , i2, i3, i4]};

    println!("{}", p.IsVegan());
    println!("{}", p.IsVegetarian());
}
