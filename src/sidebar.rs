use fltk::{button::*, enums::*, frame::*, group::*, prelude::*, window::*};
use std::cell::RefCell;
use std::rc::Rc;


/*

Created:0.0.1
updated:0.0.1

description:
  Contains Sidebar widgets and functionality
*/



pub fn create(wind: &mut DoubleWindow, at: i32) -> BarUi{
    let frame = Frame::default()
        .with_size(200, 400)
        .center_of(wind);
    let mut team_button = Button::new(0,0,100,100,"poggers");

    let functionality = SidebarFunctionality::new(at);
    team_button.set_callback({
        let mut c = functionality.clone();
        move |_| c.test()
    });

    //return a ui.
    BarUi{
        frame: frame.clone(),
        team_button: team_button.clone(),
        functionality: functionality.clone()
    }
}

//struct to store the UI
pub struct BarUi{
    frame: Frame,
    team_button: Button,

    functionality: SidebarFunctionality,
}

//The following code handles the functionality of the sidebar

#[derive(Clone)]
pub struct SidebarFunctionality {
    account_type: Rc<RefCell<i32>>,
}

impl SidebarFunctionality{
    pub fn new(at:i32) -> Self{
        SidebarFunctionality{
            account_type: Rc::from(RefCell::from(at))
        }
    }

    pub fn test(&mut self){
        let x = &self.account_type;
        println!("{}",x.try_borrow().unwrap());
    }
}