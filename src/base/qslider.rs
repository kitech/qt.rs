// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qevent::QEvent;
use super::qsize::QSize;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const QMetaObject * QSlider::metaObject();
  fn _ZNK7QSlider10metaObjectEv(qthis: *mut c_void);
  // proto:  bool QSlider::event(QEvent * event);
  fn _ZN7QSlider5eventEP6QEvent(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  int QSlider::tickInterval();
  fn _ZNK7QSlider12tickIntervalEv(qthis: *mut c_void) -> c_int;
  // proto:  QSize QSlider::sizeHint();
  fn _ZNK7QSlider8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSlider::setTickInterval(int ti);
  fn _ZN7QSlider15setTickIntervalEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QSlider::QSlider(const QSlider & );
  fn _ZN7QSliderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSlider::~QSlider();
  fn _ZN7QSliderD0Ev(qthis: *mut c_void);
  // proto:  void QSlider::QSlider(QWidget * parent);
  fn _ZN7QSliderC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSize QSlider::minimumSizeHint();
  fn _ZNK7QSlider15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QSlider)=1
pub struct QSlider {
  pub qclsinst: *mut c_void,
}

  // proto:  const QMetaObject * QSlider::metaObject();
impl /*struct*/ QSlider {
  pub fn metaObject<RetType, T: QSlider_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSlider_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QSlider) -> RetType;
}

  // proto:  const QMetaObject * QSlider::metaObject();
impl<'a> /*trait*/ QSlider_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSlider10metaObjectEv()};
     unsafe {_ZNK7QSlider10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QSlider::event(QEvent * event);
impl /*struct*/ QSlider {
  pub fn event<RetType, T: QSlider_event<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.event(self);
    // return 1;
  }
}

pub trait QSlider_event<RetType> {
  fn event(self , rsthis: &mut QSlider) -> RetType;
}

  // proto:  bool QSlider::event(QEvent * event);
impl<'a> /*trait*/ QSlider_event<i8> for (QEvent) {
  fn event(self , rsthis: &mut QSlider) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSlider5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QSlider5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QSlider::tickInterval();
impl /*struct*/ QSlider {
  pub fn tickInterval<RetType, T: QSlider_tickInterval<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tickInterval(self);
    // return 1;
  }
}

pub trait QSlider_tickInterval<RetType> {
  fn tickInterval(self , rsthis: &mut QSlider) -> RetType;
}

  // proto:  int QSlider::tickInterval();
impl<'a> /*trait*/ QSlider_tickInterval<i32> for () {
  fn tickInterval(self , rsthis: &mut QSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSlider12tickIntervalEv()};
    let mut ret = unsafe {_ZNK7QSlider12tickIntervalEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSize QSlider::sizeHint();
impl /*struct*/ QSlider {
  pub fn sizeHint<RetType, T: QSlider_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QSlider_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QSlider) -> RetType;
}

  // proto:  QSize QSlider::sizeHint();
impl<'a> /*trait*/ QSlider_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QSlider) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSlider8sizeHintEv()};
    let mut ret = unsafe {_ZNK7QSlider8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QSlider::setTickInterval(int ti);
impl /*struct*/ QSlider {
  pub fn setTickInterval<RetType, T: QSlider_setTickInterval<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTickInterval(self);
    // return 1;
  }
}

pub trait QSlider_setTickInterval<RetType> {
  fn setTickInterval(self , rsthis: &mut QSlider) -> RetType;
}

  // proto:  void QSlider::setTickInterval(int ti);
impl<'a> /*trait*/ QSlider_setTickInterval<()> for (i32) {
  fn setTickInterval(self , rsthis: &mut QSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSlider15setTickIntervalEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QSlider15setTickIntervalEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSlider::QSlider(const QSlider & );
impl /*struct*/ QSlider {
  pub fn NewQSlider<T: QSlider_NewQSlider>(value: T) -> QSlider {
    let rsthis = value.NewQSlider();
    return rsthis;
    // return 1;
  }
}

pub trait QSlider_NewQSlider {
  fn NewQSlider(self) -> QSlider;
}

  // proto:  void QSlider::QSlider(const QSlider & );
impl<'a> /*trait*/ QSlider_NewQSlider for (QSlider) {
  fn NewQSlider(self) -> QSlider {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSliderC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QSliderC1ERKS_(qthis, arg0)};
    let rsthis = QSlider{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSlider::~QSlider();
impl /*struct*/ QSlider {
  pub fn FreeQSlider<RetType, T: QSlider_FreeQSlider<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQSlider(self);
    // return 1;
  }
}

pub trait QSlider_FreeQSlider<RetType> {
  fn FreeQSlider(self , rsthis: &mut QSlider) -> RetType;
}

  // proto:  void QSlider::~QSlider();
impl<'a> /*trait*/ QSlider_FreeQSlider<()> for () {
  fn FreeQSlider(self , rsthis: &mut QSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSliderD0Ev()};
     unsafe {_ZN7QSliderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSlider::QSlider(QWidget * parent);
impl<'a> /*trait*/ QSlider_NewQSlider for (QWidget) {
  fn NewQSlider(self) -> QSlider {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSliderC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QSliderC1EP7QWidget(qthis, arg0)};
    let rsthis = QSlider{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSize QSlider::minimumSizeHint();
impl /*struct*/ QSlider {
  pub fn minimumSizeHint<RetType, T: QSlider_minimumSizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QSlider_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: &mut QSlider) -> RetType;
}

  // proto:  QSize QSlider::minimumSizeHint();
impl<'a> /*trait*/ QSlider_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: &mut QSlider) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSlider15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK7QSlider15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

