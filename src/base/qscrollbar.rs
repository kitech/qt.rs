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
  // proto:  bool QScrollBar::event(QEvent * event);
  fn _ZN10QScrollBar5eventEP6QEvent(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QScrollBar::NewQScrollBar(const QScrollBar & );
  fn _ZN10QScrollBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QScrollBar::metaObject();
  fn _ZNK10QScrollBar10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QSize QScrollBar::sizeHint();
  fn _ZNK10QScrollBar8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScrollBar::NewQScrollBar(QWidget * parent);
  fn _ZN10QScrollBarC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QScrollBar::FreeQScrollBar();
  fn _ZN10QScrollBarD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QScrollBar)=1
pub struct QScrollBar {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QScrollBar {
  pub fn event<T: QScrollBar_event>(&mut self, value: T) -> i8 {
    return value.event(self);
    // return 1;
  }
}

pub trait QScrollBar_event {
  fn event(self, rsthis: &mut QScrollBar) -> i8;
}

// proto:  bool QScrollBar::event(QEvent * event);
impl<'a> /*trait*/ QScrollBar_event for (&'a mut QEvent) {
  fn event(self, rsthis: &mut QScrollBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QScrollBar5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QScrollBar5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QScrollBar {
  pub fn NewQScrollBar<T: QScrollBar_NewQScrollBar>(value: T) -> QScrollBar {
    let rsthis = value.NewQScrollBar();
    return rsthis;
    // return 1;
  }
}

pub trait QScrollBar_NewQScrollBar {
  fn NewQScrollBar(self) -> QScrollBar;
}

// proto: void QScrollBar::NewQScrollBar(const QScrollBar & );
impl<'a> /*trait*/ QScrollBar_NewQScrollBar for (&'a  QScrollBar) {
  fn NewQScrollBar(self) -> QScrollBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QScrollBarC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QScrollBarC1ERKS_(qthis, arg0)};
    let rsthis = QScrollBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QScrollBar {
  pub fn metaObject<T: QScrollBar_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QScrollBar_metaObject {
  fn metaObject(self, rsthis: &mut QScrollBar) ;
}

// proto:  const QMetaObject * QScrollBar::metaObject();
impl<'a> /*trait*/ QScrollBar_metaObject for () {
  fn metaObject(self, rsthis: &mut QScrollBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QScrollBar10metaObjectEv()};
     unsafe {_ZNK10QScrollBar10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QScrollBar {
  pub fn sizeHint<T: QScrollBar_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QScrollBar_sizeHint {
  fn sizeHint(self, rsthis: &mut QScrollBar) -> QSize;
}

// proto:  QSize QScrollBar::sizeHint();
impl<'a> /*trait*/ QScrollBar_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QScrollBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QScrollBar8sizeHintEv()};
    let mut ret = unsafe {_ZNK10QScrollBar8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QScrollBar::NewQScrollBar(QWidget * parent);
impl<'a> /*trait*/ QScrollBar_NewQScrollBar for (&'a mut QWidget) {
  fn NewQScrollBar(self) -> QScrollBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QScrollBarC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QScrollBarC1EP7QWidget(qthis, arg0)};
    let rsthis = QScrollBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QScrollBar {
  pub fn FreeQScrollBar<T: QScrollBar_FreeQScrollBar>(&mut self, value: T)  {
     value.FreeQScrollBar(self);
    // return 1;
  }
}

pub trait QScrollBar_FreeQScrollBar {
  fn FreeQScrollBar(self, rsthis: &mut QScrollBar) ;
}

// proto:  void QScrollBar::FreeQScrollBar();
impl<'a> /*trait*/ QScrollBar_FreeQScrollBar for () {
  fn FreeQScrollBar(self, rsthis: &mut QScrollBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QScrollBarD0Ev()};
     unsafe {_ZN10QScrollBarD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

