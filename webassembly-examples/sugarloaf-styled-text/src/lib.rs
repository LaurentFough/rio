use sugarloaf::Sugarloaf;
use sugarloaf::{
    primitives::{Sugar, SugarStack},
    layout::SugarloafLayout,
};
use sugarloaf::primitives::style::{SugarDecoration, SugarStyle};
use wasm_bindgen::prelude::*;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowBuilderExtWebSys;

async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let width = 600.0;
    let height = 400.0;

    #[cfg(target_arch = "wasm32")]
    let canvas_element = {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.get_element_by_id("sugarloaf_canvas"))
            .and_then(|element| element.dyn_into::<HtmlCanvasElement>().ok())
            .expect("Get canvas element")
    };

    #[cfg(target_arch = "wasm32")]
    let window = winit::window::WindowBuilder::new()
        .with_title("sugarloaf-wasm")
        .with_inner_size(winit::dpi::LogicalSize::new(width, height))
        .with_resizable(false)
        .with_canvas(Some(canvas_element))
        .build(&event_loop)
        .unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    let window = winit::window::Window::new(&event_loop).unwrap();

    let scale_factor = window.scale_factor();
    let font_size = 60.;
    let line_height = 1.0;

    let sugarloaf_layout = SugarloafLayout::new(
        width as f32,
        height as f32,
        (10.0, 10.0, 0.0),
        scale_factor as f32,
        font_size,
        line_height,
        (2, 1),
    );

    let mut sugarloaf = match Sugarloaf::new(
        &window,
        wgpu::PowerPreference::HighPerformance,
        sugarloaf::font::fonts::SugarloafFonts::default(),
        sugarloaf_layout,
        None,
    )
    .await
    {
        Ok(instance) => instance,
        Err(instance_with_errors) => instance_with_errors.instance,
    };

    log::info!("started scale_factor: {scale_factor:?}");

    sugarloaf.calculate_bounds();

    let _ = event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        let mut sugar = SugarStack::new();
        sugar.add(Sugar {
            content: 'u',
            foreground_color: [1.0, 1.0, 1.0, 1.0],
            background_color: [0.0, 0.0, 0.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 58.),
                size: (1.0, 0.050),
                color: [0.0, 0.0, 0.0, 1.0],
            }),
        });
        sugar.add(Sugar {
            content: 'n',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 1.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 58.),
                size: (1.0, 0.025),
                color: [0.0, 0.0, 0.0, 1.0],
            }),
        });
        sugar.add(Sugar {
            content: 'd',
            foreground_color: [1.0, 1.0, 1.0, 1.0],
            background_color: [0.0, 0.0, 0.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 58.),
                size: (1.0, 0.025),
                color: [0.0, 0.0, 0.0, 1.0],
            }),
        });
        sugar.add(Sugar {
            content: 'e',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 1.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 58.),
                size: (1.0, 0.025),
                color: [0.0, 0.0, 0.0, 1.0],
            }),
        });
        sugar.add(Sugar {
            content: 'r',
            foreground_color: [1.0, 1.0, 1.0, 1.0],
            background_color: [0.0, 0.0, 0.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 58.),
                size: (1.0, 0.025),
                color: [0.0, 0.0, 0.0, 1.0],
            }),
        });
        sugar.add(Sugar {
            content: 'l',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [0.0, 0.0, 1.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 58.),
                size: (1.0, 0.025),
                color: [0.0, 0.0, 0.0, 1.0],
            }),
        });
        sugar.add(Sugar {
            content: '!',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [0.0, 0.0, 1.0, 1.0],
            style: None,
            decoration: None,
        });
        sugar.add(Sugar {
            content: 'i',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 1.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 58.),
                size: (1.0, 0.025),
                color: [0.0, 0.0, 0.0, 1.0],
            }),
        });
        sugar.add(Sugar {
            content: 'n',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 1.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 58.),
                size: (1.0, 0.025),
                color: [0.0, 0.0, 0.0, 1.0],
            }),
        });
        sugar.add(Sugar {
            content: 'e',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 1.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 58.),
                size: (1.0, 0.025),
                color: [0.0, 0.0, 0.0, 1.0],
            }),
        });
        sugar.add(Sugar {
            content: ' ',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 1.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 58.),
                size: (1.0, 0.025),
                color: [0.0, 0.0, 0.0, 1.0],
            }),
        });
        sugar.add(Sugar {
            content: ' ',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 1.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 58.),
                size: (1.0, 0.025),
                color: [0.0, 0.0, 0.0, 1.0],
            }),
        });
        sugar.add(Sugar {
            content: ' ',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 1.0, 1.0],
            style: None,
            decoration: None,
        });

        let mut italic_and_bold = SugarStack::new();
        italic_and_bold.add(Sugar {
            content: 'i',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 1.0, 1.0],
            style: Some(SugarStyle {
                is_italic: true,
                is_bold_italic: false,
                is_bold: false,
            }),
            decoration: None,
        });
        italic_and_bold.add(Sugar {
            content: 't',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 1.0, 1.0],
            style: Some(SugarStyle {
                is_italic: true,
                is_bold_italic: false,
                is_bold: false,
            }),
            decoration: None,
        });
        italic_and_bold.add(Sugar {
            content: 'a',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 1.0, 1.0],
            style: Some(SugarStyle {
                is_italic: true,
                is_bold_italic: false,
                is_bold: false,
            }),
            decoration: None,
        });
        italic_and_bold.add(Sugar {
            content: 'l',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 1.0, 1.0],
            style: Some(SugarStyle {
                is_italic: true,
                is_bold_italic: false,
                is_bold: false,
            }),
            decoration: None,
        });
        italic_and_bold.add(Sugar {
            content: 'i',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 1.0, 1.0],
            style: Some(SugarStyle {
                is_italic: true,
                is_bold_italic: false,
                is_bold: false,
            }),
            decoration: None,
        });
        italic_and_bold.add(Sugar {
            content: 'c',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 1.0, 1.0],
            style: Some(SugarStyle {
                is_italic: true,
                is_bold_italic: false,
                is_bold: false,
            }),
            decoration: None,
        });
        italic_and_bold.add(Sugar {
            content: ' ',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [0.5, 0.5, 1.0, 1.0],
            style: Some(SugarStyle {
                is_italic: false,
                is_bold_italic: false,
                is_bold: true,
            }),
            decoration: None,
        });
        italic_and_bold.add(Sugar {
            content: 'b',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 0.3, 1.0],
            style: Some(SugarStyle {
                is_italic: false,
                is_bold_italic: false,
                is_bold: true,
            }),
            decoration: None,
        });
        italic_and_bold.add(Sugar {
            content: 'o',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 0.3, 1.0],
            style: Some(SugarStyle {
                is_italic: false,
                is_bold_italic: false,
                is_bold: true,
            }),
            decoration: None,
        });
        italic_and_bold.add(Sugar {
            content: 'l',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 0.3, 1.0],
            style: Some(SugarStyle {
                is_italic: false,
                is_bold_italic: false,
                is_bold: true,
            }),
            decoration: None,
        });
        italic_and_bold.add(Sugar {
            content: 'd',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 0.3, 1.0],
            style: Some(SugarStyle {
                is_italic: false,
                is_bold_italic: false,
                is_bold: true,
            }),
            decoration: None,
        });

        let mut rio = SugarStack::new();
        rio.add(Sugar {
            content: 'r',
            foreground_color: [1.0, 1.0, 1.0, 1.0],
            background_color: [0.0, 0.0, 0.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 58.),
                size: (1.0, 0.05),
                color: [0.0, 0.0, 0.0, 1.0],
            }),
        });
        rio.add(Sugar {
            content: 'e',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [0.0, 0.0, 1.0, 1.0],
            style: None,
            decoration: None,
        });
        rio.add(Sugar {
            content: 'g',
            foreground_color: [1.0, 1.0, 1.0, 1.0],
            background_color: [0.0, 0.0, 0.0, 1.0],
            style: None,
            decoration: None,
        });
        rio.add(Sugar {
            content: 'u',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 1.0, 1.0],
            style: None,
            decoration: None,
        });
        rio.add(Sugar {
            content: 'l',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [0.0, 1.0, 0.0, 1.0],
            style: None,
            decoration: None,
        });
        rio.add(Sugar {
            content: 'a',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [1.0, 1.0, 0.0, 1.0],
            style: None,
            decoration: None,
        });
        rio.add(Sugar {
            content: 'r',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [0.0, 1.0, 0.0, 1.0],
            style: None,
            decoration: None,
        });

        let mut strike = SugarStack::new();
        strike.add(Sugar {
            content: 's',
            foreground_color: [1.0, 1.0, 1.0, 1.0],
            background_color: [0.0, 0.0, 0.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 30.),
                size: (1.0, 0.025),
                color: [0.5, 0.5, 0.0, 1.0],
            }),
        });
        strike.add(Sugar {
            content: 't',
            foreground_color: [1.0, 1.0, 1.0, 1.0],
            background_color: [0.0, 0.0, 0.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 30.),
                size: (1.0, 0.025),
                color: [0.5, 0.5, 0.0, 1.0],
            }),
        });
        strike.add(Sugar {
            content: 'r',
            foreground_color: [1.0, 1.0, 1.0, 1.0],
            background_color: [0.0, 0.0, 0.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 30.),
                size: (1.0, 0.025),
                color: [0.5, 0.5, 0.0, 1.0],
            }),
        });
        strike.add(Sugar {
            content: 'i',
            foreground_color: [1.0, 1.0, 1.0, 1.0],
            background_color: [0.0, 0.0, 0.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 30.),
                size: (1.0, 0.025),
                color: [0.5, 0.5, 0.0, 1.0],
            }),
        });
        strike.add(Sugar {
            content: 'k',
            foreground_color: [1.0, 1.0, 1.0, 1.0],
            background_color: [0.0, 0.0, 0.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 30.),
                size: (1.0, 0.025),
                color: [0.5, 0.5, 0.0, 1.0],
            }),
        });
        strike.add(Sugar {
            content: 'e',
            foreground_color: [1.0, 1.0, 1.0, 1.0],
            background_color: [0.0, 0.0, 0.0, 1.0],
            style: None,
            decoration: Some(SugarDecoration {
                relative_position: (0.0, 30.),
                size: (1.0, 0.025),
                color: [0.5, 0.5, 0.0, 1.0],
            }),
        });

        let block = Some(SugarDecoration {
            relative_position: (0.0, 0.0),
            size: (1.0, 1.0),
            color: [1.0, 0.4, 1.0, 1.0],
        });

        let underline = Some(SugarDecoration {
            relative_position: (0.0, 58.),
            size: (1.0, 0.05),
            color: [1.0, 0.4, 1.0, 1.0],
        });

        let beam = Some(SugarDecoration {
            relative_position: (0.0, 0.0),
            size: (0.1, 1.0),
            color: [1.0, 0.4, 1.0, 1.0],
        });

        let mut cursors = SugarStack::new();
        cursors.add(Sugar {
            content: ' ',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [0.0, 1.0, 1.0, 1.0],
            style: None,
            decoration: block,
        });
        cursors.add(Sugar {
            content: ' ',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [0.0, 1.0, 1.0, 1.0],
            style: None,
            decoration: None,
        });
        cursors.add(Sugar {
            content: ' ',
            foreground_color: [1.0, 1.0, 1.0, 1.0],
            background_color: [0.0, 0.0, 0.0, 1.0],
            style: None,
            decoration: underline,
        });
        cursors.add(Sugar {
            content: ' ',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [0.0, 1.0, 1.0, 1.0],
            style: None,
            decoration: None,
        });
        cursors.add(Sugar {
            content: ' ',
            foreground_color: [0.0, 0.0, 0.0, 1.0],
            background_color: [0.0, 1.0, 1.0, 1.0],
            style: None,
            decoration: beam,
        });

        match event {
            Event::Resumed => {
                sugarloaf
                    .set_background_color(wgpu::Color::RED)
                    .calculate_bounds();
                window.request_redraw();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                    log::info!("changed scale_factor: {scale_factor:?}");

                    let scale_factor_f32 = scale_factor as f32;
                    sugarloaf.rescale(scale_factor_f32).calculate_bounds();
                    window.request_redraw();
                }
                _ => (),
            },
            Event::RedrawRequested { .. } => {
                sugarloaf.stack(sugar);
                sugarloaf.stack(italic_and_bold);
                sugarloaf.stack(rio);
                sugarloaf.stack(strike);
                sugarloaf.stack(cursors);
                sugarloaf.render();
            }
            _ => {
                *control_flow = ControlFlow::Wait;
            }
        }
    });
}

#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().expect("could not initialize logger");
    wasm_bindgen_futures::spawn_local(run());
}
