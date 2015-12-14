// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrectf::QRectF;
use super::qpointf::QPointF;
use super::qsizef::QSizeF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QScrollPrepareEvent::setContentPosRange(const QRectF & rect);
  fn _ZN19QScrollPrepareEvent18setContentPosRangeERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QScrollPrepareEvent::setContentPos(const QPointF & pos);
  fn _ZN19QScrollPrepareEvent13setContentPosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRectF QScrollPrepareEvent::contentPosRange();
  fn _ZNK19QScrollPrepareEvent15contentPosRangeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QScrollPrepareEvent::contentPos();
  fn _ZNK19QScrollPrepareEvent10contentPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScrollPrepareEvent::setViewportSize(const QSizeF & size);
  fn _ZN19QScrollPrepareEvent15setViewportSizeERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QScrollPrepareEvent::NewQScrollPrepareEvent(const QPointF & startPos);
  fn _ZN19QScrollPrepareEventC1ERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPointF QScrollPrepareEvent::startPos();
  fn _ZNK19QScrollPrepareEvent8startPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSizeF QScrollPrepareEvent::viewportSize();
  fn _ZNK19QScrollPrepareEvent12viewportSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScrollPrepareEvent::FreeQScrollPrepareEvent();
  fn _ZN19QScrollPrepareEventD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QScrollPrepareEvent)=112
pub struct QScrollPrepareEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QScrollPrepareEvent {
  pub fn setContentPosRange<T: QScrollPrepareEvent_setContentPosRange>(&mut self, value: T)  {
     value.setContentPosRange(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_setContentPosRange {
  fn setContentPosRange(self, rsthis: &mut QScrollPrepareEvent) ;
}

// proto:  void QScrollPrepareEvent::setContentPosRange(const QRectF & rect);
impl<'a> /*trait*/ QScrollPrepareEvent_setContentPosRange for (&'a  QRectF) {
  fn setContentPosRange(self, rsthis: &mut QScrollPrepareEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZN19QScrollPrepareEvent18setContentPosRangeERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QScrollPrepareEvent18setContentPosRangeERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QScrollPrepareEvent {
  pub fn setContentPos<T: QScrollPrepareEvent_setContentPos>(&mut self, value: T)  {
     value.setContentPos(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_setContentPos {
  fn setContentPos(self, rsthis: &mut QScrollPrepareEvent) ;
}

// proto:  void QScrollPrepareEvent::setContentPos(const QPointF & pos);
impl<'a> /*trait*/ QScrollPrepareEvent_setContentPos for (&'a  QPointF) {
  fn setContentPos(self, rsthis: &mut QScrollPrepareEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZN19QScrollPrepareEvent13setContentPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QScrollPrepareEvent13setContentPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QScrollPrepareEvent {
  pub fn contentPosRange<T: QScrollPrepareEvent_contentPosRange>(&mut self, value: T) -> QRectF {
    return value.contentPosRange(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_contentPosRange {
  fn contentPosRange(self, rsthis: &mut QScrollPrepareEvent) -> QRectF;
}

// proto:  QRectF QScrollPrepareEvent::contentPosRange();
impl<'a> /*trait*/ QScrollPrepareEvent_contentPosRange for () {
  fn contentPosRange(self, rsthis: &mut QScrollPrepareEvent) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZNK19QScrollPrepareEvent15contentPosRangeEv()};
    let mut ret = unsafe {_ZNK19QScrollPrepareEvent15contentPosRangeEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScrollPrepareEvent {
  pub fn contentPos<T: QScrollPrepareEvent_contentPos>(&mut self, value: T) -> QPointF {
    return value.contentPos(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_contentPos {
  fn contentPos(self, rsthis: &mut QScrollPrepareEvent) -> QPointF;
}

// proto:  QPointF QScrollPrepareEvent::contentPos();
impl<'a> /*trait*/ QScrollPrepareEvent_contentPos for () {
  fn contentPos(self, rsthis: &mut QScrollPrepareEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZNK19QScrollPrepareEvent10contentPosEv()};
    let mut ret = unsafe {_ZNK19QScrollPrepareEvent10contentPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScrollPrepareEvent {
  pub fn setViewportSize<T: QScrollPrepareEvent_setViewportSize>(&mut self, value: T)  {
     value.setViewportSize(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_setViewportSize {
  fn setViewportSize(self, rsthis: &mut QScrollPrepareEvent) ;
}

// proto:  void QScrollPrepareEvent::setViewportSize(const QSizeF & size);
impl<'a> /*trait*/ QScrollPrepareEvent_setViewportSize for (&'a  QSizeF) {
  fn setViewportSize(self, rsthis: &mut QScrollPrepareEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZN19QScrollPrepareEvent15setViewportSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QScrollPrepareEvent15setViewportSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QScrollPrepareEvent {
  pub fn NewQScrollPrepareEvent<T: QScrollPrepareEvent_NewQScrollPrepareEvent>(value: T) -> QScrollPrepareEvent {
    let rsthis = value.NewQScrollPrepareEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QScrollPrepareEvent_NewQScrollPrepareEvent {
  fn NewQScrollPrepareEvent(self) -> QScrollPrepareEvent;
}

// proto: void QScrollPrepareEvent::NewQScrollPrepareEvent(const QPointF & startPos);
impl<'a> /*trait*/ QScrollPrepareEvent_NewQScrollPrepareEvent for (&'a  QPointF) {
  fn NewQScrollPrepareEvent(self) -> QScrollPrepareEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZN19QScrollPrepareEventC1ERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QScrollPrepareEventC1ERK7QPointF(qthis, arg0)};
    let rsthis = QScrollPrepareEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QScrollPrepareEvent {
  pub fn startPos<T: QScrollPrepareEvent_startPos>(&mut self, value: T) -> QPointF {
    return value.startPos(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_startPos {
  fn startPos(self, rsthis: &mut QScrollPrepareEvent) -> QPointF;
}

// proto:  QPointF QScrollPrepareEvent::startPos();
impl<'a> /*trait*/ QScrollPrepareEvent_startPos for () {
  fn startPos(self, rsthis: &mut QScrollPrepareEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZNK19QScrollPrepareEvent8startPosEv()};
    let mut ret = unsafe {_ZNK19QScrollPrepareEvent8startPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScrollPrepareEvent {
  pub fn viewportSize<T: QScrollPrepareEvent_viewportSize>(&mut self, value: T) -> QSizeF {
    return value.viewportSize(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_viewportSize {
  fn viewportSize(self, rsthis: &mut QScrollPrepareEvent) -> QSizeF;
}

// proto:  QSizeF QScrollPrepareEvent::viewportSize();
impl<'a> /*trait*/ QScrollPrepareEvent_viewportSize for () {
  fn viewportSize(self, rsthis: &mut QScrollPrepareEvent) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZNK19QScrollPrepareEvent12viewportSizeEv()};
    let mut ret = unsafe {_ZNK19QScrollPrepareEvent12viewportSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScrollPrepareEvent {
  pub fn FreeQScrollPrepareEvent<T: QScrollPrepareEvent_FreeQScrollPrepareEvent>(&mut self, value: T)  {
     value.FreeQScrollPrepareEvent(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_FreeQScrollPrepareEvent {
  fn FreeQScrollPrepareEvent(self, rsthis: &mut QScrollPrepareEvent) ;
}

// proto:  void QScrollPrepareEvent::FreeQScrollPrepareEvent();
impl<'a> /*trait*/ QScrollPrepareEvent_FreeQScrollPrepareEvent for () {
  fn FreeQScrollPrepareEvent(self, rsthis: &mut QScrollPrepareEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZN19QScrollPrepareEventD0Ev()};
     unsafe {_ZN19QScrollPrepareEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

