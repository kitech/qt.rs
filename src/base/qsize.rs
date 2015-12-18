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
  // proto:  QSize QSize::boundedTo(const QSize & );
  fn _ZNK5QSize9boundedToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QSize::isValid();
  fn _ZNK5QSize7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QSize::isNull();
  fn _ZNK5QSize6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QSize::NewQSize();
  fn _ZN5QSizeC1Ev(qthis: *mut c_void) ;
  // proto:  QSize QSize::expandedTo(const QSize & );
  fn _ZNK5QSize10expandedToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QSize::height();
  fn _ZNK5QSize6heightEv(qthis: *mut c_void) -> c_int;
  // proto:  int & QSize::rheight();
  fn _ZN5QSize7rheightEv(qthis: *mut c_void) ;
  // proto:  void QSize::NewQSize(int w, int h);
  fn _ZN5QSizeC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  int QSize::width();
  fn _ZNK5QSize5widthEv(qthis: *mut c_void) -> c_int;
  // proto:  QSize QSize::transposed();
  fn _ZNK5QSize10transposedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int & QSize::rwidth();
  fn _ZN5QSize6rwidthEv(qthis: *mut c_void) ;
  // proto:  void QSize::setHeight(int h);
  fn _ZN5QSize9setHeightEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QSize::isEmpty();
  fn _ZNK5QSize7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QSize::setWidth(int w);
  fn _ZN5QSize8setWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QSize::transpose();
  fn _ZN5QSize9transposeEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QSize)=8
pub struct QSize {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSize {
  pub fn boundedTo<RetType, T: QSize_boundedTo<RetType>>(&mut self, value: T) -> RetType {
    return value.boundedTo(self);
    // return 1;
  }
}

pub trait QSize_boundedTo<RetType> {
  fn boundedTo(self, rsthis: &mut QSize) -> RetType;
}

// proto:  QSize QSize::boundedTo(const QSize & );
impl<'a> /*trait*/ QSize_boundedTo<QSize> for (&'a  QSize) {
  fn boundedTo(self, rsthis: &mut QSize) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize9boundedToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QSize9boundedToERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn isValid<RetType, T: QSize_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QSize_isValid<RetType> {
  fn isValid(self, rsthis: &mut QSize) -> RetType;
}

// proto:  bool QSize::isValid();
impl<'a> /*trait*/ QSize_isValid<i8> for () {
  fn isValid(self, rsthis: &mut QSize) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize7isValidEv()};
    let mut ret = unsafe {_ZNK5QSize7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn isNull<RetType, T: QSize_isNull<RetType>>(&mut self, value: T) -> RetType {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QSize_isNull<RetType> {
  fn isNull(self, rsthis: &mut QSize) -> RetType;
}

// proto:  bool QSize::isNull();
impl<'a> /*trait*/ QSize_isNull<i8> for () {
  fn isNull(self, rsthis: &mut QSize) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize6isNullEv()};
    let mut ret = unsafe {_ZNK5QSize6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn NewQSize<T: QSize_NewQSize>(value: T) -> QSize {
    let rsthis = value.NewQSize();
    return rsthis;
    // return 1;
  }
}

pub trait QSize_NewQSize {
  fn NewQSize(self) -> QSize;
}

// proto: void QSize::NewQSize();
impl<'a> /*trait*/ QSize_NewQSize for () {
  fn NewQSize(self) -> QSize {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSizeC1Ev()};
    unsafe {_ZN5QSizeC1Ev(qthis)};
    let rsthis = QSize{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn expandedTo<RetType, T: QSize_expandedTo<RetType>>(&mut self, value: T) -> RetType {
    return value.expandedTo(self);
    // return 1;
  }
}

pub trait QSize_expandedTo<RetType> {
  fn expandedTo(self, rsthis: &mut QSize) -> RetType;
}

// proto:  QSize QSize::expandedTo(const QSize & );
impl<'a> /*trait*/ QSize_expandedTo<QSize> for (&'a  QSize) {
  fn expandedTo(self, rsthis: &mut QSize) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize10expandedToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QSize10expandedToERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn height<RetType, T: QSize_height<RetType>>(&mut self, value: T) -> RetType {
    return value.height(self);
    // return 1;
  }
}

pub trait QSize_height<RetType> {
  fn height(self, rsthis: &mut QSize) -> RetType;
}

// proto:  int QSize::height();
impl<'a> /*trait*/ QSize_height<i32> for () {
  fn height(self, rsthis: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize6heightEv()};
    let mut ret = unsafe {_ZNK5QSize6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn rheight<RetType, T: QSize_rheight<RetType>>(&mut self, value: T) -> RetType {
    return value.rheight(self);
    // return 1;
  }
}

pub trait QSize_rheight<RetType> {
  fn rheight(self, rsthis: &mut QSize) -> RetType;
}

// proto:  int & QSize::rheight();
impl<'a> /*trait*/ QSize_rheight<()> for () {
  fn rheight(self, rsthis: &mut QSize) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize7rheightEv()};
     unsafe {_ZN5QSize7rheightEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QSize::NewQSize(int w, int h);
impl<'a> /*trait*/ QSize_NewQSize for (i32, i32) {
  fn NewQSize(self) -> QSize {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSizeC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN5QSizeC1Eii(qthis, arg0, arg1)};
    let rsthis = QSize{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn width<RetType, T: QSize_width<RetType>>(&mut self, value: T) -> RetType {
    return value.width(self);
    // return 1;
  }
}

pub trait QSize_width<RetType> {
  fn width(self, rsthis: &mut QSize) -> RetType;
}

// proto:  int QSize::width();
impl<'a> /*trait*/ QSize_width<i32> for () {
  fn width(self, rsthis: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize5widthEv()};
    let mut ret = unsafe {_ZNK5QSize5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn transposed<RetType, T: QSize_transposed<RetType>>(&mut self, value: T) -> RetType {
    return value.transposed(self);
    // return 1;
  }
}

pub trait QSize_transposed<RetType> {
  fn transposed(self, rsthis: &mut QSize) -> RetType;
}

// proto:  QSize QSize::transposed();
impl<'a> /*trait*/ QSize_transposed<QSize> for () {
  fn transposed(self, rsthis: &mut QSize) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize10transposedEv()};
    let mut ret = unsafe {_ZNK5QSize10transposedEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn rwidth<RetType, T: QSize_rwidth<RetType>>(&mut self, value: T) -> RetType {
    return value.rwidth(self);
    // return 1;
  }
}

pub trait QSize_rwidth<RetType> {
  fn rwidth(self, rsthis: &mut QSize) -> RetType;
}

// proto:  int & QSize::rwidth();
impl<'a> /*trait*/ QSize_rwidth<()> for () {
  fn rwidth(self, rsthis: &mut QSize) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize6rwidthEv()};
     unsafe {_ZN5QSize6rwidthEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn setHeight<RetType, T: QSize_setHeight<RetType>>(&mut self, value: T) -> RetType {
    return value.setHeight(self);
    // return 1;
  }
}

pub trait QSize_setHeight<RetType> {
  fn setHeight(self, rsthis: &mut QSize) -> RetType;
}

// proto:  void QSize::setHeight(int h);
impl<'a> /*trait*/ QSize_setHeight<()> for (i32) {
  fn setHeight(self, rsthis: &mut QSize) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize9setHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QSize9setHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn isEmpty<RetType, T: QSize_isEmpty<RetType>>(&mut self, value: T) -> RetType {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QSize_isEmpty<RetType> {
  fn isEmpty(self, rsthis: &mut QSize) -> RetType;
}

// proto:  bool QSize::isEmpty();
impl<'a> /*trait*/ QSize_isEmpty<i8> for () {
  fn isEmpty(self, rsthis: &mut QSize) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize7isEmptyEv()};
    let mut ret = unsafe {_ZNK5QSize7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn setWidth<RetType, T: QSize_setWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.setWidth(self);
    // return 1;
  }
}

pub trait QSize_setWidth<RetType> {
  fn setWidth(self, rsthis: &mut QSize) -> RetType;
}

// proto:  void QSize::setWidth(int w);
impl<'a> /*trait*/ QSize_setWidth<()> for (i32) {
  fn setWidth(self, rsthis: &mut QSize) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize8setWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QSize8setWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn transpose<RetType, T: QSize_transpose<RetType>>(&mut self, value: T) -> RetType {
    return value.transpose(self);
    // return 1;
  }
}

pub trait QSize_transpose<RetType> {
  fn transpose(self, rsthis: &mut QSize) -> RetType;
}

// proto:  void QSize::transpose();
impl<'a> /*trait*/ QSize_transpose<()> for () {
  fn transpose(self, rsthis: &mut QSize) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize9transposeEv()};
     unsafe {_ZN5QSize9transposeEv(rsthis.qclsinst)};
    // return 1;
  }
}

