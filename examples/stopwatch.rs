// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Circle, Line, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{Settings, State, Window, run},
};

use std::f64::consts::PI;

struct Stopwatch {
    elapsed: f64,
    hours: f64,
    minutes: f64,
    seconds: f64,
}

impl State for Stopwatch {
    fn new() -> Result<Stopwatch> {
        Ok(Stopwatch {elapsed: 0., hours: 0., minutes: 0., seconds: 0.})
    }

    fn update(&mut self, _window: &mut Window, delta_time: f64) -> Result<()> {
        //println!("Last Update: {:.10?} ms ago", delta_time);
        self.elapsed += delta_time;
        self.seconds = (self.elapsed / 1000.) % 60.;
        self.minutes = ((self.elapsed / 1000.) / 60.) % 60.;
        self.hours = ((self.elapsed / 1000.) / 60. / 24.) % 24.;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window, _delta_time: f64) -> Result<()> {
        //println!("Last Draw: {:.10?} ms ago", delta_time);

        // clear everything
        window.clear(Color::WHITE)?;

        // draw the frame
        window.draw(&Circle::new((400, 300), 203), Col(Color::BLACK));
        window.draw(&Circle::new((400, 300), 200), Col(Color::WHITE));

        // draw the hour markers
        for i in 1..=12 {
            let angle = 2. * PI * ((i as f64 + 9.) * 2. / 24.);
            let pos_x = 400. + 200. * angle.cos() as f32;
            let pos_y = 300. + 200. * angle.sin() as f32;
            let line = Line::new((400, 300), (pos_x, pos_y)).with_thickness(5);
            window.draw(&line, Col(Color::BLACK));
        }

        window.draw(&Circle::new((400, 300), 180), Col(Color::WHITE));


        let hour_angle = 360. * ((self.hours+9.) * 2. / 24.);
        let minute_angle = 360. * ((self.minutes+45.) / 60.);
        let second_angle = 360. * ((self.seconds+45.) / 60.);

        let hour_pos = Vector::from_angle(hour_angle as f32) * 150. + Vector::new(400, 300);
        let min_pos = Vector::from_angle(minute_angle as f32) * 180. + Vector::new(400, 300);
        let second_pos = Vector::from_angle(second_angle as f32) * 180. + Vector::new(400, 300);

        let hour = Line::new((400, 300), hour_pos).with_thickness(10);
        let minute = Line::new((400, 300), min_pos).with_thickness(5);
        let second = Line::new((400, 300), second_pos).with_thickness(3);

        window.draw(&hour, Col(Color::BLACK));
        window.draw(&minute, Col(Color::BLUE));
        window.draw(&second, Col(Color::RED));

        Ok(())
    }
}

fn main() {
    run::<Stopwatch>("Stopwatch", Vector::new(800, 600), Settings {
        draw_rate: 1000. / 60., // 60 fps
        update_rate: 1., // every ms to make it appear "smooth"
        ..Settings::default()
    });
}

