// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qitemselectionmodel.h
// dst-file: /src/core/qitemselectionmodel.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
// use super::qitemselectionmodel::QItemSelectionRange; // 773
use super::qabstractitemmodel::QModelIndex; // 773
use super::qabstractitemmodel::QPersistentModelIndex; // 773
// use super::qitemselectionmodel::QItemSelection; // 773
use super::qobject::QObject; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto: static void QItemSelection::split(const QItemSelectionRange & range, const QItemSelectionRange & other, QItemSelection * result);
  fn _ZN14QItemSelection5splitERK19QItemSelectionRangeS2_PS_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QModelIndexList QItemSelection::indexes();
  fn _ZNK14QItemSelection7indexesEv(qthis: *mut c_void);
  // proto:  void QItemSelection::QItemSelection();
  fn _ZN14QItemSelectionC1Ev(qthis: *mut c_void);
  // proto:  bool QItemSelection::contains(const QModelIndex & index);
  fn _ZNK14QItemSelection8containsERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QItemSelection::select(const QModelIndex & topLeft, const QModelIndex & bottomRight);
  fn _ZN14QItemSelection6selectERK11QModelIndexS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QItemSelection::QItemSelection(const QModelIndex & topLeft, const QModelIndex & bottomRight);
  fn _ZN14QItemSelectionC1ERK11QModelIndexS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  int QItemSelectionRange::left();
  fn _ZNK19QItemSelectionRange4leftEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QItemSelectionRange::contains(const QModelIndex & index);
  fn _ZNK19QItemSelectionRange8containsERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QItemSelectionRange QItemSelectionRange::intersected(const QItemSelectionRange & other);
  fn _ZNK19QItemSelectionRange11intersectedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QItemSelectionRange::bottom();
  fn _ZNK19QItemSelectionRange6bottomEv(qthis: *mut c_void) -> c_int;
  // proto:  QModelIndexList QItemSelectionRange::indexes();
  fn _ZNK19QItemSelectionRange7indexesEv(qthis: *mut c_void);
  // proto:  bool QItemSelectionRange::isValid();
  fn _ZNK19QItemSelectionRange7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  const QAbstractItemModel * QItemSelectionRange::model();
  fn _ZNK19QItemSelectionRange5modelEv(qthis: *mut c_void);
  // proto:  int QItemSelectionRange::height();
  fn _ZNK19QItemSelectionRange6heightEv(qthis: *mut c_void) -> c_int;
  // proto:  int QItemSelectionRange::right();
  fn _ZNK19QItemSelectionRange5rightEv(qthis: *mut c_void) -> c_int;
  // proto:  QModelIndex QItemSelectionRange::parent();
  fn _ZNK19QItemSelectionRange6parentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QItemSelectionRange::QItemSelectionRange();
  fn _ZN19QItemSelectionRangeC1Ev(qthis: *mut c_void);
  // proto:  int QItemSelectionRange::width();
  fn _ZNK19QItemSelectionRange5widthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QItemSelectionRange::QItemSelectionRange(const QModelIndex & topLeft, const QModelIndex & bottomRight);
  fn _ZN19QItemSelectionRangeC1ERK11QModelIndexS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  const QPersistentModelIndex & QItemSelectionRange::topLeft();
  fn _ZNK19QItemSelectionRange7topLeftEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QItemSelectionRange::contains(int row, int column, const QModelIndex & parentIndex);
  fn _ZNK19QItemSelectionRange8containsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  void QItemSelectionRange::QItemSelectionRange(const QModelIndex & index);
  fn _ZN19QItemSelectionRangeC1ERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QItemSelectionRange::intersects(const QItemSelectionRange & other);
  fn _ZNK19QItemSelectionRange10intersectsERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  const QPersistentModelIndex & QItemSelectionRange::bottomRight();
  fn _ZNK19QItemSelectionRange11bottomRightEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QItemSelectionRange::top();
  fn _ZNK19QItemSelectionRange3topEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QItemSelectionRange::isEmpty();
  fn _ZNK19QItemSelectionRange7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  void QItemSelectionRange::QItemSelectionRange(const QItemSelectionRange & other);
  fn _ZN19QItemSelectionRangeC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QItemSelectionModel::currentColumnChanged(const QModelIndex & current, const QModelIndex & previous);
  fn _ZN19QItemSelectionModel20currentColumnChangedERK11QModelIndexS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  const QMetaObject * QItemSelectionModel::metaObject();
  fn _ZNK19QItemSelectionModel10metaObjectEv(qthis: *mut c_void);
  // proto:  void QItemSelectionModel::currentChanged(const QModelIndex & current, const QModelIndex & previous);
  fn _ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  const QItemSelection QItemSelectionModel::selection();
  fn _ZNK19QItemSelectionModel9selectionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QItemSelectionModel::isSelected(const QModelIndex & index);
  fn _ZNK19QItemSelectionModel10isSelectedERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QItemSelectionModel::selectionChanged(const QItemSelection & selected, const QItemSelection & deselected);
  fn _ZN19QItemSelectionModel16selectionChangedERK14QItemSelectionS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QItemSelectionModel::clearSelection();
  fn _ZN19QItemSelectionModel14clearSelectionEv(qthis: *mut c_void);
  // proto:  QModelIndex QItemSelectionModel::currentIndex();
  fn _ZNK19QItemSelectionModel12currentIndexEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QItemSelectionModel::QItemSelectionModel(const QItemSelectionModel & );
  fn _ZN19QItemSelectionModelC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QItemSelectionModel::rowIntersectsSelection(int row, const QModelIndex & parent);
  fn _ZNK19QItemSelectionModel22rowIntersectsSelectionEiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  void QItemSelectionModel::reset();
  fn _ZN19QItemSelectionModel5resetEv(qthis: *mut c_void);
  // proto:  void QItemSelectionModel::clearCurrentIndex();
  fn _ZN19QItemSelectionModel17clearCurrentIndexEv(qthis: *mut c_void);
  // proto:  QModelIndexList QItemSelectionModel::selectedIndexes();
  fn _ZNK19QItemSelectionModel15selectedIndexesEv(qthis: *mut c_void);
  // proto:  QModelIndexList QItemSelectionModel::selectedColumns(int row);
  fn _ZNK19QItemSelectionModel15selectedColumnsEi(qthis: *mut c_void, arg0: c_int);
  // proto:  bool QItemSelectionModel::isColumnSelected(int column, const QModelIndex & parent);
  fn _ZNK19QItemSelectionModel16isColumnSelectedEiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  bool QItemSelectionModel::columnIntersectsSelection(int column, const QModelIndex & parent);
  fn _ZNK19QItemSelectionModel25columnIntersectsSelectionEiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  void QItemSelectionModel::~QItemSelectionModel();
  fn _ZN19QItemSelectionModelD0Ev(qthis: *mut c_void);
  // proto:  bool QItemSelectionModel::isRowSelected(int row, const QModelIndex & parent);
  fn _ZNK19QItemSelectionModel13isRowSelectedEiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  void QItemSelectionModel::clear();
  fn _ZN19QItemSelectionModel5clearEv(qthis: *mut c_void);
  // proto:  void QItemSelectionModel::currentRowChanged(const QModelIndex & current, const QModelIndex & previous);
  fn _ZN19QItemSelectionModel17currentRowChangedERK11QModelIndexS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QItemSelectionModel::hasSelection();
  fn _ZNK19QItemSelectionModel12hasSelectionEv(qthis: *mut c_void) -> c_char;
  // proto:  QAbstractItemModel * QItemSelectionModel::model();
  fn _ZN19QItemSelectionModel5modelEv(qthis: *mut c_void);
  // proto:  QModelIndexList QItemSelectionModel::selectedRows(int column);
  fn _ZNK19QItemSelectionModel12selectedRowsEi(qthis: *mut c_void, arg0: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QItemSelection)=1
pub struct QItemSelection {
  pub qclsinst: *mut c_void,
}

// class sizeof(QItemSelectionRange)=16
pub struct QItemSelectionRange {
  pub qclsinst: *mut c_void,
}

// class sizeof(QItemSelectionModel)=1
pub struct QItemSelectionModel {
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
impl<'a> /*trait*/ QItemSelection_split_s<()> for (QItemSelectionRange, QItemSelectionRange, QItemSelection) {
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

  // proto:  QModelIndexList QItemSelection::indexes();
impl /*struct*/ QItemSelection {
  pub fn indexes<RetType, T: QItemSelection_indexes<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.indexes(self);
    // return 1;
  }
}

pub trait QItemSelection_indexes<RetType> {
  fn indexes(self , rsthis: &mut QItemSelection) -> RetType;
}

  // proto:  QModelIndexList QItemSelection::indexes();
impl<'a> /*trait*/ QItemSelection_indexes<()> for () {
  fn indexes(self , rsthis: &mut QItemSelection) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QItemSelection7indexesEv()};
     unsafe {_ZNK14QItemSelection7indexesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QItemSelection::QItemSelection();
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

  // proto:  void QItemSelection::QItemSelection();
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
  pub fn contains<RetType, T: QItemSelection_contains<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QItemSelection_contains<RetType> {
  fn contains(self , rsthis: &mut QItemSelection) -> RetType;
}

  // proto:  bool QItemSelection::contains(const QModelIndex & index);
impl<'a> /*trait*/ QItemSelection_contains<i8> for (QModelIndex) {
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
  pub fn select<RetType, T: QItemSelection_select<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.select(self);
    // return 1;
  }
}

pub trait QItemSelection_select<RetType> {
  fn select(self , rsthis: &mut QItemSelection) -> RetType;
}

  // proto:  void QItemSelection::select(const QModelIndex & topLeft, const QModelIndex & bottomRight);
impl<'a> /*trait*/ QItemSelection_select<()> for (QModelIndex, QModelIndex) {
  fn select(self , rsthis: &mut QItemSelection) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QItemSelection6selectERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN14QItemSelection6selectERK11QModelIndexS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QItemSelection::QItemSelection(const QModelIndex & topLeft, const QModelIndex & bottomRight);
impl<'a> /*trait*/ QItemSelection_NewQItemSelection for (QModelIndex, QModelIndex) {
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

  // proto:  int QItemSelectionRange::left();
impl /*struct*/ QItemSelectionRange {
  pub fn left<RetType, T: QItemSelectionRange_left<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.left(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_left<RetType> {
  fn left(self , rsthis: &mut QItemSelectionRange) -> RetType;
}

  // proto:  int QItemSelectionRange::left();
impl<'a> /*trait*/ QItemSelectionRange_left<i32> for () {
  fn left(self , rsthis: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange4leftEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange4leftEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QItemSelectionRange::contains(const QModelIndex & index);
impl /*struct*/ QItemSelectionRange {
  pub fn contains<RetType, T: QItemSelectionRange_contains<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_contains<RetType> {
  fn contains(self , rsthis: &mut QItemSelectionRange) -> RetType;
}

  // proto:  bool QItemSelectionRange::contains(const QModelIndex & index);
impl<'a> /*trait*/ QItemSelectionRange_contains<i8> for (QModelIndex) {
  fn contains(self , rsthis: &mut QItemSelectionRange) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange8containsERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionRange8containsERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QItemSelectionRange QItemSelectionRange::intersected(const QItemSelectionRange & other);
impl /*struct*/ QItemSelectionRange {
  pub fn intersected<RetType, T: QItemSelectionRange_intersected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intersected(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_intersected<RetType> {
  fn intersected(self , rsthis: &mut QItemSelectionRange) -> RetType;
}

  // proto:  QItemSelectionRange QItemSelectionRange::intersected(const QItemSelectionRange & other);
impl<'a> /*trait*/ QItemSelectionRange_intersected<QItemSelectionRange> for (QItemSelectionRange) {
  fn intersected(self , rsthis: &mut QItemSelectionRange) -> QItemSelectionRange {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange11intersectedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionRange11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QItemSelectionRange{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QItemSelectionRange::bottom();
impl /*struct*/ QItemSelectionRange {
  pub fn bottom<RetType, T: QItemSelectionRange_bottom<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bottom(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_bottom<RetType> {
  fn bottom(self , rsthis: &mut QItemSelectionRange) -> RetType;
}

  // proto:  int QItemSelectionRange::bottom();
impl<'a> /*trait*/ QItemSelectionRange_bottom<i32> for () {
  fn bottom(self , rsthis: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange6bottomEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange6bottomEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QModelIndexList QItemSelectionRange::indexes();
impl /*struct*/ QItemSelectionRange {
  pub fn indexes<RetType, T: QItemSelectionRange_indexes<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.indexes(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_indexes<RetType> {
  fn indexes(self , rsthis: &mut QItemSelectionRange) -> RetType;
}

  // proto:  QModelIndexList QItemSelectionRange::indexes();
impl<'a> /*trait*/ QItemSelectionRange_indexes<()> for () {
  fn indexes(self , rsthis: &mut QItemSelectionRange) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange7indexesEv()};
     unsafe {_ZNK19QItemSelectionRange7indexesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QItemSelectionRange::isValid();
impl /*struct*/ QItemSelectionRange {
  pub fn isValid<RetType, T: QItemSelectionRange_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_isValid<RetType> {
  fn isValid(self , rsthis: &mut QItemSelectionRange) -> RetType;
}

  // proto:  bool QItemSelectionRange::isValid();
impl<'a> /*trait*/ QItemSelectionRange_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QItemSelectionRange) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange7isValidEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QAbstractItemModel * QItemSelectionRange::model();
impl /*struct*/ QItemSelectionRange {
  pub fn model<RetType, T: QItemSelectionRange_model<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.model(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_model<RetType> {
  fn model(self , rsthis: &mut QItemSelectionRange) -> RetType;
}

  // proto:  const QAbstractItemModel * QItemSelectionRange::model();
impl<'a> /*trait*/ QItemSelectionRange_model<()> for () {
  fn model(self , rsthis: &mut QItemSelectionRange) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange5modelEv()};
     unsafe {_ZNK19QItemSelectionRange5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QItemSelectionRange::height();
impl /*struct*/ QItemSelectionRange {
  pub fn height<RetType, T: QItemSelectionRange_height<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_height<RetType> {
  fn height(self , rsthis: &mut QItemSelectionRange) -> RetType;
}

  // proto:  int QItemSelectionRange::height();
impl<'a> /*trait*/ QItemSelectionRange_height<i32> for () {
  fn height(self , rsthis: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange6heightEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QItemSelectionRange::right();
impl /*struct*/ QItemSelectionRange {
  pub fn right<RetType, T: QItemSelectionRange_right<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.right(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_right<RetType> {
  fn right(self , rsthis: &mut QItemSelectionRange) -> RetType;
}

  // proto:  int QItemSelectionRange::right();
impl<'a> /*trait*/ QItemSelectionRange_right<i32> for () {
  fn right(self , rsthis: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange5rightEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange5rightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QModelIndex QItemSelectionRange::parent();
impl /*struct*/ QItemSelectionRange {
  pub fn parent<RetType, T: QItemSelectionRange_parent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_parent<RetType> {
  fn parent(self , rsthis: &mut QItemSelectionRange) -> RetType;
}

  // proto:  QModelIndex QItemSelectionRange::parent();
impl<'a> /*trait*/ QItemSelectionRange_parent<QModelIndex> for () {
  fn parent(self , rsthis: &mut QItemSelectionRange) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange6parentEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange6parentEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QItemSelectionRange::QItemSelectionRange();
impl /*struct*/ QItemSelectionRange {
  pub fn NewQItemSelectionRange<T: QItemSelectionRange_NewQItemSelectionRange>(value: T) -> QItemSelectionRange {
    let rsthis = value.NewQItemSelectionRange();
    return rsthis;
    // return 1;
  }
}

pub trait QItemSelectionRange_NewQItemSelectionRange {
  fn NewQItemSelectionRange(self) -> QItemSelectionRange;
}

  // proto:  void QItemSelectionRange::QItemSelectionRange();
impl<'a> /*trait*/ QItemSelectionRange_NewQItemSelectionRange for () {
  fn NewQItemSelectionRange(self) -> QItemSelectionRange {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionRangeC1Ev()};
    unsafe {_ZN19QItemSelectionRangeC1Ev(qthis)};
    let rsthis = QItemSelectionRange{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QItemSelectionRange::width();
impl /*struct*/ QItemSelectionRange {
  pub fn width<RetType, T: QItemSelectionRange_width<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_width<RetType> {
  fn width(self , rsthis: &mut QItemSelectionRange) -> RetType;
}

  // proto:  int QItemSelectionRange::width();
impl<'a> /*trait*/ QItemSelectionRange_width<i32> for () {
  fn width(self , rsthis: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange5widthEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QItemSelectionRange::QItemSelectionRange(const QModelIndex & topLeft, const QModelIndex & bottomRight);
impl<'a> /*trait*/ QItemSelectionRange_NewQItemSelectionRange for (QModelIndex, QModelIndex) {
  fn NewQItemSelectionRange(self) -> QItemSelectionRange {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionRangeC1ERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN19QItemSelectionRangeC1ERK11QModelIndexS2_(qthis, arg0, arg1)};
    let rsthis = QItemSelectionRange{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QPersistentModelIndex & QItemSelectionRange::topLeft();
impl /*struct*/ QItemSelectionRange {
  pub fn topLeft<RetType, T: QItemSelectionRange_topLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.topLeft(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_topLeft<RetType> {
  fn topLeft(self , rsthis: &mut QItemSelectionRange) -> RetType;
}

  // proto:  const QPersistentModelIndex & QItemSelectionRange::topLeft();
impl<'a> /*trait*/ QItemSelectionRange_topLeft<QPersistentModelIndex> for () {
  fn topLeft(self , rsthis: &mut QItemSelectionRange) -> QPersistentModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange7topLeftEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange7topLeftEv(rsthis.qclsinst)};
    let mut ret1 = QPersistentModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QItemSelectionRange::contains(int row, int column, const QModelIndex & parentIndex);
impl<'a> /*trait*/ QItemSelectionRange_contains<i8> for (i32, i32, QModelIndex) {
  fn contains(self , rsthis: &mut QItemSelectionRange) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange8containsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionRange8containsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QItemSelectionRange::QItemSelectionRange(const QModelIndex & index);
impl<'a> /*trait*/ QItemSelectionRange_NewQItemSelectionRange for (QModelIndex) {
  fn NewQItemSelectionRange(self) -> QItemSelectionRange {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionRangeC1ERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QItemSelectionRangeC1ERK11QModelIndex(qthis, arg0)};
    let rsthis = QItemSelectionRange{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QItemSelectionRange::intersects(const QItemSelectionRange & other);
impl /*struct*/ QItemSelectionRange {
  pub fn intersects<RetType, T: QItemSelectionRange_intersects<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intersects(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_intersects<RetType> {
  fn intersects(self , rsthis: &mut QItemSelectionRange) -> RetType;
}

  // proto:  bool QItemSelectionRange::intersects(const QItemSelectionRange & other);
impl<'a> /*trait*/ QItemSelectionRange_intersects<i8> for (QItemSelectionRange) {
  fn intersects(self , rsthis: &mut QItemSelectionRange) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange10intersectsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionRange10intersectsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QPersistentModelIndex & QItemSelectionRange::bottomRight();
impl /*struct*/ QItemSelectionRange {
  pub fn bottomRight<RetType, T: QItemSelectionRange_bottomRight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bottomRight(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_bottomRight<RetType> {
  fn bottomRight(self , rsthis: &mut QItemSelectionRange) -> RetType;
}

  // proto:  const QPersistentModelIndex & QItemSelectionRange::bottomRight();
impl<'a> /*trait*/ QItemSelectionRange_bottomRight<QPersistentModelIndex> for () {
  fn bottomRight(self , rsthis: &mut QItemSelectionRange) -> QPersistentModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange11bottomRightEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange11bottomRightEv(rsthis.qclsinst)};
    let mut ret1 = QPersistentModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QItemSelectionRange::top();
impl /*struct*/ QItemSelectionRange {
  pub fn top<RetType, T: QItemSelectionRange_top<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.top(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_top<RetType> {
  fn top(self , rsthis: &mut QItemSelectionRange) -> RetType;
}

  // proto:  int QItemSelectionRange::top();
impl<'a> /*trait*/ QItemSelectionRange_top<i32> for () {
  fn top(self , rsthis: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange3topEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange3topEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QItemSelectionRange::isEmpty();
impl /*struct*/ QItemSelectionRange {
  pub fn isEmpty<RetType, T: QItemSelectionRange_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QItemSelectionRange) -> RetType;
}

  // proto:  bool QItemSelectionRange::isEmpty();
impl<'a> /*trait*/ QItemSelectionRange_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QItemSelectionRange) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange7isEmptyEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QItemSelectionRange::QItemSelectionRange(const QItemSelectionRange & other);
impl<'a> /*trait*/ QItemSelectionRange_NewQItemSelectionRange for (QItemSelectionRange) {
  fn NewQItemSelectionRange(self) -> QItemSelectionRange {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionRangeC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QItemSelectionRangeC1ERKS_(qthis, arg0)};
    let rsthis = QItemSelectionRange{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::currentColumnChanged(const QModelIndex & current, const QModelIndex & previous);
impl /*struct*/ QItemSelectionModel {
  pub fn currentColumnChanged<RetType, T: QItemSelectionModel_currentColumnChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentColumnChanged(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_currentColumnChanged<RetType> {
  fn currentColumnChanged(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::currentColumnChanged(const QModelIndex & current, const QModelIndex & previous);
impl<'a> /*trait*/ QItemSelectionModel_currentColumnChanged<()> for (QModelIndex, QModelIndex) {
  fn currentColumnChanged(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel20currentColumnChangedERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN19QItemSelectionModel20currentColumnChangedERK11QModelIndexS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QItemSelectionModel::metaObject();
impl /*struct*/ QItemSelectionModel {
  pub fn metaObject<RetType, T: QItemSelectionModel_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  const QMetaObject * QItemSelectionModel::metaObject();
impl<'a> /*trait*/ QItemSelectionModel_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel10metaObjectEv()};
     unsafe {_ZNK19QItemSelectionModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::currentChanged(const QModelIndex & current, const QModelIndex & previous);
impl /*struct*/ QItemSelectionModel {
  pub fn currentChanged<RetType, T: QItemSelectionModel_currentChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentChanged(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_currentChanged<RetType> {
  fn currentChanged(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::currentChanged(const QModelIndex & current, const QModelIndex & previous);
impl<'a> /*trait*/ QItemSelectionModel_currentChanged<()> for (QModelIndex, QModelIndex) {
  fn currentChanged(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QItemSelection QItemSelectionModel::selection();
impl /*struct*/ QItemSelectionModel {
  pub fn selection<RetType, T: QItemSelectionModel_selection<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selection<RetType> {
  fn selection(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  const QItemSelection QItemSelectionModel::selection();
impl<'a> /*trait*/ QItemSelectionModel_selection<QItemSelection> for () {
  fn selection(self , rsthis: &mut QItemSelectionModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel9selectionEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionModel9selectionEv(rsthis.qclsinst)};
    let mut ret1 = QItemSelection{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QItemSelectionModel::isSelected(const QModelIndex & index);
impl /*struct*/ QItemSelectionModel {
  pub fn isSelected<RetType, T: QItemSelectionModel_isSelected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isSelected(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_isSelected<RetType> {
  fn isSelected(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::isSelected(const QModelIndex & index);
impl<'a> /*trait*/ QItemSelectionModel_isSelected<i8> for (QModelIndex) {
  fn isSelected(self , rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel10isSelectedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionModel10isSelectedERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::selectionChanged(const QItemSelection & selected, const QItemSelection & deselected);
impl /*struct*/ QItemSelectionModel {
  pub fn selectionChanged<RetType, T: QItemSelectionModel_selectionChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selectionChanged<RetType> {
  fn selectionChanged(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::selectionChanged(const QItemSelection & selected, const QItemSelection & deselected);
impl<'a> /*trait*/ QItemSelectionModel_selectionChanged<()> for (QItemSelection, QItemSelection) {
  fn selectionChanged(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel16selectionChangedERK14QItemSelectionS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN19QItemSelectionModel16selectionChangedERK14QItemSelectionS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::clearSelection();
impl /*struct*/ QItemSelectionModel {
  pub fn clearSelection<RetType, T: QItemSelectionModel_clearSelection<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clearSelection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_clearSelection<RetType> {
  fn clearSelection(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::clearSelection();
impl<'a> /*trait*/ QItemSelectionModel_clearSelection<()> for () {
  fn clearSelection(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel14clearSelectionEv()};
     unsafe {_ZN19QItemSelectionModel14clearSelectionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndex QItemSelectionModel::currentIndex();
impl /*struct*/ QItemSelectionModel {
  pub fn currentIndex<RetType, T: QItemSelectionModel_currentIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentIndex(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_currentIndex<RetType> {
  fn currentIndex(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  QModelIndex QItemSelectionModel::currentIndex();
impl<'a> /*trait*/ QItemSelectionModel_currentIndex<QModelIndex> for () {
  fn currentIndex(self , rsthis: &mut QItemSelectionModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel12currentIndexEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionModel12currentIndexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::QItemSelectionModel(const QItemSelectionModel & );
impl /*struct*/ QItemSelectionModel {
  pub fn NewQItemSelectionModel<T: QItemSelectionModel_NewQItemSelectionModel>(value: T) -> QItemSelectionModel {
    let rsthis = value.NewQItemSelectionModel();
    return rsthis;
    // return 1;
  }
}

pub trait QItemSelectionModel_NewQItemSelectionModel {
  fn NewQItemSelectionModel(self) -> QItemSelectionModel;
}

  // proto:  void QItemSelectionModel::QItemSelectionModel(const QItemSelectionModel & );
impl<'a> /*trait*/ QItemSelectionModel_NewQItemSelectionModel for (QItemSelectionModel) {
  fn NewQItemSelectionModel(self) -> QItemSelectionModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModelC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QItemSelectionModelC1ERKS_(qthis, arg0)};
    let rsthis = QItemSelectionModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QItemSelectionModel::rowIntersectsSelection(int row, const QModelIndex & parent);
impl /*struct*/ QItemSelectionModel {
  pub fn rowIntersectsSelection<RetType, T: QItemSelectionModel_rowIntersectsSelection<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rowIntersectsSelection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_rowIntersectsSelection<RetType> {
  fn rowIntersectsSelection(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::rowIntersectsSelection(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_rowIntersectsSelection<i8> for (i32, QModelIndex) {
  fn rowIntersectsSelection(self , rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel22rowIntersectsSelectionEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionModel22rowIntersectsSelectionEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::reset();
impl /*struct*/ QItemSelectionModel {
  pub fn reset<RetType, T: QItemSelectionModel_reset<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_reset<RetType> {
  fn reset(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::reset();
impl<'a> /*trait*/ QItemSelectionModel_reset<()> for () {
  fn reset(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel5resetEv()};
     unsafe {_ZN19QItemSelectionModel5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::clearCurrentIndex();
impl /*struct*/ QItemSelectionModel {
  pub fn clearCurrentIndex<RetType, T: QItemSelectionModel_clearCurrentIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clearCurrentIndex(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_clearCurrentIndex<RetType> {
  fn clearCurrentIndex(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::clearCurrentIndex();
impl<'a> /*trait*/ QItemSelectionModel_clearCurrentIndex<()> for () {
  fn clearCurrentIndex(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel17clearCurrentIndexEv()};
     unsafe {_ZN19QItemSelectionModel17clearCurrentIndexEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndexList QItemSelectionModel::selectedIndexes();
impl /*struct*/ QItemSelectionModel {
  pub fn selectedIndexes<RetType, T: QItemSelectionModel_selectedIndexes<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectedIndexes(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selectedIndexes<RetType> {
  fn selectedIndexes(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  QModelIndexList QItemSelectionModel::selectedIndexes();
impl<'a> /*trait*/ QItemSelectionModel_selectedIndexes<()> for () {
  fn selectedIndexes(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel15selectedIndexesEv()};
     unsafe {_ZNK19QItemSelectionModel15selectedIndexesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndexList QItemSelectionModel::selectedColumns(int row);
impl /*struct*/ QItemSelectionModel {
  pub fn selectedColumns<RetType, T: QItemSelectionModel_selectedColumns<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectedColumns(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selectedColumns<RetType> {
  fn selectedColumns(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  QModelIndexList QItemSelectionModel::selectedColumns(int row);
impl<'a> /*trait*/ QItemSelectionModel_selectedColumns<()> for (i32) {
  fn selectedColumns(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel15selectedColumnsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK19QItemSelectionModel15selectedColumnsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QItemSelectionModel::isColumnSelected(int column, const QModelIndex & parent);
impl /*struct*/ QItemSelectionModel {
  pub fn isColumnSelected<RetType, T: QItemSelectionModel_isColumnSelected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isColumnSelected(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_isColumnSelected<RetType> {
  fn isColumnSelected(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::isColumnSelected(int column, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_isColumnSelected<i8> for (i32, QModelIndex) {
  fn isColumnSelected(self , rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel16isColumnSelectedEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionModel16isColumnSelectedEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QItemSelectionModel::columnIntersectsSelection(int column, const QModelIndex & parent);
impl /*struct*/ QItemSelectionModel {
  pub fn columnIntersectsSelection<RetType, T: QItemSelectionModel_columnIntersectsSelection<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnIntersectsSelection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_columnIntersectsSelection<RetType> {
  fn columnIntersectsSelection(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::columnIntersectsSelection(int column, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_columnIntersectsSelection<i8> for (i32, QModelIndex) {
  fn columnIntersectsSelection(self , rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel25columnIntersectsSelectionEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionModel25columnIntersectsSelectionEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::~QItemSelectionModel();
impl /*struct*/ QItemSelectionModel {
  pub fn FreeQItemSelectionModel<RetType, T: QItemSelectionModel_FreeQItemSelectionModel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQItemSelectionModel(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_FreeQItemSelectionModel<RetType> {
  fn FreeQItemSelectionModel(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::~QItemSelectionModel();
impl<'a> /*trait*/ QItemSelectionModel_FreeQItemSelectionModel<()> for () {
  fn FreeQItemSelectionModel(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModelD0Ev()};
     unsafe {_ZN19QItemSelectionModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QItemSelectionModel::isRowSelected(int row, const QModelIndex & parent);
impl /*struct*/ QItemSelectionModel {
  pub fn isRowSelected<RetType, T: QItemSelectionModel_isRowSelected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isRowSelected(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_isRowSelected<RetType> {
  fn isRowSelected(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::isRowSelected(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_isRowSelected<i8> for (i32, QModelIndex) {
  fn isRowSelected(self , rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel13isRowSelectedEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionModel13isRowSelectedEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::clear();
impl /*struct*/ QItemSelectionModel {
  pub fn clear<RetType, T: QItemSelectionModel_clear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_clear<RetType> {
  fn clear(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::clear();
impl<'a> /*trait*/ QItemSelectionModel_clear<()> for () {
  fn clear(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel5clearEv()};
     unsafe {_ZN19QItemSelectionModel5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::currentRowChanged(const QModelIndex & current, const QModelIndex & previous);
impl /*struct*/ QItemSelectionModel {
  pub fn currentRowChanged<RetType, T: QItemSelectionModel_currentRowChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentRowChanged(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_currentRowChanged<RetType> {
  fn currentRowChanged(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::currentRowChanged(const QModelIndex & current, const QModelIndex & previous);
impl<'a> /*trait*/ QItemSelectionModel_currentRowChanged<()> for (QModelIndex, QModelIndex) {
  fn currentRowChanged(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel17currentRowChangedERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN19QItemSelectionModel17currentRowChangedERK11QModelIndexS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QItemSelectionModel::hasSelection();
impl /*struct*/ QItemSelectionModel {
  pub fn hasSelection<RetType, T: QItemSelectionModel_hasSelection<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasSelection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_hasSelection<RetType> {
  fn hasSelection(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::hasSelection();
impl<'a> /*trait*/ QItemSelectionModel_hasSelection<i8> for () {
  fn hasSelection(self , rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel12hasSelectionEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionModel12hasSelectionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QAbstractItemModel * QItemSelectionModel::model();
impl /*struct*/ QItemSelectionModel {
  pub fn model<RetType, T: QItemSelectionModel_model<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.model(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_model<RetType> {
  fn model(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  QAbstractItemModel * QItemSelectionModel::model();
impl<'a> /*trait*/ QItemSelectionModel_model<()> for () {
  fn model(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel5modelEv()};
     unsafe {_ZN19QItemSelectionModel5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndexList QItemSelectionModel::selectedRows(int column);
impl /*struct*/ QItemSelectionModel {
  pub fn selectedRows<RetType, T: QItemSelectionModel_selectedRows<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectedRows(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selectedRows<RetType> {
  fn selectedRows(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  QModelIndexList QItemSelectionModel::selectedRows(int column);
impl<'a> /*trait*/ QItemSelectionModel_selectedRows<()> for (i32) {
  fn selectedRows(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel12selectedRowsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK19QItemSelectionModel12selectedRowsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

