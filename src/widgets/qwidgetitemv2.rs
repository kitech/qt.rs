// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsize::QSize;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QSize QWidgetItemV2::sizeHint();
  fn _ZNK13QWidgetItemV28sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QWidgetItemV2::minimumSize();
  fn _ZNK13QWidgetItemV211minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QWidgetItemV2::heightForWidth(int width);
  fn _ZNK13QWidgetItemV214heightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QWidgetItemV2::~QWidgetItemV2();
  fn _ZN13QWidgetItemV2D0Ev(qthis: *mut c_void);
  // proto:  void QWidgetItemV2::QWidgetItemV2(QWidget * widget);
  fn _ZN13QWidgetItemV2C1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSize QWidgetItemV2::maximumSize();
  fn _ZNK13QWidgetItemV211maximumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidgetItemV2::QWidgetItemV2(const QWidgetItemV2 & );
  fn _ZN13QWidgetItemV2C1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QWidgetItemV2)=1
pub struct QWidgetItemV2 {
  pub qclsinst: *mut c_void,
}

  // proto:  QSize QWidgetItemV2::sizeHint();
impl /*struct*/ QWidgetItemV2 {
  pub fn sizeHint<RetType, T: QWidgetItemV2_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QWidgetItemV2_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QWidgetItemV2) -> RetType;
}

  // proto:  QSize QWidgetItemV2::sizeHint();
impl<'a> /*trait*/ QWidgetItemV2_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QWidgetItemV2) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetItemV28sizeHintEv()};
    let mut ret = unsafe {_ZNK13QWidgetItemV28sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QWidgetItemV2::minimumSize();
impl /*struct*/ QWidgetItemV2 {
  pub fn minimumSize<RetType, T: QWidgetItemV2_minimumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QWidgetItemV2_minimumSize<RetType> {
  fn minimumSize(self , rsthis: &mut QWidgetItemV2) -> RetType;
}

  // proto:  QSize QWidgetItemV2::minimumSize();
impl<'a> /*trait*/ QWidgetItemV2_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: &mut QWidgetItemV2) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetItemV211minimumSizeEv()};
    let mut ret = unsafe {_ZNK13QWidgetItemV211minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QWidgetItemV2::heightForWidth(int width);
impl /*struct*/ QWidgetItemV2 {
  pub fn heightForWidth<RetType, T: QWidgetItemV2_heightForWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QWidgetItemV2_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: &mut QWidgetItemV2) -> RetType;
}

  // proto:  int QWidgetItemV2::heightForWidth(int width);
impl<'a> /*trait*/ QWidgetItemV2_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: &mut QWidgetItemV2) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetItemV214heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QWidgetItemV214heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QWidgetItemV2::~QWidgetItemV2();
impl /*struct*/ QWidgetItemV2 {
  pub fn FreeQWidgetItemV2<RetType, T: QWidgetItemV2_FreeQWidgetItemV2<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQWidgetItemV2(self);
    // return 1;
  }
}

pub trait QWidgetItemV2_FreeQWidgetItemV2<RetType> {
  fn FreeQWidgetItemV2(self , rsthis: &mut QWidgetItemV2) -> RetType;
}

  // proto:  void QWidgetItemV2::~QWidgetItemV2();
impl<'a> /*trait*/ QWidgetItemV2_FreeQWidgetItemV2<()> for () {
  fn FreeQWidgetItemV2(self , rsthis: &mut QWidgetItemV2) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetItemV2D0Ev()};
     unsafe {_ZN13QWidgetItemV2D0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidgetItemV2::QWidgetItemV2(QWidget * widget);
impl /*struct*/ QWidgetItemV2 {
  pub fn NewQWidgetItemV2<T: QWidgetItemV2_NewQWidgetItemV2>(value: T) -> QWidgetItemV2 {
    let rsthis = value.NewQWidgetItemV2();
    return rsthis;
    // return 1;
  }
}

pub trait QWidgetItemV2_NewQWidgetItemV2 {
  fn NewQWidgetItemV2(self) -> QWidgetItemV2;
}

  // proto:  void QWidgetItemV2::QWidgetItemV2(QWidget * widget);
impl<'a> /*trait*/ QWidgetItemV2_NewQWidgetItemV2 for (QWidget) {
  fn NewQWidgetItemV2(self) -> QWidgetItemV2 {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetItemV2C1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QWidgetItemV2C1EP7QWidget(qthis, arg0)};
    let rsthis = QWidgetItemV2{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSize QWidgetItemV2::maximumSize();
impl /*struct*/ QWidgetItemV2 {
  pub fn maximumSize<RetType, T: QWidgetItemV2_maximumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QWidgetItemV2_maximumSize<RetType> {
  fn maximumSize(self , rsthis: &mut QWidgetItemV2) -> RetType;
}

  // proto:  QSize QWidgetItemV2::maximumSize();
impl<'a> /*trait*/ QWidgetItemV2_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: &mut QWidgetItemV2) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetItemV211maximumSizeEv()};
    let mut ret = unsafe {_ZNK13QWidgetItemV211maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidgetItemV2::QWidgetItemV2(const QWidgetItemV2 & );
impl<'a> /*trait*/ QWidgetItemV2_NewQWidgetItemV2 for (QWidgetItemV2) {
  fn NewQWidgetItemV2(self) -> QWidgetItemV2 {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetItemV2C1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QWidgetItemV2C1ERKS_(qthis, arg0)};
    let rsthis = QWidgetItemV2{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

