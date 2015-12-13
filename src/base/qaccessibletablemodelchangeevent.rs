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
  // proto: void QAccessibleTableModelChangeEvent::setFirstColumn(int col);
  fn _ZN32QAccessibleTableModelChangeEvent14setFirstColumnEi(arg0: c_int) -> i32;
  // proto: void QAccessibleTableModelChangeEvent::setFirstRow(int row);
  fn _ZN32QAccessibleTableModelChangeEvent11setFirstRowEi(arg0: c_int) -> i32;
  // proto: int QAccessibleTableModelChangeEvent::firstRow();
  fn _ZNK32QAccessibleTableModelChangeEvent8firstRowEv() -> i32;
  // proto: void QAccessibleTableModelChangeEvent::setLastColumn(int col);
  fn _ZN32QAccessibleTableModelChangeEvent13setLastColumnEi(arg0: c_int) -> i32;
  // proto: int QAccessibleTableModelChangeEvent::firstColumn();
  fn _ZNK32QAccessibleTableModelChangeEvent11firstColumnEv() -> i32;
  // proto: int QAccessibleTableModelChangeEvent::lastColumn();
  fn _ZNK32QAccessibleTableModelChangeEvent10lastColumnEv() -> i32;
  // proto: void QAccessibleTableModelChangeEvent::setLastRow(int row);
  fn _ZN32QAccessibleTableModelChangeEvent10setLastRowEi(arg0: c_int) -> i32;
  // proto: int QAccessibleTableModelChangeEvent::lastRow();
  fn _ZNK32QAccessibleTableModelChangeEvent7lastRowEv() -> i32;
}

// body block begin
// class sizeof(QAccessibleTableModelChangeEvent)=48
pub struct QAccessibleTableModelChangeEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setFirstColumn<T: QAccessibleTableModelChangeEvent_setFirstColumn>(&mut self, value: T) -> i32 {
    value.setFirstColumn(self);
    return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_setFirstColumn {
  fn setFirstColumn(self, this: &mut QAccessibleTableModelChangeEvent) -> i32;
}

// proto: void QAccessibleTableModelChangeEvent::setFirstColumn(int col);
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setFirstColumn for (i32) {
  fn setFirstColumn(self, this: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN32QAccessibleTableModelChangeEvent14setFirstColumnEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN32QAccessibleTableModelChangeEvent14setFirstColumnEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setFirstRow<T: QAccessibleTableModelChangeEvent_setFirstRow>(&mut self, value: T) -> i32 {
    value.setFirstRow(self);
    return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_setFirstRow {
  fn setFirstRow(self, this: &mut QAccessibleTableModelChangeEvent) -> i32;
}

// proto: void QAccessibleTableModelChangeEvent::setFirstRow(int row);
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setFirstRow for (i32) {
  fn setFirstRow(self, this: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN32QAccessibleTableModelChangeEvent11setFirstRowEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN32QAccessibleTableModelChangeEvent11setFirstRowEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn firstRow<T: QAccessibleTableModelChangeEvent_firstRow>(&mut self, value: T) -> i32 {
    value.firstRow(self);
    return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_firstRow {
  fn firstRow(self, this: &mut QAccessibleTableModelChangeEvent) -> i32;
}

// proto: int QAccessibleTableModelChangeEvent::firstRow();
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_firstRow for () {
  fn firstRow(self, this: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK32QAccessibleTableModelChangeEvent8firstRowEv()};
    unsafe {_ZNK32QAccessibleTableModelChangeEvent8firstRowEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setLastColumn<T: QAccessibleTableModelChangeEvent_setLastColumn>(&mut self, value: T) -> i32 {
    value.setLastColumn(self);
    return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_setLastColumn {
  fn setLastColumn(self, this: &mut QAccessibleTableModelChangeEvent) -> i32;
}

// proto: void QAccessibleTableModelChangeEvent::setLastColumn(int col);
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setLastColumn for (i32) {
  fn setLastColumn(self, this: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN32QAccessibleTableModelChangeEvent13setLastColumnEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN32QAccessibleTableModelChangeEvent13setLastColumnEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn firstColumn<T: QAccessibleTableModelChangeEvent_firstColumn>(&mut self, value: T) -> i32 {
    value.firstColumn(self);
    return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_firstColumn {
  fn firstColumn(self, this: &mut QAccessibleTableModelChangeEvent) -> i32;
}

// proto: int QAccessibleTableModelChangeEvent::firstColumn();
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_firstColumn for () {
  fn firstColumn(self, this: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK32QAccessibleTableModelChangeEvent11firstColumnEv()};
    unsafe {_ZNK32QAccessibleTableModelChangeEvent11firstColumnEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn lastColumn<T: QAccessibleTableModelChangeEvent_lastColumn>(&mut self, value: T) -> i32 {
    value.lastColumn(self);
    return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_lastColumn {
  fn lastColumn(self, this: &mut QAccessibleTableModelChangeEvent) -> i32;
}

// proto: int QAccessibleTableModelChangeEvent::lastColumn();
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_lastColumn for () {
  fn lastColumn(self, this: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK32QAccessibleTableModelChangeEvent10lastColumnEv()};
    unsafe {_ZNK32QAccessibleTableModelChangeEvent10lastColumnEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setLastRow<T: QAccessibleTableModelChangeEvent_setLastRow>(&mut self, value: T) -> i32 {
    value.setLastRow(self);
    return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_setLastRow {
  fn setLastRow(self, this: &mut QAccessibleTableModelChangeEvent) -> i32;
}

// proto: void QAccessibleTableModelChangeEvent::setLastRow(int row);
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setLastRow for (i32) {
  fn setLastRow(self, this: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN32QAccessibleTableModelChangeEvent10setLastRowEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN32QAccessibleTableModelChangeEvent10setLastRowEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn lastRow<T: QAccessibleTableModelChangeEvent_lastRow>(&mut self, value: T) -> i32 {
    value.lastRow(self);
    return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_lastRow {
  fn lastRow(self, this: &mut QAccessibleTableModelChangeEvent) -> i32;
}

// proto: int QAccessibleTableModelChangeEvent::lastRow();
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_lastRow for () {
  fn lastRow(self, this: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK32QAccessibleTableModelChangeEvent7lastRowEv()};
    unsafe {_ZNK32QAccessibleTableModelChangeEvent7lastRowEv()};
    return 1;
  }
}

