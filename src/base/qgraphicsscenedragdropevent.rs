// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qpoint::QPoint;
use super::qmimedata::QMimeData;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QWidget * QGraphicsSceneDragDropEvent::source();
  fn _ZNK27QGraphicsSceneDragDropEvent6sourceEv() -> i32;
  // proto: QPointF QGraphicsSceneDragDropEvent::scenePos();
  fn _ZNK27QGraphicsSceneDragDropEvent8scenePosEv() -> i32;
  // proto: void QGraphicsSceneDragDropEvent::setPos(const QPointF & pos);
  fn _ZN27QGraphicsSceneDragDropEvent6setPosERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsSceneDragDropEvent::NewQGraphicsSceneDragDropEvent(const QGraphicsSceneDragDropEvent & );
  fn _ZN27QGraphicsSceneDragDropEventC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QGraphicsSceneDragDropEvent::setScreenPos(const QPoint & pos);
  fn _ZN27QGraphicsSceneDragDropEvent12setScreenPosERK6QPoint(arg0: *const c_void) -> i32;
  // proto: QPointF QGraphicsSceneDragDropEvent::pos();
  fn _ZNK27QGraphicsSceneDragDropEvent3posEv() -> i32;
  // proto: QPoint QGraphicsSceneDragDropEvent::screenPos();
  fn _ZNK27QGraphicsSceneDragDropEvent9screenPosEv() -> i32;
  // proto: const QMimeData * QGraphicsSceneDragDropEvent::mimeData();
  fn _ZNK27QGraphicsSceneDragDropEvent8mimeDataEv() -> i32;
  // proto: void QGraphicsSceneDragDropEvent::FreeQGraphicsSceneDragDropEvent();
  fn _ZN27QGraphicsSceneDragDropEventD0Ev() -> i32;
  // proto: void QGraphicsSceneDragDropEvent::setMimeData(const QMimeData * data);
  fn _ZN27QGraphicsSceneDragDropEvent11setMimeDataEPK9QMimeData(arg0: *const c_void) -> i32;
  // proto: void QGraphicsSceneDragDropEvent::setSource(QWidget * source);
  fn _ZN27QGraphicsSceneDragDropEvent9setSourceEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QGraphicsSceneDragDropEvent::setScenePos(const QPointF & pos);
  fn _ZN27QGraphicsSceneDragDropEvent11setScenePosERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsSceneDragDropEvent::acceptProposedAction();
  fn _ZN27QGraphicsSceneDragDropEvent20acceptProposedActionEv() -> i32;
}

// body block begin
// class sizeof(QGraphicsSceneDragDropEvent)=1
pub struct QGraphicsSceneDragDropEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn source<T: QGraphicsSceneDragDropEvent_source>(&mut self, value: T) -> i32 {
    value.source(self);
    return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_source {
  fn source(self, this: &mut QGraphicsSceneDragDropEvent) -> i32;
}

// proto: QWidget * QGraphicsSceneDragDropEvent::source();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_source for () {
  fn source(self, this: &mut QGraphicsSceneDragDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent6sourceEv()};
    unsafe {_ZNK27QGraphicsSceneDragDropEvent6sourceEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn scenePos<T: QGraphicsSceneDragDropEvent_scenePos>(&mut self, value: T) -> i32 {
    value.scenePos(self);
    return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_scenePos {
  fn scenePos(self, this: &mut QGraphicsSceneDragDropEvent) -> i32;
}

// proto: QPointF QGraphicsSceneDragDropEvent::scenePos();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_scenePos for () {
  fn scenePos(self, this: &mut QGraphicsSceneDragDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent8scenePosEv()};
    unsafe {_ZNK27QGraphicsSceneDragDropEvent8scenePosEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setPos<T: QGraphicsSceneDragDropEvent_setPos>(&mut self, value: T) -> i32 {
    value.setPos(self);
    return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setPos {
  fn setPos(self, this: &mut QGraphicsSceneDragDropEvent) -> i32;
}

// proto: void QGraphicsSceneDragDropEvent::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setPos for (&'a  QPointF) {
  fn setPos(self, this: &mut QGraphicsSceneDragDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN27QGraphicsSceneDragDropEvent6setPosERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn NewQGraphicsSceneDragDropEvent<T: QGraphicsSceneDragDropEvent_NewQGraphicsSceneDragDropEvent>(value: T) -> QGraphicsSceneDragDropEvent {
    let rsthis = value.NewQGraphicsSceneDragDropEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_NewQGraphicsSceneDragDropEvent {
  fn NewQGraphicsSceneDragDropEvent(self) -> QGraphicsSceneDragDropEvent;
}

// proto: void QGraphicsSceneDragDropEvent::NewQGraphicsSceneDragDropEvent(const QGraphicsSceneDragDropEvent & );
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_NewQGraphicsSceneDragDropEvent for (&'a  QGraphicsSceneDragDropEvent) {
  fn NewQGraphicsSceneDragDropEvent(self) -> QGraphicsSceneDragDropEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEventC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN27QGraphicsSceneDragDropEventC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsSceneDragDropEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setScreenPos<T: QGraphicsSceneDragDropEvent_setScreenPos>(&mut self, value: T) -> i32 {
    value.setScreenPos(self);
    return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setScreenPos {
  fn setScreenPos(self, this: &mut QGraphicsSceneDragDropEvent) -> i32;
}

// proto: void QGraphicsSceneDragDropEvent::setScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setScreenPos for (&'a  QPoint) {
  fn setScreenPos(self, this: &mut QGraphicsSceneDragDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent12setScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN27QGraphicsSceneDragDropEvent12setScreenPosERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn pos<T: QGraphicsSceneDragDropEvent_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_pos {
  fn pos(self, this: &mut QGraphicsSceneDragDropEvent) -> i32;
}

// proto: QPointF QGraphicsSceneDragDropEvent::pos();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_pos for () {
  fn pos(self, this: &mut QGraphicsSceneDragDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent3posEv()};
    unsafe {_ZNK27QGraphicsSceneDragDropEvent3posEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn screenPos<T: QGraphicsSceneDragDropEvent_screenPos>(&mut self, value: T) -> i32 {
    value.screenPos(self);
    return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_screenPos {
  fn screenPos(self, this: &mut QGraphicsSceneDragDropEvent) -> i32;
}

// proto: QPoint QGraphicsSceneDragDropEvent::screenPos();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_screenPos for () {
  fn screenPos(self, this: &mut QGraphicsSceneDragDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent9screenPosEv()};
    unsafe {_ZNK27QGraphicsSceneDragDropEvent9screenPosEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn mimeData<T: QGraphicsSceneDragDropEvent_mimeData>(&mut self, value: T) -> i32 {
    value.mimeData(self);
    return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_mimeData {
  fn mimeData(self, this: &mut QGraphicsSceneDragDropEvent) -> i32;
}

// proto: const QMimeData * QGraphicsSceneDragDropEvent::mimeData();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_mimeData for () {
  fn mimeData(self, this: &mut QGraphicsSceneDragDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent8mimeDataEv()};
    unsafe {_ZNK27QGraphicsSceneDragDropEvent8mimeDataEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn FreeQGraphicsSceneDragDropEvent<T: QGraphicsSceneDragDropEvent_FreeQGraphicsSceneDragDropEvent>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsSceneDragDropEvent(self);
    return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_FreeQGraphicsSceneDragDropEvent {
  fn FreeQGraphicsSceneDragDropEvent(self, this: &mut QGraphicsSceneDragDropEvent) -> i32;
}

// proto: void QGraphicsSceneDragDropEvent::FreeQGraphicsSceneDragDropEvent();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_FreeQGraphicsSceneDragDropEvent for () {
  fn FreeQGraphicsSceneDragDropEvent(self, this: &mut QGraphicsSceneDragDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEventD0Ev()};
    unsafe {_ZN27QGraphicsSceneDragDropEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setMimeData<T: QGraphicsSceneDragDropEvent_setMimeData>(&mut self, value: T) -> i32 {
    value.setMimeData(self);
    return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setMimeData {
  fn setMimeData(self, this: &mut QGraphicsSceneDragDropEvent) -> i32;
}

// proto: void QGraphicsSceneDragDropEvent::setMimeData(const QMimeData * data);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setMimeData for (&'a  QMimeData) {
  fn setMimeData(self, this: &mut QGraphicsSceneDragDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent11setMimeDataEPK9QMimeData()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN27QGraphicsSceneDragDropEvent11setMimeDataEPK9QMimeData(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setSource<T: QGraphicsSceneDragDropEvent_setSource>(&mut self, value: T) -> i32 {
    value.setSource(self);
    return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setSource {
  fn setSource(self, this: &mut QGraphicsSceneDragDropEvent) -> i32;
}

// proto: void QGraphicsSceneDragDropEvent::setSource(QWidget * source);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setSource for (&'a mut QWidget) {
  fn setSource(self, this: &mut QGraphicsSceneDragDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent9setSourceEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN27QGraphicsSceneDragDropEvent9setSourceEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setScenePos<T: QGraphicsSceneDragDropEvent_setScenePos>(&mut self, value: T) -> i32 {
    value.setScenePos(self);
    return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setScenePos {
  fn setScenePos(self, this: &mut QGraphicsSceneDragDropEvent) -> i32;
}

// proto: void QGraphicsSceneDragDropEvent::setScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setScenePos for (&'a  QPointF) {
  fn setScenePos(self, this: &mut QGraphicsSceneDragDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent11setScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN27QGraphicsSceneDragDropEvent11setScenePosERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn acceptProposedAction<T: QGraphicsSceneDragDropEvent_acceptProposedAction>(&mut self, value: T) -> i32 {
    value.acceptProposedAction(self);
    return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_acceptProposedAction {
  fn acceptProposedAction(self, this: &mut QGraphicsSceneDragDropEvent) -> i32;
}

// proto: void QGraphicsSceneDragDropEvent::acceptProposedAction();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_acceptProposedAction for () {
  fn acceptProposedAction(self, this: &mut QGraphicsSceneDragDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent20acceptProposedActionEv()};
    unsafe {_ZN27QGraphicsSceneDragDropEvent20acceptProposedActionEv()};
    return 1;
  }
}

