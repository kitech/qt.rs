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
  // proto:  void QSharedData::QSharedData();
  fn _ZN11QSharedDataC1Ev(qthis: *mut c_void);
  // proto:  void QSharedData::QSharedData(const QSharedData & );
  fn _ZN11QSharedDataC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QSharedData)=1
pub struct QSharedData {
  pub qclsinst: *mut c_void,
}

  // proto:  void QSharedData::QSharedData();
impl /*struct*/ QSharedData {
  pub fn NewQSharedData<T: QSharedData_NewQSharedData>(value: T) -> QSharedData {
    let rsthis = value.NewQSharedData();
    return rsthis;
    // return 1;
  }
}

pub trait QSharedData_NewQSharedData {
  fn NewQSharedData(self) -> QSharedData;
}

  // proto:  void QSharedData::QSharedData();
impl<'a> /*trait*/ QSharedData_NewQSharedData for () {
  fn NewQSharedData(self) -> QSharedData {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSharedDataC1Ev()};
    unsafe {_ZN11QSharedDataC1Ev(qthis)};
    let rsthis = QSharedData{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSharedData::QSharedData(const QSharedData & );
impl<'a> /*trait*/ QSharedData_NewQSharedData for (QSharedData) {
  fn NewQSharedData(self) -> QSharedData {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSharedDataC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QSharedDataC1ERKS_(qthis, arg0)};
    let rsthis = QSharedData{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

