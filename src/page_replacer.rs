use crate::config::{FrameId};

pub trait PageReplacer {
    fn victim(&self) -> Option<FrameId>;
    fn pin(&self, id: FrameId);
    fn unpin(&self, id: FrameId);
    fn size(&self) -> usize;
}
