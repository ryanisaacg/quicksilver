// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
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

    fn update(&mut self, window: &mut Window, delta_time: f64) -> Result<()> {
        //println!("Last Update: {:.10?} ms ago", delta_time);
        self.elapsed += delta_time;
        let seconds = (self.elapsed / 1000.) % 60.;
        let minutes = ((self.elapsed / 1000.) / 60.) % 60.;
        let hours = ((self.elapsed / 1000.) / 60. / 24.) % 24.;

        self.seconds = seconds;
        self.minutes = minutes;
        self.hours = hours;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window, delta_time: f64) -> Result<()> {
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
            let line = Line::new((400, 300), (pos_x, pos_y)).with_thickness(5.);
            window.draw(&line, Col(Color::BLACK));
        }

        window.draw(&Circle::new((400, 300), 180), Col(Color::WHITE));


        let hour_angle = 2. * PI * ((self.hours+9.) * 2. / 24.);
        let minute_angle = 2. * PI * ((self.minutes+45.) / 60.);
        let second_angle = 2. * PI * ((self.seconds+45.) / 60.);

        let hour_pos_x = 400. + 150. * hour_angle.cos() as f32;
        let hour_pos_y = 300. + 150. * hour_angle.sin() as f32;

        let min_pos_x = 400. + 180. * minute_angle.cos() as f32;
        let min_pos_y = 300. + 180. * minute_angle.sin() as f32;

        let sec_pos_x = 400. + 180. * second_angle.cos() as f32;
        let sec_pos_y = 300. + 180. * second_angle.sin() as f32;



        let hour = Line::new((400, 300), (hour_pos_x, hour_pos_y)).with_thickness(10.);
        let minute = Line::new((400, 300), (min_pos_x, min_pos_y)).with_thickness(5.);
        let second = Line::new((400, 300), (sec_pos_x, sec_pos_y)).with_thickness(3.);

        window.draw(&hour, Col(Color::BLACK));
        window.draw(&minute, Col(Color::BLUE));
        window.draw(&second, Col(Color::RED));

        Ok(())
    }
}

fn main() {
    let mut settings = Settings::default();
    settings.draw_rate = 1000. / 60.; // 60 fps
    settings.update_rate = 1.;

    run::<Stopwatch>("Rates", Vector::new(800, 600), settings);
}

