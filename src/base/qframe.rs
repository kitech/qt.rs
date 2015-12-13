// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QFrame::setFrameRect(const QRect & );
  fn _ZN6QFrame12setFrameRectERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QFrame::lineWidth();
  fn _ZNK6QFrame9lineWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QFrame::setFrameStyle(int );
  fn _ZN6QFrame13setFrameStyleEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QRect QFrame::frameRect();
  fn _ZNK6QFrame9frameRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QFrame::sizeHint();
  fn _ZNK6QFrame8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFrame::NewQFrame(const QFrame & );
  fn _ZN6QFrameC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QFrame::frameStyle();
  fn _ZNK6QFrame10frameStyleEv(qthis: *mut c_void) -> c_int;
  // proto:  int QFrame::midLineWidth();
  fn _ZNK6QFrame12midLineWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QFrame::setLineWidth(int );
  fn _ZN6QFrame12setLineWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QFrame::setMidLineWidth(int );
  fn _ZN6QFrame15setMidLineWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  const QMetaObject * QFrame::metaObject();
  fn _ZNK6QFrame10metaObjectEv(qthis: *mut c_void) ;
  // proto:  int QFrame::frameWidth();
  fn _ZNK6QFrame10frameWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QFrame::FreeQFrame();
  fn _ZN6QFrameD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QFrame)=1
pub struct QFrame {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFrame {
  pub fn setFrameRect<T: QFrame_setFrameRect>(&mut self, value: T)  {
     value.setFrameRect(self);
    // return 1;
  }
}

pub trait QFrame_setFrameRect {
  fn setFrameRect(self, rsthis: &mut QFrame) ;
}

// proto:  void QFrame::setFrameRect(const QRect & );
impl<'a> /*trait*/ QFrame_setFrameRect for (&'a  QRect) {
  fn setFrameRect(self, rsthis: &mut QFrame)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrame12setFrameRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QFrame12setFrameRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn lineWidth<T: QFrame_lineWidth>(&mut self, value: T) -> i32 {
    return value.lineWidth(self);
    // return 1;
  }
}

pub trait QFrame_lineWidth {
  fn lineWidth(self, rsthis: &mut QFrame) -> i32;
}

// proto:  int QFrame::lineWidth();
impl<'a> /*trait*/ QFrame_lineWidth for () {
  fn lineWidth(self, rsthis: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame9lineWidthEv()};
    let mut ret = unsafe {_ZNK6QFrame9lineWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn setFrameStyle<T: QFrame_setFrameStyle>(&mut self, value: T)  {
     value.setFrameStyle(self);
    // return 1;
  }
}

pub trait QFrame_setFrameStyle {
  fn setFrameStyle(self, rsthis: &mut QFrame) ;
}

// proto:  void QFrame::setFrameStyle(int );
impl<'a> /*trait*/ QFrame_setFrameStyle for (i32) {
  fn setFrameStyle(self, rsthis: &mut QFrame)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrame13setFrameStyleEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QFrame13setFrameStyleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn frameRect<T: QFrame_frameRect>(&mut self, value: T) -> QRect {
    return value.frameRect(self);
    // return 1;
  }
}

pub trait QFrame_frameRect {
  fn frameRect(self, rsthis: &mut QFrame) -> QRect;
}

// proto:  QRect QFrame::frameRect();
impl<'a> /*trait*/ QFrame_frameRect for () {
  fn frameRect(self, rsthis: &mut QFrame) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame9frameRectEv()};
    let mut ret = unsafe {_ZNK6QFrame9frameRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn sizeHint<T: QFrame_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QFrame_sizeHint {
  fn sizeHint(self, rsthis: &mut QFrame) -> QSize;
}

// proto:  QSize QFrame::sizeHint();
impl<'a> /*trait*/ QFrame_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QFrame) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame8sizeHintEv()};
    let mut ret = unsafe {_ZNK6QFrame8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn NewQFrame<T: QFrame_NewQFrame>(value: T) -> QFrame {
    let rsthis = value.NewQFrame();
    return rsthis;
    // return 1;
  }
}

pub trait QFrame_NewQFrame {
  fn NewQFrame(self) -> QFrame;
}

// proto: void QFrame::NewQFrame(const QFrame & );
impl<'a> /*trait*/ QFrame_NewQFrame for (&'a  QFrame) {
  fn NewQFrame(self) -> QFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrameC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QFrameC1ERKS_(qthis, arg0)};
    let rsthis = QFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn frameStyle<T: QFrame_frameStyle>(&mut self, value: T) -> i32 {
    return value.frameStyle(self);
    // return 1;
  }
}

pub trait QFrame_frameStyle {
  fn frameStyle(self, rsthis: &mut QFrame) -> i32;
}

// proto:  int QFrame::frameStyle();
impl<'a> /*trait*/ QFrame_frameStyle for () {
  fn frameStyle(self, rsthis: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame10frameStyleEv()};
    let mut ret = unsafe {_ZNK6QFrame10frameStyleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn midLineWidth<T: QFrame_midLineWidth>(&mut self, value: T) -> i32 {
    return value.midLineWidth(self);
    // return 1;
  }
}

pub trait QFrame_midLineWidth {
  fn midLineWidth(self, rsthis: &mut QFrame) -> i32;
}

// proto:  int QFrame::midLineWidth();
impl<'a> /*trait*/ QFrame_midLineWidth for () {
  fn midLineWidth(self, rsthis: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame12midLineWidthEv()};
    let mut ret = unsafe {_ZNK6QFrame12midLineWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn setLineWidth<T: QFrame_setLineWidth>(&mut self, value: T)  {
     value.setLineWidth(self);
    // return 1;
  }
}

pub trait QFrame_setLineWidth {
  fn setLineWidth(self, rsthis: &mut QFrame) ;
}

// proto:  void QFrame::setLineWidth(int );
impl<'a> /*trait*/ QFrame_setLineWidth for (i32) {
  fn setLineWidth(self, rsthis: &mut QFrame)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrame12setLineWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QFrame12setLineWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn setMidLineWidth<T: QFrame_setMidLineWidth>(&mut self, value: T)  {
     value.setMidLineWidth(self);
    // return 1;
  }
}

pub trait QFrame_setMidLineWidth {
  fn setMidLineWidth(self, rsthis: &mut QFrame) ;
}

// proto:  void QFrame::setMidLineWidth(int );
impl<'a> /*trait*/ QFrame_setMidLineWidth for (i32) {
  fn setMidLineWidth(self, rsthis: &mut QFrame)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrame15setMidLineWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QFrame15setMidLineWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn metaObject<T: QFrame_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QFrame_metaObject {
  fn metaObject(self, rsthis: &mut QFrame) ;
}

// proto:  const QMetaObject * QFrame::metaObject();
impl<'a> /*trait*/ QFrame_metaObject for () {
  fn metaObject(self, rsthis: &mut QFrame)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame10metaObjectEv()};
     unsafe {_ZNK6QFrame10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn frameWidth<T: QFrame_frameWidth>(&mut self, value: T) -> i32 {
    return value.frameWidth(self);
    // return 1;
  }
}

pub trait QFrame_frameWidth {
  fn frameWidth(self, rsthis: &mut QFrame) -> i32;
}

// proto:  int QFrame::frameWidth();
impl<'a> /*trait*/ QFrame_frameWidth for () {
  fn frameWidth(self, rsthis: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame10frameWidthEv()};
    let mut ret = unsafe {_ZNK6QFrame10frameWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn FreeQFrame<T: QFrame_FreeQFrame>(&mut self, value: T)  {
     value.FreeQFrame(self);
    // return 1;
  }
}

pub trait QFrame_FreeQFrame {
  fn FreeQFrame(self, rsthis: &mut QFrame) ;
}

// proto:  void QFrame::FreeQFrame();
impl<'a> /*trait*/ QFrame_FreeQFrame for () {
  fn FreeQFrame(self, rsthis: &mut QFrame)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrameD0Ev()};
     unsafe {_ZN6QFrameD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

