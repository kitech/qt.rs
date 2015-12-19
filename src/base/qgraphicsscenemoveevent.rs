// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QPointF QGraphicsSceneMoveEvent::newPos();
  fn _ZNK23QGraphicsSceneMoveEvent6newPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QGraphicsSceneMoveEvent::oldPos();
  fn _ZNK23QGraphicsSceneMoveEvent6oldPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneMoveEvent::FreeQGraphicsSceneMoveEvent();
  fn _ZN23QGraphicsSceneMoveEventD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsSceneMoveEvent::NewQGraphicsSceneMoveEvent(const QGraphicsSceneMoveEvent & );
  fn _ZN23QGraphicsSceneMoveEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSceneMoveEvent::setNewPos(const QPointF & pos);
  fn _ZN23QGraphicsSceneMoveEvent9setNewPosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSceneMoveEvent::NewQGraphicsSceneMoveEvent();
  fn _ZN23QGraphicsSceneMoveEventC1Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsSceneMoveEvent::setOldPos(const QPointF & pos);
  fn _ZN23QGraphicsSceneMoveEvent9setOldPosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QGraphicsSceneMoveEvent)=1
pub struct QGraphicsSceneMoveEvent {
  pub qclsinst: *mut c_void,
}

// proto:  QPointF QGraphicsSceneMoveEvent::newPos();
impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn newPos<RetType, T: QGraphicsSceneMoveEvent_newPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.newPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_newPos<RetType> {
  fn newPos(self , rsthis: &mut QGraphicsSceneMoveEvent) -> RetType;
}

// proto:  QPointF QGraphicsSceneMoveEvent::newPos();
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_newPos<QPointF> for () {
  fn newPos(self , rsthis: &mut QGraphicsSceneMoveEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSceneMoveEvent6newPosEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSceneMoveEvent6newPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QPointF QGraphicsSceneMoveEvent::oldPos();
impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn oldPos<RetType, T: QGraphicsSceneMoveEvent_oldPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.oldPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_oldPos<RetType> {
  fn oldPos(self , rsthis: &mut QGraphicsSceneMoveEvent) -> RetType;
}

// proto:  QPointF QGraphicsSceneMoveEvent::oldPos();
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_oldPos<QPointF> for () {
  fn oldPos(self , rsthis: &mut QGraphicsSceneMoveEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSceneMoveEvent6oldPosEv()};
    let mut ret = unsafe {_ZNK23QGraphicsSceneMoveEvent6oldPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsSceneMoveEvent::FreeQGraphicsSceneMoveEvent();
impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn FreeQGraphicsSceneMoveEvent<RetType, T: QGraphicsSceneMoveEvent_FreeQGraphicsSceneMoveEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsSceneMoveEvent(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_FreeQGraphicsSceneMoveEvent<RetType> {
  fn FreeQGraphicsSceneMoveEvent(self , rsthis: &mut QGraphicsSceneMoveEvent) -> RetType;
}

// proto:  void QGraphicsSceneMoveEvent::FreeQGraphicsSceneMoveEvent();
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_FreeQGraphicsSceneMoveEvent<()> for () {
  fn FreeQGraphicsSceneMoveEvent(self , rsthis: &mut QGraphicsSceneMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneMoveEventD0Ev()};
     unsafe {_ZN23QGraphicsSceneMoveEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn NewQGraphicsSceneMoveEvent<T: QGraphicsSceneMoveEvent_NewQGraphicsSceneMoveEvent>(value: T) -> QGraphicsSceneMoveEvent {
    let rsthis = value.NewQGraphicsSceneMoveEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_NewQGraphicsSceneMoveEvent {
  fn NewQGraphicsSceneMoveEvent(self) -> QGraphicsSceneMoveEvent;
}

// proto: void QGraphicsSceneMoveEvent::NewQGraphicsSceneMoveEvent(const QGraphicsSceneMoveEvent & );
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_NewQGraphicsSceneMoveEvent for (&'a  QGraphicsSceneMoveEvent) {
  fn NewQGraphicsSceneMoveEvent(self) -> QGraphicsSceneMoveEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneMoveEventC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QGraphicsSceneMoveEventC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsSceneMoveEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QGraphicsSceneMoveEvent::setNewPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn setNewPos<RetType, T: QGraphicsSceneMoveEvent_setNewPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setNewPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_setNewPos<RetType> {
  fn setNewPos(self , rsthis: &mut QGraphicsSceneMoveEvent) -> RetType;
}

// proto:  void QGraphicsSceneMoveEvent::setNewPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_setNewPos<()> for (&'a  QPointF) {
  fn setNewPos(self , rsthis: &mut QGraphicsSceneMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneMoveEvent9setNewPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsSceneMoveEvent9setNewPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QGraphicsSceneMoveEvent::NewQGraphicsSceneMoveEvent();
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_NewQGraphicsSceneMoveEvent for () {
  fn NewQGraphicsSceneMoveEvent(self) -> QGraphicsSceneMoveEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneMoveEventC1Ev()};
    unsafe {_ZN23QGraphicsSceneMoveEventC1Ev(qthis)};
    let rsthis = QGraphicsSceneMoveEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QGraphicsSceneMoveEvent::setOldPos(const QPointF & pos);
impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn setOldPos<RetType, T: QGraphicsSceneMoveEvent_setOldPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setOldPos(self);
    // return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_setOldPos<RetType> {
  fn setOldPos(self , rsthis: &mut QGraphicsSceneMoveEvent) -> RetType;
}

// proto:  void QGraphicsSceneMoveEvent::setOldPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_setOldPos<()> for (&'a  QPointF) {
  fn setOldPos(self , rsthis: &mut QGraphicsSceneMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSceneMoveEvent9setOldPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsSceneMoveEvent9setOldPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

