use std::ptr;
use std::ffi::CString;
use std::rc::Rc;

use libc::{c_int, c_uint};
use std::os::raw::c_void;
use x11::{xlib, glx};
use gl;
use image;

use error;

pub struct Display {
	context: Rc<gl::backend::Context>,
	backend: Rc<Backend>,
}

pub struct Backend {
	display: *mut xlib::Display,
	root:    xlib::Window,
	context: glx::GLXContext,
	window:  xlib::Window,

	width:  u32,
	height: u32,
}

impl Display {
	pub fn open<N: AsRef<str>>(name: N, screen: i32, id: u64) -> error::Result<Display> {
		unsafe {
			let name    = CString::new(name.as_ref().as_bytes()).unwrap();
			let display = xlib::XOpenDisplay(name.as_ptr()).as_mut().ok_or(error::Display::NotFound)?;
			let root    = xlib::XRootWindow(display, screen);
			let width   = xlib::XDisplayWidth(display, screen) as c_uint;
			let height  = xlib::XDisplayHeight(display, screen) as c_uint;

			let info = glx::glXChooseVisual(display, screen,
				[glx::GLX_RGBA, glx::GLX_DEPTH_SIZE, 24, glx::GLX_DOUBLEBUFFER, 0].as_ptr() as *mut _)
					.as_mut().ok_or(error::Display::NoVisual)?;

			let context = glx::glXCreateContext(display, info, ptr::null_mut(), 1)
				.as_mut().ok_or(error::Display::NoContext)?;

			let backend = Rc::new(Backend {
				display: display,
				root:    root,
				context: context,
				window:  id,

				width:  width,
				height: height,
			});

			Ok(Display {
				backend: backend.clone(),
				context: gl::backend::Context::new(backend.clone(), false, Default::default())?,
			})
		}
	}

	pub fn context(&self) -> Rc<gl::backend::Context> {
		self.context.clone()
	}

	pub fn draw(&self) -> gl::Frame {
		gl::Frame::new(self.context.clone(), self.context.get_framebuffer_dimensions())
	}

	pub fn screenshot(&self) -> image::DynamicImage {
		unsafe {
			let ximage = xlib::XGetImage(self.backend.display, self.backend.root,
				0, 0, self.backend.width, self.backend.height,
				xlib::XAllPlanes(), xlib::ZPixmap)
					.as_mut().unwrap();

			let r = (*ximage).red_mask;
			let g = (*ximage).green_mask;
			let b = (*ximage).blue_mask;

			let mut image = image::DynamicImage::new_rgb8(self.backend.width, self.backend.height);

			for (x, y, px) in image.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
				let pixel = xlib::XGetPixel(ximage, x as c_int, y as c_int);

				px[0] = ((pixel & r) >> 16) as u8;
				px[1] = ((pixel & g) >> 8)  as u8;
				px[2] = ((pixel & b) >> 0)  as u8;
			}

			xlib::XDestroyImage(ximage);

			image
		}
	}
}

unsafe impl gl::backend::Backend for Backend {
	fn swap_buffers(&self) -> Result<(), gl::SwapBuffersError> {
		unsafe {
			glx::glXSwapBuffers(self.display, self.window);
		}

		Ok(())
	}

	unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
		let addr = CString::new(symbol.as_bytes()).unwrap();

		if let Some(func) = glx::glXGetProcAddress(addr.as_ptr() as *const _) {
			func as *const _
		}
		else {
			ptr::null()
		}
	}

	fn get_framebuffer_dimensions(&self) -> (u32, u32) {
		(self.width, self.height)
	}

	fn is_current(&self) -> bool {
		unimplemented!()
	}

	unsafe fn make_current(&self) {
		glx::glXMakeCurrent(self.display, self.window, self.context);
	}
}

impl Drop for Backend {
	fn drop(&mut self) {
		unsafe {
			glx::glXDestroyContext(self.display, self.context);
			xlib::XCloseDisplay(self.display);
		}
	}
}
