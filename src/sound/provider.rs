pub trait AudioProvider {
    fn new() -> Self where Self: Sized;
    fn play(&mut self, data: SoundData, volume);
    fn get_generation(&self) -> u32;
    fn play(&mut self);
    fn pause(&mut self);
    fn is_paused(&self) -> bool;
}
