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
  // proto:  void QSizeGrip::NewQSizeGrip(const QSizeGrip & );
  fn _ZN9QSizeGripC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSizeGrip::NewQSizeGrip(QWidget * parent);
  fn _ZN9QSizeGripC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSizeGrip::FreeQSizeGrip();
  fn _ZN9QSizeGripD0Ev(qthis: *mut c_void) ;
  // proto:  void QSizeGrip::setVisible(bool );
  fn _ZN9QSizeGrip10setVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  const QMetaObject * QSizeGrip::metaObject();
  fn _ZNK9QSizeGrip10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QSize QSizeGrip::sizeHint();
  fn _ZNK9QSizeGrip8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QSizeGrip)=1
pub struct QSizeGrip {
  pub qclsinst: *mut c_void,
}

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

// proto: void QSizeGrip::NewQSizeGrip(const QSizeGrip & );
impl<'a> /*trait*/ QSizeGrip_NewQSizeGrip for (&'a  QSizeGrip) {
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

// proto: void QSizeGrip::NewQSizeGrip(QWidget * parent);
impl<'a> /*trait*/ QSizeGrip_NewQSizeGrip for (&'a mut QWidget) {
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

impl /*struct*/ QSizeGrip {
  pub fn FreeQSizeGrip<T: QSizeGrip_FreeQSizeGrip>(&mut self, value: T)  {
     value.FreeQSizeGrip(self);
    // return 1;
  }
}

pub trait QSizeGrip_FreeQSizeGrip {
  fn FreeQSizeGrip(self, rsthis: &mut QSizeGrip) ;
}

// proto:  void QSizeGrip::FreeQSizeGrip();
impl<'a> /*trait*/ QSizeGrip_FreeQSizeGrip for () {
  fn FreeQSizeGrip(self, rsthis: &mut QSizeGrip)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSizeGripD0Ev()};
     unsafe {_ZN9QSizeGripD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSizeGrip {
  pub fn setVisible<T: QSizeGrip_setVisible>(&mut self, value: T)  {
     value.setVisible(self);
    // return 1;
  }
}

pub trait QSizeGrip_setVisible {
  fn setVisible(self, rsthis: &mut QSizeGrip) ;
}

// proto:  void QSizeGrip::setVisible(bool );
impl<'a> /*trait*/ QSizeGrip_setVisible for (i8) {
  fn setVisible(self, rsthis: &mut QSizeGrip)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSizeGrip10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QSizeGrip10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSizeGrip {
  pub fn metaObject<T: QSizeGrip_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QSizeGrip_metaObject {
  fn metaObject(self, rsthis: &mut QSizeGrip) ;
}

// proto:  const QMetaObject * QSizeGrip::metaObject();
impl<'a> /*trait*/ QSizeGrip_metaObject for () {
  fn metaObject(self, rsthis: &mut QSizeGrip)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSizeGrip10metaObjectEv()};
     unsafe {_ZNK9QSizeGrip10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSizeGrip {
  pub fn sizeHint<T: QSizeGrip_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QSizeGrip_sizeHint {
  fn sizeHint(self, rsthis: &mut QSizeGrip) -> QSize;
}

// proto:  QSize QSizeGrip::sizeHint();
impl<'a> /*trait*/ QSizeGrip_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QSizeGrip) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSizeGrip8sizeHintEv()};
    let mut ret = unsafe {_ZNK9QSizeGrip8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

