mod checkerboard;

use floem::peniko::Color;
use floem::reactive::{create_rw_signal, RwSignal};
use floem::view::View;
use floem::views::{h_stack, v_stack, Decorators, scroll, label, container};
use floem::unit::UnitExt;
use floem::views::scroll::ScrollState;
use crate::checkerboard::{checkerboard, CheckerboardDisplayMode};

pub const COLUMN_WIDTH: f32 = 80.0;
pub const ROW_HEIGHT: f32 = 20.0;
pub fn top_left(left_column_width: RwSignal<f64>, header_height: RwSignal<f64>) -> impl View {
    let inner = label(|| "TopLeft".to_string())
      .style(move |s| s
        .flex()
      );

    let result = container(inner)
      .style(move |s| s
        .height(header_height.get())
        .width(left_column_width.get())
        .background(Color::LIGHT_YELLOW)
      );
    result

}

pub fn left_gutter(left_column_width: RwSignal<f64>,
                   main_height: RwSignal<f64>,
                   list_height: RwSignal<f64>
) -> impl View {
    let inner = checkerboard(CheckerboardDisplayMode::Rows(ROW_HEIGHT))
      .style( move |s| s
        .width(left_column_width.get())
        .height(list_height.get())
      );

    let result = container(inner)
      .style(move |s| s
        .height(list_height.get())
        .width(left_column_width.get())
        .background(Color::LIGHT_BLUE)
      );
    result

}

pub fn header(right_column_width: RwSignal<f64>,
              header_height: RwSignal<f64>,
              list_width: RwSignal<f64>) -> impl View {
    let inner = checkerboard(CheckerboardDisplayMode::Columns(COLUMN_WIDTH))
      .style( move |s|
        s.width(list_width.get())
      );
    let result = container(inner)
      .style(move |s| s
        .height(header_height.get())
        .width(list_width.get())
        .background(Color::LIGHT_GREEN)
      );
    result
}

pub fn main_grid(right_column_width: RwSignal<f64>,
                 main_height: RwSignal<f64>,
                 list_width: RwSignal<f64>,
                 list_height: RwSignal<f64>
) -> impl View {
    let inner = checkerboard(CheckerboardDisplayMode::Both(COLUMN_WIDTH, ROW_HEIGHT))
      .style( move |s|
        s.width(list_width.get())
          .height(list_height.get())
      );

    let result = container(inner)
      .style(move |s| s
        .height(list_height.get())
        .width(list_width.get())
        .background(Color::LIGHT_GRAY)
      );
    result

}

pub(crate) fn app_view() -> impl View {
    let header_height = create_rw_signal(50.0);
    let main_height = create_rw_signal(450.0);
    let left_column_width = create_rw_signal(90.0);
    let right_column_width = create_rw_signal(510.0);
    let list_width = create_rw_signal(2000.0);
    let list_height = create_rw_signal(5000.0);

    let top_left = top_left(left_column_width.clone(), header_height.clone());
    let left_gutter = scroll(left_gutter(left_column_width.clone(), main_height.clone(), list_height.clone()))
      .hide_bar(|| true)
      .style(move |s| s.flex()
        .width(left_column_width.get())
        .height(main_height.get())
      );
    let header = scroll(header(right_column_width.clone(), header_height.clone(), list_width.clone()))
      .hide_bar(|| true);
    let header_id = header.id();
    let left_gutter_id = left_gutter.id();
    let scroll_handle_color = Color::DARK_CYAN;
    let scroll_handle_active_color = Color::BLUE;

    let main_grid = scroll(main_grid(right_column_width.clone(), main_height.clone(), list_width.clone(), list_height.clone()))
      .style(move |s| s.flex()
        .width(right_column_width.get())
        .height(main_height.get())
        .border(1.0)
        .border_color(Color::BLACK)
        .class(scroll::Handle, |s| {
            s.border_radius(4.0)
              // .background(Color::rgba8(166, 166, 166, 140))
              .background(scroll_handle_color)
              .set(scroll::Thickness, 12.0)
              .set(scroll::Rounded, true)
              .active(|s| s.background(scroll_handle_active_color))
              .hover(|s| s.background(scroll_handle_active_color))
        })

      )
      .on_scroll(move |r| {
          // println!("Scroll: {:?}", r);
          // scroll_x.set(r.x0);
          // scroll_y.set(r.y0);
          header_id.update_state(ScrollState::ScrollTo(((r.x0), (0.0)).into()), false);
          left_gutter_id.update_state(ScrollState::ScrollTo(((0.0), (r.y0)).into()), false);
      })

      ;

    let left_column = v_stack((top_left, left_gutter))
      .style(move |s| s.width(left_column_width.get()));

    let right_column = v_stack((header, main_grid))
      .style(move |s| s.width(right_column_width.get()));
    let root = h_stack((left_column, right_column))
      .style(move |s| s.width_full()
        .height_full()
      );
    // root.id().inspect();
    root
}
fn main() {
    floem::launch(app_view);
}
