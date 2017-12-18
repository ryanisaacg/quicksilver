extern "C" {
    pub fn start_image_load();
    pub fn add_image_path_char(c: char);
    pub fn end_image_load() -> bool;
    pub fn get_image_id() -> u32;
    pub fn get_image_width() -> i32;
    pub fn get_image_height() -> i32;
}
