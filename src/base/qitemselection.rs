// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qitemselectionrange::QItemSelectionRange;
use super::qmodelindex::QModelIndex;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN14QItemSelection5splitERK19QItemSelectionRangeS2_PS_(arg0: *const c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  fn _ZNK14QItemSelection7indexesEv() -> i32;
  fn _ZN14QItemSelectionC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK14QItemSelection8containsERK11QModelIndex(arg0: *const c_void) -> i32;
  fn _ZN14QItemSelection6selectERK11QModelIndexS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN14QItemSelectionC1ERK11QModelIndexS2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
}

// body block begin
// class sizeof(QItemSelection)=1
pub struct QItemSelection {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QItemSelection {
  pub fn split<T: QItemSelection_split>(&mut self, value: T) -> i32 {
    value.split(self);
    return 1;
  }
}

pub trait QItemSelection_split {
  fn split(self, this: &mut QItemSelection) -> i32;
}

// proto: void QItemSelection::split(const QItemSelectionRange & range, const QItemSelectionRange & other, QItemSelection * result);
impl<'a> /*trait*/ QItemSelection_split for (&'a  QItemSelectionRange, &'a  QItemSelectionRange, &'a mut QItemSelection) {
  fn split(self, this: &mut QItemSelection) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QItemSelection5splitERK19QItemSelectionRangeS2_PS_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN14QItemSelection5splitERK19QItemSelectionRangeS2_PS_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QItemSelection {
  pub fn indexes<T: QItemSelection_indexes>(&mut self, value: T) -> i32 {
    value.indexes(self);
    return 1;
  }
}

pub trait QItemSelection_indexes {
  fn indexes(self, this: &mut QItemSelection) -> i32;
}

// proto: QList<QModelIndex> QItemSelection::indexes();
impl<'a> /*trait*/ QItemSelection_indexes for () {
  fn indexes(self, this: &mut QItemSelection) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QItemSelection7indexesEv()};
    unsafe {_ZNK14QItemSelection7indexesEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelection {
  pub fn NewQItemSelection<T: QItemSelection_NewQItemSelection>(value: T) -> QItemSelection {
    let rsthis = value.NewQItemSelection();
    return rsthis;
    // return 1;
  }
}

pub trait QItemSelection_NewQItemSelection {
  fn NewQItemSelection(self) -> QItemSelection;
}

// proto: void QItemSelection::NewQItemSelection();
impl<'a> /*trait*/ QItemSelection_NewQItemSelection for () {
  fn NewQItemSelection(self) -> QItemSelection {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QItemSelectionC1Ev()};
    unsafe {_ZN14QItemSelectionC1Ev(qthis)};
    let rsthis = QItemSelection{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QItemSelection {
  pub fn contains<T: QItemSelection_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QItemSelection_contains {
  fn contains(self, this: &mut QItemSelection) -> i32;
}

// proto: bool QItemSelection::contains(const QModelIndex & index);
impl<'a> /*trait*/ QItemSelection_contains for (&'a  QModelIndex) {
  fn contains(self, this: &mut QItemSelection) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QItemSelection8containsERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK14QItemSelection8containsERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QItemSelection {
  pub fn select<T: QItemSelection_select>(&mut self, value: T) -> i32 {
    value.select(self);
    return 1;
  }
}

pub trait QItemSelection_select {
  fn select(self, this: &mut QItemSelection) -> i32;
}

// proto: void QItemSelection::select(const QModelIndex & topLeft, const QModelIndex & bottomRight);
impl<'a> /*trait*/ QItemSelection_select for (&'a  QModelIndex, &'a  QModelIndex) {
  fn select(self, this: &mut QItemSelection) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QItemSelection6selectERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN14QItemSelection6selectERK11QModelIndexS2_(arg0, arg1)};
    return 1;
  }
}

// proto: void QItemSelection::NewQItemSelection(const QModelIndex & topLeft, const QModelIndex & bottomRight);
impl<'a> /*trait*/ QItemSelection_NewQItemSelection for (&'a  QModelIndex, &'a  QModelIndex) {
  fn NewQItemSelection(self) -> QItemSelection {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QItemSelectionC1ERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN14QItemSelectionC1ERK11QModelIndexS2_(qthis, arg0, arg1)};
    let rsthis = QItemSelection{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

