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
  // proto:  void QGenericReturnArgument::NewQGenericReturnArgument(const char * aName, void * aData);
  fn _ZN22QGenericReturnArgumentC1EPKcPv(qthis: *mut c_void, arg0: *const c_char, arg1: *mut uint8_t) ;
}

// body block begin
// class sizeof(QGenericReturnArgument)=16
pub struct QGenericReturnArgument {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGenericReturnArgument {
  pub fn NewQGenericReturnArgument<T: QGenericReturnArgument_NewQGenericReturnArgument>(value: T) -> QGenericReturnArgument {
    let rsthis = value.NewQGenericReturnArgument();
    return rsthis;
    // return 1;
  }
}

pub trait QGenericReturnArgument_NewQGenericReturnArgument {
  fn NewQGenericReturnArgument(self) -> QGenericReturnArgument;
}

// proto: void QGenericReturnArgument::NewQGenericReturnArgument(const char * aName, void * aData);
impl<'a> /*trait*/ QGenericReturnArgument_NewQGenericReturnArgument for (&'a  String, &'a mut u8) {
  fn NewQGenericReturnArgument(self) -> QGenericReturnArgument {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGenericReturnArgumentC1EPKcPv()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as *mut uint8_t;
    unsafe {_ZN22QGenericReturnArgumentC1EPKcPv(qthis, arg0, arg1)};
    let rsthis = QGenericReturnArgument{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

