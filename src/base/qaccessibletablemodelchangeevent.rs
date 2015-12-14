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

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setFirstColumn<T: QAccessibleTableModelChangeEvent_setFirstColumn>(&mut self, value: T)  {
     value.setFirstColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_setFirstColumn {
  fn setFirstColumn(self, rsthis: &mut QAccessibleTableModelChangeEvent) ;
}

// proto:  void QAccessibleTableModelChangeEvent::setFirstColumn(int col);
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setFirstColumn for (i32) {
  fn setFirstColumn(self, rsthis: &mut QAccessibleTableModelChangeEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN32QAccessibleTableModelChangeEvent14setFirstColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN32QAccessibleTableModelChangeEvent14setFirstColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setFirstRow<T: QAccessibleTableModelChangeEvent_setFirstRow>(&mut self, value: T)  {
     value.setFirstRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_setFirstRow {
  fn setFirstRow(self, rsthis: &mut QAccessibleTableModelChangeEvent) ;
}

// proto:  void QAccessibleTableModelChangeEvent::setFirstRow(int row);
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setFirstRow for (i32) {
  fn setFirstRow(self, rsthis: &mut QAccessibleTableModelChangeEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN32QAccessibleTableModelChangeEvent11setFirstRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN32QAccessibleTableModelChangeEvent11setFirstRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn firstRow<T: QAccessibleTableModelChangeEvent_firstRow>(&mut self, value: T) -> i32 {
    return value.firstRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_firstRow {
  fn firstRow(self, rsthis: &mut QAccessibleTableModelChangeEvent) -> i32;
}

// proto:  int QAccessibleTableModelChangeEvent::firstRow();
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_firstRow for () {
  fn firstRow(self, rsthis: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK32QAccessibleTableModelChangeEvent8firstRowEv()};
    let mut ret = unsafe {_ZNK32QAccessibleTableModelChangeEvent8firstRowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setLastColumn<T: QAccessibleTableModelChangeEvent_setLastColumn>(&mut self, value: T)  {
     value.setLastColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_setLastColumn {
  fn setLastColumn(self, rsthis: &mut QAccessibleTableModelChangeEvent) ;
}

// proto:  void QAccessibleTableModelChangeEvent::setLastColumn(int col);
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setLastColumn for (i32) {
  fn setLastColumn(self, rsthis: &mut QAccessibleTableModelChangeEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN32QAccessibleTableModelChangeEvent13setLastColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN32QAccessibleTableModelChangeEvent13setLastColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn firstColumn<T: QAccessibleTableModelChangeEvent_firstColumn>(&mut self, value: T) -> i32 {
    return value.firstColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_firstColumn {
  fn firstColumn(self, rsthis: &mut QAccessibleTableModelChangeEvent) -> i32;
}

// proto:  int QAccessibleTableModelChangeEvent::firstColumn();
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_firstColumn for () {
  fn firstColumn(self, rsthis: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK32QAccessibleTableModelChangeEvent11firstColumnEv()};
    let mut ret = unsafe {_ZNK32QAccessibleTableModelChangeEvent11firstColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn lastColumn<T: QAccessibleTableModelChangeEvent_lastColumn>(&mut self, value: T) -> i32 {
    return value.lastColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_lastColumn {
  fn lastColumn(self, rsthis: &mut QAccessibleTableModelChangeEvent) -> i32;
}

// proto:  int QAccessibleTableModelChangeEvent::lastColumn();
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_lastColumn for () {
  fn lastColumn(self, rsthis: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK32QAccessibleTableModelChangeEvent10lastColumnEv()};
    let mut ret = unsafe {_ZNK32QAccessibleTableModelChangeEvent10lastColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setLastRow<T: QAccessibleTableModelChangeEvent_setLastRow>(&mut self, value: T)  {
     value.setLastRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_setLastRow {
  fn setLastRow(self, rsthis: &mut QAccessibleTableModelChangeEvent) ;
}

// proto:  void QAccessibleTableModelChangeEvent::setLastRow(int row);
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setLastRow for (i32) {
  fn setLastRow(self, rsthis: &mut QAccessibleTableModelChangeEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN32QAccessibleTableModelChangeEvent10setLastRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN32QAccessibleTableModelChangeEvent10setLastRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn lastRow<T: QAccessibleTableModelChangeEvent_lastRow>(&mut self, value: T) -> i32 {
    return value.lastRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_lastRow {
  fn lastRow(self, rsthis: &mut QAccessibleTableModelChangeEvent) -> i32;
}

// proto:  int QAccessibleTableModelChangeEvent::lastRow();
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_lastRow for () {
  fn lastRow(self, rsthis: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK32QAccessibleTableModelChangeEvent7lastRowEv()};
    let mut ret = unsafe {_ZNK32QAccessibleTableModelChangeEvent7lastRowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

