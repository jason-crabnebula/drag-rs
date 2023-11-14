// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use drag::{set_drag_source, start_drag, DragItem, Image};
use tao::{
    dpi::LogicalSize,
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android"
)))]
use tao::platform::unix::WindowExtUnix;

enum UserEvent {
    StartDrag,
}

fn main() -> wry::Result<()> {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(400., 100.))
        .with_title("Drag Example")
        .build(&event_loop)
        .unwrap();

    const HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <style>
      #drag {
        border:2px solid black;
        border-radius:3px;
        width: 100%;
        height: calc(100vh - 20px);
        display: flex;
        align-items: center;
        justify-content: center;
        -webkit-user-select: none;
      }
    </style>
  </head>

  <body>
    <div draggable="true" id="drag">
      Drag me
    </div>
    <script type="text/javascript">
      document.getElementById('drag').ondragstart = (event) => {
        event.preventDefault();
        window.ipc.postMessage('startDrag');
      };
    </script>
  </body>
</html>
  "#;

    let proxy = event_loop.create_proxy();
    let handler = move |req: String| match req.as_str() {
        "startDrag" => {
            let _ = proxy.send_event(UserEvent::StartDrag);
        }
        _ => {}
    };

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let builder = WebViewBuilder::new(&window);

    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let builder = {
        use wry::WebViewBuilderExtUnix;

        let vbox = window.default_vbox().unwrap();
        WebViewBuilder::new_gtk(vbox)
    };

    let _webview = builder
        .with_html(HTML)?
        .with_ipc_handler(handler)
        .with_accept_first_mouse(true)
        .build()?;

    set_drag_source();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry application started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::UserEvent(e) => match e {
                UserEvent::StartDrag => {
                    let window = &window;

                    #[cfg(feature = "gtk")]
                    let window = window.gtk_window();

                    start_drag(
                        &window,
                        DragItem::Files(vec![std::path::PathBuf::from(
                            std::fs::canonicalize("examples/icon.png").unwrap(),
                        )]),
                        // Image::Raw(include_bytes!("../examples/icon.png").to_vec()),
                        Image::File("examples/icon.png".into()),
                    );
                }
            },
            _ => (),
        }
    });
}
