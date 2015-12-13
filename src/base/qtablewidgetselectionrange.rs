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
  // proto: void QTableWidgetSelectionRange::NewQTableWidgetSelectionRange(int top, int left, int bottom, int right);
  fn _ZN26QTableWidgetSelectionRangeC1Eiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: int QTableWidgetSelectionRange::columnCount();
  fn _ZNK26QTableWidgetSelectionRange11columnCountEv() -> i32;
  // proto: int QTableWidgetSelectionRange::rowCount();
  fn _ZNK26QTableWidgetSelectionRange8rowCountEv() -> i32;
  // proto: int QTableWidgetSelectionRange::leftColumn();
  fn _ZNK26QTableWidgetSelectionRange10leftColumnEv() -> i32;
  // proto: void QTableWidgetSelectionRange::FreeQTableWidgetSelectionRange();
  fn _ZN26QTableWidgetSelectionRangeD0Ev() -> i32;
  // proto: int QTableWidgetSelectionRange::topRow();
  fn _ZNK26QTableWidgetSelectionRange6topRowEv() -> i32;
  // proto: int QTableWidgetSelectionRange::rightColumn();
  fn _ZNK26QTableWidgetSelectionRange11rightColumnEv() -> i32;
  // proto: void QTableWidgetSelectionRange::NewQTableWidgetSelectionRange();
  fn _ZN26QTableWidgetSelectionRangeC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QTableWidgetSelectionRange::NewQTableWidgetSelectionRange(const QTableWidgetSelectionRange & other);
  fn _ZN26QTableWidgetSelectionRangeC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: int QTableWidgetSelectionRange::bottomRow();
  fn _ZNK26QTableWidgetSelectionRange9bottomRowEv() -> i32;
}

// body block begin
// class sizeof(QTableWidgetSelectionRange)=16
pub struct QTableWidgetSelectionRange {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTableWidgetSelectionRange {
  pub fn NewQTableWidgetSelectionRange<T: QTableWidgetSelectionRange_NewQTableWidgetSelectionRange>(value: T) -> QTableWidgetSelectionRange {
    let rsthis = value.NewQTableWidgetSelectionRange();
    return rsthis;
    // return 1;
  }
}

pub trait QTableWidgetSelectionRange_NewQTableWidgetSelectionRange {
  fn NewQTableWidgetSelectionRange(self) -> QTableWidgetSelectionRange;
}

// proto: void QTableWidgetSelectionRange::NewQTableWidgetSelectionRange(int top, int left, int bottom, int right);
impl<'a> /*trait*/ QTableWidgetSelectionRange_NewQTableWidgetSelectionRange for (i32, i32, i32, i32) {
  fn NewQTableWidgetSelectionRange(self) -> QTableWidgetSelectionRange {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QTableWidgetSelectionRangeC1Eiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN26QTableWidgetSelectionRangeC1Eiiii(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QTableWidgetSelectionRange{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetSelectionRange {
  pub fn columnCount<T: QTableWidgetSelectionRange_columnCount>(&mut self, value: T) -> i32 {
    value.columnCount(self);
    return 1;
  }
}

pub trait QTableWidgetSelectionRange_columnCount {
  fn columnCount(self, this: &mut QTableWidgetSelectionRange) -> i32;
}

// proto: int QTableWidgetSelectionRange::columnCount();
impl<'a> /*trait*/ QTableWidgetSelectionRange_columnCount for () {
  fn columnCount(self, this: &mut QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QTableWidgetSelectionRange11columnCountEv()};
    unsafe {_ZNK26QTableWidgetSelectionRange11columnCountEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetSelectionRange {
  pub fn rowCount<T: QTableWidgetSelectionRange_rowCount>(&mut self, value: T) -> i32 {
    value.rowCount(self);
    return 1;
  }
}

pub trait QTableWidgetSelectionRange_rowCount {
  fn rowCount(self, this: &mut QTableWidgetSelectionRange) -> i32;
}

// proto: int QTableWidgetSelectionRange::rowCount();
impl<'a> /*trait*/ QTableWidgetSelectionRange_rowCount for () {
  fn rowCount(self, this: &mut QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QTableWidgetSelectionRange8rowCountEv()};
    unsafe {_ZNK26QTableWidgetSelectionRange8rowCountEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetSelectionRange {
  pub fn leftColumn<T: QTableWidgetSelectionRange_leftColumn>(&mut self, value: T) -> i32 {
    value.leftColumn(self);
    return 1;
  }
}

pub trait QTableWidgetSelectionRange_leftColumn {
  fn leftColumn(self, this: &mut QTableWidgetSelectionRange) -> i32;
}

// proto: int QTableWidgetSelectionRange::leftColumn();
impl<'a> /*trait*/ QTableWidgetSelectionRange_leftColumn for () {
  fn leftColumn(self, this: &mut QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QTableWidgetSelectionRange10leftColumnEv()};
    unsafe {_ZNK26QTableWidgetSelectionRange10leftColumnEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetSelectionRange {
  pub fn FreeQTableWidgetSelectionRange<T: QTableWidgetSelectionRange_FreeQTableWidgetSelectionRange>(&mut self, value: T) -> i32 {
    value.FreeQTableWidgetSelectionRange(self);
    return 1;
  }
}

pub trait QTableWidgetSelectionRange_FreeQTableWidgetSelectionRange {
  fn FreeQTableWidgetSelectionRange(self, this: &mut QTableWidgetSelectionRange) -> i32;
}

// proto: void QTableWidgetSelectionRange::FreeQTableWidgetSelectionRange();
impl<'a> /*trait*/ QTableWidgetSelectionRange_FreeQTableWidgetSelectionRange for () {
  fn FreeQTableWidgetSelectionRange(self, this: &mut QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QTableWidgetSelectionRangeD0Ev()};
    unsafe {_ZN26QTableWidgetSelectionRangeD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetSelectionRange {
  pub fn topRow<T: QTableWidgetSelectionRange_topRow>(&mut self, value: T) -> i32 {
    value.topRow(self);
    return 1;
  }
}

pub trait QTableWidgetSelectionRange_topRow {
  fn topRow(self, this: &mut QTableWidgetSelectionRange) -> i32;
}

// proto: int QTableWidgetSelectionRange::topRow();
impl<'a> /*trait*/ QTableWidgetSelectionRange_topRow for () {
  fn topRow(self, this: &mut QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QTableWidgetSelectionRange6topRowEv()};
    unsafe {_ZNK26QTableWidgetSelectionRange6topRowEv()};
    return 1;
  }
}

impl /*struct*/ QTableWidgetSelectionRange {
  pub fn rightColumn<T: QTableWidgetSelectionRange_rightColumn>(&mut self, value: T) -> i32 {
    value.rightColumn(self);
    return 1;
  }
}

pub trait QTableWidgetSelectionRange_rightColumn {
  fn rightColumn(self, this: &mut QTableWidgetSelectionRange) -> i32;
}

// proto: int QTableWidgetSelectionRange::rightColumn();
impl<'a> /*trait*/ QTableWidgetSelectionRange_rightColumn for () {
  fn rightColumn(self, this: &mut QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QTableWidgetSelectionRange11rightColumnEv()};
    unsafe {_ZNK26QTableWidgetSelectionRange11rightColumnEv()};
    return 1;
  }
}

// proto: void QTableWidgetSelectionRange::NewQTableWidgetSelectionRange();
impl<'a> /*trait*/ QTableWidgetSelectionRange_NewQTableWidgetSelectionRange for () {
  fn NewQTableWidgetSelectionRange(self) -> QTableWidgetSelectionRange {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QTableWidgetSelectionRangeC1Ev()};
    unsafe {_ZN26QTableWidgetSelectionRangeC1Ev(qthis)};
    let rsthis = QTableWidgetSelectionRange{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QTableWidgetSelectionRange::NewQTableWidgetSelectionRange(const QTableWidgetSelectionRange & other);
impl<'a> /*trait*/ QTableWidgetSelectionRange_NewQTableWidgetSelectionRange for (&'a  QTableWidgetSelectionRange) {
  fn NewQTableWidgetSelectionRange(self) -> QTableWidgetSelectionRange {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QTableWidgetSelectionRangeC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN26QTableWidgetSelectionRangeC1ERKS_(qthis, arg0)};
    let rsthis = QTableWidgetSelectionRange{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetSelectionRange {
  pub fn bottomRow<T: QTableWidgetSelectionRange_bottomRow>(&mut self, value: T) -> i32 {
    value.bottomRow(self);
    return 1;
  }
}

pub trait QTableWidgetSelectionRange_bottomRow {
  fn bottomRow(self, this: &mut QTableWidgetSelectionRange) -> i32;
}

// proto: int QTableWidgetSelectionRange::bottomRow();
impl<'a> /*trait*/ QTableWidgetSelectionRange_bottomRow for () {
  fn bottomRow(self, this: &mut QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QTableWidgetSelectionRange9bottomRowEv()};
    unsafe {_ZNK26QTableWidgetSelectionRange9bottomRowEv()};
    return 1;
  }
}

