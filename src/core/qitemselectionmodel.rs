// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
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
use std::ops::Deref;
// use super::qitemselectionmodel::QItemSelectionRange; // 773
use super::qabstractitemmodel::QModelIndex; // 773
use super::qabstractitemmodel::QPersistentModelIndex; // 773
use super::qobject::QObject; // 773
use super::qabstractitemmodel::QAbstractItemModel; // 773
// use super::qitemselectionmodel::QItemSelection; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QItemSelection_Class_Size() -> c_int;
  // proto: static void QItemSelection::split(const QItemSelectionRange & range, const QItemSelectionRange & other, QItemSelection * result);
  fn _ZN14QItemSelection5splitERK19QItemSelectionRangeS2_PS_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QModelIndexList QItemSelection::indexes();
  fn _ZNK14QItemSelection7indexesEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QItemSelection::QItemSelection();
  fn dector_ZN14QItemSelectionC1Ev() -> *mut c_void;
  fn _ZN14QItemSelectionC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QItemSelection::contains(const QModelIndex & index);
  fn _ZNK14QItemSelection8containsERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QItemSelection::select(const QModelIndex & topLeft, const QModelIndex & bottomRight);
  fn _ZN14QItemSelection6selectERK11QModelIndexS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QItemSelection::QItemSelection(const QModelIndex & topLeft, const QModelIndex & bottomRight);
  fn dector_ZN14QItemSelectionC1ERK11QModelIndexS2_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN14QItemSelectionC1ERK11QModelIndexS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  fn QItemSelectionRange_Class_Size() -> c_int;
  // proto:  int QItemSelectionRange::left();
  fn demth_ZNK19QItemSelectionRange4leftEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QItemSelectionRange::contains(const QModelIndex & index);
  fn demth_ZNK19QItemSelectionRange8containsERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QItemSelectionRange QItemSelectionRange::intersected(const QItemSelectionRange & other);
  fn _ZNK19QItemSelectionRange11intersectedERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QItemSelectionRange::bottom();
  fn demth_ZNK19QItemSelectionRange6bottomEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QModelIndexList QItemSelectionRange::indexes();
  fn _ZNK19QItemSelectionRange7indexesEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QItemSelectionRange::isValid();
  fn demth_ZNK19QItemSelectionRange7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QAbstractItemModel * QItemSelectionRange::model();
  fn demth_ZNK19QItemSelectionRange5modelEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QItemSelectionRange::height();
  fn demth_ZNK19QItemSelectionRange6heightEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QItemSelectionRange::right();
  fn demth_ZNK19QItemSelectionRange5rightEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QModelIndex QItemSelectionRange::parent();
  fn demth_ZNK19QItemSelectionRange6parentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QItemSelectionRange::QItemSelectionRange();
  fn dector_ZN19QItemSelectionRangeC1Ev() -> *mut c_void;
  fn demth_ZN19QItemSelectionRangeC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QItemSelectionRange::width();
  fn demth_ZNK19QItemSelectionRange5widthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QItemSelectionRange::QItemSelectionRange(const QModelIndex & topLeft, const QModelIndex & bottomRight);
  fn dector_ZN19QItemSelectionRangeC1ERK11QModelIndexS2_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn demth_ZN19QItemSelectionRangeC1ERK11QModelIndexS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  const QPersistentModelIndex & QItemSelectionRange::topLeft();
  fn demth_ZNK19QItemSelectionRange7topLeftEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QItemSelectionRange::contains(int row, int column, const QModelIndex & parentIndex);
  fn demth_ZNK19QItemSelectionRange8containsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  void QItemSelectionRange::QItemSelectionRange(const QModelIndex & index);
  fn dector_ZN19QItemSelectionRangeC1ERK11QModelIndex(arg0: *mut c_void) -> *mut c_void;
  fn demth_ZN19QItemSelectionRangeC1ERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QItemSelectionRange::intersects(const QItemSelectionRange & other);
  fn _ZNK19QItemSelectionRange10intersectsERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  const QPersistentModelIndex & QItemSelectionRange::bottomRight();
  fn demth_ZNK19QItemSelectionRange11bottomRightEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QItemSelectionRange::top();
  fn demth_ZNK19QItemSelectionRange3topEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QItemSelectionRange::isEmpty();
  fn _ZNK19QItemSelectionRange7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QItemSelectionRange::QItemSelectionRange(const QItemSelectionRange & other);
  fn dector_ZN19QItemSelectionRangeC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn demth_ZN19QItemSelectionRangeC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QItemSelectionModel_Class_Size() -> c_int;
  // proto:  void QItemSelectionModel::currentColumnChanged(const QModelIndex & current, const QModelIndex & previous);
  fn _ZN19QItemSelectionModel20currentColumnChangedERK11QModelIndexS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  const QMetaObject * QItemSelectionModel::metaObject();
  fn _ZNK19QItemSelectionModel10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QItemSelectionModel::currentChanged(const QModelIndex & current, const QModelIndex & previous);
  fn _ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QItemSelectionModel::modelChanged(QAbstractItemModel * model);
  fn _ZN19QItemSelectionModel12modelChangedEP18QAbstractItemModel(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QItemSelection QItemSelectionModel::selection();
  fn _ZNK19QItemSelectionModel9selectionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QItemSelectionModel::isSelected(const QModelIndex & index);
  fn _ZNK19QItemSelectionModel10isSelectedERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QItemSelectionModel::selectionChanged(const QItemSelection & selected, const QItemSelection & deselected);
  fn _ZN19QItemSelectionModel16selectionChangedERK14QItemSelectionS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QItemSelectionModel::clearSelection();
  fn _ZN19QItemSelectionModel14clearSelectionEv(qthis: u64 /* *mut c_void*/);
  // proto:  QModelIndex QItemSelectionModel::currentIndex();
  fn _ZNK19QItemSelectionModel12currentIndexEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QItemSelectionModel::setModel(QAbstractItemModel * model);
  fn _ZN19QItemSelectionModel8setModelEP18QAbstractItemModel(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QItemSelectionModel::QItemSelectionModel(const QItemSelectionModel & );
  fn dector_ZN19QItemSelectionModelC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QItemSelectionModelC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QItemSelectionModel::rowIntersectsSelection(int row, const QModelIndex & parent);
  fn _ZNK19QItemSelectionModel22rowIntersectsSelectionEiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  void QItemSelectionModel::QItemSelectionModel(QAbstractItemModel * model);
  fn dector_ZN19QItemSelectionModelC1EP18QAbstractItemModel(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QItemSelectionModelC1EP18QAbstractItemModel(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QItemSelectionModel::reset();
  fn _ZN19QItemSelectionModel5resetEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QItemSelectionModel::clearCurrentIndex();
  fn _ZN19QItemSelectionModel17clearCurrentIndexEv(qthis: u64 /* *mut c_void*/);
  // proto:  QModelIndexList QItemSelectionModel::selectedIndexes();
  fn _ZNK19QItemSelectionModel15selectedIndexesEv(qthis: u64 /* *mut c_void*/);
  // proto:  QModelIndexList QItemSelectionModel::selectedColumns(int row);
  fn _ZNK19QItemSelectionModel15selectedColumnsEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QItemSelectionModel::isColumnSelected(int column, const QModelIndex & parent);
  fn _ZNK19QItemSelectionModel16isColumnSelectedEiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  bool QItemSelectionModel::columnIntersectsSelection(int column, const QModelIndex & parent);
  fn _ZNK19QItemSelectionModel25columnIntersectsSelectionEiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  void QItemSelectionModel::~QItemSelectionModel();
  fn _ZN19QItemSelectionModelD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QItemSelectionModel::isRowSelected(int row, const QModelIndex & parent);
  fn _ZNK19QItemSelectionModel13isRowSelectedEiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  void QItemSelectionModel::QItemSelectionModel(QAbstractItemModel * model, QObject * parent);
  fn dector_ZN19QItemSelectionModelC1EP18QAbstractItemModelP7QObject(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN19QItemSelectionModelC1EP18QAbstractItemModelP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QItemSelectionModel::clear();
  fn _ZN19QItemSelectionModel5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QItemSelectionModel::currentRowChanged(const QModelIndex & current, const QModelIndex & previous);
  fn _ZN19QItemSelectionModel17currentRowChangedERK11QModelIndexS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QItemSelectionModel::hasSelection();
  fn _ZNK19QItemSelectionModel12hasSelectionEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QAbstractItemModel * QItemSelectionModel::model();
  fn _ZN19QItemSelectionModel5modelEv(qthis: u64 /* *mut c_void*/);
  // proto:  QModelIndexList QItemSelectionModel::selectedRows(int column);
  fn _ZNK19QItemSelectionModel12selectedRowsEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  fn QItemSelectionModel_SlotProxy_connect__ZN19QItemSelectionModel16selectionChangedERK14QItemSelectionS2_(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QItemSelectionModel_SlotProxy_connect__ZN19QItemSelectionModel17currentRowChangedERK11QModelIndexS2_(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QItemSelectionModel_SlotProxy_connect__ZN19QItemSelectionModel20currentColumnChangedERK11QModelIndexS2_(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QItemSelectionModel_SlotProxy_connect__ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QItemSelectionModel_SlotProxy_connect__ZN19QItemSelectionModel12modelChangedEP18QAbstractItemModel(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QItemSelection)=1
#[derive(Default)]
pub struct QItemSelection {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QItemSelectionRange)=16
#[derive(Default)]
pub struct QItemSelectionRange {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QItemSelectionModel)=1
#[derive(Default)]
pub struct QItemSelectionModel {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _currentRowChanged_1: QItemSelectionModel_currentRowChanged_signal,
  pub _currentColumnChanged_1: QItemSelectionModel_currentColumnChanged_signal,
  pub _modelChanged_1: QItemSelectionModel_modelChanged_signal,
  pub _selectionChanged_1: QItemSelectionModel_selectionChanged_signal,
  pub _currentChanged_1: QItemSelectionModel_currentChanged_signal,
}

impl /*struct*/ QItemSelection {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QItemSelection {
    return QItemSelection{qclsinst: qthis, ..Default::default()};
  }
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
impl<'a> /*trait*/ QItemSelection_split_s<()> for (&'a QItemSelectionRange, &'a QItemSelectionRange, &'a QItemSelection) {
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
  pub fn indexes<RetType, T: QItemSelection_indexes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexes(self);
    // return 1;
  }
}

pub trait QItemSelection_indexes<RetType> {
  fn indexes(self , rsthis: & QItemSelection) -> RetType;
}

  // proto:  QModelIndexList QItemSelection::indexes();
impl<'a> /*trait*/ QItemSelection_indexes<()> for () {
  fn indexes(self , rsthis: & QItemSelection) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QItemSelection7indexesEv()};
     unsafe {_ZNK14QItemSelection7indexesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QItemSelection::QItemSelection();
impl /*struct*/ QItemSelection {
  pub fn new<T: QItemSelection_new>(value: T) -> QItemSelection {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QItemSelection_new {
  fn new(self) -> QItemSelection;
}

  // proto:  void QItemSelection::QItemSelection();
impl<'a> /*trait*/ QItemSelection_new for () {
  fn new(self) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QItemSelectionC1Ev()};
    let ctysz: c_int = unsafe{QItemSelection_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN14QItemSelectionC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN14QItemSelectionC1Ev()} as u64;
    let rsthis = QItemSelection{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QItemSelection::contains(const QModelIndex & index);
impl /*struct*/ QItemSelection {
  pub fn contains<RetType, T: QItemSelection_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QItemSelection_contains<RetType> {
  fn contains(self , rsthis: & QItemSelection) -> RetType;
}

  // proto:  bool QItemSelection::contains(const QModelIndex & index);
impl<'a> /*trait*/ QItemSelection_contains<i8> for (&'a QModelIndex) {
  fn contains(self , rsthis: & QItemSelection) -> i8 {
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
  pub fn select<RetType, T: QItemSelection_select<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.select(self);
    // return 1;
  }
}

pub trait QItemSelection_select<RetType> {
  fn select(self , rsthis: & QItemSelection) -> RetType;
}

  // proto:  void QItemSelection::select(const QModelIndex & topLeft, const QModelIndex & bottomRight);
impl<'a> /*trait*/ QItemSelection_select<()> for (&'a QModelIndex, &'a QModelIndex) {
  fn select(self , rsthis: & QItemSelection) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QItemSelection6selectERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN14QItemSelection6selectERK11QModelIndexS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QItemSelection::QItemSelection(const QModelIndex & topLeft, const QModelIndex & bottomRight);
impl<'a> /*trait*/ QItemSelection_new for (&'a QModelIndex, &'a QModelIndex) {
  fn new(self) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QItemSelectionC1ERK11QModelIndexS2_()};
    let ctysz: c_int = unsafe{QItemSelection_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN14QItemSelectionC1ERK11QModelIndexS2_(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN14QItemSelectionC1ERK11QModelIndexS2_(arg0, arg1)} as u64;
    let rsthis = QItemSelection{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QItemSelectionRange {
    return QItemSelectionRange{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  int QItemSelectionRange::left();
impl /*struct*/ QItemSelectionRange {
  pub fn left<RetType, T: QItemSelectionRange_left<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.left(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_left<RetType> {
  fn left(self , rsthis: & QItemSelectionRange) -> RetType;
}

  // proto:  int QItemSelectionRange::left();
impl<'a> /*trait*/ QItemSelectionRange_left<i32> for () {
  fn left(self , rsthis: & QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange4leftEv()};
    let mut ret = unsafe {demth_ZNK19QItemSelectionRange4leftEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QItemSelectionRange::contains(const QModelIndex & index);
impl /*struct*/ QItemSelectionRange {
  pub fn contains<RetType, T: QItemSelectionRange_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_contains<RetType> {
  fn contains(self , rsthis: & QItemSelectionRange) -> RetType;
}

  // proto:  bool QItemSelectionRange::contains(const QModelIndex & index);
impl<'a> /*trait*/ QItemSelectionRange_contains<i8> for (&'a QModelIndex) {
  fn contains(self , rsthis: & QItemSelectionRange) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange8containsERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {demth_ZNK19QItemSelectionRange8containsERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QItemSelectionRange QItemSelectionRange::intersected(const QItemSelectionRange & other);
impl /*struct*/ QItemSelectionRange {
  pub fn intersected<RetType, T: QItemSelectionRange_intersected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.intersected(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_intersected<RetType> {
  fn intersected(self , rsthis: & QItemSelectionRange) -> RetType;
}

  // proto:  QItemSelectionRange QItemSelectionRange::intersected(const QItemSelectionRange & other);
impl<'a> /*trait*/ QItemSelectionRange_intersected<QItemSelectionRange> for (&'a QItemSelectionRange) {
  fn intersected(self , rsthis: & QItemSelectionRange) -> QItemSelectionRange {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange11intersectedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionRange11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QItemSelectionRange::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QItemSelectionRange::bottom();
impl /*struct*/ QItemSelectionRange {
  pub fn bottom<RetType, T: QItemSelectionRange_bottom<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bottom(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_bottom<RetType> {
  fn bottom(self , rsthis: & QItemSelectionRange) -> RetType;
}

  // proto:  int QItemSelectionRange::bottom();
impl<'a> /*trait*/ QItemSelectionRange_bottom<i32> for () {
  fn bottom(self , rsthis: & QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange6bottomEv()};
    let mut ret = unsafe {demth_ZNK19QItemSelectionRange6bottomEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QModelIndexList QItemSelectionRange::indexes();
impl /*struct*/ QItemSelectionRange {
  pub fn indexes<RetType, T: QItemSelectionRange_indexes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexes(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_indexes<RetType> {
  fn indexes(self , rsthis: & QItemSelectionRange) -> RetType;
}

  // proto:  QModelIndexList QItemSelectionRange::indexes();
impl<'a> /*trait*/ QItemSelectionRange_indexes<()> for () {
  fn indexes(self , rsthis: & QItemSelectionRange) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange7indexesEv()};
     unsafe {_ZNK19QItemSelectionRange7indexesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QItemSelectionRange::isValid();
impl /*struct*/ QItemSelectionRange {
  pub fn isValid<RetType, T: QItemSelectionRange_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_isValid<RetType> {
  fn isValid(self , rsthis: & QItemSelectionRange) -> RetType;
}

  // proto:  bool QItemSelectionRange::isValid();
impl<'a> /*trait*/ QItemSelectionRange_isValid<i8> for () {
  fn isValid(self , rsthis: & QItemSelectionRange) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange7isValidEv()};
    let mut ret = unsafe {demth_ZNK19QItemSelectionRange7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QAbstractItemModel * QItemSelectionRange::model();
impl /*struct*/ QItemSelectionRange {
  pub fn model<RetType, T: QItemSelectionRange_model<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.model(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_model<RetType> {
  fn model(self , rsthis: & QItemSelectionRange) -> RetType;
}

  // proto:  const QAbstractItemModel * QItemSelectionRange::model();
impl<'a> /*trait*/ QItemSelectionRange_model<()> for () {
  fn model(self , rsthis: & QItemSelectionRange) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange5modelEv()};
     unsafe {demth_ZNK19QItemSelectionRange5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QItemSelectionRange::height();
impl /*struct*/ QItemSelectionRange {
  pub fn height<RetType, T: QItemSelectionRange_height<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_height<RetType> {
  fn height(self , rsthis: & QItemSelectionRange) -> RetType;
}

  // proto:  int QItemSelectionRange::height();
impl<'a> /*trait*/ QItemSelectionRange_height<i32> for () {
  fn height(self , rsthis: & QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange6heightEv()};
    let mut ret = unsafe {demth_ZNK19QItemSelectionRange6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QItemSelectionRange::right();
impl /*struct*/ QItemSelectionRange {
  pub fn right<RetType, T: QItemSelectionRange_right<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.right(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_right<RetType> {
  fn right(self , rsthis: & QItemSelectionRange) -> RetType;
}

  // proto:  int QItemSelectionRange::right();
impl<'a> /*trait*/ QItemSelectionRange_right<i32> for () {
  fn right(self , rsthis: & QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange5rightEv()};
    let mut ret = unsafe {demth_ZNK19QItemSelectionRange5rightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QModelIndex QItemSelectionRange::parent();
impl /*struct*/ QItemSelectionRange {
  pub fn parent<RetType, T: QItemSelectionRange_parent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_parent<RetType> {
  fn parent(self , rsthis: & QItemSelectionRange) -> RetType;
}

  // proto:  QModelIndex QItemSelectionRange::parent();
impl<'a> /*trait*/ QItemSelectionRange_parent<QModelIndex> for () {
  fn parent(self , rsthis: & QItemSelectionRange) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange6parentEv()};
    let mut ret = unsafe {demth_ZNK19QItemSelectionRange6parentEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QItemSelectionRange::QItemSelectionRange();
impl /*struct*/ QItemSelectionRange {
  pub fn new<T: QItemSelectionRange_new>(value: T) -> QItemSelectionRange {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QItemSelectionRange_new {
  fn new(self) -> QItemSelectionRange;
}

  // proto:  void QItemSelectionRange::QItemSelectionRange();
impl<'a> /*trait*/ QItemSelectionRange_new for () {
  fn new(self) -> QItemSelectionRange {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionRangeC1Ev()};
    let ctysz: c_int = unsafe{QItemSelectionRange_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN19QItemSelectionRangeC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN19QItemSelectionRangeC1Ev()} as u64;
    let rsthis = QItemSelectionRange{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QItemSelectionRange::width();
impl /*struct*/ QItemSelectionRange {
  pub fn width<RetType, T: QItemSelectionRange_width<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_width<RetType> {
  fn width(self , rsthis: & QItemSelectionRange) -> RetType;
}

  // proto:  int QItemSelectionRange::width();
impl<'a> /*trait*/ QItemSelectionRange_width<i32> for () {
  fn width(self , rsthis: & QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange5widthEv()};
    let mut ret = unsafe {demth_ZNK19QItemSelectionRange5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QItemSelectionRange::QItemSelectionRange(const QModelIndex & topLeft, const QModelIndex & bottomRight);
impl<'a> /*trait*/ QItemSelectionRange_new for (&'a QModelIndex, &'a QModelIndex) {
  fn new(self) -> QItemSelectionRange {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionRangeC1ERK11QModelIndexS2_()};
    let ctysz: c_int = unsafe{QItemSelectionRange_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN19QItemSelectionRangeC1ERK11QModelIndexS2_(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN19QItemSelectionRangeC1ERK11QModelIndexS2_(arg0, arg1)} as u64;
    let rsthis = QItemSelectionRange{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QPersistentModelIndex & QItemSelectionRange::topLeft();
impl /*struct*/ QItemSelectionRange {
  pub fn topLeft<RetType, T: QItemSelectionRange_topLeft<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.topLeft(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_topLeft<RetType> {
  fn topLeft(self , rsthis: & QItemSelectionRange) -> RetType;
}

  // proto:  const QPersistentModelIndex & QItemSelectionRange::topLeft();
impl<'a> /*trait*/ QItemSelectionRange_topLeft<QPersistentModelIndex> for () {
  fn topLeft(self , rsthis: & QItemSelectionRange) -> QPersistentModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange7topLeftEv()};
    let mut ret = unsafe {demth_ZNK19QItemSelectionRange7topLeftEv(rsthis.qclsinst)};
    let mut ret1 = QPersistentModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QItemSelectionRange::contains(int row, int column, const QModelIndex & parentIndex);
impl<'a> /*trait*/ QItemSelectionRange_contains<i8> for (i32, i32, &'a QModelIndex) {
  fn contains(self , rsthis: & QItemSelectionRange) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange8containsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {demth_ZNK19QItemSelectionRange8containsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QItemSelectionRange::QItemSelectionRange(const QModelIndex & index);
impl<'a> /*trait*/ QItemSelectionRange_new for (&'a QModelIndex) {
  fn new(self) -> QItemSelectionRange {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionRangeC1ERK11QModelIndex()};
    let ctysz: c_int = unsafe{QItemSelectionRange_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QItemSelectionRangeC1ERK11QModelIndex(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN19QItemSelectionRangeC1ERK11QModelIndex(arg0)} as u64;
    let rsthis = QItemSelectionRange{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QItemSelectionRange::intersects(const QItemSelectionRange & other);
impl /*struct*/ QItemSelectionRange {
  pub fn intersects<RetType, T: QItemSelectionRange_intersects<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.intersects(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_intersects<RetType> {
  fn intersects(self , rsthis: & QItemSelectionRange) -> RetType;
}

  // proto:  bool QItemSelectionRange::intersects(const QItemSelectionRange & other);
impl<'a> /*trait*/ QItemSelectionRange_intersects<i8> for (&'a QItemSelectionRange) {
  fn intersects(self , rsthis: & QItemSelectionRange) -> i8 {
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
  pub fn bottomRight<RetType, T: QItemSelectionRange_bottomRight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bottomRight(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_bottomRight<RetType> {
  fn bottomRight(self , rsthis: & QItemSelectionRange) -> RetType;
}

  // proto:  const QPersistentModelIndex & QItemSelectionRange::bottomRight();
impl<'a> /*trait*/ QItemSelectionRange_bottomRight<QPersistentModelIndex> for () {
  fn bottomRight(self , rsthis: & QItemSelectionRange) -> QPersistentModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange11bottomRightEv()};
    let mut ret = unsafe {demth_ZNK19QItemSelectionRange11bottomRightEv(rsthis.qclsinst)};
    let mut ret1 = QPersistentModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QItemSelectionRange::top();
impl /*struct*/ QItemSelectionRange {
  pub fn top<RetType, T: QItemSelectionRange_top<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.top(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_top<RetType> {
  fn top(self , rsthis: & QItemSelectionRange) -> RetType;
}

  // proto:  int QItemSelectionRange::top();
impl<'a> /*trait*/ QItemSelectionRange_top<i32> for () {
  fn top(self , rsthis: & QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange3topEv()};
    let mut ret = unsafe {demth_ZNK19QItemSelectionRange3topEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QItemSelectionRange::isEmpty();
impl /*struct*/ QItemSelectionRange {
  pub fn isEmpty<RetType, T: QItemSelectionRange_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QItemSelectionRange) -> RetType;
}

  // proto:  bool QItemSelectionRange::isEmpty();
impl<'a> /*trait*/ QItemSelectionRange_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QItemSelectionRange) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange7isEmptyEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QItemSelectionRange::QItemSelectionRange(const QItemSelectionRange & other);
impl<'a> /*trait*/ QItemSelectionRange_new for (&'a QItemSelectionRange) {
  fn new(self) -> QItemSelectionRange {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionRangeC1ERKS_()};
    let ctysz: c_int = unsafe{QItemSelectionRange_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QItemSelectionRangeC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN19QItemSelectionRangeC1ERKS_(arg0)} as u64;
    let rsthis = QItemSelectionRange{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QItemSelectionModel {
    return QItemSelectionModel{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QItemSelectionModel {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QItemSelectionModel {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QItemSelectionModel::currentColumnChanged(const QModelIndex & current, const QModelIndex & previous);
impl /*struct*/ QItemSelectionModel {
  pub fn currentColumnChanged<RetType, T: QItemSelectionModel_currentColumnChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentColumnChanged(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_currentColumnChanged<RetType> {
  fn currentColumnChanged(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::currentColumnChanged(const QModelIndex & current, const QModelIndex & previous);
impl<'a> /*trait*/ QItemSelectionModel_currentColumnChanged<()> for (&'a QModelIndex, &'a QModelIndex) {
  fn currentColumnChanged(self , rsthis: & QItemSelectionModel) -> () {
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
  pub fn metaObject<RetType, T: QItemSelectionModel_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_metaObject<RetType> {
  fn metaObject(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  const QMetaObject * QItemSelectionModel::metaObject();
impl<'a> /*trait*/ QItemSelectionModel_metaObject<()> for () {
  fn metaObject(self , rsthis: & QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel10metaObjectEv()};
     unsafe {_ZNK19QItemSelectionModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::currentChanged(const QModelIndex & current, const QModelIndex & previous);
impl /*struct*/ QItemSelectionModel {
  pub fn currentChanged<RetType, T: QItemSelectionModel_currentChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentChanged(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_currentChanged<RetType> {
  fn currentChanged(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::currentChanged(const QModelIndex & current, const QModelIndex & previous);
impl<'a> /*trait*/ QItemSelectionModel_currentChanged<()> for (&'a QModelIndex, &'a QModelIndex) {
  fn currentChanged(self , rsthis: & QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::modelChanged(QAbstractItemModel * model);
impl /*struct*/ QItemSelectionModel {
  pub fn modelChanged<RetType, T: QItemSelectionModel_modelChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.modelChanged(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_modelChanged<RetType> {
  fn modelChanged(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::modelChanged(QAbstractItemModel * model);
impl<'a> /*trait*/ QItemSelectionModel_modelChanged<()> for (&'a QAbstractItemModel) {
  fn modelChanged(self , rsthis: & QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel12modelChangedEP18QAbstractItemModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QItemSelectionModel12modelChangedEP18QAbstractItemModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QItemSelection QItemSelectionModel::selection();
impl /*struct*/ QItemSelectionModel {
  pub fn selection<RetType, T: QItemSelectionModel_selection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selection<RetType> {
  fn selection(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  const QItemSelection QItemSelectionModel::selection();
impl<'a> /*trait*/ QItemSelectionModel_selection<QItemSelection> for () {
  fn selection(self , rsthis: & QItemSelectionModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel9selectionEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionModel9selectionEv(rsthis.qclsinst)};
    let mut ret1 = QItemSelection::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QItemSelectionModel::isSelected(const QModelIndex & index);
impl /*struct*/ QItemSelectionModel {
  pub fn isSelected<RetType, T: QItemSelectionModel_isSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSelected(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_isSelected<RetType> {
  fn isSelected(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::isSelected(const QModelIndex & index);
impl<'a> /*trait*/ QItemSelectionModel_isSelected<i8> for (&'a QModelIndex) {
  fn isSelected(self , rsthis: & QItemSelectionModel) -> i8 {
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
  pub fn selectionChanged<RetType, T: QItemSelectionModel_selectionChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selectionChanged<RetType> {
  fn selectionChanged(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::selectionChanged(const QItemSelection & selected, const QItemSelection & deselected);
impl<'a> /*trait*/ QItemSelectionModel_selectionChanged<()> for (&'a QItemSelection, &'a QItemSelection) {
  fn selectionChanged(self , rsthis: & QItemSelectionModel) -> () {
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
  pub fn clearSelection<RetType, T: QItemSelectionModel_clearSelection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearSelection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_clearSelection<RetType> {
  fn clearSelection(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::clearSelection();
impl<'a> /*trait*/ QItemSelectionModel_clearSelection<()> for () {
  fn clearSelection(self , rsthis: & QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel14clearSelectionEv()};
     unsafe {_ZN19QItemSelectionModel14clearSelectionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndex QItemSelectionModel::currentIndex();
impl /*struct*/ QItemSelectionModel {
  pub fn currentIndex<RetType, T: QItemSelectionModel_currentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentIndex(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_currentIndex<RetType> {
  fn currentIndex(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  QModelIndex QItemSelectionModel::currentIndex();
impl<'a> /*trait*/ QItemSelectionModel_currentIndex<QModelIndex> for () {
  fn currentIndex(self , rsthis: & QItemSelectionModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel12currentIndexEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionModel12currentIndexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::setModel(QAbstractItemModel * model);
impl /*struct*/ QItemSelectionModel {
  pub fn setModel<RetType, T: QItemSelectionModel_setModel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setModel(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_setModel<RetType> {
  fn setModel(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::setModel(QAbstractItemModel * model);
impl<'a> /*trait*/ QItemSelectionModel_setModel<()> for (&'a QAbstractItemModel) {
  fn setModel(self , rsthis: & QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel8setModelEP18QAbstractItemModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QItemSelectionModel8setModelEP18QAbstractItemModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::QItemSelectionModel(const QItemSelectionModel & );
impl /*struct*/ QItemSelectionModel {
  pub fn new<T: QItemSelectionModel_new>(value: T) -> QItemSelectionModel {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QItemSelectionModel_new {
  fn new(self) -> QItemSelectionModel;
}

  // proto:  void QItemSelectionModel::QItemSelectionModel(const QItemSelectionModel & );
impl<'a> /*trait*/ QItemSelectionModel_new for (&'a QItemSelectionModel) {
  fn new(self) -> QItemSelectionModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModelC1ERKS_()};
    let ctysz: c_int = unsafe{QItemSelectionModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QItemSelectionModelC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN19QItemSelectionModelC1ERKS_(arg0)} as u64;
    let rsthis = QItemSelectionModel{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QItemSelectionModel::rowIntersectsSelection(int row, const QModelIndex & parent);
impl /*struct*/ QItemSelectionModel {
  pub fn rowIntersectsSelection<RetType, T: QItemSelectionModel_rowIntersectsSelection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowIntersectsSelection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_rowIntersectsSelection<RetType> {
  fn rowIntersectsSelection(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::rowIntersectsSelection(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_rowIntersectsSelection<i8> for (i32, &'a QModelIndex) {
  fn rowIntersectsSelection(self , rsthis: & QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel22rowIntersectsSelectionEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionModel22rowIntersectsSelectionEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::QItemSelectionModel(QAbstractItemModel * model);
impl<'a> /*trait*/ QItemSelectionModel_new for (&'a QAbstractItemModel) {
  fn new(self) -> QItemSelectionModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModelC1EP18QAbstractItemModel()};
    let ctysz: c_int = unsafe{QItemSelectionModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QItemSelectionModelC1EP18QAbstractItemModel(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN19QItemSelectionModelC1EP18QAbstractItemModel(arg0)} as u64;
    let rsthis = QItemSelectionModel{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::reset();
impl /*struct*/ QItemSelectionModel {
  pub fn reset<RetType, T: QItemSelectionModel_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_reset<RetType> {
  fn reset(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::reset();
impl<'a> /*trait*/ QItemSelectionModel_reset<()> for () {
  fn reset(self , rsthis: & QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel5resetEv()};
     unsafe {_ZN19QItemSelectionModel5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::clearCurrentIndex();
impl /*struct*/ QItemSelectionModel {
  pub fn clearCurrentIndex<RetType, T: QItemSelectionModel_clearCurrentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearCurrentIndex(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_clearCurrentIndex<RetType> {
  fn clearCurrentIndex(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::clearCurrentIndex();
impl<'a> /*trait*/ QItemSelectionModel_clearCurrentIndex<()> for () {
  fn clearCurrentIndex(self , rsthis: & QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel17clearCurrentIndexEv()};
     unsafe {_ZN19QItemSelectionModel17clearCurrentIndexEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndexList QItemSelectionModel::selectedIndexes();
impl /*struct*/ QItemSelectionModel {
  pub fn selectedIndexes<RetType, T: QItemSelectionModel_selectedIndexes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectedIndexes(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selectedIndexes<RetType> {
  fn selectedIndexes(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  QModelIndexList QItemSelectionModel::selectedIndexes();
impl<'a> /*trait*/ QItemSelectionModel_selectedIndexes<()> for () {
  fn selectedIndexes(self , rsthis: & QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel15selectedIndexesEv()};
     unsafe {_ZNK19QItemSelectionModel15selectedIndexesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndexList QItemSelectionModel::selectedColumns(int row);
impl /*struct*/ QItemSelectionModel {
  pub fn selectedColumns<RetType, T: QItemSelectionModel_selectedColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectedColumns(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selectedColumns<RetType> {
  fn selectedColumns(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  QModelIndexList QItemSelectionModel::selectedColumns(int row);
impl<'a> /*trait*/ QItemSelectionModel_selectedColumns<()> for (i32) {
  fn selectedColumns(self , rsthis: & QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel15selectedColumnsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK19QItemSelectionModel15selectedColumnsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QItemSelectionModel::isColumnSelected(int column, const QModelIndex & parent);
impl /*struct*/ QItemSelectionModel {
  pub fn isColumnSelected<RetType, T: QItemSelectionModel_isColumnSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isColumnSelected(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_isColumnSelected<RetType> {
  fn isColumnSelected(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::isColumnSelected(int column, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_isColumnSelected<i8> for (i32, &'a QModelIndex) {
  fn isColumnSelected(self , rsthis: & QItemSelectionModel) -> i8 {
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
  pub fn columnIntersectsSelection<RetType, T: QItemSelectionModel_columnIntersectsSelection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnIntersectsSelection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_columnIntersectsSelection<RetType> {
  fn columnIntersectsSelection(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::columnIntersectsSelection(int column, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_columnIntersectsSelection<i8> for (i32, &'a QModelIndex) {
  fn columnIntersectsSelection(self , rsthis: & QItemSelectionModel) -> i8 {
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
  pub fn free<RetType, T: QItemSelectionModel_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_free<RetType> {
  fn free(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::~QItemSelectionModel();
impl<'a> /*trait*/ QItemSelectionModel_free<()> for () {
  fn free(self , rsthis: & QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModelD0Ev()};
     unsafe {_ZN19QItemSelectionModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QItemSelectionModel::isRowSelected(int row, const QModelIndex & parent);
impl /*struct*/ QItemSelectionModel {
  pub fn isRowSelected<RetType, T: QItemSelectionModel_isRowSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRowSelected(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_isRowSelected<RetType> {
  fn isRowSelected(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::isRowSelected(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_isRowSelected<i8> for (i32, &'a QModelIndex) {
  fn isRowSelected(self , rsthis: & QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel13isRowSelectedEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionModel13isRowSelectedEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::QItemSelectionModel(QAbstractItemModel * model, QObject * parent);
impl<'a> /*trait*/ QItemSelectionModel_new for (&'a QAbstractItemModel, &'a QObject) {
  fn new(self) -> QItemSelectionModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModelC1EP18QAbstractItemModelP7QObject()};
    let ctysz: c_int = unsafe{QItemSelectionModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN19QItemSelectionModelC1EP18QAbstractItemModelP7QObject(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN19QItemSelectionModelC1EP18QAbstractItemModelP7QObject(arg0, arg1)} as u64;
    let rsthis = QItemSelectionModel{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::clear();
impl /*struct*/ QItemSelectionModel {
  pub fn clear<RetType, T: QItemSelectionModel_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_clear<RetType> {
  fn clear(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::clear();
impl<'a> /*trait*/ QItemSelectionModel_clear<()> for () {
  fn clear(self , rsthis: & QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel5clearEv()};
     unsafe {_ZN19QItemSelectionModel5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::currentRowChanged(const QModelIndex & current, const QModelIndex & previous);
impl /*struct*/ QItemSelectionModel {
  pub fn currentRowChanged<RetType, T: QItemSelectionModel_currentRowChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentRowChanged(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_currentRowChanged<RetType> {
  fn currentRowChanged(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::currentRowChanged(const QModelIndex & current, const QModelIndex & previous);
impl<'a> /*trait*/ QItemSelectionModel_currentRowChanged<()> for (&'a QModelIndex, &'a QModelIndex) {
  fn currentRowChanged(self , rsthis: & QItemSelectionModel) -> () {
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
  pub fn hasSelection<RetType, T: QItemSelectionModel_hasSelection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasSelection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_hasSelection<RetType> {
  fn hasSelection(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::hasSelection();
impl<'a> /*trait*/ QItemSelectionModel_hasSelection<i8> for () {
  fn hasSelection(self , rsthis: & QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel12hasSelectionEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionModel12hasSelectionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QAbstractItemModel * QItemSelectionModel::model();
impl /*struct*/ QItemSelectionModel {
  pub fn model<RetType, T: QItemSelectionModel_model<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.model(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_model<RetType> {
  fn model(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  QAbstractItemModel * QItemSelectionModel::model();
impl<'a> /*trait*/ QItemSelectionModel_model<()> for () {
  fn model(self , rsthis: & QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel5modelEv()};
     unsafe {_ZN19QItemSelectionModel5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndexList QItemSelectionModel::selectedRows(int column);
impl /*struct*/ QItemSelectionModel {
  pub fn selectedRows<RetType, T: QItemSelectionModel_selectedRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectedRows(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selectedRows<RetType> {
  fn selectedRows(self , rsthis: & QItemSelectionModel) -> RetType;
}

  // proto:  QModelIndexList QItemSelectionModel::selectedRows(int column);
impl<'a> /*trait*/ QItemSelectionModel_selectedRows<()> for (i32) {
  fn selectedRows(self , rsthis: & QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel12selectedRowsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK19QItemSelectionModel12selectedRowsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QItemSelectionModel_currentRowChanged
pub struct QItemSelectionModel_currentRowChanged_signal{poi:u64}
impl /* struct */ QItemSelectionModel {
  pub fn currentRowChanged_1(&self) -> QItemSelectionModel_currentRowChanged_signal {
     return QItemSelectionModel_currentRowChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QItemSelectionModel_currentRowChanged_signal {
  pub fn connect<T: QItemSelectionModel_currentRowChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QItemSelectionModel_currentRowChanged_signal_connect {
  fn connect(self, sigthis: QItemSelectionModel_currentRowChanged_signal);
}

#[derive(Default)] // for QItemSelectionModel_currentColumnChanged
pub struct QItemSelectionModel_currentColumnChanged_signal{poi:u64}
impl /* struct */ QItemSelectionModel {
  pub fn currentColumnChanged_1(&self) -> QItemSelectionModel_currentColumnChanged_signal {
     return QItemSelectionModel_currentColumnChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QItemSelectionModel_currentColumnChanged_signal {
  pub fn connect<T: QItemSelectionModel_currentColumnChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QItemSelectionModel_currentColumnChanged_signal_connect {
  fn connect(self, sigthis: QItemSelectionModel_currentColumnChanged_signal);
}

#[derive(Default)] // for QItemSelectionModel_modelChanged
pub struct QItemSelectionModel_modelChanged_signal{poi:u64}
impl /* struct */ QItemSelectionModel {
  pub fn modelChanged_1(&self) -> QItemSelectionModel_modelChanged_signal {
     return QItemSelectionModel_modelChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QItemSelectionModel_modelChanged_signal {
  pub fn connect<T: QItemSelectionModel_modelChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QItemSelectionModel_modelChanged_signal_connect {
  fn connect(self, sigthis: QItemSelectionModel_modelChanged_signal);
}

#[derive(Default)] // for QItemSelectionModel_selectionChanged
pub struct QItemSelectionModel_selectionChanged_signal{poi:u64}
impl /* struct */ QItemSelectionModel {
  pub fn selectionChanged_1(&self) -> QItemSelectionModel_selectionChanged_signal {
     return QItemSelectionModel_selectionChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QItemSelectionModel_selectionChanged_signal {
  pub fn connect<T: QItemSelectionModel_selectionChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QItemSelectionModel_selectionChanged_signal_connect {
  fn connect(self, sigthis: QItemSelectionModel_selectionChanged_signal);
}

#[derive(Default)] // for QItemSelectionModel_currentChanged
pub struct QItemSelectionModel_currentChanged_signal{poi:u64}
impl /* struct */ QItemSelectionModel {
  pub fn currentChanged_1(&self) -> QItemSelectionModel_currentChanged_signal {
     return QItemSelectionModel_currentChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QItemSelectionModel_currentChanged_signal {
  pub fn connect<T: QItemSelectionModel_currentChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QItemSelectionModel_currentChanged_signal_connect {
  fn connect(self, sigthis: QItemSelectionModel_currentChanged_signal);
}

// selectionChanged(const class QItemSelection &, const class QItemSelection &)
extern fn QItemSelectionModel_selectionChanged_signal_connect_cb_0(rsfptr:fn(QItemSelection, QItemSelection), arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QItemSelection::inheritFrom(arg0 as u64);
  let rsarg1 = QItemSelection::inheritFrom(arg1 as u64);
  rsfptr(rsarg0,rsarg1);
}
extern fn QItemSelectionModel_selectionChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Fn(QItemSelection, QItemSelection), arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QItemSelection::inheritFrom(arg0 as u64);
  let rsarg1 = QItemSelection::inheritFrom(arg1 as u64);
  rsfptr(rsarg0,rsarg1);
}
impl /* trait */ QItemSelectionModel_selectionChanged_signal_connect for fn(QItemSelection, QItemSelection) {
  fn connect(self, sigthis: QItemSelectionModel_selectionChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QItemSelectionModel_selectionChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QItemSelectionModel_SlotProxy_connect__ZN19QItemSelectionModel16selectionChangedERK14QItemSelectionS2_(arg0, arg1, arg2)};
  }
}
impl /* trait */ QItemSelectionModel_selectionChanged_signal_connect for Box<Fn(QItemSelection, QItemSelection)> {
  fn connect(self, sigthis: QItemSelectionModel_selectionChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QItemSelectionModel_selectionChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QItemSelectionModel_SlotProxy_connect__ZN19QItemSelectionModel16selectionChangedERK14QItemSelectionS2_(arg0, arg1, arg2)};
  }
}
// currentRowChanged(const class QModelIndex &, const class QModelIndex &)
extern fn QItemSelectionModel_currentRowChanged_signal_connect_cb_1(rsfptr:fn(QModelIndex, QModelIndex), arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QModelIndex::inheritFrom(arg0 as u64);
  let rsarg1 = QModelIndex::inheritFrom(arg1 as u64);
  rsfptr(rsarg0,rsarg1);
}
extern fn QItemSelectionModel_currentRowChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Fn(QModelIndex, QModelIndex), arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QModelIndex::inheritFrom(arg0 as u64);
  let rsarg1 = QModelIndex::inheritFrom(arg1 as u64);
  rsfptr(rsarg0,rsarg1);
}
impl /* trait */ QItemSelectionModel_currentRowChanged_signal_connect for fn(QModelIndex, QModelIndex) {
  fn connect(self, sigthis: QItemSelectionModel_currentRowChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QItemSelectionModel_currentRowChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QItemSelectionModel_SlotProxy_connect__ZN19QItemSelectionModel17currentRowChangedERK11QModelIndexS2_(arg0, arg1, arg2)};
  }
}
impl /* trait */ QItemSelectionModel_currentRowChanged_signal_connect for Box<Fn(QModelIndex, QModelIndex)> {
  fn connect(self, sigthis: QItemSelectionModel_currentRowChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QItemSelectionModel_currentRowChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QItemSelectionModel_SlotProxy_connect__ZN19QItemSelectionModel17currentRowChangedERK11QModelIndexS2_(arg0, arg1, arg2)};
  }
}
// currentColumnChanged(const class QModelIndex &, const class QModelIndex &)
extern fn QItemSelectionModel_currentColumnChanged_signal_connect_cb_2(rsfptr:fn(QModelIndex, QModelIndex), arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QModelIndex::inheritFrom(arg0 as u64);
  let rsarg1 = QModelIndex::inheritFrom(arg1 as u64);
  rsfptr(rsarg0,rsarg1);
}
extern fn QItemSelectionModel_currentColumnChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Fn(QModelIndex, QModelIndex), arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QModelIndex::inheritFrom(arg0 as u64);
  let rsarg1 = QModelIndex::inheritFrom(arg1 as u64);
  rsfptr(rsarg0,rsarg1);
}
impl /* trait */ QItemSelectionModel_currentColumnChanged_signal_connect for fn(QModelIndex, QModelIndex) {
  fn connect(self, sigthis: QItemSelectionModel_currentColumnChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QItemSelectionModel_currentColumnChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QItemSelectionModel_SlotProxy_connect__ZN19QItemSelectionModel20currentColumnChangedERK11QModelIndexS2_(arg0, arg1, arg2)};
  }
}
impl /* trait */ QItemSelectionModel_currentColumnChanged_signal_connect for Box<Fn(QModelIndex, QModelIndex)> {
  fn connect(self, sigthis: QItemSelectionModel_currentColumnChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QItemSelectionModel_currentColumnChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QItemSelectionModel_SlotProxy_connect__ZN19QItemSelectionModel20currentColumnChangedERK11QModelIndexS2_(arg0, arg1, arg2)};
  }
}
// currentChanged(const class QModelIndex &, const class QModelIndex &)
extern fn QItemSelectionModel_currentChanged_signal_connect_cb_3(rsfptr:fn(QModelIndex, QModelIndex), arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QModelIndex::inheritFrom(arg0 as u64);
  let rsarg1 = QModelIndex::inheritFrom(arg1 as u64);
  rsfptr(rsarg0,rsarg1);
}
extern fn QItemSelectionModel_currentChanged_signal_connect_cb_box_3(rsfptr_raw:*mut Fn(QModelIndex, QModelIndex), arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QModelIndex::inheritFrom(arg0 as u64);
  let rsarg1 = QModelIndex::inheritFrom(arg1 as u64);
  rsfptr(rsarg0,rsarg1);
}
impl /* trait */ QItemSelectionModel_currentChanged_signal_connect for fn(QModelIndex, QModelIndex) {
  fn connect(self, sigthis: QItemSelectionModel_currentChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QItemSelectionModel_currentChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QItemSelectionModel_SlotProxy_connect__ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_(arg0, arg1, arg2)};
  }
}
impl /* trait */ QItemSelectionModel_currentChanged_signal_connect for Box<Fn(QModelIndex, QModelIndex)> {
  fn connect(self, sigthis: QItemSelectionModel_currentChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QItemSelectionModel_currentChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QItemSelectionModel_SlotProxy_connect__ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_(arg0, arg1, arg2)};
  }
}
// modelChanged(class QAbstractItemModel *)
extern fn QItemSelectionModel_modelChanged_signal_connect_cb_4(rsfptr:fn(QAbstractItemModel), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QAbstractItemModel::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QItemSelectionModel_modelChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Fn(QAbstractItemModel), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QAbstractItemModel::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
impl /* trait */ QItemSelectionModel_modelChanged_signal_connect for fn(QAbstractItemModel) {
  fn connect(self, sigthis: QItemSelectionModel_modelChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QItemSelectionModel_modelChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QItemSelectionModel_SlotProxy_connect__ZN19QItemSelectionModel12modelChangedEP18QAbstractItemModel(arg0, arg1, arg2)};
  }
}
impl /* trait */ QItemSelectionModel_modelChanged_signal_connect for Box<Fn(QAbstractItemModel)> {
  fn connect(self, sigthis: QItemSelectionModel_modelChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QItemSelectionModel_modelChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QItemSelectionModel_SlotProxy_connect__ZN19QItemSelectionModel12modelChangedEP18QAbstractItemModel(arg0, arg1, arg2)};
  }
}
// <= body block end

