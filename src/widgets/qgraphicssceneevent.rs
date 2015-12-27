// auto generated, do not modify.
// created: Sun Dec 27 22:52:02 2015
// src-file: /QtWidgets/qgraphicssceneevent.h
// dst-file: /src/widgets/qgraphicssceneevent.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
// use super::qgraphicssceneevent::QGraphicsSceneEvent; // 773
use std::ops::Deref;
use super::super::core::qpoint::QPointF; // 771
use super::super::core::qpoint::QPoint; // 771
use super::qwidget::QWidget; // 773
use super::super::core::qmimedata::QMimeData; // 771
use super::super::core::qcoreevent::QEvent; // 771
use super::super::core::qsize::QSizeF; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGraphicsSceneMoveEvent_Class_Size() -> c_int;
  // proto:  QPointF QGraphicsSceneMoveEvent::newPos();
  fn _ZNK23QGraphicsSceneMoveEvent6newPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPointF QGraphicsSceneMoveEvent::oldPos();
  fn _ZNK23QGraphicsSceneMoveEvent6oldPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneMoveEvent::~QGraphicsSceneMoveEvent();
  fn _ZN23QGraphicsSceneMoveEventD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsSceneMoveEvent::QGraphicsSceneMoveEvent(const QGraphicsSceneMoveEvent & );
  fn dector_ZN23QGraphicsSceneMoveEventC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN23QGraphicsSceneMoveEventC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneMoveEvent::setNewPos(const QPointF & pos);
  fn _ZN23QGraphicsSceneMoveEvent9setNewPosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneMoveEvent::QGraphicsSceneMoveEvent();
  fn dector_ZN23QGraphicsSceneMoveEventC1Ev() -> *mut c_void;
  fn _ZN23QGraphicsSceneMoveEventC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsSceneMoveEvent::setOldPos(const QPointF & pos);
  fn _ZN23QGraphicsSceneMoveEvent9setOldPosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QGraphicsSceneContextMenuEvent_Class_Size() -> c_int;
  // proto:  void QGraphicsSceneContextMenuEvent::QGraphicsSceneContextMenuEvent(const QGraphicsSceneContextMenuEvent & );
  fn dector_ZN30QGraphicsSceneContextMenuEventC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN30QGraphicsSceneContextMenuEventC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPointF QGraphicsSceneContextMenuEvent::scenePos();
  fn _ZNK30QGraphicsSceneContextMenuEvent8scenePosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneContextMenuEvent::~QGraphicsSceneContextMenuEvent();
  fn _ZN30QGraphicsSceneContextMenuEventD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QPointF QGraphicsSceneContextMenuEvent::pos();
  fn _ZNK30QGraphicsSceneContextMenuEvent3posEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneContextMenuEvent::setScreenPos(const QPoint & pos);
  fn _ZN30QGraphicsSceneContextMenuEvent12setScreenPosERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneContextMenuEvent::setPos(const QPointF & pos);
  fn _ZN30QGraphicsSceneContextMenuEvent6setPosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPoint QGraphicsSceneContextMenuEvent::screenPos();
  fn _ZNK30QGraphicsSceneContextMenuEvent9screenPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneContextMenuEvent::setScenePos(const QPointF & pos);
  fn _ZN30QGraphicsSceneContextMenuEvent11setScenePosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QGraphicsSceneMouseEvent_Class_Size() -> c_int;
  // proto:  QPoint QGraphicsSceneMouseEvent::screenPos();
  fn _ZNK24QGraphicsSceneMouseEvent9screenPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneMouseEvent::QGraphicsSceneMouseEvent(const QGraphicsSceneMouseEvent & );
  fn dector_ZN24QGraphicsSceneMouseEventC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN24QGraphicsSceneMouseEventC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPointF QGraphicsSceneMouseEvent::lastScenePos();
  fn _ZNK24QGraphicsSceneMouseEvent12lastScenePosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneMouseEvent::~QGraphicsSceneMouseEvent();
  fn _ZN24QGraphicsSceneMouseEventD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QPointF QGraphicsSceneMouseEvent::pos();
  fn _ZNK24QGraphicsSceneMouseEvent3posEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneMouseEvent::setLastPos(const QPointF & pos);
  fn _ZN24QGraphicsSceneMouseEvent10setLastPosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneMouseEvent::setLastScenePos(const QPointF & pos);
  fn _ZN24QGraphicsSceneMouseEvent15setLastScenePosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPoint QGraphicsSceneMouseEvent::lastScreenPos();
  fn _ZNK24QGraphicsSceneMouseEvent13lastScreenPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneMouseEvent::setScreenPos(const QPoint & pos);
  fn _ZN24QGraphicsSceneMouseEvent12setScreenPosERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneMouseEvent::setLastScreenPos(const QPoint & pos);
  fn _ZN24QGraphicsSceneMouseEvent16setLastScreenPosERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneMouseEvent::setScenePos(const QPointF & pos);
  fn _ZN24QGraphicsSceneMouseEvent11setScenePosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPointF QGraphicsSceneMouseEvent::lastPos();
  fn _ZNK24QGraphicsSceneMouseEvent7lastPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPointF QGraphicsSceneMouseEvent::scenePos();
  fn _ZNK24QGraphicsSceneMouseEvent8scenePosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneMouseEvent::setPos(const QPointF & pos);
  fn _ZN24QGraphicsSceneMouseEvent6setPosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QGraphicsSceneHelpEvent_Class_Size() -> c_int;
  // proto:  void QGraphicsSceneHelpEvent::setScenePos(const QPointF & pos);
  fn _ZN23QGraphicsSceneHelpEvent11setScenePosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPoint QGraphicsSceneHelpEvent::screenPos();
  fn _ZNK23QGraphicsSceneHelpEvent9screenPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneHelpEvent::~QGraphicsSceneHelpEvent();
  fn _ZN23QGraphicsSceneHelpEventD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsSceneHelpEvent::QGraphicsSceneHelpEvent(const QGraphicsSceneHelpEvent & );
  fn dector_ZN23QGraphicsSceneHelpEventC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN23QGraphicsSceneHelpEventC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneHelpEvent::setScreenPos(const QPoint & pos);
  fn _ZN23QGraphicsSceneHelpEvent12setScreenPosERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPointF QGraphicsSceneHelpEvent::scenePos();
  fn _ZNK23QGraphicsSceneHelpEvent8scenePosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QGraphicsSceneHoverEvent_Class_Size() -> c_int;
  // proto:  QPointF QGraphicsSceneHoverEvent::scenePos();
  fn _ZNK24QGraphicsSceneHoverEvent8scenePosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneHoverEvent::setLastPos(const QPointF & pos);
  fn _ZN24QGraphicsSceneHoverEvent10setLastPosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPointF QGraphicsSceneHoverEvent::lastPos();
  fn _ZNK24QGraphicsSceneHoverEvent7lastPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneHoverEvent::QGraphicsSceneHoverEvent(const QGraphicsSceneHoverEvent & );
  fn dector_ZN24QGraphicsSceneHoverEventC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN24QGraphicsSceneHoverEventC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPointF QGraphicsSceneHoverEvent::lastScenePos();
  fn _ZNK24QGraphicsSceneHoverEvent12lastScenePosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneHoverEvent::setLastScreenPos(const QPoint & pos);
  fn _ZN24QGraphicsSceneHoverEvent16setLastScreenPosERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneHoverEvent::setScenePos(const QPointF & pos);
  fn _ZN24QGraphicsSceneHoverEvent11setScenePosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneHoverEvent::setPos(const QPointF & pos);
  fn _ZN24QGraphicsSceneHoverEvent6setPosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPoint QGraphicsSceneHoverEvent::screenPos();
  fn _ZNK24QGraphicsSceneHoverEvent9screenPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPoint QGraphicsSceneHoverEvent::lastScreenPos();
  fn _ZNK24QGraphicsSceneHoverEvent13lastScreenPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneHoverEvent::setLastScenePos(const QPointF & pos);
  fn _ZN24QGraphicsSceneHoverEvent15setLastScenePosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPointF QGraphicsSceneHoverEvent::pos();
  fn _ZNK24QGraphicsSceneHoverEvent3posEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneHoverEvent::setScreenPos(const QPoint & pos);
  fn _ZN24QGraphicsSceneHoverEvent12setScreenPosERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneHoverEvent::~QGraphicsSceneHoverEvent();
  fn _ZN24QGraphicsSceneHoverEventD0Ev(qthis: u64 /* *mut c_void*/);
  fn QGraphicsSceneWheelEvent_Class_Size() -> c_int;
  // proto:  QPointF QGraphicsSceneWheelEvent::pos();
  fn _ZNK24QGraphicsSceneWheelEvent3posEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneWheelEvent::~QGraphicsSceneWheelEvent();
  fn _ZN24QGraphicsSceneWheelEventD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsSceneWheelEvent::setDelta(int delta);
  fn _ZN24QGraphicsSceneWheelEvent8setDeltaEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QGraphicsSceneWheelEvent::setScenePos(const QPointF & pos);
  fn _ZN24QGraphicsSceneWheelEvent11setScenePosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneWheelEvent::QGraphicsSceneWheelEvent(const QGraphicsSceneWheelEvent & );
  fn dector_ZN24QGraphicsSceneWheelEventC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN24QGraphicsSceneWheelEventC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneWheelEvent::setPos(const QPointF & pos);
  fn _ZN24QGraphicsSceneWheelEvent6setPosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneWheelEvent::setScreenPos(const QPoint & pos);
  fn _ZN24QGraphicsSceneWheelEvent12setScreenPosERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QGraphicsSceneWheelEvent::delta();
  fn _ZNK24QGraphicsSceneWheelEvent5deltaEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QPointF QGraphicsSceneWheelEvent::scenePos();
  fn _ZNK24QGraphicsSceneWheelEvent8scenePosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPoint QGraphicsSceneWheelEvent::screenPos();
  fn _ZNK24QGraphicsSceneWheelEvent9screenPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QGraphicsSceneDragDropEvent_Class_Size() -> c_int;
  // proto:  QWidget * QGraphicsSceneDragDropEvent::source();
  fn _ZNK27QGraphicsSceneDragDropEvent6sourceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPointF QGraphicsSceneDragDropEvent::scenePos();
  fn _ZNK27QGraphicsSceneDragDropEvent8scenePosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneDragDropEvent::setPos(const QPointF & pos);
  fn _ZN27QGraphicsSceneDragDropEvent6setPosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneDragDropEvent::QGraphicsSceneDragDropEvent(const QGraphicsSceneDragDropEvent & );
  fn dector_ZN27QGraphicsSceneDragDropEventC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN27QGraphicsSceneDragDropEventC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneDragDropEvent::setScreenPos(const QPoint & pos);
  fn _ZN27QGraphicsSceneDragDropEvent12setScreenPosERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPointF QGraphicsSceneDragDropEvent::pos();
  fn _ZNK27QGraphicsSceneDragDropEvent3posEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPoint QGraphicsSceneDragDropEvent::screenPos();
  fn _ZNK27QGraphicsSceneDragDropEvent9screenPosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMimeData * QGraphicsSceneDragDropEvent::mimeData();
  fn _ZNK27QGraphicsSceneDragDropEvent8mimeDataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneDragDropEvent::~QGraphicsSceneDragDropEvent();
  fn _ZN27QGraphicsSceneDragDropEventD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsSceneDragDropEvent::setMimeData(const QMimeData * data);
  fn _ZN27QGraphicsSceneDragDropEvent11setMimeDataEPK9QMimeData(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneDragDropEvent::setSource(QWidget * source);
  fn _ZN27QGraphicsSceneDragDropEvent9setSourceEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneDragDropEvent::setScenePos(const QPointF & pos);
  fn _ZN27QGraphicsSceneDragDropEvent11setScenePosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneDragDropEvent::acceptProposedAction();
  fn _ZN27QGraphicsSceneDragDropEvent20acceptProposedActionEv(qthis: u64 /* *mut c_void*/);
  fn QGraphicsSceneEvent_Class_Size() -> c_int;
  // proto:  void QGraphicsSceneEvent::QGraphicsSceneEvent(const QGraphicsSceneEvent & );
  fn dector_ZN19QGraphicsSceneEventC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QGraphicsSceneEventC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QGraphicsSceneEvent::widget();
  fn _ZNK19QGraphicsSceneEvent6widgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneEvent::setWidget(QWidget * widget);
  fn _ZN19QGraphicsSceneEvent9setWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneEvent::~QGraphicsSceneEvent();
  fn _ZN19QGraphicsSceneEventD0Ev(qthis: u64 /* *mut c_void*/);
  fn QGraphicsSceneResizeEvent_Class_Size() -> c_int;
  // proto:  QSizeF QGraphicsSceneResizeEvent::newSize();
  fn _ZNK25QGraphicsSceneResizeEvent7newSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSizeF QGraphicsSceneResizeEvent::oldSize();
  fn _ZNK25QGraphicsSceneResizeEvent7oldSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSceneResizeEvent::~QGraphicsSceneResizeEvent();
  fn _ZN25QGraphicsSceneResizeEventD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsSceneResizeEvent::setNewSize(const QSizeF & size);
  fn _ZN25QGraphicsSceneResizeEvent10setNewSizeERK6QSizeF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneResizeEvent::QGraphicsSceneResizeEvent();
  fn dector_ZN25QGraphicsSceneResizeEventC1Ev() -> *mut c_void;
  fn _ZN25QGraphicsSceneResizeEventC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsSceneResizeEvent::setOldSize(const QSizeF & size);
  fn _ZN25QGraphicsSceneResizeEvent10setOldSizeERK6QSizeF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsSceneResizeEvent::QGraphicsSceneResizeEvent(const QGraphicsSceneResizeEvent & );
  fn dector_ZN25QGraphicsSceneResizeEventC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN25QGraphicsSceneResizeEventC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsSceneMoveEvent)=1
#[derive(Default)]
pub struct QGraphicsSceneMoveEvent {
  qbase: QGraphicsSceneEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGraphicsSceneContextMenuEvent)=1
#[derive(Default)]
pub struct QGraphicsSceneContextMenuEvent {
  qbase: QGraphicsSceneEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGraphicsSceneMouseEvent)=1
#[derive(Default)]
pub struct QGraphicsSceneMouseEvent {
  qbase: QGraphicsSceneEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGraphicsSceneHelpEvent)=1
#[derive(Default)]
pub struct QGraphicsSceneHelpEvent {
  qbase: QGraphicsSceneEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGraphicsSceneHoverEvent)=1
#[derive(Default)]
pub struct QGraphicsSceneHoverEvent {
  qbase: QGraphicsSceneEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGraphicsSceneWheelEvent)=1
#[derive(Default)]
pub struct QGraphicsSceneWheelEvent {
  qbase: QGraphicsSceneEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGraphicsSceneDragDropEvent)=1
#[derive(Default)]
pub struct QGraphicsSceneDragDropEvent {
  qbase: QGraphicsSceneEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGraphicsSceneEvent)=1
#[derive(Default)]
pub struct QGraphicsSceneEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGraphicsSceneResizeEvent)=1
#[derive(Default)]
pub struct QGraphicsSceneResizeEvent {
  qbase: QGraphicsSceneEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsSceneMoveEvent {
    return QGraphicsSceneMoveEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsSceneMoveEvent {
  type Target = QGraphicsSceneEvent;

  fn deref(&self) -> &QGraphicsSceneEvent {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsSceneEvent> for QGraphicsSceneMoveEvent {
  fn as_ref(& self) -> & QGraphicsSceneEvent {
    return & self.qbase;
  }
}
  // proto:  QPointF QGraphicsSceneMoveEvent::newPos();
impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn newPos<RetType, T: QGraphicsSceneMoveEvent_newPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.newPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_newPos<RetType> {
  fn newPos(self , rsthis: & QGraphicsSceneMoveEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneMoveEvent::newPos();
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_newPos<QPointF> for () {
  fn newPos(self , rsthis: & QGraphicsSceneMoveEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSceneMoveEvent6newPosEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSceneMoveEvent6newPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsSceneMoveEvent::oldPos();
impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn oldPos<RetType, T: QGraphicsSceneMoveEvent_oldPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.oldPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_oldPos<RetType> {
  fn oldPos(self , rsthis: & QGraphicsSceneMoveEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneMoveEvent::oldPos();
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_oldPos<QPointF> for () {
  fn oldPos(self , rsthis: & QGraphicsSceneMoveEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSceneMoveEvent6oldPosEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSceneMoveEvent6oldPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneMoveEvent::~QGraphicsSceneMoveEvent();
impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn Free<RetType, T: QGraphicsSceneMoveEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_Free<RetType> {
  fn Free(self , rsthis: & QGraphicsSceneMoveEvent) -> RetType;
}

  // proto:  void QGraphicsSceneMoveEvent::~QGraphicsSceneMoveEvent();
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_Free<()> for () {
  fn Free(self , rsthis: & QGraphicsSceneMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneMoveEventD0Ev()};
     unsafe {_ZN23QGraphicsSceneMoveEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneMoveEvent::QGraphicsSceneMoveEvent(const QGraphicsSceneMoveEvent & );
impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn New<T: QGraphicsSceneMoveEvent_New>(value: T) -> QGraphicsSceneMoveEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_New {
  fn New(self) -> QGraphicsSceneMoveEvent;
}

  // proto:  void QGraphicsSceneMoveEvent::QGraphicsSceneMoveEvent(const QGraphicsSceneMoveEvent & );
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_New for (&'a QGraphicsSceneMoveEvent) {
  fn New(self) -> QGraphicsSceneMoveEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneMoveEventC1ERKS_()};
    let ctysz: c_int = unsafe{QGraphicsSceneMoveEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN23QGraphicsSceneMoveEventC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN23QGraphicsSceneMoveEventC1ERKS_(arg0)} as u64;
    let rsthis = QGraphicsSceneMoveEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneMoveEvent::setNewPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn setNewPos<RetType, T: QGraphicsSceneMoveEvent_setNewPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNewPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_setNewPos<RetType> {
  fn setNewPos(self , rsthis: & QGraphicsSceneMoveEvent) -> RetType;
}

  // proto:  void QGraphicsSceneMoveEvent::setNewPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_setNewPos<()> for (&'a QPointF) {
  fn setNewPos(self , rsthis: & QGraphicsSceneMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneMoveEvent9setNewPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsSceneMoveEvent9setNewPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneMoveEvent::QGraphicsSceneMoveEvent();
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_New for () {
  fn New(self) -> QGraphicsSceneMoveEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneMoveEventC1Ev()};
    let ctysz: c_int = unsafe{QGraphicsSceneMoveEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN23QGraphicsSceneMoveEventC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN23QGraphicsSceneMoveEventC1Ev()} as u64;
    let rsthis = QGraphicsSceneMoveEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneMoveEvent::setOldPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn setOldPos<RetType, T: QGraphicsSceneMoveEvent_setOldPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOldPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_setOldPos<RetType> {
  fn setOldPos(self , rsthis: & QGraphicsSceneMoveEvent) -> RetType;
}

  // proto:  void QGraphicsSceneMoveEvent::setOldPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_setOldPos<()> for (&'a QPointF) {
  fn setOldPos(self , rsthis: & QGraphicsSceneMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneMoveEvent9setOldPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsSceneMoveEvent9setOldPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsSceneContextMenuEvent {
    return QGraphicsSceneContextMenuEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsSceneContextMenuEvent {
  type Target = QGraphicsSceneEvent;

  fn deref(&self) -> &QGraphicsSceneEvent {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsSceneEvent> for QGraphicsSceneContextMenuEvent {
  fn as_ref(& self) -> & QGraphicsSceneEvent {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsSceneContextMenuEvent::QGraphicsSceneContextMenuEvent(const QGraphicsSceneContextMenuEvent & );
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn New<T: QGraphicsSceneContextMenuEvent_New>(value: T) -> QGraphicsSceneContextMenuEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_New {
  fn New(self) -> QGraphicsSceneContextMenuEvent;
}

  // proto:  void QGraphicsSceneContextMenuEvent::QGraphicsSceneContextMenuEvent(const QGraphicsSceneContextMenuEvent & );
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_New for (&'a QGraphicsSceneContextMenuEvent) {
  fn New(self) -> QGraphicsSceneContextMenuEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QGraphicsSceneContextMenuEventC1ERKS_()};
    let ctysz: c_int = unsafe{QGraphicsSceneContextMenuEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN30QGraphicsSceneContextMenuEventC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN30QGraphicsSceneContextMenuEventC1ERKS_(arg0)} as u64;
    let rsthis = QGraphicsSceneContextMenuEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsSceneContextMenuEvent::scenePos();
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn scenePos<RetType, T: QGraphicsSceneContextMenuEvent_scenePos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_scenePos<RetType> {
  fn scenePos(self , rsthis: & QGraphicsSceneContextMenuEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneContextMenuEvent::scenePos();
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_scenePos<QPointF> for () {
  fn scenePos(self , rsthis: & QGraphicsSceneContextMenuEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK30QGraphicsSceneContextMenuEvent8scenePosEv()};
    let mut ret = unsafe {_ZNK30QGraphicsSceneContextMenuEvent8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneContextMenuEvent::~QGraphicsSceneContextMenuEvent();
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn Free<RetType, T: QGraphicsSceneContextMenuEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_Free<RetType> {
  fn Free(self , rsthis: & QGraphicsSceneContextMenuEvent) -> RetType;
}

  // proto:  void QGraphicsSceneContextMenuEvent::~QGraphicsSceneContextMenuEvent();
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_Free<()> for () {
  fn Free(self , rsthis: & QGraphicsSceneContextMenuEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QGraphicsSceneContextMenuEventD0Ev()};
     unsafe {_ZN30QGraphicsSceneContextMenuEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsSceneContextMenuEvent::pos();
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn pos<RetType, T: QGraphicsSceneContextMenuEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_pos<RetType> {
  fn pos(self , rsthis: & QGraphicsSceneContextMenuEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneContextMenuEvent::pos();
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_pos<QPointF> for () {
  fn pos(self , rsthis: & QGraphicsSceneContextMenuEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK30QGraphicsSceneContextMenuEvent3posEv()};
    let mut ret = unsafe {_ZNK30QGraphicsSceneContextMenuEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneContextMenuEvent::setScreenPos(const QPoint & pos);
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn setScreenPos<RetType, T: QGraphicsSceneContextMenuEvent_setScreenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_setScreenPos<RetType> {
  fn setScreenPos(self , rsthis: & QGraphicsSceneContextMenuEvent) -> RetType;
}

  // proto:  void QGraphicsSceneContextMenuEvent::setScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_setScreenPos<()> for (&'a QPoint) {
  fn setScreenPos(self , rsthis: & QGraphicsSceneContextMenuEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QGraphicsSceneContextMenuEvent12setScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN30QGraphicsSceneContextMenuEvent12setScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneContextMenuEvent::setPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn setPos<RetType, T: QGraphicsSceneContextMenuEvent_setPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_setPos<RetType> {
  fn setPos(self , rsthis: & QGraphicsSceneContextMenuEvent) -> RetType;
}

  // proto:  void QGraphicsSceneContextMenuEvent::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_setPos<()> for (&'a QPointF) {
  fn setPos(self , rsthis: & QGraphicsSceneContextMenuEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QGraphicsSceneContextMenuEvent6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN30QGraphicsSceneContextMenuEvent6setPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPoint QGraphicsSceneContextMenuEvent::screenPos();
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn screenPos<RetType, T: QGraphicsSceneContextMenuEvent_screenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: & QGraphicsSceneContextMenuEvent) -> RetType;
}

  // proto:  QPoint QGraphicsSceneContextMenuEvent::screenPos();
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_screenPos<QPoint> for () {
  fn screenPos(self , rsthis: & QGraphicsSceneContextMenuEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK30QGraphicsSceneContextMenuEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK30QGraphicsSceneContextMenuEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneContextMenuEvent::setScenePos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn setScenePos<RetType, T: QGraphicsSceneContextMenuEvent_setScenePos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_setScenePos<RetType> {
  fn setScenePos(self , rsthis: & QGraphicsSceneContextMenuEvent) -> RetType;
}

  // proto:  void QGraphicsSceneContextMenuEvent::setScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_setScenePos<()> for (&'a QPointF) {
  fn setScenePos(self , rsthis: & QGraphicsSceneContextMenuEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QGraphicsSceneContextMenuEvent11setScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN30QGraphicsSceneContextMenuEvent11setScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsSceneMouseEvent {
    return QGraphicsSceneMouseEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsSceneMouseEvent {
  type Target = QGraphicsSceneEvent;

  fn deref(&self) -> &QGraphicsSceneEvent {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsSceneEvent> for QGraphicsSceneMouseEvent {
  fn as_ref(& self) -> & QGraphicsSceneEvent {
    return & self.qbase;
  }
}
  // proto:  QPoint QGraphicsSceneMouseEvent::screenPos();
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn screenPos<RetType, T: QGraphicsSceneMouseEvent_screenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}

  // proto:  QPoint QGraphicsSceneMouseEvent::screenPos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_screenPos<QPoint> for () {
  fn screenPos(self , rsthis: & QGraphicsSceneMouseEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneMouseEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneMouseEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneMouseEvent::QGraphicsSceneMouseEvent(const QGraphicsSceneMouseEvent & );
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn New<T: QGraphicsSceneMouseEvent_New>(value: T) -> QGraphicsSceneMouseEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_New {
  fn New(self) -> QGraphicsSceneMouseEvent;
}

  // proto:  void QGraphicsSceneMouseEvent::QGraphicsSceneMouseEvent(const QGraphicsSceneMouseEvent & );
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_New for (&'a QGraphicsSceneMouseEvent) {
  fn New(self) -> QGraphicsSceneMouseEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEventC1ERKS_()};
    let ctysz: c_int = unsafe{QGraphicsSceneMouseEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN24QGraphicsSceneMouseEventC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN24QGraphicsSceneMouseEventC1ERKS_(arg0)} as u64;
    let rsthis = QGraphicsSceneMouseEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsSceneMouseEvent::lastScenePos();
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn lastScenePos<RetType, T: QGraphicsSceneMouseEvent_lastScenePos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_lastScenePos<RetType> {
  fn lastScenePos(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneMouseEvent::lastScenePos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_lastScenePos<QPointF> for () {
  fn lastScenePos(self , rsthis: & QGraphicsSceneMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneMouseEvent12lastScenePosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneMouseEvent12lastScenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneMouseEvent::~QGraphicsSceneMouseEvent();
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn Free<RetType, T: QGraphicsSceneMouseEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_Free<RetType> {
  fn Free(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}

  // proto:  void QGraphicsSceneMouseEvent::~QGraphicsSceneMouseEvent();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_Free<()> for () {
  fn Free(self , rsthis: & QGraphicsSceneMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEventD0Ev()};
     unsafe {_ZN24QGraphicsSceneMouseEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsSceneMouseEvent::pos();
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn pos<RetType, T: QGraphicsSceneMouseEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_pos<RetType> {
  fn pos(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneMouseEvent::pos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_pos<QPointF> for () {
  fn pos(self , rsthis: & QGraphicsSceneMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneMouseEvent3posEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneMouseEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneMouseEvent::setLastPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setLastPos<RetType, T: QGraphicsSceneMouseEvent_setLastPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLastPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setLastPos<RetType> {
  fn setLastPos(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}

  // proto:  void QGraphicsSceneMouseEvent::setLastPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setLastPos<()> for (&'a QPointF) {
  fn setLastPos(self , rsthis: & QGraphicsSceneMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent10setLastPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent10setLastPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneMouseEvent::setLastScenePos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setLastScenePos<RetType, T: QGraphicsSceneMouseEvent_setLastScenePos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLastScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setLastScenePos<RetType> {
  fn setLastScenePos(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}

  // proto:  void QGraphicsSceneMouseEvent::setLastScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setLastScenePos<()> for (&'a QPointF) {
  fn setLastScenePos(self , rsthis: & QGraphicsSceneMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent15setLastScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent15setLastScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPoint QGraphicsSceneMouseEvent::lastScreenPos();
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn lastScreenPos<RetType, T: QGraphicsSceneMouseEvent_lastScreenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_lastScreenPos<RetType> {
  fn lastScreenPos(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}

  // proto:  QPoint QGraphicsSceneMouseEvent::lastScreenPos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_lastScreenPos<QPoint> for () {
  fn lastScreenPos(self , rsthis: & QGraphicsSceneMouseEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneMouseEvent13lastScreenPosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneMouseEvent13lastScreenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneMouseEvent::setScreenPos(const QPoint & pos);
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setScreenPos<RetType, T: QGraphicsSceneMouseEvent_setScreenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setScreenPos<RetType> {
  fn setScreenPos(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}

  // proto:  void QGraphicsSceneMouseEvent::setScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setScreenPos<()> for (&'a QPoint) {
  fn setScreenPos(self , rsthis: & QGraphicsSceneMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent12setScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent12setScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneMouseEvent::setLastScreenPos(const QPoint & pos);
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setLastScreenPos<RetType, T: QGraphicsSceneMouseEvent_setLastScreenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLastScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setLastScreenPos<RetType> {
  fn setLastScreenPos(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}

  // proto:  void QGraphicsSceneMouseEvent::setLastScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setLastScreenPos<()> for (&'a QPoint) {
  fn setLastScreenPos(self , rsthis: & QGraphicsSceneMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent16setLastScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent16setLastScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneMouseEvent::setScenePos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setScenePos<RetType, T: QGraphicsSceneMouseEvent_setScenePos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setScenePos<RetType> {
  fn setScenePos(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}

  // proto:  void QGraphicsSceneMouseEvent::setScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setScenePos<()> for (&'a QPointF) {
  fn setScenePos(self , rsthis: & QGraphicsSceneMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent11setScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent11setScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsSceneMouseEvent::lastPos();
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn lastPos<RetType, T: QGraphicsSceneMouseEvent_lastPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_lastPos<RetType> {
  fn lastPos(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneMouseEvent::lastPos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_lastPos<QPointF> for () {
  fn lastPos(self , rsthis: & QGraphicsSceneMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneMouseEvent7lastPosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneMouseEvent7lastPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsSceneMouseEvent::scenePos();
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn scenePos<RetType, T: QGraphicsSceneMouseEvent_scenePos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_scenePos<RetType> {
  fn scenePos(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneMouseEvent::scenePos();
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_scenePos<QPointF> for () {
  fn scenePos(self , rsthis: & QGraphicsSceneMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneMouseEvent8scenePosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneMouseEvent8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneMouseEvent::setPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setPos<RetType, T: QGraphicsSceneMouseEvent_setPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_setPos<RetType> {
  fn setPos(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}

  // proto:  void QGraphicsSceneMouseEvent::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setPos<()> for (&'a QPointF) {
  fn setPos(self , rsthis: & QGraphicsSceneMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneMouseEvent6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneMouseEvent6setPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsSceneHelpEvent {
    return QGraphicsSceneHelpEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsSceneHelpEvent {
  type Target = QGraphicsSceneEvent;

  fn deref(&self) -> &QGraphicsSceneEvent {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsSceneEvent> for QGraphicsSceneHelpEvent {
  fn as_ref(& self) -> & QGraphicsSceneEvent {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsSceneHelpEvent::setScenePos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn setScenePos<RetType, T: QGraphicsSceneHelpEvent_setScenePos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHelpEvent_setScenePos<RetType> {
  fn setScenePos(self , rsthis: & QGraphicsSceneHelpEvent) -> RetType;
}

  // proto:  void QGraphicsSceneHelpEvent::setScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneHelpEvent_setScenePos<()> for (&'a QPointF) {
  fn setScenePos(self , rsthis: & QGraphicsSceneHelpEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneHelpEvent11setScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsSceneHelpEvent11setScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPoint QGraphicsSceneHelpEvent::screenPos();
impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn screenPos<RetType, T: QGraphicsSceneHelpEvent_screenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHelpEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: & QGraphicsSceneHelpEvent) -> RetType;
}

  // proto:  QPoint QGraphicsSceneHelpEvent::screenPos();
impl<'a> /*trait*/ QGraphicsSceneHelpEvent_screenPos<QPoint> for () {
  fn screenPos(self , rsthis: & QGraphicsSceneHelpEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSceneHelpEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSceneHelpEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneHelpEvent::~QGraphicsSceneHelpEvent();
impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn Free<RetType, T: QGraphicsSceneHelpEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHelpEvent_Free<RetType> {
  fn Free(self , rsthis: & QGraphicsSceneHelpEvent) -> RetType;
}

  // proto:  void QGraphicsSceneHelpEvent::~QGraphicsSceneHelpEvent();
impl<'a> /*trait*/ QGraphicsSceneHelpEvent_Free<()> for () {
  fn Free(self , rsthis: & QGraphicsSceneHelpEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneHelpEventD0Ev()};
     unsafe {_ZN23QGraphicsSceneHelpEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneHelpEvent::QGraphicsSceneHelpEvent(const QGraphicsSceneHelpEvent & );
impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn New<T: QGraphicsSceneHelpEvent_New>(value: T) -> QGraphicsSceneHelpEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneHelpEvent_New {
  fn New(self) -> QGraphicsSceneHelpEvent;
}

  // proto:  void QGraphicsSceneHelpEvent::QGraphicsSceneHelpEvent(const QGraphicsSceneHelpEvent & );
impl<'a> /*trait*/ QGraphicsSceneHelpEvent_New for (&'a QGraphicsSceneHelpEvent) {
  fn New(self) -> QGraphicsSceneHelpEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneHelpEventC1ERKS_()};
    let ctysz: c_int = unsafe{QGraphicsSceneHelpEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN23QGraphicsSceneHelpEventC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN23QGraphicsSceneHelpEventC1ERKS_(arg0)} as u64;
    let rsthis = QGraphicsSceneHelpEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneHelpEvent::setScreenPos(const QPoint & pos);
impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn setScreenPos<RetType, T: QGraphicsSceneHelpEvent_setScreenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHelpEvent_setScreenPos<RetType> {
  fn setScreenPos(self , rsthis: & QGraphicsSceneHelpEvent) -> RetType;
}

  // proto:  void QGraphicsSceneHelpEvent::setScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneHelpEvent_setScreenPos<()> for (&'a QPoint) {
  fn setScreenPos(self , rsthis: & QGraphicsSceneHelpEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneHelpEvent12setScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsSceneHelpEvent12setScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsSceneHelpEvent::scenePos();
impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn scenePos<RetType, T: QGraphicsSceneHelpEvent_scenePos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHelpEvent_scenePos<RetType> {
  fn scenePos(self , rsthis: & QGraphicsSceneHelpEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneHelpEvent::scenePos();
impl<'a> /*trait*/ QGraphicsSceneHelpEvent_scenePos<QPointF> for () {
  fn scenePos(self , rsthis: & QGraphicsSceneHelpEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSceneHelpEvent8scenePosEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSceneHelpEvent8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsSceneHoverEvent {
    return QGraphicsSceneHoverEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsSceneHoverEvent {
  type Target = QGraphicsSceneEvent;

  fn deref(&self) -> &QGraphicsSceneEvent {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsSceneEvent> for QGraphicsSceneHoverEvent {
  fn as_ref(& self) -> & QGraphicsSceneEvent {
    return & self.qbase;
  }
}
  // proto:  QPointF QGraphicsSceneHoverEvent::scenePos();
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn scenePos<RetType, T: QGraphicsSceneHoverEvent_scenePos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_scenePos<RetType> {
  fn scenePos(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneHoverEvent::scenePos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_scenePos<QPointF> for () {
  fn scenePos(self , rsthis: & QGraphicsSceneHoverEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent8scenePosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneHoverEvent8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneHoverEvent::setLastPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setLastPos<RetType, T: QGraphicsSceneHoverEvent_setLastPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLastPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setLastPos<RetType> {
  fn setLastPos(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}

  // proto:  void QGraphicsSceneHoverEvent::setLastPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setLastPos<()> for (&'a QPointF) {
  fn setLastPos(self , rsthis: & QGraphicsSceneHoverEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent10setLastPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneHoverEvent10setLastPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsSceneHoverEvent::lastPos();
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn lastPos<RetType, T: QGraphicsSceneHoverEvent_lastPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_lastPos<RetType> {
  fn lastPos(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneHoverEvent::lastPos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_lastPos<QPointF> for () {
  fn lastPos(self , rsthis: & QGraphicsSceneHoverEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent7lastPosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneHoverEvent7lastPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneHoverEvent::QGraphicsSceneHoverEvent(const QGraphicsSceneHoverEvent & );
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn New<T: QGraphicsSceneHoverEvent_New>(value: T) -> QGraphicsSceneHoverEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_New {
  fn New(self) -> QGraphicsSceneHoverEvent;
}

  // proto:  void QGraphicsSceneHoverEvent::QGraphicsSceneHoverEvent(const QGraphicsSceneHoverEvent & );
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_New for (&'a QGraphicsSceneHoverEvent) {
  fn New(self) -> QGraphicsSceneHoverEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEventC1ERKS_()};
    let ctysz: c_int = unsafe{QGraphicsSceneHoverEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN24QGraphicsSceneHoverEventC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN24QGraphicsSceneHoverEventC1ERKS_(arg0)} as u64;
    let rsthis = QGraphicsSceneHoverEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsSceneHoverEvent::lastScenePos();
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn lastScenePos<RetType, T: QGraphicsSceneHoverEvent_lastScenePos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_lastScenePos<RetType> {
  fn lastScenePos(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneHoverEvent::lastScenePos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_lastScenePos<QPointF> for () {
  fn lastScenePos(self , rsthis: & QGraphicsSceneHoverEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent12lastScenePosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneHoverEvent12lastScenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneHoverEvent::setLastScreenPos(const QPoint & pos);
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setLastScreenPos<RetType, T: QGraphicsSceneHoverEvent_setLastScreenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLastScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setLastScreenPos<RetType> {
  fn setLastScreenPos(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}

  // proto:  void QGraphicsSceneHoverEvent::setLastScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setLastScreenPos<()> for (&'a QPoint) {
  fn setLastScreenPos(self , rsthis: & QGraphicsSceneHoverEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent16setLastScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneHoverEvent16setLastScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneHoverEvent::setScenePos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setScenePos<RetType, T: QGraphicsSceneHoverEvent_setScenePos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setScenePos<RetType> {
  fn setScenePos(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}

  // proto:  void QGraphicsSceneHoverEvent::setScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setScenePos<()> for (&'a QPointF) {
  fn setScenePos(self , rsthis: & QGraphicsSceneHoverEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent11setScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneHoverEvent11setScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneHoverEvent::setPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setPos<RetType, T: QGraphicsSceneHoverEvent_setPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setPos<RetType> {
  fn setPos(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}

  // proto:  void QGraphicsSceneHoverEvent::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setPos<()> for (&'a QPointF) {
  fn setPos(self , rsthis: & QGraphicsSceneHoverEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneHoverEvent6setPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPoint QGraphicsSceneHoverEvent::screenPos();
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn screenPos<RetType, T: QGraphicsSceneHoverEvent_screenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}

  // proto:  QPoint QGraphicsSceneHoverEvent::screenPos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_screenPos<QPoint> for () {
  fn screenPos(self , rsthis: & QGraphicsSceneHoverEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneHoverEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QGraphicsSceneHoverEvent::lastScreenPos();
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn lastScreenPos<RetType, T: QGraphicsSceneHoverEvent_lastScreenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_lastScreenPos<RetType> {
  fn lastScreenPos(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}

  // proto:  QPoint QGraphicsSceneHoverEvent::lastScreenPos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_lastScreenPos<QPoint> for () {
  fn lastScreenPos(self , rsthis: & QGraphicsSceneHoverEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent13lastScreenPosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneHoverEvent13lastScreenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneHoverEvent::setLastScenePos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setLastScenePos<RetType, T: QGraphicsSceneHoverEvent_setLastScenePos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLastScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setLastScenePos<RetType> {
  fn setLastScenePos(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}

  // proto:  void QGraphicsSceneHoverEvent::setLastScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setLastScenePos<()> for (&'a QPointF) {
  fn setLastScenePos(self , rsthis: & QGraphicsSceneHoverEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent15setLastScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneHoverEvent15setLastScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsSceneHoverEvent::pos();
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn pos<RetType, T: QGraphicsSceneHoverEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_pos<RetType> {
  fn pos(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneHoverEvent::pos();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_pos<QPointF> for () {
  fn pos(self , rsthis: & QGraphicsSceneHoverEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneHoverEvent3posEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneHoverEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneHoverEvent::setScreenPos(const QPoint & pos);
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setScreenPos<RetType, T: QGraphicsSceneHoverEvent_setScreenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_setScreenPos<RetType> {
  fn setScreenPos(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}

  // proto:  void QGraphicsSceneHoverEvent::setScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setScreenPos<()> for (&'a QPoint) {
  fn setScreenPos(self , rsthis: & QGraphicsSceneHoverEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEvent12setScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneHoverEvent12setScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneHoverEvent::~QGraphicsSceneHoverEvent();
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn Free<RetType, T: QGraphicsSceneHoverEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_Free<RetType> {
  fn Free(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}

  // proto:  void QGraphicsSceneHoverEvent::~QGraphicsSceneHoverEvent();
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_Free<()> for () {
  fn Free(self , rsthis: & QGraphicsSceneHoverEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneHoverEventD0Ev()};
     unsafe {_ZN24QGraphicsSceneHoverEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsSceneWheelEvent {
    return QGraphicsSceneWheelEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsSceneWheelEvent {
  type Target = QGraphicsSceneEvent;

  fn deref(&self) -> &QGraphicsSceneEvent {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsSceneEvent> for QGraphicsSceneWheelEvent {
  fn as_ref(& self) -> & QGraphicsSceneEvent {
    return & self.qbase;
  }
}
  // proto:  QPointF QGraphicsSceneWheelEvent::pos();
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn pos<RetType, T: QGraphicsSceneWheelEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_pos<RetType> {
  fn pos(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneWheelEvent::pos();
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_pos<QPointF> for () {
  fn pos(self , rsthis: & QGraphicsSceneWheelEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneWheelEvent3posEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneWheelEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneWheelEvent::~QGraphicsSceneWheelEvent();
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn Free<RetType, T: QGraphicsSceneWheelEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_Free<RetType> {
  fn Free(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  void QGraphicsSceneWheelEvent::~QGraphicsSceneWheelEvent();
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_Free<()> for () {
  fn Free(self , rsthis: & QGraphicsSceneWheelEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneWheelEventD0Ev()};
     unsafe {_ZN24QGraphicsSceneWheelEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneWheelEvent::setDelta(int delta);
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn setDelta<RetType, T: QGraphicsSceneWheelEvent_setDelta<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDelta(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_setDelta<RetType> {
  fn setDelta(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  void QGraphicsSceneWheelEvent::setDelta(int delta);
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_setDelta<()> for (i32) {
  fn setDelta(self , rsthis: & QGraphicsSceneWheelEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneWheelEvent8setDeltaEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN24QGraphicsSceneWheelEvent8setDeltaEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneWheelEvent::setScenePos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn setScenePos<RetType, T: QGraphicsSceneWheelEvent_setScenePos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_setScenePos<RetType> {
  fn setScenePos(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  void QGraphicsSceneWheelEvent::setScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_setScenePos<()> for (&'a QPointF) {
  fn setScenePos(self , rsthis: & QGraphicsSceneWheelEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneWheelEvent11setScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneWheelEvent11setScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneWheelEvent::QGraphicsSceneWheelEvent(const QGraphicsSceneWheelEvent & );
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn New<T: QGraphicsSceneWheelEvent_New>(value: T) -> QGraphicsSceneWheelEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_New {
  fn New(self) -> QGraphicsSceneWheelEvent;
}

  // proto:  void QGraphicsSceneWheelEvent::QGraphicsSceneWheelEvent(const QGraphicsSceneWheelEvent & );
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_New for (&'a QGraphicsSceneWheelEvent) {
  fn New(self) -> QGraphicsSceneWheelEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneWheelEventC1ERKS_()};
    let ctysz: c_int = unsafe{QGraphicsSceneWheelEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN24QGraphicsSceneWheelEventC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN24QGraphicsSceneWheelEventC1ERKS_(arg0)} as u64;
    let rsthis = QGraphicsSceneWheelEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneWheelEvent::setPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn setPos<RetType, T: QGraphicsSceneWheelEvent_setPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_setPos<RetType> {
  fn setPos(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  void QGraphicsSceneWheelEvent::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_setPos<()> for (&'a QPointF) {
  fn setPos(self , rsthis: & QGraphicsSceneWheelEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneWheelEvent6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneWheelEvent6setPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneWheelEvent::setScreenPos(const QPoint & pos);
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn setScreenPos<RetType, T: QGraphicsSceneWheelEvent_setScreenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_setScreenPos<RetType> {
  fn setScreenPos(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  void QGraphicsSceneWheelEvent::setScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_setScreenPos<()> for (&'a QPoint) {
  fn setScreenPos(self , rsthis: & QGraphicsSceneWheelEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QGraphicsSceneWheelEvent12setScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QGraphicsSceneWheelEvent12setScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QGraphicsSceneWheelEvent::delta();
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn delta<RetType, T: QGraphicsSceneWheelEvent_delta<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.delta(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_delta<RetType> {
  fn delta(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  int QGraphicsSceneWheelEvent::delta();
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_delta<i32> for () {
  fn delta(self , rsthis: & QGraphicsSceneWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneWheelEvent5deltaEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneWheelEvent5deltaEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsSceneWheelEvent::scenePos();
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn scenePos<RetType, T: QGraphicsSceneWheelEvent_scenePos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_scenePos<RetType> {
  fn scenePos(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneWheelEvent::scenePos();
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_scenePos<QPointF> for () {
  fn scenePos(self , rsthis: & QGraphicsSceneWheelEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneWheelEvent8scenePosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneWheelEvent8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QGraphicsSceneWheelEvent::screenPos();
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn screenPos<RetType, T: QGraphicsSceneWheelEvent_screenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}

  // proto:  QPoint QGraphicsSceneWheelEvent::screenPos();
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_screenPos<QPoint> for () {
  fn screenPos(self , rsthis: & QGraphicsSceneWheelEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QGraphicsSceneWheelEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK24QGraphicsSceneWheelEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsSceneDragDropEvent {
    return QGraphicsSceneDragDropEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsSceneDragDropEvent {
  type Target = QGraphicsSceneEvent;

  fn deref(&self) -> &QGraphicsSceneEvent {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsSceneEvent> for QGraphicsSceneDragDropEvent {
  fn as_ref(& self) -> & QGraphicsSceneEvent {
    return & self.qbase;
  }
}
  // proto:  QWidget * QGraphicsSceneDragDropEvent::source();
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn source<RetType, T: QGraphicsSceneDragDropEvent_source<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.source(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_source<RetType> {
  fn source(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}

  // proto:  QWidget * QGraphicsSceneDragDropEvent::source();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_source<QWidget> for () {
  fn source(self , rsthis: & QGraphicsSceneDragDropEvent) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent6sourceEv()};
    let mut ret = unsafe {_ZNK27QGraphicsSceneDragDropEvent6sourceEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsSceneDragDropEvent::scenePos();
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn scenePos<RetType, T: QGraphicsSceneDragDropEvent_scenePos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_scenePos<RetType> {
  fn scenePos(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneDragDropEvent::scenePos();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_scenePos<QPointF> for () {
  fn scenePos(self , rsthis: & QGraphicsSceneDragDropEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent8scenePosEv()};
    let mut ret = unsafe {_ZNK27QGraphicsSceneDragDropEvent8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneDragDropEvent::setPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setPos<RetType, T: QGraphicsSceneDragDropEvent_setPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setPos<RetType> {
  fn setPos(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}

  // proto:  void QGraphicsSceneDragDropEvent::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setPos<()> for (&'a QPointF) {
  fn setPos(self , rsthis: & QGraphicsSceneDragDropEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QGraphicsSceneDragDropEvent6setPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneDragDropEvent::QGraphicsSceneDragDropEvent(const QGraphicsSceneDragDropEvent & );
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn New<T: QGraphicsSceneDragDropEvent_New>(value: T) -> QGraphicsSceneDragDropEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_New {
  fn New(self) -> QGraphicsSceneDragDropEvent;
}

  // proto:  void QGraphicsSceneDragDropEvent::QGraphicsSceneDragDropEvent(const QGraphicsSceneDragDropEvent & );
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_New for (&'a QGraphicsSceneDragDropEvent) {
  fn New(self) -> QGraphicsSceneDragDropEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEventC1ERKS_()};
    let ctysz: c_int = unsafe{QGraphicsSceneDragDropEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN27QGraphicsSceneDragDropEventC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN27QGraphicsSceneDragDropEventC1ERKS_(arg0)} as u64;
    let rsthis = QGraphicsSceneDragDropEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneDragDropEvent::setScreenPos(const QPoint & pos);
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setScreenPos<RetType, T: QGraphicsSceneDragDropEvent_setScreenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScreenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setScreenPos<RetType> {
  fn setScreenPos(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}

  // proto:  void QGraphicsSceneDragDropEvent::setScreenPos(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setScreenPos<()> for (&'a QPoint) {
  fn setScreenPos(self , rsthis: & QGraphicsSceneDragDropEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent12setScreenPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QGraphicsSceneDragDropEvent12setScreenPosERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsSceneDragDropEvent::pos();
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn pos<RetType, T: QGraphicsSceneDragDropEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_pos<RetType> {
  fn pos(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}

  // proto:  QPointF QGraphicsSceneDragDropEvent::pos();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_pos<QPointF> for () {
  fn pos(self , rsthis: & QGraphicsSceneDragDropEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent3posEv()};
    let mut ret = unsafe {_ZNK27QGraphicsSceneDragDropEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QGraphicsSceneDragDropEvent::screenPos();
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn screenPos<RetType, T: QGraphicsSceneDragDropEvent_screenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}

  // proto:  QPoint QGraphicsSceneDragDropEvent::screenPos();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_screenPos<QPoint> for () {
  fn screenPos(self , rsthis: & QGraphicsSceneDragDropEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK27QGraphicsSceneDragDropEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMimeData * QGraphicsSceneDragDropEvent::mimeData();
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn mimeData<RetType, T: QGraphicsSceneDragDropEvent_mimeData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mimeData(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_mimeData<RetType> {
  fn mimeData(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}

  // proto:  const QMimeData * QGraphicsSceneDragDropEvent::mimeData();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_mimeData<QMimeData> for () {
  fn mimeData(self , rsthis: & QGraphicsSceneDragDropEvent) -> QMimeData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QGraphicsSceneDragDropEvent8mimeDataEv()};
    let mut ret = unsafe {_ZNK27QGraphicsSceneDragDropEvent8mimeDataEv(rsthis.qclsinst)};
    let mut ret1 = QMimeData::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneDragDropEvent::~QGraphicsSceneDragDropEvent();
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn Free<RetType, T: QGraphicsSceneDragDropEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_Free<RetType> {
  fn Free(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}

  // proto:  void QGraphicsSceneDragDropEvent::~QGraphicsSceneDragDropEvent();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_Free<()> for () {
  fn Free(self , rsthis: & QGraphicsSceneDragDropEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEventD0Ev()};
     unsafe {_ZN27QGraphicsSceneDragDropEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneDragDropEvent::setMimeData(const QMimeData * data);
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setMimeData<RetType, T: QGraphicsSceneDragDropEvent_setMimeData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMimeData(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setMimeData<RetType> {
  fn setMimeData(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}

  // proto:  void QGraphicsSceneDragDropEvent::setMimeData(const QMimeData * data);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setMimeData<()> for (&'a QMimeData) {
  fn setMimeData(self , rsthis: & QGraphicsSceneDragDropEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent11setMimeDataEPK9QMimeData()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QGraphicsSceneDragDropEvent11setMimeDataEPK9QMimeData(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneDragDropEvent::setSource(QWidget * source);
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setSource<RetType, T: QGraphicsSceneDragDropEvent_setSource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSource(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setSource<RetType> {
  fn setSource(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}

  // proto:  void QGraphicsSceneDragDropEvent::setSource(QWidget * source);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setSource<()> for (&'a QWidget) {
  fn setSource(self , rsthis: & QGraphicsSceneDragDropEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent9setSourceEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QGraphicsSceneDragDropEvent9setSourceEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneDragDropEvent::setScenePos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setScenePos<RetType, T: QGraphicsSceneDragDropEvent_setScenePos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScenePos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_setScenePos<RetType> {
  fn setScenePos(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}

  // proto:  void QGraphicsSceneDragDropEvent::setScenePos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setScenePos<()> for (&'a QPointF) {
  fn setScenePos(self , rsthis: & QGraphicsSceneDragDropEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent11setScenePosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QGraphicsSceneDragDropEvent11setScenePosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneDragDropEvent::acceptProposedAction();
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn acceptProposedAction<RetType, T: QGraphicsSceneDragDropEvent_acceptProposedAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.acceptProposedAction(self);
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_acceptProposedAction<RetType> {
  fn acceptProposedAction(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}

  // proto:  void QGraphicsSceneDragDropEvent::acceptProposedAction();
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_acceptProposedAction<()> for () {
  fn acceptProposedAction(self , rsthis: & QGraphicsSceneDragDropEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QGraphicsSceneDragDropEvent20acceptProposedActionEv()};
     unsafe {_ZN27QGraphicsSceneDragDropEvent20acceptProposedActionEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsSceneEvent {
    return QGraphicsSceneEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsSceneEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QGraphicsSceneEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsSceneEvent::QGraphicsSceneEvent(const QGraphicsSceneEvent & );
impl /*struct*/ QGraphicsSceneEvent {
  pub fn New<T: QGraphicsSceneEvent_New>(value: T) -> QGraphicsSceneEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneEvent_New {
  fn New(self) -> QGraphicsSceneEvent;
}

  // proto:  void QGraphicsSceneEvent::QGraphicsSceneEvent(const QGraphicsSceneEvent & );
impl<'a> /*trait*/ QGraphicsSceneEvent_New for (&'a QGraphicsSceneEvent) {
  fn New(self) -> QGraphicsSceneEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsSceneEventC1ERKS_()};
    let ctysz: c_int = unsafe{QGraphicsSceneEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QGraphicsSceneEventC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN19QGraphicsSceneEventC1ERKS_(arg0)} as u64;
    let rsthis = QGraphicsSceneEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QWidget * QGraphicsSceneEvent::widget();
impl /*struct*/ QGraphicsSceneEvent {
  pub fn widget<RetType, T: QGraphicsSceneEvent_widget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QGraphicsSceneEvent_widget<RetType> {
  fn widget(self , rsthis: & QGraphicsSceneEvent) -> RetType;
}

  // proto:  QWidget * QGraphicsSceneEvent::widget();
impl<'a> /*trait*/ QGraphicsSceneEvent_widget<QWidget> for () {
  fn widget(self , rsthis: & QGraphicsSceneEvent) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsSceneEvent6widgetEv()};
    let mut ret = unsafe {_ZNK19QGraphicsSceneEvent6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneEvent::setWidget(QWidget * widget);
impl /*struct*/ QGraphicsSceneEvent {
  pub fn setWidget<RetType, T: QGraphicsSceneEvent_setWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidget(self);
    // return 1;
  }
}

pub trait QGraphicsSceneEvent_setWidget<RetType> {
  fn setWidget(self , rsthis: & QGraphicsSceneEvent) -> RetType;
}

  // proto:  void QGraphicsSceneEvent::setWidget(QWidget * widget);
impl<'a> /*trait*/ QGraphicsSceneEvent_setWidget<()> for (&'a QWidget) {
  fn setWidget(self , rsthis: & QGraphicsSceneEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsSceneEvent9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsSceneEvent9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneEvent::~QGraphicsSceneEvent();
impl /*struct*/ QGraphicsSceneEvent {
  pub fn Free<RetType, T: QGraphicsSceneEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGraphicsSceneEvent_Free<RetType> {
  fn Free(self , rsthis: & QGraphicsSceneEvent) -> RetType;
}

  // proto:  void QGraphicsSceneEvent::~QGraphicsSceneEvent();
impl<'a> /*trait*/ QGraphicsSceneEvent_Free<()> for () {
  fn Free(self , rsthis: & QGraphicsSceneEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsSceneEventD0Ev()};
     unsafe {_ZN19QGraphicsSceneEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsSceneResizeEvent {
    return QGraphicsSceneResizeEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsSceneResizeEvent {
  type Target = QGraphicsSceneEvent;

  fn deref(&self) -> &QGraphicsSceneEvent {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsSceneEvent> for QGraphicsSceneResizeEvent {
  fn as_ref(& self) -> & QGraphicsSceneEvent {
    return & self.qbase;
  }
}
  // proto:  QSizeF QGraphicsSceneResizeEvent::newSize();
impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn newSize<RetType, T: QGraphicsSceneResizeEvent_newSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.newSize(self);
    // return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_newSize<RetType> {
  fn newSize(self , rsthis: & QGraphicsSceneResizeEvent) -> RetType;
}

  // proto:  QSizeF QGraphicsSceneResizeEvent::newSize();
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_newSize<QSizeF> for () {
  fn newSize(self , rsthis: & QGraphicsSceneResizeEvent) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsSceneResizeEvent7newSizeEv()};
    let mut ret = unsafe {_ZNK25QGraphicsSceneResizeEvent7newSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSizeF QGraphicsSceneResizeEvent::oldSize();
impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn oldSize<RetType, T: QGraphicsSceneResizeEvent_oldSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.oldSize(self);
    // return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_oldSize<RetType> {
  fn oldSize(self , rsthis: & QGraphicsSceneResizeEvent) -> RetType;
}

  // proto:  QSizeF QGraphicsSceneResizeEvent::oldSize();
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_oldSize<QSizeF> for () {
  fn oldSize(self , rsthis: & QGraphicsSceneResizeEvent) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsSceneResizeEvent7oldSizeEv()};
    let mut ret = unsafe {_ZNK25QGraphicsSceneResizeEvent7oldSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneResizeEvent::~QGraphicsSceneResizeEvent();
impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn Free<RetType, T: QGraphicsSceneResizeEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_Free<RetType> {
  fn Free(self , rsthis: & QGraphicsSceneResizeEvent) -> RetType;
}

  // proto:  void QGraphicsSceneResizeEvent::~QGraphicsSceneResizeEvent();
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_Free<()> for () {
  fn Free(self , rsthis: & QGraphicsSceneResizeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsSceneResizeEventD0Ev()};
     unsafe {_ZN25QGraphicsSceneResizeEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneResizeEvent::setNewSize(const QSizeF & size);
impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn setNewSize<RetType, T: QGraphicsSceneResizeEvent_setNewSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNewSize(self);
    // return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_setNewSize<RetType> {
  fn setNewSize(self , rsthis: & QGraphicsSceneResizeEvent) -> RetType;
}

  // proto:  void QGraphicsSceneResizeEvent::setNewSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_setNewSize<()> for (&'a QSizeF) {
  fn setNewSize(self , rsthis: & QGraphicsSceneResizeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsSceneResizeEvent10setNewSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QGraphicsSceneResizeEvent10setNewSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneResizeEvent::QGraphicsSceneResizeEvent();
impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn New<T: QGraphicsSceneResizeEvent_New>(value: T) -> QGraphicsSceneResizeEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_New {
  fn New(self) -> QGraphicsSceneResizeEvent;
}

  // proto:  void QGraphicsSceneResizeEvent::QGraphicsSceneResizeEvent();
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_New for () {
  fn New(self) -> QGraphicsSceneResizeEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsSceneResizeEventC1Ev()};
    let ctysz: c_int = unsafe{QGraphicsSceneResizeEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN25QGraphicsSceneResizeEventC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN25QGraphicsSceneResizeEventC1Ev()} as u64;
    let rsthis = QGraphicsSceneResizeEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsSceneResizeEvent::setOldSize(const QSizeF & size);
impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn setOldSize<RetType, T: QGraphicsSceneResizeEvent_setOldSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOldSize(self);
    // return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_setOldSize<RetType> {
  fn setOldSize(self , rsthis: & QGraphicsSceneResizeEvent) -> RetType;
}

  // proto:  void QGraphicsSceneResizeEvent::setOldSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_setOldSize<()> for (&'a QSizeF) {
  fn setOldSize(self , rsthis: & QGraphicsSceneResizeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsSceneResizeEvent10setOldSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QGraphicsSceneResizeEvent10setOldSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsSceneResizeEvent::QGraphicsSceneResizeEvent(const QGraphicsSceneResizeEvent & );
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_New for (&'a QGraphicsSceneResizeEvent) {
  fn New(self) -> QGraphicsSceneResizeEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsSceneResizeEventC1ERKS_()};
    let ctysz: c_int = unsafe{QGraphicsSceneResizeEvent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN25QGraphicsSceneResizeEventC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN25QGraphicsSceneResizeEventC1ERKS_(arg0)} as u64;
    let rsthis = QGraphicsSceneResizeEvent{qbase: QGraphicsSceneEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

