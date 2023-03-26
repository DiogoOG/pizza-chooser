
pub struct Ingredient
{
    pub name : String,
    pub meat : bool,
    pub animal_origin : bool
}

pub struct Pizza
{
    pub name : String,
    pub ingredients : Vec<Ingredient>
}

impl Pizza
{
    pub fn IsVegan(&self) -> bool
    {
        for ingredient in &self.ingredients
        {
            if ingredient.animal_origin
            {
                return false;
            }
        }
        return true;
    }

    pub fn IsVegetarian(&self) -> bool
    {
        for ingredient in &self.ingredients
        {
            if ingredient.meat
            {
                return false;
            }
        }
        return true;
    }
}

pub struct Menu
{
    pub name : String,
    pub pizzas : Vec<Pizza>
}

impl Menu
{
    pub fn GetVeganPizzas(&self) -> Vec<&Pizza>
    {
        let mut vegan_pizzas : Vec<&Pizza> = Vec::new();

        for pizza in &self.pizzas
        {
            if pizza.IsVegan()
            {
                vegan_pizzas.push(pizza);
            }
        }

        return vegan_pizzas;
    }

    pub fn GetVegetarianPizzas(&self) -> Vec<&Pizza>
    {
        let mut vegetarian_pizzas : Vec<&Pizza> = Vec::new();

        for pizza in &self.pizzas
        {
            if pizza.IsVegetarian()
            {
                vegetarian_pizzas.push(pizza);
            }
        }

        return vegetarian_pizzas;
    }
}