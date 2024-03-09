pub trait Particle: Send + Sync {
    fn update(&mut self) -> ParticleMessage;
    fn remove(&mut self);
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParticleMessage {
    None,
    Done,
}
