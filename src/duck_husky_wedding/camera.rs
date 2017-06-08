use glm;

struct ViewPort {
    dims: glm::IVec2,
    translation: glm::IVec2,
}

impl ViewPort {
    fn new(dims: glm::IVec2) -> ViewPort {
        let translation = glm::ivec2(0, 0);
        ViewPort { dims, translation }
    }

    fn translate(&mut self, t: glm::IVec2) {
        self.translation = self.translation + t;
    }

    fn center(&mut self, center: glm::IVec2) {
        self.translation = self.dims / 2 + self.translation - center;
    }
}
