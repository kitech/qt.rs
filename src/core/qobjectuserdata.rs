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
  // proto:  void QObjectUserData::~QObjectUserData();
  fn _ZN15QObjectUserDataD0Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QObjectUserData)=8
pub struct QObjectUserData {
  pub qclsinst: *mut c_void,
}

  // proto:  void QObjectUserData::~QObjectUserData();
impl /*struct*/ QObjectUserData {
  pub fn FreeQObjectUserData<RetType, T: QObjectUserData_FreeQObjectUserData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQObjectUserData(self);
    // return 1;
  }
}

pub trait QObjectUserData_FreeQObjectUserData<RetType> {
  fn FreeQObjectUserData(self , rsthis: &mut QObjectUserData) -> RetType;
}

  // proto:  void QObjectUserData::~QObjectUserData();
impl<'a> /*trait*/ QObjectUserData_FreeQObjectUserData<()> for () {
  fn FreeQObjectUserData(self , rsthis: &mut QObjectUserData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QObjectUserDataD0Ev()};
     unsafe {_ZN15QObjectUserDataD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

