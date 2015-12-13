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
  // proto: void QGraphicsSceneEvent::NewQGraphicsSceneEvent(const QGraphicsSceneEvent & );
  fn _ZN19QGraphicsSceneEventC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QWidget * QGraphicsSceneEvent::widget();
  fn _ZNK19QGraphicsSceneEvent6widgetEv() -> i32;
  // proto: void QGraphicsSceneEvent::setWidget(QWidget * widget);
  fn _ZN19QGraphicsSceneEvent9setWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QGraphicsSceneEvent::FreeQGraphicsSceneEvent();
  fn _ZN19QGraphicsSceneEventD0Ev() -> i32;
}

// body block begin
// class sizeof(QGraphicsSceneEvent)=1
pub struct QGraphicsSceneEvent {
  pub qclsinst: *mut c_void,
}

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

// proto: void QGraphicsSceneEvent::NewQGraphicsSceneEvent(const QGraphicsSceneEvent & );
impl<'a> /*trait*/ QGraphicsSceneEvent_NewQGraphicsSceneEvent for (&'a  QGraphicsSceneEvent) {
  fn NewQGraphicsSceneEvent(self) -> QGraphicsSceneEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsSceneEventC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QGraphicsSceneEventC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsSceneEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneEvent {
  pub fn widget<T: QGraphicsSceneEvent_widget>(&mut self, value: T) -> i32 {
    value.widget(self);
    return 1;
  }
}

pub trait QGraphicsSceneEvent_widget {
  fn widget(self, this: &mut QGraphicsSceneEvent) -> i32;
}

// proto: QWidget * QGraphicsSceneEvent::widget();
impl<'a> /*trait*/ QGraphicsSceneEvent_widget for () {
  fn widget(self, this: &mut QGraphicsSceneEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsSceneEvent6widgetEv()};
    unsafe {_ZNK19QGraphicsSceneEvent6widgetEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneEvent {
  pub fn setWidget<T: QGraphicsSceneEvent_setWidget>(&mut self, value: T) -> i32 {
    value.setWidget(self);
    return 1;
  }
}

pub trait QGraphicsSceneEvent_setWidget {
  fn setWidget(self, this: &mut QGraphicsSceneEvent) -> i32;
}

// proto: void QGraphicsSceneEvent::setWidget(QWidget * widget);
impl<'a> /*trait*/ QGraphicsSceneEvent_setWidget for (&'a mut QWidget) {
  fn setWidget(self, this: &mut QGraphicsSceneEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsSceneEvent9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QGraphicsSceneEvent9setWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneEvent {
  pub fn FreeQGraphicsSceneEvent<T: QGraphicsSceneEvent_FreeQGraphicsSceneEvent>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsSceneEvent(self);
    return 1;
  }
}

pub trait QGraphicsSceneEvent_FreeQGraphicsSceneEvent {
  fn FreeQGraphicsSceneEvent(self, this: &mut QGraphicsSceneEvent) -> i32;
}

// proto: void QGraphicsSceneEvent::FreeQGraphicsSceneEvent();
impl<'a> /*trait*/ QGraphicsSceneEvent_FreeQGraphicsSceneEvent for () {
  fn FreeQGraphicsSceneEvent(self, this: &mut QGraphicsSceneEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsSceneEventD0Ev()};
    unsafe {_ZN19QGraphicsSceneEventD0Ev()};
    return 1;
  }
}

