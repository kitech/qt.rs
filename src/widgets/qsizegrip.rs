// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QSizeGrip::QSizeGrip(const QSizeGrip & );
  fn _ZN9QSizeGripC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSizeGrip::QSizeGrip(QWidget * parent);
  fn _ZN9QSizeGripC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSizeGrip::~QSizeGrip();
  fn _ZN9QSizeGripD0Ev(qthis: *mut c_void);
  // proto:  void QSizeGrip::setVisible(bool );
  fn _ZN9QSizeGrip10setVisibleEb(qthis: *mut c_void, arg0: c_char);
  // proto:  const QMetaObject * QSizeGrip::metaObject();
  fn _ZNK9QSizeGrip10metaObjectEv(qthis: *mut c_void);
  // proto:  QSize QSizeGrip::sizeHint();
  fn _ZNK9QSizeGrip8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QSizeGrip)=1
pub struct QSizeGrip {
  pub qclsinst: *mut c_void,
}

  // proto:  void QSizeGrip::QSizeGrip(const QSizeGrip & );
impl /*struct*/ QSizeGrip {
  pub fn NewQSizeGrip<T: QSizeGrip_NewQSizeGrip>(value: T) -> QSizeGrip {
    let rsthis = value.NewQSizeGrip();
    return rsthis;
    // return 1;
  }
}

pub trait QSizeGrip_NewQSizeGrip {
  fn NewQSizeGrip(self) -> QSizeGrip;
}

  // proto:  void QSizeGrip::QSizeGrip(const QSizeGrip & );
impl<'a> /*trait*/ QSizeGrip_NewQSizeGrip for (QSizeGrip) {
  fn NewQSizeGrip(self) -> QSizeGrip {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSizeGripC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QSizeGripC1ERKS_(qthis, arg0)};
    let rsthis = QSizeGrip{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSizeGrip::QSizeGrip(QWidget * parent);
impl<'a> /*trait*/ QSizeGrip_NewQSizeGrip for (QWidget) {
  fn NewQSizeGrip(self) -> QSizeGrip {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSizeGripC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QSizeGripC1EP7QWidget(qthis, arg0)};
    let rsthis = QSizeGrip{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSizeGrip::~QSizeGrip();
impl /*struct*/ QSizeGrip {
  pub fn FreeQSizeGrip<RetType, T: QSizeGrip_FreeQSizeGrip<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQSizeGrip(self);
    // return 1;
  }
}

pub trait QSizeGrip_FreeQSizeGrip<RetType> {
  fn FreeQSizeGrip(self , rsthis: &mut QSizeGrip) -> RetType;
}

  // proto:  void QSizeGrip::~QSizeGrip();
impl<'a> /*trait*/ QSizeGrip_FreeQSizeGrip<()> for () {
  fn FreeQSizeGrip(self , rsthis: &mut QSizeGrip) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSizeGripD0Ev()};
     unsafe {_ZN9QSizeGripD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSizeGrip::setVisible(bool );
impl /*struct*/ QSizeGrip {
  pub fn setVisible<RetType, T: QSizeGrip_setVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QSizeGrip_setVisible<RetType> {
  fn setVisible(self , rsthis: &mut QSizeGrip) -> RetType;
}

  // proto:  void QSizeGrip::setVisible(bool );
impl<'a> /*trait*/ QSizeGrip_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: &mut QSizeGrip) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSizeGrip10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QSizeGrip10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QSizeGrip::metaObject();
impl /*struct*/ QSizeGrip {
  pub fn metaObject<RetType, T: QSizeGrip_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSizeGrip_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QSizeGrip) -> RetType;
}

  // proto:  const QMetaObject * QSizeGrip::metaObject();
impl<'a> /*trait*/ QSizeGrip_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QSizeGrip) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSizeGrip10metaObjectEv()};
     unsafe {_ZNK9QSizeGrip10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QSizeGrip::sizeHint();
impl /*struct*/ QSizeGrip {
  pub fn sizeHint<RetType, T: QSizeGrip_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QSizeGrip_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QSizeGrip) -> RetType;
}

  // proto:  QSize QSizeGrip::sizeHint();
impl<'a> /*trait*/ QSizeGrip_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QSizeGrip) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSizeGrip8sizeHintEv()};
    let mut ret = unsafe {_ZNK9QSizeGrip8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}
