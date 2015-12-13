// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QFocusFrame::FreeQFocusFrame();
  fn _ZN11QFocusFrameD0Ev(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QFocusFrame::metaObject();
  fn _ZNK11QFocusFrame10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QFocusFrame::NewQFocusFrame(const QFocusFrame & );
  fn _ZN11QFocusFrameC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWidget * QFocusFrame::widget();
  fn _ZNK11QFocusFrame6widgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFocusFrame::NewQFocusFrame(QWidget * parent);
  fn _ZN11QFocusFrameC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFocusFrame::setWidget(QWidget * widget);
  fn _ZN11QFocusFrame9setWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QFocusFrame)=1
pub struct QFocusFrame {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFocusFrame {
  pub fn FreeQFocusFrame<T: QFocusFrame_FreeQFocusFrame>(&mut self, value: T)  {
     value.FreeQFocusFrame(self);
    // return 1;
  }
}

pub trait QFocusFrame_FreeQFocusFrame {
  fn FreeQFocusFrame(self, rsthis: &mut QFocusFrame) ;
}

// proto:  void QFocusFrame::FreeQFocusFrame();
impl<'a> /*trait*/ QFocusFrame_FreeQFocusFrame for () {
  fn FreeQFocusFrame(self, rsthis: &mut QFocusFrame)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFocusFrameD0Ev()};
     unsafe {_ZN11QFocusFrameD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFocusFrame {
  pub fn metaObject<T: QFocusFrame_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QFocusFrame_metaObject {
  fn metaObject(self, rsthis: &mut QFocusFrame) ;
}

// proto:  const QMetaObject * QFocusFrame::metaObject();
impl<'a> /*trait*/ QFocusFrame_metaObject for () {
  fn metaObject(self, rsthis: &mut QFocusFrame)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFocusFrame10metaObjectEv()};
     unsafe {_ZNK11QFocusFrame10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFocusFrame {
  pub fn NewQFocusFrame<T: QFocusFrame_NewQFocusFrame>(value: T) -> QFocusFrame {
    let rsthis = value.NewQFocusFrame();
    return rsthis;
    // return 1;
  }
}

pub trait QFocusFrame_NewQFocusFrame {
  fn NewQFocusFrame(self) -> QFocusFrame;
}

// proto: void QFocusFrame::NewQFocusFrame(const QFocusFrame & );
impl<'a> /*trait*/ QFocusFrame_NewQFocusFrame for (&'a  QFocusFrame) {
  fn NewQFocusFrame(self) -> QFocusFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFocusFrameC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFocusFrameC1ERKS_(qthis, arg0)};
    let rsthis = QFocusFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFocusFrame {
  pub fn widget<T: QFocusFrame_widget>(&mut self, value: T) -> QWidget {
    return value.widget(self);
    // return 1;
  }
}

pub trait QFocusFrame_widget {
  fn widget(self, rsthis: &mut QFocusFrame) -> QWidget;
}

// proto:  QWidget * QFocusFrame::widget();
impl<'a> /*trait*/ QFocusFrame_widget for () {
  fn widget(self, rsthis: &mut QFocusFrame) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFocusFrame6widgetEv()};
    let mut ret = unsafe {_ZNK11QFocusFrame6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QFocusFrame::NewQFocusFrame(QWidget * parent);
impl<'a> /*trait*/ QFocusFrame_NewQFocusFrame for (&'a mut QWidget) {
  fn NewQFocusFrame(self) -> QFocusFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFocusFrameC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFocusFrameC1EP7QWidget(qthis, arg0)};
    let rsthis = QFocusFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFocusFrame {
  pub fn setWidget<T: QFocusFrame_setWidget>(&mut self, value: T)  {
     value.setWidget(self);
    // return 1;
  }
}

pub trait QFocusFrame_setWidget {
  fn setWidget(self, rsthis: &mut QFocusFrame) ;
}

// proto:  void QFocusFrame::setWidget(QWidget * widget);
impl<'a> /*trait*/ QFocusFrame_setWidget for (&'a mut QWidget) {
  fn setWidget(self, rsthis: &mut QFocusFrame)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFocusFrame9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFocusFrame9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

