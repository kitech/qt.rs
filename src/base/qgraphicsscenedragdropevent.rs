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

// proto:  QWidget * QGraphicsSceneDragDropEvent::source();
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn source<RetType, T: QGraphicsSceneDragDropEvent_source<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.source(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_source<RetType> {
  fn source(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> RetType;
}

// proto:  QWidget * QGraphicsSceneDragDropEvent::source();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_source<QWidget> for () {
  fn source(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent6sourceEv()};
    let mut ret = unsafe {_ZNK27QGraphicsSceneDragDropEvent6sourceEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QPointF QGraphicsSceneDragDropEvent::scenePos();
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn scenePos<RetType, T: QGraphicsSceneDragDropEvent_scenePos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_scenePos<RetType> {
  fn scenePos(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> RetType;
}

// proto:  QPointF QGraphicsSceneDragDropEvent::scenePos();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_scenePos<QPointF> for () {
  fn scenePos(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent8scenePosEv()};
    let mut ret = unsafe {_ZNK27QGraphicsSceneDragDropEvent8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsSceneDragDropEvent::setPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setPos<RetType, T: QGraphicsSceneDragDropEvent_setPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setPos<RetType> {
  fn setPos(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> RetType;
}

// proto:  void QGraphicsSceneDragDropEvent::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setPos<()> for (&'a  QPointF) {
  fn setPos(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> () {
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

// proto:  void QGraphicsSceneDragDropEvent::setScreenPos(const QPoint & pos);
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setScreenPos<RetType, T: QGraphicsSceneDragDropEvent_setScreenPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setScreenPos<RetType> {
  fn setScreenPos(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> RetType;
}

// proto:  void QGraphicsSceneDragDropEvent::setScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setScreenPos<()> for (&'a  QPoint) {
  fn setScreenPos(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent12setScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QGraphicsSceneDragDropEvent12setScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPointF QGraphicsSceneDragDropEvent::pos();
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn pos<RetType, T: QGraphicsSceneDragDropEvent_pos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_pos<RetType> {
  fn pos(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> RetType;
}

// proto:  QPointF QGraphicsSceneDragDropEvent::pos();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_pos<QPointF> for () {
  fn pos(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent3posEv()};
    let mut ret = unsafe {_ZNK27QGraphicsSceneDragDropEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QPoint QGraphicsSceneDragDropEvent::screenPos();
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn screenPos<RetType, T: QGraphicsSceneDragDropEvent_screenPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> RetType;
}

// proto:  QPoint QGraphicsSceneDragDropEvent::screenPos();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_screenPos<QPoint> for () {
  fn screenPos(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK27QGraphicsSceneDragDropEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QMimeData * QGraphicsSceneDragDropEvent::mimeData();
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn mimeData<RetType, T: QGraphicsSceneDragDropEvent_mimeData<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.mimeData(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_mimeData<RetType> {
  fn mimeData(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> RetType;
}

// proto:  const QMimeData * QGraphicsSceneDragDropEvent::mimeData();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_mimeData<QMimeData> for () {
  fn mimeData(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> QMimeData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent8mimeDataEv()};
    let mut ret = unsafe {_ZNK27QGraphicsSceneDragDropEvent8mimeDataEv(rsthis.qclsinst)};
    let mut ret1 = QMimeData{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsSceneDragDropEvent::FreeQGraphicsSceneDragDropEvent();
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn FreeQGraphicsSceneDragDropEvent<RetType, T: QGraphicsSceneDragDropEvent_FreeQGraphicsSceneDragDropEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsSceneDragDropEvent(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_FreeQGraphicsSceneDragDropEvent<RetType> {
  fn FreeQGraphicsSceneDragDropEvent(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> RetType;
}

// proto:  void QGraphicsSceneDragDropEvent::FreeQGraphicsSceneDragDropEvent();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_FreeQGraphicsSceneDragDropEvent<()> for () {
  fn FreeQGraphicsSceneDragDropEvent(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEventD0Ev()};
     unsafe {_ZN27QGraphicsSceneDragDropEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QGraphicsSceneDragDropEvent::setMimeData(const QMimeData * data);
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setMimeData<RetType, T: QGraphicsSceneDragDropEvent_setMimeData<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setMimeData(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setMimeData<RetType> {
  fn setMimeData(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> RetType;
}

// proto:  void QGraphicsSceneDragDropEvent::setMimeData(const QMimeData * data);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setMimeData<()> for (&'a  QMimeData) {
  fn setMimeData(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent11setMimeDataEPK9QMimeData()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QGraphicsSceneDragDropEvent11setMimeDataEPK9QMimeData(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGraphicsSceneDragDropEvent::setSource(QWidget * source);
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setSource<RetType, T: QGraphicsSceneDragDropEvent_setSource<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setSource(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setSource<RetType> {
  fn setSource(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> RetType;
}

// proto:  void QGraphicsSceneDragDropEvent::setSource(QWidget * source);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setSource<()> for (&'a mut QWidget) {
  fn setSource(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent9setSourceEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QGraphicsSceneDragDropEvent9setSourceEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGraphicsSceneDragDropEvent::setScenePos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setScenePos<RetType, T: QGraphicsSceneDragDropEvent_setScenePos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setScenePos<RetType> {
  fn setScenePos(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> RetType;
}

// proto:  void QGraphicsSceneDragDropEvent::setScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setScenePos<()> for (&'a  QPointF) {
  fn setScenePos(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent11setScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QGraphicsSceneDragDropEvent11setScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGraphicsSceneDragDropEvent::acceptProposedAction();
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn acceptProposedAction<RetType, T: QGraphicsSceneDragDropEvent_acceptProposedAction<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.acceptProposedAction(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_acceptProposedAction<RetType> {
  fn acceptProposedAction(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> RetType;
}

// proto:  void QGraphicsSceneDragDropEvent::acceptProposedAction();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_acceptProposedAction<()> for () {
  fn acceptProposedAction(self , rsthis: &mut QGraphicsSceneDragDropEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent20acceptProposedActionEv()};
     unsafe {_ZN27QGraphicsSceneDragDropEvent20acceptProposedActionEv(rsthis.qclsinst)};
    // return 1;
  }
}

