// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qpointf::QPointF;
use super::qpoint::QPoint;
use super::qmimedata::QMimeData;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QWidget * QGraphicsSceneDragDropEvent::source();
  fn _ZNK27QGraphicsSceneDragDropEvent6sourceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QGraphicsSceneDragDropEvent::scenePos();
  fn _ZNK27QGraphicsSceneDragDropEvent8scenePosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneDragDropEvent::setPos(const QPointF & pos);
  fn _ZN27QGraphicsSceneDragDropEvent6setPosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSceneDragDropEvent::NewQGraphicsSceneDragDropEvent(const QGraphicsSceneDragDropEvent & );
  fn _ZN27QGraphicsSceneDragDropEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSceneDragDropEvent::setScreenPos(const QPoint & pos);
  fn _ZN27QGraphicsSceneDragDropEvent12setScreenPosERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPointF QGraphicsSceneDragDropEvent::pos();
  fn _ZNK27QGraphicsSceneDragDropEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QGraphicsSceneDragDropEvent::screenPos();
  fn _ZNK27QGraphicsSceneDragDropEvent9screenPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMimeData * QGraphicsSceneDragDropEvent::mimeData();
  fn _ZNK27QGraphicsSceneDragDropEvent8mimeDataEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneDragDropEvent::FreeQGraphicsSceneDragDropEvent();
  fn _ZN27QGraphicsSceneDragDropEventD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsSceneDragDropEvent::setMimeData(const QMimeData * data);
  fn _ZN27QGraphicsSceneDragDropEvent11setMimeDataEPK9QMimeData(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSceneDragDropEvent::setSource(QWidget * source);
  fn _ZN27QGraphicsSceneDragDropEvent9setSourceEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSceneDragDropEvent::setScenePos(const QPointF & pos);
  fn _ZN27QGraphicsSceneDragDropEvent11setScenePosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSceneDragDropEvent::acceptProposedAction();
  fn _ZN27QGraphicsSceneDragDropEvent20acceptProposedActionEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QGraphicsSceneDragDropEvent)=1
pub struct QGraphicsSceneDragDropEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn source<T: QGraphicsSceneDragDropEvent_source>(&mut self, value: T) -> QWidget {
    return value.source(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_source {
  fn source(self, rsthis: &mut QGraphicsSceneDragDropEvent) -> QWidget;
}

// proto:  QWidget * QGraphicsSceneDragDropEvent::source();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_source for () {
  fn source(self, rsthis: &mut QGraphicsSceneDragDropEvent) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent6sourceEv()};
    let mut ret = unsafe {_ZNK27QGraphicsSceneDragDropEvent6sourceEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn scenePos<T: QGraphicsSceneDragDropEvent_scenePos>(&mut self, value: T) -> QPointF {
    return value.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_scenePos {
  fn scenePos(self, rsthis: &mut QGraphicsSceneDragDropEvent) -> QPointF;
}

// proto:  QPointF QGraphicsSceneDragDropEvent::scenePos();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_scenePos for () {
  fn scenePos(self, rsthis: &mut QGraphicsSceneDragDropEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent8scenePosEv()};
    let mut ret = unsafe {_ZNK27QGraphicsSceneDragDropEvent8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setPos<T: QGraphicsSceneDragDropEvent_setPos>(&mut self, value: T)  {
     value.setPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setPos {
  fn setPos(self, rsthis: &mut QGraphicsSceneDragDropEvent) ;
}

// proto:  void QGraphicsSceneDragDropEvent::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setPos for (&'a  QPointF) {
  fn setPos(self, rsthis: &mut QGraphicsSceneDragDropEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QGraphicsSceneDragDropEvent6setPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN27QGraphicsSceneDragDropEventC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsSceneDragDropEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setScreenPos<T: QGraphicsSceneDragDropEvent_setScreenPos>(&mut self, value: T)  {
     value.setScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setScreenPos {
  fn setScreenPos(self, rsthis: &mut QGraphicsSceneDragDropEvent) ;
}

// proto:  void QGraphicsSceneDragDropEvent::setScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setScreenPos for (&'a  QPoint) {
  fn setScreenPos(self, rsthis: &mut QGraphicsSceneDragDropEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent12setScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QGraphicsSceneDragDropEvent12setScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn pos<T: QGraphicsSceneDragDropEvent_pos>(&mut self, value: T) -> QPointF {
    return value.pos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_pos {
  fn pos(self, rsthis: &mut QGraphicsSceneDragDropEvent) -> QPointF;
}

// proto:  QPointF QGraphicsSceneDragDropEvent::pos();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_pos for () {
  fn pos(self, rsthis: &mut QGraphicsSceneDragDropEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent3posEv()};
    let mut ret = unsafe {_ZNK27QGraphicsSceneDragDropEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn screenPos<T: QGraphicsSceneDragDropEvent_screenPos>(&mut self, value: T) -> QPoint {
    return value.screenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_screenPos {
  fn screenPos(self, rsthis: &mut QGraphicsSceneDragDropEvent) -> QPoint;
}

// proto:  QPoint QGraphicsSceneDragDropEvent::screenPos();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_screenPos for () {
  fn screenPos(self, rsthis: &mut QGraphicsSceneDragDropEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK27QGraphicsSceneDragDropEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn mimeData<T: QGraphicsSceneDragDropEvent_mimeData>(&mut self, value: T) -> QMimeData {
    return value.mimeData(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_mimeData {
  fn mimeData(self, rsthis: &mut QGraphicsSceneDragDropEvent) -> QMimeData;
}

// proto:  const QMimeData * QGraphicsSceneDragDropEvent::mimeData();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_mimeData for () {
  fn mimeData(self, rsthis: &mut QGraphicsSceneDragDropEvent) -> QMimeData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent8mimeDataEv()};
    let mut ret = unsafe {_ZNK27QGraphicsSceneDragDropEvent8mimeDataEv(rsthis.qclsinst)};
    let mut ret1 = QMimeData{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn FreeQGraphicsSceneDragDropEvent<T: QGraphicsSceneDragDropEvent_FreeQGraphicsSceneDragDropEvent>(&mut self, value: T)  {
     value.FreeQGraphicsSceneDragDropEvent(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_FreeQGraphicsSceneDragDropEvent {
  fn FreeQGraphicsSceneDragDropEvent(self, rsthis: &mut QGraphicsSceneDragDropEvent) ;
}

// proto:  void QGraphicsSceneDragDropEvent::FreeQGraphicsSceneDragDropEvent();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_FreeQGraphicsSceneDragDropEvent for () {
  fn FreeQGraphicsSceneDragDropEvent(self, rsthis: &mut QGraphicsSceneDragDropEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEventD0Ev()};
     unsafe {_ZN27QGraphicsSceneDragDropEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setMimeData<T: QGraphicsSceneDragDropEvent_setMimeData>(&mut self, value: T)  {
     value.setMimeData(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setMimeData {
  fn setMimeData(self, rsthis: &mut QGraphicsSceneDragDropEvent) ;
}

// proto:  void QGraphicsSceneDragDropEvent::setMimeData(const QMimeData * data);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setMimeData for (&'a  QMimeData) {
  fn setMimeData(self, rsthis: &mut QGraphicsSceneDragDropEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent11setMimeDataEPK9QMimeData()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QGraphicsSceneDragDropEvent11setMimeDataEPK9QMimeData(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setSource<T: QGraphicsSceneDragDropEvent_setSource>(&mut self, value: T)  {
     value.setSource(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setSource {
  fn setSource(self, rsthis: &mut QGraphicsSceneDragDropEvent) ;
}

// proto:  void QGraphicsSceneDragDropEvent::setSource(QWidget * source);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setSource for (&'a mut QWidget) {
  fn setSource(self, rsthis: &mut QGraphicsSceneDragDropEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent9setSourceEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QGraphicsSceneDragDropEvent9setSourceEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setScenePos<T: QGraphicsSceneDragDropEvent_setScenePos>(&mut self, value: T)  {
     value.setScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setScenePos {
  fn setScenePos(self, rsthis: &mut QGraphicsSceneDragDropEvent) ;
}

// proto:  void QGraphicsSceneDragDropEvent::setScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setScenePos for (&'a  QPointF) {
  fn setScenePos(self, rsthis: &mut QGraphicsSceneDragDropEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent11setScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QGraphicsSceneDragDropEvent11setScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn acceptProposedAction<T: QGraphicsSceneDragDropEvent_acceptProposedAction>(&mut self, value: T)  {
     value.acceptProposedAction(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_acceptProposedAction {
  fn acceptProposedAction(self, rsthis: &mut QGraphicsSceneDragDropEvent) ;
}

// proto:  void QGraphicsSceneDragDropEvent::acceptProposedAction();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_acceptProposedAction for () {
  fn acceptProposedAction(self, rsthis: &mut QGraphicsSceneDragDropEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent20acceptProposedActionEv()};
     unsafe {_ZN27QGraphicsSceneDragDropEvent20acceptProposedActionEv(rsthis.qclsinst)};
    // return 1;
  }
}

