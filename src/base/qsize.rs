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
  pub fn boundedTo<T: QSize_boundedTo>(&mut self, value: T) -> QSize {
    return value.boundedTo(self);
    // return 1;
  }
}

pub trait QSize_boundedTo {
  fn boundedTo(self, rsthis: &mut QSize) -> QSize;
}

// proto:  QSize QSize::boundedTo(const QSize & );
impl<'a> /*trait*/ QSize_boundedTo for (&'a  QSize) {
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
  pub fn isValid<T: QSize_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QSize_isValid {
  fn isValid(self, rsthis: &mut QSize) -> i8;
}

// proto:  bool QSize::isValid();
impl<'a> /*trait*/ QSize_isValid for () {
  fn isValid(self, rsthis: &mut QSize) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize7isValidEv()};
    let mut ret = unsafe {_ZNK5QSize7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn isNull<T: QSize_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QSize_isNull {
  fn isNull(self, rsthis: &mut QSize) -> i8;
}

// proto:  bool QSize::isNull();
impl<'a> /*trait*/ QSize_isNull for () {
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
  pub fn expandedTo<T: QSize_expandedTo>(&mut self, value: T) -> QSize {
    return value.expandedTo(self);
    // return 1;
  }
}

pub trait QSize_expandedTo {
  fn expandedTo(self, rsthis: &mut QSize) -> QSize;
}

// proto:  QSize QSize::expandedTo(const QSize & );
impl<'a> /*trait*/ QSize_expandedTo for (&'a  QSize) {
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
  pub fn height<T: QSize_height>(&mut self, value: T) -> i32 {
    return value.height(self);
    // return 1;
  }
}

pub trait QSize_height {
  fn height(self, rsthis: &mut QSize) -> i32;
}

// proto:  int QSize::height();
impl<'a> /*trait*/ QSize_height for () {
  fn height(self, rsthis: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize6heightEv()};
    let mut ret = unsafe {_ZNK5QSize6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn rheight<T: QSize_rheight>(&mut self, value: T)  {
     value.rheight(self);
    // return 1;
  }
}

pub trait QSize_rheight {
  fn rheight(self, rsthis: &mut QSize) ;
}

// proto:  int & QSize::rheight();
impl<'a> /*trait*/ QSize_rheight for () {
  fn rheight(self, rsthis: &mut QSize)  {
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
  pub fn width<T: QSize_width>(&mut self, value: T) -> i32 {
    return value.width(self);
    // return 1;
  }
}

pub trait QSize_width {
  fn width(self, rsthis: &mut QSize) -> i32;
}

// proto:  int QSize::width();
impl<'a> /*trait*/ QSize_width for () {
  fn width(self, rsthis: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize5widthEv()};
    let mut ret = unsafe {_ZNK5QSize5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn transposed<T: QSize_transposed>(&mut self, value: T) -> QSize {
    return value.transposed(self);
    // return 1;
  }
}

pub trait QSize_transposed {
  fn transposed(self, rsthis: &mut QSize) -> QSize;
}

// proto:  QSize QSize::transposed();
impl<'a> /*trait*/ QSize_transposed for () {
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
  pub fn rwidth<T: QSize_rwidth>(&mut self, value: T)  {
     value.rwidth(self);
    // return 1;
  }
}

pub trait QSize_rwidth {
  fn rwidth(self, rsthis: &mut QSize) ;
}

// proto:  int & QSize::rwidth();
impl<'a> /*trait*/ QSize_rwidth for () {
  fn rwidth(self, rsthis: &mut QSize)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize6rwidthEv()};
     unsafe {_ZN5QSize6rwidthEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn setHeight<T: QSize_setHeight>(&mut self, value: T)  {
     value.setHeight(self);
    // return 1;
  }
}

pub trait QSize_setHeight {
  fn setHeight(self, rsthis: &mut QSize) ;
}

// proto:  void QSize::setHeight(int h);
impl<'a> /*trait*/ QSize_setHeight for (i32) {
  fn setHeight(self, rsthis: &mut QSize)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize9setHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QSize9setHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn isEmpty<T: QSize_isEmpty>(&mut self, value: T) -> i8 {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QSize_isEmpty {
  fn isEmpty(self, rsthis: &mut QSize) -> i8;
}

// proto:  bool QSize::isEmpty();
impl<'a> /*trait*/ QSize_isEmpty for () {
  fn isEmpty(self, rsthis: &mut QSize) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize7isEmptyEv()};
    let mut ret = unsafe {_ZNK5QSize7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn setWidth<T: QSize_setWidth>(&mut self, value: T)  {
     value.setWidth(self);
    // return 1;
  }
}

pub trait QSize_setWidth {
  fn setWidth(self, rsthis: &mut QSize) ;
}

// proto:  void QSize::setWidth(int w);
impl<'a> /*trait*/ QSize_setWidth for (i32) {
  fn setWidth(self, rsthis: &mut QSize)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize8setWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QSize8setWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSize {
  pub fn transpose<T: QSize_transpose>(&mut self, value: T)  {
     value.transpose(self);
    // return 1;
  }
}

pub trait QSize_transpose {
  fn transpose(self, rsthis: &mut QSize) ;
}

// proto:  void QSize::transpose();
impl<'a> /*trait*/ QSize_transpose for () {
  fn transpose(self, rsthis: &mut QSize)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize9transposeEv()};
     unsafe {_ZN5QSize9transposeEv(rsthis.qclsinst)};
    // return 1;
  }
}

