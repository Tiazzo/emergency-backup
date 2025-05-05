#[derive(Debug)]
pub enum Status {
    InsideCurrentArea,
    InsideNextArea,
    ActivationShapeCompleted,
    ConfirmationShapeCompleted,
    OutsideBoundaries,
}

#[derive(Copy, Clone, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct Area {
    pub top_left: Coordinate,
    pub bottom_right: Coordinate,
}

impl Area {
    pub fn new(start: Coordinate, end: Coordinate) -> Self {
        Area { top_left: start, bottom_right: end }
    }

    pub fn contains(&self, point: &Coordinate) -> bool {
        point.x >= self.top_left.x && point.x <= self.bottom_right.x &&
        point.y >= self.top_left.y && point.y <= self.bottom_right.y
    }
}

pub struct Tracker {
    shape: u8,
    screen_width: i32,
    screen_height: i32,
    path: Vec<Area>,
    position: usize,
}

impl Tracker {
    pub fn new(width: i32, height: i32) -> Self {
        Tracker {
            shape: 0,
            screen_width: width,
            screen_height: height,
            path: generate_path(width, height, 0),
            position: 0,
        }
    }

    // Updates the tracker based on mouse position
    pub fn update(&mut self, point: Coordinate) -> Status {
        let current_area = &self.path[self.position];
        
        if current_area.contains(&point) {
            Status::InsideCurrentArea
        } else if self.position < self.path.len() - 1 && self.path[self.position + 1].contains(&point) {
            self.position += 1;
            if self.position == self.path.len() - 1 {
                self.handle_shape_completion()
            } else {
                Status::InsideNextArea
            }
        } else {
            self.position = 0;
            Status::OutsideBoundaries
        }
    }

    // Handles completion of shapes and switches modes
    fn handle_shape_completion(&mut self) -> Status {
        if self.shape == 0 {
            self.shape = 1;
            self.position = 0;
            self.path = generate_path(self.screen_width, self.screen_height, 1);
            Status::ActivationShapeCompleted
        } else {
            self.shape = 0;
            self.position = 0;
            self.path = generate_path(self.screen_width, self.screen_height, 0);
            Status::ConfirmationShapeCompleted
        }
    }
}

// General function to generate paths
fn generate_path(w: i32, h: i32, shape: u8) -> Vec<Area> {
    let mut path: Vec<Area> = Vec::<Area>::new();

    let min_cells = 8;
    let cell_size = h / min_cells;
    let extra_cells = (w - h) / cell_size;
    let mut i = 0;

    if shape == 0 {

        // vertical, left line
        while i < min_cells - 1 {
            path.push(Area {
                top_left: Coordinate { x: -100, y: i * cell_size },
                bottom_right: Coordinate { x: cell_size, y: (i + 1) * cell_size },
            });
            i += 1;
        }
        path.push(Area {
            top_left: Coordinate { x: -100, y: i * cell_size },
            bottom_right: Coordinate { x: cell_size, y: h + 100},
        });

        // horizontal, bottom line
        i = 1;
        while i < min_cells + extra_cells - 1 {
            path.push(Area {
                top_left: Coordinate { x: i * cell_size, y: h - cell_size },
                bottom_right: Coordinate { x: (i + 1) * cell_size, y: h + 100 },
            });
            i += 1;
        }
        path.push(Area {
            top_left: Coordinate { x: i * cell_size, y: h - cell_size },
            bottom_right: Coordinate { x: w + 100, y: h + 100 },
        });

        // vertical, right line
        i = 1;
        while i < min_cells - 1 {
            path.push(Area {
                top_left: Coordinate { x: w - cell_size, y: h - (i + 1) * cell_size },
                bottom_right: Coordinate { x: w + 100, y: h - i * cell_size },
            });
            i += 1;
        }
        path.push(Area {
            top_left: Coordinate { x: w - cell_size, y: -100 },
            bottom_right: Coordinate { x: w + 100, y: h - i * cell_size },
        });

        // horizontal, top line
        i = 1;
        while i < min_cells + extra_cells - 2 {
            path.push(Area {
                top_left: Coordinate { x: w - (i + 1) * cell_size, y: -100 },
                bottom_right: Coordinate { x: w - i * cell_size, y: cell_size },
            });
            i += 1;
        }
        path.push(Area {
            top_left: Coordinate { x: cell_size, y: -100 },
            bottom_right: Coordinate { x: w - i * cell_size, y: cell_size },
        });

    } else {
        // Generates a horizontal line shape path
        while i < min_cells + extra_cells - 1 {
            path.push(Area {
                top_left: Coordinate { x: i * cell_size, y: h / 2 - 2 * cell_size },
                bottom_right: Coordinate { x: (i + 1) * cell_size, y: h / 2 + 2 * cell_size },
            });
            i += 1;
        }
        path.push(Area {
            top_left: Coordinate { x: i * cell_size, y: h / 2 - 2 * cell_size },
            bottom_right: Coordinate { x: w, y: h / 2 + 2 * cell_size },
        });
    }
    path

}