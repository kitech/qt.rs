// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QPlatformSurfaceEvent::FreeQPlatformSurfaceEvent();
  fn _ZN21QPlatformSurfaceEventD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QPlatformSurfaceEvent)=24
pub struct QPlatformSurfaceEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPlatformSurfaceEvent {
  pub fn FreeQPlatformSurfaceEvent<T: QPlatformSurfaceEvent_FreeQPlatformSurfaceEvent>(&mut self, value: T)  {
     value.FreeQPlatformSurfaceEvent(self);
    // return 1;
  }
}

pub trait QPlatformSurfaceEvent_FreeQPlatformSurfaceEvent {
  fn FreeQPlatformSurfaceEvent(self, rsthis: &mut QPlatformSurfaceEvent) ;
}

// proto:  void QPlatformSurfaceEvent::FreeQPlatformSurfaceEvent();
impl<'a> /*trait*/ QPlatformSurfaceEvent_FreeQPlatformSurfaceEvent for () {
  fn FreeQPlatformSurfaceEvent(self, rsthis: &mut QPlatformSurfaceEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPlatformSurfaceEventD0Ev()};
     unsafe {_ZN21QPlatformSurfaceEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

