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
  // proto:  void QStyleOptionSlider::NewQStyleOptionSlider(const QStyleOptionSlider & other);
  fn _ZN18QStyleOptionSliderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStyleOptionSlider::NewQStyleOptionSlider(int version);
  fn _ZN18QStyleOptionSliderC1Ei(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QStyleOptionSlider::NewQStyleOptionSlider();
  fn _ZN18QStyleOptionSliderC1Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QStyleOptionSlider)=1
pub struct QStyleOptionSlider {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleOptionSlider {
  pub fn NewQStyleOptionSlider<T: QStyleOptionSlider_NewQStyleOptionSlider>(value: T) -> QStyleOptionSlider {
    let rsthis = value.NewQStyleOptionSlider();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSlider_NewQStyleOptionSlider {
  fn NewQStyleOptionSlider(self) -> QStyleOptionSlider;
}

// proto: void QStyleOptionSlider::NewQStyleOptionSlider(const QStyleOptionSlider & other);
impl<'a> /*trait*/ QStyleOptionSlider_NewQStyleOptionSlider for (&'a  QStyleOptionSlider) {
  fn NewQStyleOptionSlider(self) -> QStyleOptionSlider {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionSliderC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QStyleOptionSliderC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionSlider{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionSlider::NewQStyleOptionSlider(int version);
impl<'a> /*trait*/ QStyleOptionSlider_NewQStyleOptionSlider for (i32) {
  fn NewQStyleOptionSlider(self) -> QStyleOptionSlider {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionSliderC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN18QStyleOptionSliderC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionSlider{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionSlider::NewQStyleOptionSlider();
impl<'a> /*trait*/ QStyleOptionSlider_NewQStyleOptionSlider for () {
  fn NewQStyleOptionSlider(self) -> QStyleOptionSlider {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionSliderC1Ev()};
    unsafe {_ZN18QStyleOptionSliderC1Ev(qthis)};
    let rsthis = QStyleOptionSlider{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

