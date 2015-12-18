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
  pub fn setFrameRect<RetType, T: QFrame_setFrameRect<RetType>>(&mut self, value: T) -> RetType {
    return value.setFrameRect(self);
    // return 1;
  }
}

pub trait QFrame_setFrameRect<RetType> {
  fn setFrameRect(self, rsthis: &mut QFrame) -> RetType;
}

// proto:  void QFrame::setFrameRect(const QRect & );
impl<'a> /*trait*/ QFrame_setFrameRect<()> for (&'a  QRect) {
  fn setFrameRect(self, rsthis: &mut QFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrame12setFrameRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QFrame12setFrameRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn lineWidth<RetType, T: QFrame_lineWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.lineWidth(self);
    // return 1;
  }
}

pub trait QFrame_lineWidth<RetType> {
  fn lineWidth(self, rsthis: &mut QFrame) -> RetType;
}

// proto:  int QFrame::lineWidth();
impl<'a> /*trait*/ QFrame_lineWidth<i32> for () {
  fn lineWidth(self, rsthis: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame9lineWidthEv()};
    let mut ret = unsafe {_ZNK6QFrame9lineWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn setFrameStyle<RetType, T: QFrame_setFrameStyle<RetType>>(&mut self, value: T) -> RetType {
    return value.setFrameStyle(self);
    // return 1;
  }
}

pub trait QFrame_setFrameStyle<RetType> {
  fn setFrameStyle(self, rsthis: &mut QFrame) -> RetType;
}

// proto:  void QFrame::setFrameStyle(int );
impl<'a> /*trait*/ QFrame_setFrameStyle<()> for (i32) {
  fn setFrameStyle(self, rsthis: &mut QFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrame13setFrameStyleEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QFrame13setFrameStyleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn frameRect<RetType, T: QFrame_frameRect<RetType>>(&mut self, value: T) -> RetType {
    return value.frameRect(self);
    // return 1;
  }
}

pub trait QFrame_frameRect<RetType> {
  fn frameRect(self, rsthis: &mut QFrame) -> RetType;
}

// proto:  QRect QFrame::frameRect();
impl<'a> /*trait*/ QFrame_frameRect<QRect> for () {
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
  pub fn sizeHint<RetType, T: QFrame_sizeHint<RetType>>(&mut self, value: T) -> RetType {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QFrame_sizeHint<RetType> {
  fn sizeHint(self, rsthis: &mut QFrame) -> RetType;
}

// proto:  QSize QFrame::sizeHint();
impl<'a> /*trait*/ QFrame_sizeHint<QSize> for () {
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
  pub fn frameStyle<RetType, T: QFrame_frameStyle<RetType>>(&mut self, value: T) -> RetType {
    return value.frameStyle(self);
    // return 1;
  }
}

pub trait QFrame_frameStyle<RetType> {
  fn frameStyle(self, rsthis: &mut QFrame) -> RetType;
}

// proto:  int QFrame::frameStyle();
impl<'a> /*trait*/ QFrame_frameStyle<i32> for () {
  fn frameStyle(self, rsthis: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame10frameStyleEv()};
    let mut ret = unsafe {_ZNK6QFrame10frameStyleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn midLineWidth<RetType, T: QFrame_midLineWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.midLineWidth(self);
    // return 1;
  }
}

pub trait QFrame_midLineWidth<RetType> {
  fn midLineWidth(self, rsthis: &mut QFrame) -> RetType;
}

// proto:  int QFrame::midLineWidth();
impl<'a> /*trait*/ QFrame_midLineWidth<i32> for () {
  fn midLineWidth(self, rsthis: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame12midLineWidthEv()};
    let mut ret = unsafe {_ZNK6QFrame12midLineWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn setLineWidth<RetType, T: QFrame_setLineWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.setLineWidth(self);
    // return 1;
  }
}

pub trait QFrame_setLineWidth<RetType> {
  fn setLineWidth(self, rsthis: &mut QFrame) -> RetType;
}

// proto:  void QFrame::setLineWidth(int );
impl<'a> /*trait*/ QFrame_setLineWidth<()> for (i32) {
  fn setLineWidth(self, rsthis: &mut QFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrame12setLineWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QFrame12setLineWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn setMidLineWidth<RetType, T: QFrame_setMidLineWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.setMidLineWidth(self);
    // return 1;
  }
}

pub trait QFrame_setMidLineWidth<RetType> {
  fn setMidLineWidth(self, rsthis: &mut QFrame) -> RetType;
}

// proto:  void QFrame::setMidLineWidth(int );
impl<'a> /*trait*/ QFrame_setMidLineWidth<()> for (i32) {
  fn setMidLineWidth(self, rsthis: &mut QFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrame15setMidLineWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QFrame15setMidLineWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn metaObject<RetType, T: QFrame_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QFrame_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QFrame) -> RetType;
}

// proto:  const QMetaObject * QFrame::metaObject();
impl<'a> /*trait*/ QFrame_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame10metaObjectEv()};
     unsafe {_ZNK6QFrame10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn frameWidth<RetType, T: QFrame_frameWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.frameWidth(self);
    // return 1;
  }
}

pub trait QFrame_frameWidth<RetType> {
  fn frameWidth(self, rsthis: &mut QFrame) -> RetType;
}

// proto:  int QFrame::frameWidth();
impl<'a> /*trait*/ QFrame_frameWidth<i32> for () {
  fn frameWidth(self, rsthis: &mut QFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QFrame10frameWidthEv()};
    let mut ret = unsafe {_ZNK6QFrame10frameWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFrame {
  pub fn FreeQFrame<RetType, T: QFrame_FreeQFrame<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQFrame(self);
    // return 1;
  }
}

pub trait QFrame_FreeQFrame<RetType> {
  fn FreeQFrame(self, rsthis: &mut QFrame) -> RetType;
}

// proto:  void QFrame::FreeQFrame();
impl<'a> /*trait*/ QFrame_FreeQFrame<()> for () {
  fn FreeQFrame(self, rsthis: &mut QFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QFrameD0Ev()};
     unsafe {_ZN6QFrameD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

