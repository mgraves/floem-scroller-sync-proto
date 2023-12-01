use floem::id::Id;
use floem::{kurbo, prop, prop_extracter, Renderer, style_class, taffy};
use floem::kurbo::{Rect, Size};
use floem::peniko::Color;
use floem::style::Style;
use floem::view::{View, ViewData};
use floem::views::Decorators;
use floem::widgets::slider::SliderClass;
use floem::style::{Background, Foreground, BorderRadius};

prop_extracter! {
    CheckerboardStyle {
        foreground: Foreground,
        background: Background,
        border_radius: BorderRadius,
    }
}


style_class!(pub CheckerboardClass);

pub enum CheckerboardDisplayMode {
  Both(f32, f32), // (cell_width, cell_height)
  Columns(f32), // (cell_width)
  Rows(f32), // (cell_height)
}
pub struct Checkerboard {
  data: ViewData,
  display_mode: CheckerboardDisplayMode,
  size: taffy::prelude::Size<f32>,
  style: CheckerboardStyle,

}

pub fn checkerboard(display_mode: CheckerboardDisplayMode) -> Checkerboard {
  let id = Id::next();

  Checkerboard {
    data: ViewData::new(id),
    display_mode,
    size: Default::default(),
    style: Default::default(),
  }
    .class(CheckerboardClass)
}

fn generate_rectangles(board_width: f32, board_height: f32, mode: &CheckerboardDisplayMode) -> Vec<Rect> {
  let mut rectangles = Vec::new();

  match mode {
    CheckerboardDisplayMode::Both(rect_width, rect_height) => {
      let mut y = 0.0;
      while y < board_height {
        let mut x = 0.0;
        while x < board_width {
          if ((x / rect_width) as i32 + (y / rect_height) as i32) % 2 == 0 {
            rectangles.push(Rect {
              x0: x as f64,
              y0: y as f64,
              x1: (x + rect_width ) as f64,
              y1: (y + rect_height) as f64,
            });
          }
          x += rect_width;
        }
        y += rect_height;
      }
    },
    CheckerboardDisplayMode::Columns(rect_width) => {
      let mut x = 0.0;
      while x < board_width {
        if (x / rect_width) as i32 % 2 == 0 {
          rectangles.push(Rect {
            x0: x as f64,
            y0: 0.0 as f64,
            x1: (x + rect_width) as f64,
            y1: board_height as f64
          });
        }
        x += rect_width;
      }
    },
    CheckerboardDisplayMode::Rows(rect_height) => {
      let mut y = 0.0;
      while y < board_height {
        if (y / rect_height) as i32 % 2 == 0 {
          rectangles.push(Rect {
            x0: 0.0 as f64,
            y0: y as f64,
            x1: board_width as f64,
            y1: (y + rect_height) as f64,
          });
        }
        y += rect_height;
      }
    },
  }

  rectangles
}

impl View for Checkerboard {
  fn view_data(&self) -> &ViewData {
    &self.data
  }

  fn view_data_mut(&mut self) -> &mut ViewData {
    &mut self.data
  }

  fn compute_layout(&mut self, cx: &mut floem::context::ComputeLayoutCx) -> Option<kurbo::Rect> {
    let layout = cx.get_layout(self.id()).unwrap();

    self.size = layout.size;
    None
  }
  fn paint(&mut self, cx: &mut floem::context::PaintCx) {
    let size = cx
      .get_layout(self.id())
      .map(|layout| Size::new(layout.size.width as f64, layout.size.height as f64))
      .unwrap_or_default();
    cx.clip(&size.to_rect());

    let rectangles = generate_rectangles(self.size.width, self.size.height, &self.display_mode);
    for rect in rectangles {
      cx.fill(
        &rect,
        Color::DARK_GRAY,
        0.,
      );
    }
  }

}