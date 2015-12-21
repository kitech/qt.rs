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
  // proto:  void QPlatformSurfaceEvent::~QPlatformSurfaceEvent();
  fn _ZN21QPlatformSurfaceEventD0Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QPlatformSurfaceEvent)=24
pub struct QPlatformSurfaceEvent {
  pub qclsinst: *mut c_void,
}

  // proto:  void QPlatformSurfaceEvent::~QPlatformSurfaceEvent();
impl /*struct*/ QPlatformSurfaceEvent {
  pub fn FreeQPlatformSurfaceEvent<RetType, T: QPlatformSurfaceEvent_FreeQPlatformSurfaceEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQPlatformSurfaceEvent(self);
    // return 1;
  }
}

pub trait QPlatformSurfaceEvent_FreeQPlatformSurfaceEvent<RetType> {
  fn FreeQPlatformSurfaceEvent(self , rsthis: &mut QPlatformSurfaceEvent) -> RetType;
}

  // proto:  void QPlatformSurfaceEvent::~QPlatformSurfaceEvent();
impl<'a> /*trait*/ QPlatformSurfaceEvent_FreeQPlatformSurfaceEvent<()> for () {
  fn FreeQPlatformSurfaceEvent(self , rsthis: &mut QPlatformSurfaceEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPlatformSurfaceEventD0Ev()};
     unsafe {_ZN21QPlatformSurfaceEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

