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
  // proto:  void QGraphicsSceneEvent::QGraphicsSceneEvent(const QGraphicsSceneEvent & );
  fn _ZN19QGraphicsSceneEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QWidget * QGraphicsSceneEvent::widget();
  fn _ZNK19QGraphicsSceneEvent6widgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneEvent::setWidget(QWidget * widget);
  fn _ZN19QGraphicsSceneEvent9setWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsSceneEvent::~QGraphicsSceneEvent();
  fn _ZN19QGraphicsSceneEventD0Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QGraphicsSceneEvent)=1
pub struct QGraphicsSceneEvent {
  pub qclsinst: *mut c_void,
}

  // proto:  void QGraphicsSceneEvent::QGraphicsSceneEvent(const QGraphicsSceneEvent & );
impl /*struct*/ QGraphicsSceneEvent {
  pub fn NewQGraphicsSceneEvent<T: QGraphicsSceneEvent_NewQGraphicsSceneEvent>(value: T) -> QGraphicsSceneEvent {
    let rsthis = value.NewQGraphicsSceneEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneEvent_NewQGraphicsSceneEvent {
  fn NewQGraphicsSceneEvent(self) -> QGraphicsSceneEvent;
}

  // proto:  void QGraphicsSceneEvent::QGraphicsSceneEvent(const QGraphicsSceneEvent & );
impl<'a> /*trait*/ QGraphicsSceneEvent_NewQGraphicsSceneEvent for (QGraphicsSceneEvent) {
  fn NewQGraphicsSceneEvent(self) -> QGraphicsSceneEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsSceneEventC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QGraphicsSceneEventC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsSceneEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QWidget * QGraphicsSceneEvent::widget();
impl /*struct*/ QGraphicsSceneEvent {
  pub fn widget<RetType, T: QGraphicsSceneEvent_widget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QGraphicsSceneEvent_widget<RetType> {
  fn widget(self , rsthis: &mut QGraphicsSceneEvent) -> RetType;
}

  // proto:  QWidget * QGraphicsSceneEvent::widget();
impl<'a> /*trait*/ QGraphicsSceneEvent_widget<QWidget> for () {
  fn widget(self , rsthis: &mut QGraphicsSceneEvent) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsSceneEvent6widgetEv()};
    let mut ret = unsafe {_ZNK19QGraphicsSceneEvent6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneEvent::setWidget(QWidget * widget);
impl /*struct*/ QGraphicsSceneEvent {
  pub fn setWidget<RetType, T: QGraphicsSceneEvent_setWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setWidget(self);
    // return 1;
  }
}

pub trait QGraphicsSceneEvent_setWidget<RetType> {
  fn setWidget(self , rsthis: &mut QGraphicsSceneEvent) -> RetType;
}

  // proto:  void QGraphicsSceneEvent::setWidget(QWidget * widget);
impl<'a> /*trait*/ QGraphicsSceneEvent_setWidget<()> for (QWidget) {
  fn setWidget(self , rsthis: &mut QGraphicsSceneEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsSceneEvent9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsSceneEvent9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneEvent::~QGraphicsSceneEvent();
impl /*struct*/ QGraphicsSceneEvent {
  pub fn FreeQGraphicsSceneEvent<RetType, T: QGraphicsSceneEvent_FreeQGraphicsSceneEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsSceneEvent(self);
    // return 1;
  }
}

pub trait QGraphicsSceneEvent_FreeQGraphicsSceneEvent<RetType> {
  fn FreeQGraphicsSceneEvent(self , rsthis: &mut QGraphicsSceneEvent) -> RetType;
}

  // proto:  void QGraphicsSceneEvent::~QGraphicsSceneEvent();
impl<'a> /*trait*/ QGraphicsSceneEvent_FreeQGraphicsSceneEvent<()> for () {
  fn FreeQGraphicsSceneEvent(self , rsthis: &mut QGraphicsSceneEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsSceneEventD0Ev()};
     unsafe {_ZN19QGraphicsSceneEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

