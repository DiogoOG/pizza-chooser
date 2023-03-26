
pub struct Ingredient
{
    pub name : String,
    pub is_vegetarian : bool,
    pub is_vegan : bool
}

pub struct Pizza<'a>
{
    pub name : String,
    pub ingredients : Vec<&'a Ingredient>
}

impl Pizza<'_>
{
    pub fn is_vegan(&self) -> bool
    {
        for ingredient in &self.ingredients
        {
            if ingredient.is_vegan
            {
                return false;
            }
        }
        return true;
    }

    pub fn is_vegetarian(&self) -> bool
    {
        for ingredient in &self.ingredients
        {
            if ingredient.is_vegetarian
            {
                return false;
            }
        }
        return true;
    }
}

pub struct Menu<'a>
{
    pub name : String,
    pub pizzas : Vec<&'a Pizza<'a>>
}

impl Menu<'_>
{
    pub fn get_vegan_pizzas(&self) -> Vec<&Pizza>
    {
        let mut vegan_pizzas : Vec<&Pizza> = Vec::new();

        for pizza in &self.pizzas
        {
            if pizza.is_vegan()
            {
                vegan_pizzas.push(pizza);
            }
        }

        return vegan_pizzas;
    }

    pub fn get_vegetarian_pizzas(&self) -> Vec<&Pizza>
    {
        let mut vegetarian_pizzas : Vec<&Pizza> = Vec::new();

        for pizza in &self.pizzas
        {
            if pizza.is_vegetarian()
            {
                vegetarian_pizzas.push(pizza);
            }
        }

        return vegetarian_pizzas;
    }
}