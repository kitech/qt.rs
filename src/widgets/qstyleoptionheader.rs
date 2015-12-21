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
  // proto:  void QStyleOptionHeader::QStyleOptionHeader();
  fn _ZN18QStyleOptionHeaderC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionHeader::QStyleOptionHeader(const QStyleOptionHeader & other);
  fn _ZN18QStyleOptionHeaderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionHeader::QStyleOptionHeader(int version);
  fn _ZN18QStyleOptionHeaderC1Ei(qthis: *mut c_void, arg0: c_int);
}

// body block begin
// class sizeof(QStyleOptionHeader)=1
pub struct QStyleOptionHeader {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStyleOptionHeader::QStyleOptionHeader();
impl /*struct*/ QStyleOptionHeader {
  pub fn NewQStyleOptionHeader<T: QStyleOptionHeader_NewQStyleOptionHeader>(value: T) -> QStyleOptionHeader {
    let rsthis = value.NewQStyleOptionHeader();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionHeader_NewQStyleOptionHeader {
  fn NewQStyleOptionHeader(self) -> QStyleOptionHeader;
}

  // proto:  void QStyleOptionHeader::QStyleOptionHeader();
impl<'a> /*trait*/ QStyleOptionHeader_NewQStyleOptionHeader for () {
  fn NewQStyleOptionHeader(self) -> QStyleOptionHeader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionHeaderC1Ev()};
    unsafe {_ZN18QStyleOptionHeaderC1Ev(qthis)};
    let rsthis = QStyleOptionHeader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionHeader::QStyleOptionHeader(const QStyleOptionHeader & other);
impl<'a> /*trait*/ QStyleOptionHeader_NewQStyleOptionHeader for (QStyleOptionHeader) {
  fn NewQStyleOptionHeader(self) -> QStyleOptionHeader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionHeaderC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QStyleOptionHeaderC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionHeader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionHeader::QStyleOptionHeader(int version);
impl<'a> /*trait*/ QStyleOptionHeader_NewQStyleOptionHeader for (i32) {
  fn NewQStyleOptionHeader(self) -> QStyleOptionHeader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionHeaderC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN18QStyleOptionHeaderC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionHeader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

