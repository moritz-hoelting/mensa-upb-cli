use crate::{Dish, Mensa};

#[derive(Debug, Clone, PartialEq)]
pub struct DailyMenu {
    mensa: Mensa,
    main_dishes: Vec<Dish>,
    side_dishes: Vec<Dish>,
    desserts: Vec<Dish>,
}

impl DailyMenu {
    pub fn new(
        mensa: Mensa,
        main_dishes: Vec<Dish>,
        side_dishes: Vec<Dish>,
        desserts: Vec<Dish>,
    ) -> DailyMenu {
        DailyMenu {
            mensa,
            main_dishes,
            side_dishes,
            desserts,
        }
    }

    pub fn get_mensa(&self) -> &Mensa {
        &self.mensa
    }
    pub fn get_main_dishes(&self) -> &[Dish] {
        &self.main_dishes
    }
    pub fn get_side_dishes(&self) -> &[Dish] {
        &self.side_dishes
    }
    pub fn get_desserts(&self) -> &[Dish] {
        &self.desserts
    }
}
