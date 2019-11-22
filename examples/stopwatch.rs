// Draw a stopwatch with changed draw and update rates and disabled vsync
extern crate quicksilver;

use quicksilver::{
    geom::{Circle, Line, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{run, Settings, State, Window},
    Result,
};

struct Stopwatch {
    elapsed: f64,
    hours: f64,
    minutes: f64,
    seconds: f64,
}

impl State for Stopwatch {
    type Message = quicksilver::lifecycle::Event;

    fn new() -> Result<Stopwatch> {
        Ok(Stopwatch {
            elapsed: 0.,
            hours: 0.,
            minutes: 0.,
            seconds: 0.,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.elapsed += window.update_rate();
        self.seconds = (self.elapsed / 1000.) % 60.;
        self.minutes = ((self.elapsed / 1000.) / 60.) % 60.;
        self.hours = ((self.elapsed / 1000.) / 60. / 24.) % 24.;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // clear everything
        window.clear(Color::WHITE)?;

        // draw the frame
        window.draw(&Circle::new((400, 300), 203), Col(Color::BLACK));
        window.draw(&Circle::new((400, 300), 200), Col(Color::WHITE));

        // draw the hour markers
        for i in 1..=12 {
            let angle = 360. * ((i as f64 + 9.) * 2. / 24.);
            let pos = Vector::from_angle(angle as f32) * 200. + Vector::new(400, 300);
            let line = Line::new((400, 300), pos).with_thickness(5);
            window.draw(&line, Col(Color::BLACK));
        }

        window.draw(&Circle::new((400, 300), 180), Col(Color::WHITE));

        let hour_angle = 360. * ((self.hours + 9.) * 2. / 24.);
        let minute_angle = 360. * ((self.minutes + 45.) / 60.);
        let second_angle = 360. * ((self.seconds + 45.) / 60.);

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
    run::<Stopwatch>(
        "Stopwatch",
        Vector::new(800, 600),
        Settings {
            draw_rate: 1000. / 10., // 10 FPS are enough
            update_rate: 1000.,     // every second to make it appear like a clock
            vsync: false,           // don't use VSync, we're limiting to 10 FPS on our own
            ..Settings::default()
        },
    );
}
