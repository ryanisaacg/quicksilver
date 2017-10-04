use geom::{Circle, Rectangle, Vector, Transform};
use graphics::{BridgeFront, Camera, Color, Drawable, TextureRegion};

pub struct Frontend<'a> {
    sender: BridgeFront<'a>,
    cam: Camera,
    ui_mode: bool
}

impl<'a> Frontend<'a> {
    pub fn new(sender: BridgeFront<'a>, cam: Camera) -> Frontend<'a> {
        Frontend {
            sender: sender, 
            cam: cam,
            ui_mode: false
        }
    }

    pub fn set_camera(&mut self, cam: Camera) {
        self.cam = cam;
    }

    pub fn get_ui_mode(&self) -> bool {
        self.ui_mode
    }

    pub fn set_ui_mode(&mut self, ui_mode: bool) {
        self.ui_mode = ui_mode;
    }

    fn camera(&self) -> Transform {
        self.cam.opengl
    }

    pub fn draw_image(&self, image: TextureRegion<'a>, area: Rectangle, trans: Transform, col: Color) {
        let trans = self.camera()
            * trans 
            * Transform::translate(area.top_left()) 
            * Transform::scale(area.size());
        let call = (Drawable::Image(image), trans, col);
        self.sender.send(call).unwrap();
    }

    pub fn draw_rect(&self, rect: Rectangle, trans: Transform, col: Color) {
        let trans = self.camera()
            * trans;
        let call = (Drawable::Rect(rect), trans, col);
        self.sender.send(call).unwrap();
    }

    pub fn draw_circle(&self, circ: Circle, trans: Transform, col: Color) {
        let trans = self.camera()
            * trans;
        let call = (Drawable::Circ(circ), trans, col);
        self.sender.send(call).unwrap();
    }
}

