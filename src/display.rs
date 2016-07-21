//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.use std::io::{Read, BufRead, BufReader, Write};

use std::ptr;
use std::ffi::CString;
use std::rc::Rc;
use std::cell::Cell;

use libc::c_int;
use std::os::raw::c_void;
use x11::{xlib, glx};
use gl;
use image;

use error;

pub struct Display {
	context: Rc<gl::backend::Context>,
	backend: Rc<Backend>,
}

#[derive(Debug)]
pub struct Backend {
	display: *mut xlib::Display,
	root:    xlib::Window,
	context: glx::GLXContext,
	id:      xlib::Window,

	screen: Cell<(u32, u32)>,
	window: Cell<(u32, u32)>,
}

impl Display {
	/// Open the matching Display.
	pub fn open<N: AsRef<str>>(name: N, screen: i32, id: u64) -> error::Result<Display> {
		unsafe {
			let name    = CString::new(name.as_ref().as_bytes()).unwrap();
			let display = xlib::XOpenDisplay(name.as_ptr()).as_mut().ok_or(error::Display::NotFound)?;
			let root    = xlib::XRootWindow(display, screen);

			let info = glx::glXChooseVisual(display, screen,
				[glx::GLX_RGBA, glx::GLX_DEPTH_SIZE, 24, glx::GLX_DOUBLEBUFFER, 0].as_ptr() as *mut _)
					.as_mut().ok_or(error::Display::Visual)?;

			let context = glx::glXCreateContext(display, info, ptr::null_mut(), 1)
				.as_mut().ok_or(error::Display::Context)?;

			let backend = Rc::new(Backend {
				display: display,
				root:    root,
				context: context,
				id:      id,

				screen: Cell::new({
					let width  = xlib::XDisplayWidth(display, screen);
					let height = xlib::XDisplayHeight(display, screen);

					(width as u32, height as u32)
				}),

				window: Cell::new({
					let mut root   = 0;
					let mut x      = 0;
					let mut y      = 0;
					let mut width  = 0;
					let mut height = 0;
					let mut border = 0;
					let mut depth  = 0;

					xlib::XGetGeometry(display, id, &mut root, &mut x, &mut y, &mut width, &mut height, &mut border, &mut depth);

					(width as u32, height as u32)
				})
			});

			Ok(Display {
				backend: backend.clone(),
				context: gl::backend::Context::new(backend.clone(), false, Default::default())?,
			})
		}
	}

	/// Get the OpenGL context.
	pub fn context(&self) -> Rc<gl::backend::Context> {
		self.context.clone()
	}

	/// Get a drawable OpenGL surface.
	pub fn draw(&mut self) -> gl::Frame {
		gl::Frame::new(self.context.clone(), self.context.get_framebuffer_dimensions())
	}

	/// Resize the Display.
	pub fn resize(&mut self, width: u32, height: u32) {
		self.backend.window.set((width, height));
	}

	/// Take a screenshot.
	pub fn screenshot(&self) -> image::DynamicImage {
		let (width, height)  = self.backend.screen.get();

		unsafe {
			let ximage = xlib::XGetImage(self.backend.display, self.backend.root,
				0, 0, width, height, xlib::XAllPlanes(), xlib::ZPixmap)
					.as_mut().unwrap();

			let r = (*ximage).red_mask;
			let g = (*ximage).green_mask;
			let b = (*ximage).blue_mask;

			let mut image = image::DynamicImage::new_rgb8(width, height);

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
			glx::glXSwapBuffers(self.display, self.id);
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
		self.window.get()
	}

	fn is_current(&self) -> bool {
		unimplemented!()
	}

	unsafe fn make_current(&self) {
		glx::glXMakeCurrent(self.display, self.id, self.context);
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
