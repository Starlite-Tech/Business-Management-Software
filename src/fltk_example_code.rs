use fltk::{app, button::*, enums::*, frame::*, group::*, prelude::*, window::*};
use std::cell::RefCell;
use std::rc::Rc;

/*

Created:0.0.1
updated:0.0.1

description:
  As the file name explains. This is just example code for any new devs to go off of.
 */


fn _main() -> Result<(), Box<dyn std::error::Error>> { //remove the Underscore if you want to run this file.

    //create app & widgets
    let app = app::App::default();
    let counter = Counter::new(0);
    let mut wind = Window::default()
        .with_size(160, 200)
        .center_screen()
        .with_label("Counter");
    let mut frame = Frame::default()
        .with_size(100, 40)
        .center_of(&wind)
        .with_label("0");
    frame.set_label_size(20);
    let mut but_inc = Button::default()
        .size_of(&frame)
        .above_of(&frame, 0)
        .with_label("+");
    let mut but_dec = Button::default()
        .size_of(&frame)
        .below_of(&frame, 0)
        .with_label("-");
    wind.make_resizable(true);
    wind.end();
    wind.show();

    // Theming
    wind.set_color(Color::from_u32(0xffebee));
    but_inc.set_color(Color::from_u32(0x304FFE));
    but_inc.set_selection_color(Color::Green);
    but_inc.set_label_size(20);
    but_inc.set_frame(FrameType::RFlatBox);
    but_inc.set_label_color(Color::White);
    but_dec.set_color(Color::from_u32(0x2962FF));
    but_dec.set_selection_color(Color::Red);
    but_dec.set_frame(FrameType::RFlatBox);
    but_dec.set_label_size(20);
    but_dec.set_label_color(Color::White);

    //add functionality via callbacks and handlers
    but_inc.set_callback({
        let mut c = counter.clone();
        move |_| c.increment()
    });

    but_dec.set_callback({
        let mut c = counter.clone();
        move |_| c.decrement()
    });

    frame.handle(move |f, ev| { //f is a clone of what is being handled. In this case, &frame. EV is the Event code.
        //println!("ev: {} && f: {}",ev as i32,f.label());
        if ev as i32 == MyEvent::CHANGED {
            f.set_label(&counter.clone().value().to_string());
            return true;
        } else {
            return false;
        }
    });


    //run the app
    Ok(app.run()?)
}

// create a counter Struct which contains all functionality for the counter app. It also Derives clone so that way we wont get errors when using move |_|
#[derive(Clone)]
pub struct Counter{
    count: Rc<RefCell<i32>>, //creates a single cell in memory that contains an i32.
}

impl Counter {
    pub fn new(val: i32) -> Self{
        return Counter {
            count: Rc::from(RefCell::from(val))
        };
    }

    pub fn increment(&mut self){
        *self.count.borrow_mut() += 1;
        app::handle_main(MyEvent::CHANGED).unwrap();
    }

    pub fn decrement(&mut self){
        *self.count.borrow_mut() -= 1;
        app::handle_main(MyEvent::CHANGED).unwrap();
    }

    pub fn value(&self) -> i32 {
        *self.count.borrow()
    }
}

//list of events you want to name.
pub struct MyEvent;
impl MyEvent{
    const CHANGED: i32 = 42;//the value of this can be anything
}