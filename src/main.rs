
mod menu;

use crate::menu::*;

fn main() {
    let i1 : Ingredient = Ingredient { name : "Tomat".to_owned() , is_vegetarian : false, is_vegan : false};
    let i2 : Ingredient = Ingredient { name : "Muzz".to_owned() , is_vegetarian : false, is_vegan : true};
    let i3 : Ingredient = Ingredient { name : "Oregano".to_owned() , is_vegetarian : false, is_vegan : false};
    let i4 : Ingredient = Ingredient { name : "Peperonnee".to_owned() , is_vegetarian : true, is_vegan : true};

    let p1 : Pizza = Pizza { name : "Pepy".to_owned() , ingredients : vec![ &i1 , &i2, &i3, &i4]};

    let p2 : Pizza = Pizza { name : "Margy".to_owned() , ingredients : vec![ &i1 , &i2, &i3]};

    let m : Menu = Menu { name : "Papi".to_owned(), pizzas : vec![&p1, &p2]};
}
