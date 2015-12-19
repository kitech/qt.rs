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
  // proto: static void QItemSelection::split(const QItemSelectionRange & range, const QItemSelectionRange & other, QItemSelection * result);
  fn _ZN14QItemSelection5splitERK19QItemSelectionRangeS2_PS_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  QList<QModelIndex> QItemSelection::indexes();
  fn _ZNK14QItemSelection7indexesEv(qthis: *mut c_void) ;
  // proto:  void QItemSelection::NewQItemSelection();
  fn _ZN14QItemSelectionC1Ev(qthis: *mut c_void) ;
  // proto:  bool QItemSelection::contains(const QModelIndex & index);
  fn _ZNK14QItemSelection8containsERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QItemSelection::select(const QModelIndex & topLeft, const QModelIndex & bottomRight);
  fn _ZN14QItemSelection6selectERK11QModelIndexS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QItemSelection::NewQItemSelection(const QModelIndex & topLeft, const QModelIndex & bottomRight);
  fn _ZN14QItemSelectionC1ERK11QModelIndexS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
}

// body block begin
// class sizeof(QItemSelection)=1
pub struct QItemSelection {
  pub qclsinst: *mut c_void,
}

// proto: static void QItemSelection::split(const QItemSelectionRange & range, const QItemSelectionRange & other, QItemSelection * result);
impl /*struct*/ QItemSelection {
  pub fn split_s<RetType, T: QItemSelection_split_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.split_s();
    // return 1;
  }
}

pub trait QItemSelection_split_s<RetType> {
  fn split_s(self ) -> RetType;
}

// proto: static void QItemSelection::split(const QItemSelectionRange & range, const QItemSelectionRange & other, QItemSelection * result);
impl<'a> /*trait*/ QItemSelection_split_s<()> for (&'a  QItemSelectionRange, &'a  QItemSelectionRange, &'a mut QItemSelection) {
  fn split_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QItemSelection5splitERK19QItemSelectionRangeS2_PS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN14QItemSelection5splitERK19QItemSelectionRangeS2_PS_(arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  QList<QModelIndex> QItemSelection::indexes();
impl /*struct*/ QItemSelection {
  pub fn indexes<RetType, T: QItemSelection_indexes<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.indexes(self);
    // return 1;
  }
}

pub trait QItemSelection_indexes<RetType> {
  fn indexes(self , rsthis: &mut QItemSelection) -> RetType;
}

// proto:  QList<QModelIndex> QItemSelection::indexes();
impl<'a> /*trait*/ QItemSelection_indexes<()> for () {
  fn indexes(self , rsthis: &mut QItemSelection) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QItemSelection7indexesEv()};
     unsafe {_ZNK14QItemSelection7indexesEv(rsthis.qclsinst)};
    // return 1;
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

// proto:  bool QItemSelection::contains(const QModelIndex & index);
impl /*struct*/ QItemSelection {
  pub fn contains<RetType, T: QItemSelection_contains<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QItemSelection_contains<RetType> {
  fn contains(self , rsthis: &mut QItemSelection) -> RetType;
}

// proto:  bool QItemSelection::contains(const QModelIndex & index);
impl<'a> /*trait*/ QItemSelection_contains<i8> for (&'a  QModelIndex) {
  fn contains(self , rsthis: &mut QItemSelection) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QItemSelection8containsERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QItemSelection8containsERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QItemSelection::select(const QModelIndex & topLeft, const QModelIndex & bottomRight);
impl /*struct*/ QItemSelection {
  pub fn select<RetType, T: QItemSelection_select<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.select(self);
    // return 1;
  }
}

pub trait QItemSelection_select<RetType> {
  fn select(self , rsthis: &mut QItemSelection) -> RetType;
}

// proto:  void QItemSelection::select(const QModelIndex & topLeft, const QModelIndex & bottomRight);
impl<'a> /*trait*/ QItemSelection_select<()> for (&'a  QModelIndex, &'a  QModelIndex) {
  fn select(self , rsthis: &mut QItemSelection) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QItemSelection6selectERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN14QItemSelection6selectERK11QModelIndexS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto: void QItemSelection::NewQItemSelection(const QModelIndex & topLeft, const QModelIndex & bottomRight);
impl<'a> /*trait*/ QItemSelection_NewQItemSelection for (&'a  QModelIndex, &'a  QModelIndex) {
  fn NewQItemSelection(self) -> QItemSelection {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QItemSelectionC1ERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN14QItemSelectionC1ERK11QModelIndexS2_(qthis, arg0, arg1)};
    let rsthis = QItemSelection{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

