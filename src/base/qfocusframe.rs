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
  // proto:  void QFocusFrame::~QFocusFrame();
  fn _ZN11QFocusFrameD0Ev(qthis: *mut c_void);
  // proto:  const QMetaObject * QFocusFrame::metaObject();
  fn _ZNK11QFocusFrame10metaObjectEv(qthis: *mut c_void);
  // proto:  void QFocusFrame::QFocusFrame(const QFocusFrame & );
  fn _ZN11QFocusFrameC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QWidget * QFocusFrame::widget();
  fn _ZNK11QFocusFrame6widgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFocusFrame::QFocusFrame(QWidget * parent);
  fn _ZN11QFocusFrameC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFocusFrame::setWidget(QWidget * widget);
  fn _ZN11QFocusFrame9setWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QFocusFrame)=1
pub struct QFocusFrame {
  pub qclsinst: *mut c_void,
}

  // proto:  void QFocusFrame::~QFocusFrame();
impl /*struct*/ QFocusFrame {
  pub fn FreeQFocusFrame<RetType, T: QFocusFrame_FreeQFocusFrame<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQFocusFrame(self);
    // return 1;
  }
}

pub trait QFocusFrame_FreeQFocusFrame<RetType> {
  fn FreeQFocusFrame(self , rsthis: &mut QFocusFrame) -> RetType;
}

  // proto:  void QFocusFrame::~QFocusFrame();
impl<'a> /*trait*/ QFocusFrame_FreeQFocusFrame<()> for () {
  fn FreeQFocusFrame(self , rsthis: &mut QFocusFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFocusFrameD0Ev()};
     unsafe {_ZN11QFocusFrameD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QFocusFrame::metaObject();
impl /*struct*/ QFocusFrame {
  pub fn metaObject<RetType, T: QFocusFrame_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFocusFrame_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QFocusFrame) -> RetType;
}

  // proto:  const QMetaObject * QFocusFrame::metaObject();
impl<'a> /*trait*/ QFocusFrame_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QFocusFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFocusFrame10metaObjectEv()};
     unsafe {_ZNK11QFocusFrame10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFocusFrame::QFocusFrame(const QFocusFrame & );
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

  // proto:  void QFocusFrame::QFocusFrame(const QFocusFrame & );
impl<'a> /*trait*/ QFocusFrame_NewQFocusFrame for (QFocusFrame) {
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

  // proto:  QWidget * QFocusFrame::widget();
impl /*struct*/ QFocusFrame {
  pub fn widget<RetType, T: QFocusFrame_widget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QFocusFrame_widget<RetType> {
  fn widget(self , rsthis: &mut QFocusFrame) -> RetType;
}

  // proto:  QWidget * QFocusFrame::widget();
impl<'a> /*trait*/ QFocusFrame_widget<QWidget> for () {
  fn widget(self , rsthis: &mut QFocusFrame) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFocusFrame6widgetEv()};
    let mut ret = unsafe {_ZNK11QFocusFrame6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QFocusFrame::QFocusFrame(QWidget * parent);
impl<'a> /*trait*/ QFocusFrame_NewQFocusFrame for (QWidget) {
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

  // proto:  void QFocusFrame::setWidget(QWidget * widget);
impl /*struct*/ QFocusFrame {
  pub fn setWidget<RetType, T: QFocusFrame_setWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setWidget(self);
    // return 1;
  }
}

pub trait QFocusFrame_setWidget<RetType> {
  fn setWidget(self , rsthis: &mut QFocusFrame) -> RetType;
}

  // proto:  void QFocusFrame::setWidget(QWidget * widget);
impl<'a> /*trait*/ QFocusFrame_setWidget<()> for (QWidget) {
  fn setWidget(self , rsthis: &mut QFocusFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFocusFrame9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFocusFrame9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

