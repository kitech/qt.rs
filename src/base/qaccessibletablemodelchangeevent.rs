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
  // proto:  void QAccessibleTableModelChangeEvent::setFirstColumn(int col);
  fn _ZN32QAccessibleTableModelChangeEvent14setFirstColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QAccessibleTableModelChangeEvent::setFirstRow(int row);
  fn _ZN32QAccessibleTableModelChangeEvent11setFirstRowEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QAccessibleTableModelChangeEvent::firstRow();
  fn _ZNK32QAccessibleTableModelChangeEvent8firstRowEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTableModelChangeEvent::setLastColumn(int col);
  fn _ZN32QAccessibleTableModelChangeEvent13setLastColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QAccessibleTableModelChangeEvent::firstColumn();
  fn _ZNK32QAccessibleTableModelChangeEvent11firstColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  int QAccessibleTableModelChangeEvent::lastColumn();
  fn _ZNK32QAccessibleTableModelChangeEvent10lastColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTableModelChangeEvent::setLastRow(int row);
  fn _ZN32QAccessibleTableModelChangeEvent10setLastRowEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QAccessibleTableModelChangeEvent::lastRow();
  fn _ZNK32QAccessibleTableModelChangeEvent7lastRowEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QAccessibleTableModelChangeEvent)=48
pub struct QAccessibleTableModelChangeEvent {
  pub qclsinst: *mut c_void,
}

// proto:  void QAccessibleTableModelChangeEvent::setFirstColumn(int col);
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setFirstColumn<RetType, T: QAccessibleTableModelChangeEvent_setFirstColumn<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFirstColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_setFirstColumn<RetType> {
  fn setFirstColumn(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> RetType;
}

// proto:  void QAccessibleTableModelChangeEvent::setFirstColumn(int col);
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setFirstColumn<()> for (i32) {
  fn setFirstColumn(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN32QAccessibleTableModelChangeEvent14setFirstColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN32QAccessibleTableModelChangeEvent14setFirstColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QAccessibleTableModelChangeEvent::setFirstRow(int row);
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setFirstRow<RetType, T: QAccessibleTableModelChangeEvent_setFirstRow<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFirstRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_setFirstRow<RetType> {
  fn setFirstRow(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> RetType;
}

// proto:  void QAccessibleTableModelChangeEvent::setFirstRow(int row);
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setFirstRow<()> for (i32) {
  fn setFirstRow(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN32QAccessibleTableModelChangeEvent11setFirstRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN32QAccessibleTableModelChangeEvent11setFirstRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QAccessibleTableModelChangeEvent::firstRow();
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn firstRow<RetType, T: QAccessibleTableModelChangeEvent_firstRow<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.firstRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_firstRow<RetType> {
  fn firstRow(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> RetType;
}

// proto:  int QAccessibleTableModelChangeEvent::firstRow();
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_firstRow<i32> for () {
  fn firstRow(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK32QAccessibleTableModelChangeEvent8firstRowEv()};
    let mut ret = unsafe {_ZNK32QAccessibleTableModelChangeEvent8firstRowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QAccessibleTableModelChangeEvent::setLastColumn(int col);
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setLastColumn<RetType, T: QAccessibleTableModelChangeEvent_setLastColumn<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setLastColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_setLastColumn<RetType> {
  fn setLastColumn(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> RetType;
}

// proto:  void QAccessibleTableModelChangeEvent::setLastColumn(int col);
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setLastColumn<()> for (i32) {
  fn setLastColumn(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN32QAccessibleTableModelChangeEvent13setLastColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN32QAccessibleTableModelChangeEvent13setLastColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QAccessibleTableModelChangeEvent::firstColumn();
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn firstColumn<RetType, T: QAccessibleTableModelChangeEvent_firstColumn<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.firstColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_firstColumn<RetType> {
  fn firstColumn(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> RetType;
}

// proto:  int QAccessibleTableModelChangeEvent::firstColumn();
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_firstColumn<i32> for () {
  fn firstColumn(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK32QAccessibleTableModelChangeEvent11firstColumnEv()};
    let mut ret = unsafe {_ZNK32QAccessibleTableModelChangeEvent11firstColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QAccessibleTableModelChangeEvent::lastColumn();
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn lastColumn<RetType, T: QAccessibleTableModelChangeEvent_lastColumn<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.lastColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_lastColumn<RetType> {
  fn lastColumn(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> RetType;
}

// proto:  int QAccessibleTableModelChangeEvent::lastColumn();
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_lastColumn<i32> for () {
  fn lastColumn(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK32QAccessibleTableModelChangeEvent10lastColumnEv()};
    let mut ret = unsafe {_ZNK32QAccessibleTableModelChangeEvent10lastColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QAccessibleTableModelChangeEvent::setLastRow(int row);
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setLastRow<RetType, T: QAccessibleTableModelChangeEvent_setLastRow<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setLastRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_setLastRow<RetType> {
  fn setLastRow(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> RetType;
}

// proto:  void QAccessibleTableModelChangeEvent::setLastRow(int row);
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setLastRow<()> for (i32) {
  fn setLastRow(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN32QAccessibleTableModelChangeEvent10setLastRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN32QAccessibleTableModelChangeEvent10setLastRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QAccessibleTableModelChangeEvent::lastRow();
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn lastRow<RetType, T: QAccessibleTableModelChangeEvent_lastRow<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.lastRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_lastRow<RetType> {
  fn lastRow(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> RetType;
}

// proto:  int QAccessibleTableModelChangeEvent::lastRow();
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_lastRow<i32> for () {
  fn lastRow(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK32QAccessibleTableModelChangeEvent7lastRowEv()};
    let mut ret = unsafe {_ZNK32QAccessibleTableModelChangeEvent7lastRowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

