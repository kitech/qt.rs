// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmodelindex::QModelIndex;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK19QItemSelectionRange4leftEv() -> i32;
  fn _ZNK19QItemSelectionRange8containsERK11QModelIndex(arg0: *const c_void) -> i32;
  fn _ZNK19QItemSelectionRange11intersectedERKS_(arg0: *const c_void) -> i32;
  fn _ZNK19QItemSelectionRange6bottomEv() -> i32;
  fn _ZNK19QItemSelectionRange7indexesEv() -> i32;
  fn _ZNK19QItemSelectionRange7isValidEv() -> i32;
  fn _ZNK19QItemSelectionRange5modelEv() -> i32;
  fn _ZNK19QItemSelectionRange6heightEv() -> i32;
  fn _ZNK19QItemSelectionRange5rightEv() -> i32;
  fn _ZNK19QItemSelectionRange6parentEv() -> i32;
  fn _ZN19QItemSelectionRangeC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK19QItemSelectionRange5widthEv() -> i32;
  fn _ZN19QItemSelectionRangeC1ERK11QModelIndexS2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK19QItemSelectionRange7topLeftEv() -> i32;
  fn _ZNK19QItemSelectionRange8containsEiiRK11QModelIndex(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  fn _ZN19QItemSelectionRangeC1ERK11QModelIndex(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK19QItemSelectionRange10intersectsERKS_(arg0: *const c_void) -> i32;
  fn _ZNK19QItemSelectionRange11bottomRightEv() -> i32;
  fn _ZNK19QItemSelectionRange3topEv() -> i32;
  fn _ZNK19QItemSelectionRange7isEmptyEv() -> i32;
  fn _ZN19QItemSelectionRangeC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QItemSelectionRange)=16
pub struct QItemSelectionRange {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QItemSelectionRange {
  pub fn left<T: QItemSelectionRange_left>(&mut self, value: T) -> i32 {
    value.left(self);
    return 1;
  }
}

pub trait QItemSelectionRange_left {
  fn left(self, this: &mut QItemSelectionRange) -> i32;
}

// proto: int QItemSelectionRange::left();
impl<'a> /*trait*/ QItemSelectionRange_left for () {
  fn left(self, this: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange4leftEv()};
    unsafe {_ZNK19QItemSelectionRange4leftEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn contains<T: QItemSelectionRange_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QItemSelectionRange_contains {
  fn contains(self, this: &mut QItemSelectionRange) -> i32;
}

// proto: bool QItemSelectionRange::contains(const QModelIndex & index);
impl<'a> /*trait*/ QItemSelectionRange_contains for (&'a  QModelIndex) {
  fn contains(self, this: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange8containsERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK19QItemSelectionRange8containsERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn intersected<T: QItemSelectionRange_intersected>(&mut self, value: T) -> i32 {
    value.intersected(self);
    return 1;
  }
}

pub trait QItemSelectionRange_intersected {
  fn intersected(self, this: &mut QItemSelectionRange) -> i32;
}

// proto: QItemSelectionRange QItemSelectionRange::intersected(const QItemSelectionRange & other);
impl<'a> /*trait*/ QItemSelectionRange_intersected for (&'a  QItemSelectionRange) {
  fn intersected(self, this: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange11intersectedERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK19QItemSelectionRange11intersectedERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn bottom<T: QItemSelectionRange_bottom>(&mut self, value: T) -> i32 {
    value.bottom(self);
    return 1;
  }
}

pub trait QItemSelectionRange_bottom {
  fn bottom(self, this: &mut QItemSelectionRange) -> i32;
}

// proto: int QItemSelectionRange::bottom();
impl<'a> /*trait*/ QItemSelectionRange_bottom for () {
  fn bottom(self, this: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange6bottomEv()};
    unsafe {_ZNK19QItemSelectionRange6bottomEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn indexes<T: QItemSelectionRange_indexes>(&mut self, value: T) -> i32 {
    value.indexes(self);
    return 1;
  }
}

pub trait QItemSelectionRange_indexes {
  fn indexes(self, this: &mut QItemSelectionRange) -> i32;
}

// proto: QList<QModelIndex> QItemSelectionRange::indexes();
impl<'a> /*trait*/ QItemSelectionRange_indexes for () {
  fn indexes(self, this: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange7indexesEv()};
    unsafe {_ZNK19QItemSelectionRange7indexesEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn isValid<T: QItemSelectionRange_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QItemSelectionRange_isValid {
  fn isValid(self, this: &mut QItemSelectionRange) -> i32;
}

// proto: bool QItemSelectionRange::isValid();
impl<'a> /*trait*/ QItemSelectionRange_isValid for () {
  fn isValid(self, this: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange7isValidEv()};
    unsafe {_ZNK19QItemSelectionRange7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn model<T: QItemSelectionRange_model>(&mut self, value: T) -> i32 {
    value.model(self);
    return 1;
  }
}

pub trait QItemSelectionRange_model {
  fn model(self, this: &mut QItemSelectionRange) -> i32;
}

// proto: const QAbstractItemModel * QItemSelectionRange::model();
impl<'a> /*trait*/ QItemSelectionRange_model for () {
  fn model(self, this: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange5modelEv()};
    unsafe {_ZNK19QItemSelectionRange5modelEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn height<T: QItemSelectionRange_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QItemSelectionRange_height {
  fn height(self, this: &mut QItemSelectionRange) -> i32;
}

// proto: int QItemSelectionRange::height();
impl<'a> /*trait*/ QItemSelectionRange_height for () {
  fn height(self, this: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange6heightEv()};
    unsafe {_ZNK19QItemSelectionRange6heightEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn right<T: QItemSelectionRange_right>(&mut self, value: T) -> i32 {
    value.right(self);
    return 1;
  }
}

pub trait QItemSelectionRange_right {
  fn right(self, this: &mut QItemSelectionRange) -> i32;
}

// proto: int QItemSelectionRange::right();
impl<'a> /*trait*/ QItemSelectionRange_right for () {
  fn right(self, this: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange5rightEv()};
    unsafe {_ZNK19QItemSelectionRange5rightEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn parent<T: QItemSelectionRange_parent>(&mut self, value: T) -> i32 {
    value.parent(self);
    return 1;
  }
}

pub trait QItemSelectionRange_parent {
  fn parent(self, this: &mut QItemSelectionRange) -> i32;
}

// proto: QModelIndex QItemSelectionRange::parent();
impl<'a> /*trait*/ QItemSelectionRange_parent for () {
  fn parent(self, this: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange6parentEv()};
    unsafe {_ZNK19QItemSelectionRange6parentEv()};
    return 1;
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
    value.width(self);
    return 1;
  }
}

pub trait QItemSelectionRange_width {
  fn width(self, this: &mut QItemSelectionRange) -> i32;
}

// proto: int QItemSelectionRange::width();
impl<'a> /*trait*/ QItemSelectionRange_width for () {
  fn width(self, this: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange5widthEv()};
    unsafe {_ZNK19QItemSelectionRange5widthEv()};
    return 1;
  }
}

// proto: void QItemSelectionRange::NewQItemSelectionRange(const QModelIndex & topLeft, const QModelIndex & bottomRight);
impl<'a> /*trait*/ QItemSelectionRange_NewQItemSelectionRange for (&'a  QModelIndex, &'a  QModelIndex) {
  fn NewQItemSelectionRange(self) -> QItemSelectionRange {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionRangeC1ERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN19QItemSelectionRangeC1ERK11QModelIndexS2_(qthis, arg0, arg1)};
    let rsthis = QItemSelectionRange{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn topLeft<T: QItemSelectionRange_topLeft>(&mut self, value: T) -> i32 {
    value.topLeft(self);
    return 1;
  }
}

pub trait QItemSelectionRange_topLeft {
  fn topLeft(self, this: &mut QItemSelectionRange) -> i32;
}

// proto: const QPersistentModelIndex & QItemSelectionRange::topLeft();
impl<'a> /*trait*/ QItemSelectionRange_topLeft for () {
  fn topLeft(self, this: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange7topLeftEv()};
    unsafe {_ZNK19QItemSelectionRange7topLeftEv()};
    return 1;
  }
}

// proto: bool QItemSelectionRange::contains(int row, int column, const QModelIndex & parentIndex);
impl<'a> /*trait*/ QItemSelectionRange_contains for (i32, i32, &'a  QModelIndex) {
  fn contains(self, this: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange8containsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZNK19QItemSelectionRange8containsEiiRK11QModelIndex(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QItemSelectionRange::NewQItemSelectionRange(const QModelIndex & index);
impl<'a> /*trait*/ QItemSelectionRange_NewQItemSelectionRange for (&'a  QModelIndex) {
  fn NewQItemSelectionRange(self) -> QItemSelectionRange {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionRangeC1ERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QItemSelectionRangeC1ERK11QModelIndex(qthis, arg0)};
    let rsthis = QItemSelectionRange{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn intersects<T: QItemSelectionRange_intersects>(&mut self, value: T) -> i32 {
    value.intersects(self);
    return 1;
  }
}

pub trait QItemSelectionRange_intersects {
  fn intersects(self, this: &mut QItemSelectionRange) -> i32;
}

// proto: bool QItemSelectionRange::intersects(const QItemSelectionRange & other);
impl<'a> /*trait*/ QItemSelectionRange_intersects for (&'a  QItemSelectionRange) {
  fn intersects(self, this: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange10intersectsERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK19QItemSelectionRange10intersectsERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn bottomRight<T: QItemSelectionRange_bottomRight>(&mut self, value: T) -> i32 {
    value.bottomRight(self);
    return 1;
  }
}

pub trait QItemSelectionRange_bottomRight {
  fn bottomRight(self, this: &mut QItemSelectionRange) -> i32;
}

// proto: const QPersistentModelIndex & QItemSelectionRange::bottomRight();
impl<'a> /*trait*/ QItemSelectionRange_bottomRight for () {
  fn bottomRight(self, this: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange11bottomRightEv()};
    unsafe {_ZNK19QItemSelectionRange11bottomRightEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn top<T: QItemSelectionRange_top>(&mut self, value: T) -> i32 {
    value.top(self);
    return 1;
  }
}

pub trait QItemSelectionRange_top {
  fn top(self, this: &mut QItemSelectionRange) -> i32;
}

// proto: int QItemSelectionRange::top();
impl<'a> /*trait*/ QItemSelectionRange_top for () {
  fn top(self, this: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange3topEv()};
    unsafe {_ZNK19QItemSelectionRange3topEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionRange {
  pub fn isEmpty<T: QItemSelectionRange_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QItemSelectionRange_isEmpty {
  fn isEmpty(self, this: &mut QItemSelectionRange) -> i32;
}

// proto: bool QItemSelectionRange::isEmpty();
impl<'a> /*trait*/ QItemSelectionRange_isEmpty for () {
  fn isEmpty(self, this: &mut QItemSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionRange7isEmptyEv()};
    unsafe {_ZNK19QItemSelectionRange7isEmptyEv()};
    return 1;
  }
}

// proto: void QItemSelectionRange::NewQItemSelectionRange(const QItemSelectionRange & other);
impl<'a> /*trait*/ QItemSelectionRange_NewQItemSelectionRange for (&'a  QItemSelectionRange) {
  fn NewQItemSelectionRange(self) -> QItemSelectionRange {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionRangeC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QItemSelectionRangeC1ERKS_(qthis, arg0)};
    let rsthis = QItemSelectionRange{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

