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
  // proto:  void QTextFrameLayoutData::~QTextFrameLayoutData();
  fn _ZN20QTextFrameLayoutDataD0Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QTextFrameLayoutData)=8
pub struct QTextFrameLayoutData {
  pub qclsinst: *mut c_void,
}

  // proto:  void QTextFrameLayoutData::~QTextFrameLayoutData();
impl /*struct*/ QTextFrameLayoutData {
  pub fn FreeQTextFrameLayoutData<RetType, T: QTextFrameLayoutData_FreeQTextFrameLayoutData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTextFrameLayoutData(self);
    // return 1;
  }
}

pub trait QTextFrameLayoutData_FreeQTextFrameLayoutData<RetType> {
  fn FreeQTextFrameLayoutData(self , rsthis: &mut QTextFrameLayoutData) -> RetType;
}

  // proto:  void QTextFrameLayoutData::~QTextFrameLayoutData();
impl<'a> /*trait*/ QTextFrameLayoutData_FreeQTextFrameLayoutData<()> for () {
  fn FreeQTextFrameLayoutData(self , rsthis: &mut QTextFrameLayoutData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextFrameLayoutDataD0Ev()};
     unsafe {_ZN20QTextFrameLayoutDataD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

