#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("cimgui.rs");

#[cfg(test)]
mod tests {

    use crate::*;
    extern crate winit;
	use std::ffi::CString;

    #[test]
    fn start_cimgui() {
      unsafe {
        let ig_context = igCreateContext( std::ptr::null_mut());

		let io = igGetIO();

		let mut pixels = std::ptr::null_mut();
		let mut width = 0;
		let mut height = 0;
		let mut pixel_size = 0;
		ImFontAtlas_GetTexDataAsRGBA32((*io).Fonts, &mut pixels, &mut width, &mut height, &mut pixel_size);

        //TODO: Winit init
        //TODO: GFX-HAL init

        igStyleColorsDark(std::ptr::null_mut());

        let mut events_loop = winit::EventsLoop::new();
        let _window = winit::Window::new(&events_loop).unwrap();

		let mut test_float = 0.0f32;
		let mut show_demo_window = true;
		let mut running = true;

		while running {

			events_loop.poll_events( |event| {
				match event {
					winit::Event::WindowEvent {
						event: winit::WindowEvent::CloseRequested,
						..
					} => { running = false; },
					_ => { },
				}
			});

			//TODO: imgui winit new frame
			let mut io = igGetIO();
			(*io).DisplaySize = ImVec2{ x: 400.0f32, y: 400.0f32};
			(*io).DeltaTime = 1.0f32 / 60.0f32;
			
			igNewFrame();
			
			igText(CString::new("Hello, world!").unwrap().as_ptr());
			igSliderFloat(CString::new("test float").unwrap().as_ptr(), &mut test_float, 0.0f32, 1.0f32, std::ptr::null(), 1.0f32);

			igShowDemoWindow(&mut show_demo_window);

			igEndFrame();
		}

		igDestroyContext(ig_context);
      }
    }
}