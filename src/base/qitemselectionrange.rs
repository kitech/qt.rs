// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmodelindex::QModelIndex;
use super::qpersistentmodelindex::QPersistentModelIndex;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QItemSelectionRange::left();
  fn _ZNK19QItemSelectionRange4leftEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QItemSelectionRange::contains(const QModelIndex & index);
  fn _ZNK19QItemSelectionRange8containsERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QItemSelectionRange QItemSelectionRange::intersected(const QItemSelectionRange & other);
  fn _ZNK19QItemSelectionRange11intersectedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QItemSelectionRange::bottom();
  fn _ZNK19QItemSelectionRange6bottomEv(qthis: *mut c_void) -> c_int;
  // proto:  QList<QModelIndex> QItemSelectionRange::indexes();
  fn _ZNK19QItemSelectionRange7indexesEv(qthis: *mut c_void) ;
  // proto:  bool QItemSelectionRange::isValid();
  fn _ZNK19QItemSelectionRange7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  const QAbstractItemModel * QItemSelectionRange::model();
  fn _ZNK19QItemSelectionRange5modelEv(qthis: *mut c_void) ;
  // proto:  int QItemSelectionRange::height();
  fn _ZNK19QItemSelectionRange6heightEv(qthis: *mut c_void) -> c_int;
  // proto:  int QItemSelectionRange::right();
  fn _ZNK19QItemSelectionRange5rightEv(qthis: *mut c_void) -> c_int;
  // proto:  QModelIndex QItemSelectionRange::parent();
  fn _ZNK19QItemSelectionRange6parentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QItemSelectionRange::NewQItemSelectionRange();
  fn _ZN19QItemSelectionRangeC1Ev(qthis: *mut c_void) ;
  // proto:  int QItemSelectionRange::width();
  fn _ZNK19QItemSelectionRange5widthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QItemSelectionRange::NewQItemSelectionRange(const QModelIndex & topLeft, const QModelIndex & bottomRight);
  fn _ZN19QItemSelectionRangeC1ERK11QModelIndexS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  const QPersistentModelIndex & QItemSelectionRange::topLeft();
  fn _ZNK19QItemSelectionRange7topLeftEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QItemSelectionRange::contains(int row, int column, const QModelIndex & parentIndex);
  fn _ZNK19QItemSelectionRange8containsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> int8_t;
  // proto:  void QItemSelectionRange::NewQItemSelectionRange(const QModelIndex & index);
  fn _ZN19QItemSelectionRangeC1ERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QItemSelectionRange::intersects(const QItemSelectionRange & other);
  fn _ZNK19QItemSelectionRange10intersectsERKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  const QPersistentModelIndex & QItemSelectionRange::bottomRight();
  fn _ZNK19QItemSelectionRange11bottomRightEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QItemSelectionRange::top();
  fn _ZNK19QItemSelectionRange3topEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QItemSelectionRange::isEmpty();
  fn _ZNK19QItemSelectionRange7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QItemSelectionRange::NewQItemSelectionRange(const QItemSelectionRange & other);
  fn _ZN19QItemSelectionRangeC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QItemSelectionRange)=16
pub struct QItemSelectionRange {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QItemSelectionRange {
  pub fn left<T: QItemSelectionRange_left>(&mut self, value: T) -> i32 {
    return value.left(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_left {
  fn left(self, rsthis: &mut QItemSelectionRange) -> i32;
}

// proto:  int QItemSelectionRange::left();
impl<'a> /*trait*/ QItemSelectionRange_left for () {
  fn left(self, rsthis: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange4leftEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange4leftEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn contains<T: QItemSelectionRange_contains>(&mut self, value: T) -> i8 {
    return value.contains(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_contains {
  fn contains(self, rsthis: &mut QItemSelectionRange) -> i8;
}

// proto:  bool QItemSelectionRange::contains(const QModelIndex & index);
impl<'a> /*trait*/ QItemSelectionRange_contains for (&'a  QModelIndex) {
  fn contains(self, rsthis: &mut QItemSelectionRange) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange8containsERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionRange8containsERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn intersected<T: QItemSelectionRange_intersected>(&mut self, value: T) -> QItemSelectionRange {
    return value.intersected(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_intersected {
  fn intersected(self, rsthis: &mut QItemSelectionRange) -> QItemSelectionRange;
}

// proto:  QItemSelectionRange QItemSelectionRange::intersected(const QItemSelectionRange & other);
impl<'a> /*trait*/ QItemSelectionRange_intersected for (&'a  QItemSelectionRange) {
  fn intersected(self, rsthis: &mut QItemSelectionRange) -> QItemSelectionRange {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange11intersectedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionRange11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QItemSelectionRange{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn bottom<T: QItemSelectionRange_bottom>(&mut self, value: T) -> i32 {
    return value.bottom(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_bottom {
  fn bottom(self, rsthis: &mut QItemSelectionRange) -> i32;
}

// proto:  int QItemSelectionRange::bottom();
impl<'a> /*trait*/ QItemSelectionRange_bottom for () {
  fn bottom(self, rsthis: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange6bottomEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange6bottomEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn indexes<T: QItemSelectionRange_indexes>(&mut self, value: T)  {
     value.indexes(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_indexes {
  fn indexes(self, rsthis: &mut QItemSelectionRange) ;
}

// proto:  QList<QModelIndex> QItemSelectionRange::indexes();
impl<'a> /*trait*/ QItemSelectionRange_indexes for () {
  fn indexes(self, rsthis: &mut QItemSelectionRange)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange7indexesEv()};
     unsafe {_ZNK19QItemSelectionRange7indexesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn isValid<T: QItemSelectionRange_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_isValid {
  fn isValid(self, rsthis: &mut QItemSelectionRange) -> i8;
}

// proto:  bool QItemSelectionRange::isValid();
impl<'a> /*trait*/ QItemSelectionRange_isValid for () {
  fn isValid(self, rsthis: &mut QItemSelectionRange) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange7isValidEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn model<T: QItemSelectionRange_model>(&mut self, value: T)  {
     value.model(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_model {
  fn model(self, rsthis: &mut QItemSelectionRange) ;
}

// proto:  const QAbstractItemModel * QItemSelectionRange::model();
impl<'a> /*trait*/ QItemSelectionRange_model for () {
  fn model(self, rsthis: &mut QItemSelectionRange)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange5modelEv()};
     unsafe {_ZNK19QItemSelectionRange5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn height<T: QItemSelectionRange_height>(&mut self, value: T) -> i32 {
    return value.height(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_height {
  fn height(self, rsthis: &mut QItemSelectionRange) -> i32;
}

// proto:  int QItemSelectionRange::height();
impl<'a> /*trait*/ QItemSelectionRange_height for () {
  fn height(self, rsthis: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange6heightEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn right<T: QItemSelectionRange_right>(&mut self, value: T) -> i32 {
    return value.right(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_right {
  fn right(self, rsthis: &mut QItemSelectionRange) -> i32;
}

// proto:  int QItemSelectionRange::right();
impl<'a> /*trait*/ QItemSelectionRange_right for () {
  fn right(self, rsthis: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange5rightEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange5rightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn parent<T: QItemSelectionRange_parent>(&mut self, value: T) -> QModelIndex {
    return value.parent(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_parent {
  fn parent(self, rsthis: &mut QItemSelectionRange) -> QModelIndex;
}

// proto:  QModelIndex QItemSelectionRange::parent();
impl<'a> /*trait*/ QItemSelectionRange_parent for () {
  fn parent(self, rsthis: &mut QItemSelectionRange) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange6parentEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange6parentEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

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

// proto: void QItemSelectionRange::NewQItemSelectionRange();
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

impl /*struct*/ QItemSelectionRange {
  pub fn width<T: QItemSelectionRange_width>(&mut self, value: T) -> i32 {
    return value.width(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_width {
  fn width(self, rsthis: &mut QItemSelectionRange) -> i32;
}

// proto:  int QItemSelectionRange::width();
impl<'a> /*trait*/ QItemSelectionRange_width for () {
  fn width(self, rsthis: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange5widthEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QItemSelectionRange::NewQItemSelectionRange(const QModelIndex & topLeft, const QModelIndex & bottomRight);
impl<'a> /*trait*/ QItemSelectionRange_NewQItemSelectionRange for (&'a  QModelIndex, &'a  QModelIndex) {
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

impl /*struct*/ QItemSelectionRange {
  pub fn topLeft<T: QItemSelectionRange_topLeft>(&mut self, value: T) -> QPersistentModelIndex {
    return value.topLeft(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_topLeft {
  fn topLeft(self, rsthis: &mut QItemSelectionRange) -> QPersistentModelIndex;
}

// proto:  const QPersistentModelIndex & QItemSelectionRange::topLeft();
impl<'a> /*trait*/ QItemSelectionRange_topLeft for () {
  fn topLeft(self, rsthis: &mut QItemSelectionRange) -> QPersistentModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange7topLeftEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange7topLeftEv(rsthis.qclsinst)};
    let mut ret1 = QPersistentModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QItemSelectionRange::contains(int row, int column, const QModelIndex & parentIndex);
impl<'a> /*trait*/ QItemSelectionRange_contains for (i32, i32, &'a  QModelIndex) {
  fn contains(self, rsthis: &mut QItemSelectionRange) -> i8 {
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

// proto: void QItemSelectionRange::NewQItemSelectionRange(const QModelIndex & index);
impl<'a> /*trait*/ QItemSelectionRange_NewQItemSelectionRange for (&'a  QModelIndex) {
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

impl /*struct*/ QItemSelectionRange {
  pub fn intersects<T: QItemSelectionRange_intersects>(&mut self, value: T) -> i8 {
    return value.intersects(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_intersects {
  fn intersects(self, rsthis: &mut QItemSelectionRange) -> i8;
}

// proto:  bool QItemSelectionRange::intersects(const QItemSelectionRange & other);
impl<'a> /*trait*/ QItemSelectionRange_intersects for (&'a  QItemSelectionRange) {
  fn intersects(self, rsthis: &mut QItemSelectionRange) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange10intersectsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionRange10intersectsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn bottomRight<T: QItemSelectionRange_bottomRight>(&mut self, value: T) -> QPersistentModelIndex {
    return value.bottomRight(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_bottomRight {
  fn bottomRight(self, rsthis: &mut QItemSelectionRange) -> QPersistentModelIndex;
}

// proto:  const QPersistentModelIndex & QItemSelectionRange::bottomRight();
impl<'a> /*trait*/ QItemSelectionRange_bottomRight for () {
  fn bottomRight(self, rsthis: &mut QItemSelectionRange) -> QPersistentModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange11bottomRightEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange11bottomRightEv(rsthis.qclsinst)};
    let mut ret1 = QPersistentModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn top<T: QItemSelectionRange_top>(&mut self, value: T) -> i32 {
    return value.top(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_top {
  fn top(self, rsthis: &mut QItemSelectionRange) -> i32;
}

// proto:  int QItemSelectionRange::top();
impl<'a> /*trait*/ QItemSelectionRange_top for () {
  fn top(self, rsthis: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange3topEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange3topEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn isEmpty<T: QItemSelectionRange_isEmpty>(&mut self, value: T) -> i8 {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QItemSelectionRange_isEmpty {
  fn isEmpty(self, rsthis: &mut QItemSelectionRange) -> i8;
}

// proto:  bool QItemSelectionRange::isEmpty();
impl<'a> /*trait*/ QItemSelectionRange_isEmpty for () {
  fn isEmpty(self, rsthis: &mut QItemSelectionRange) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange7isEmptyEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionRange7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QItemSelectionRange::NewQItemSelectionRange(const QItemSelectionRange & other);
impl<'a> /*trait*/ QItemSelectionRange_NewQItemSelectionRange for (&'a  QItemSelectionRange) {
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

