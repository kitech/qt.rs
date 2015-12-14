// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextdocument::QTextDocument;
use super::qtextformat::QTextFormat;
use super::qsizef::QSizeF;
use super::qpainter::QPainter;
use super::qrectf::QRectF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTextObjectInterface::FreeQTextObjectInterface();
  fn _ZN20QTextObjectInterfaceD0Ev(qthis: *mut c_void) ;
  // proto:  QSizeF QTextObjectInterface::intrinsicSize(QTextDocument * doc, int posInDocument, const QTextFormat & format);
  fn _ZN20QTextObjectInterface13intrinsicSizeEP13QTextDocumentiRK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QTextObjectInterface::drawObject(QPainter * painter, const QRectF & rect, QTextDocument * doc, int posInDocument, const QTextFormat & format);
  fn _ZN20QTextObjectInterface10drawObjectEP8QPainterRK6QRectFP13QTextDocumentiRK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: c_int, arg4: *mut c_void) ;
}

// body block begin
// class sizeof(QTextObjectInterface)=8
pub struct QTextObjectInterface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextObjectInterface {
  pub fn FreeQTextObjectInterface<T: QTextObjectInterface_FreeQTextObjectInterface>(&mut self, value: T)  {
     value.FreeQTextObjectInterface(self);
    // return 1;
  }
}

pub trait QTextObjectInterface_FreeQTextObjectInterface {
  fn FreeQTextObjectInterface(self, rsthis: &mut QTextObjectInterface) ;
}

// proto:  void QTextObjectInterface::FreeQTextObjectInterface();
impl<'a> /*trait*/ QTextObjectInterface_FreeQTextObjectInterface for () {
  fn FreeQTextObjectInterface(self, rsthis: &mut QTextObjectInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextObjectInterfaceD0Ev()};
     unsafe {_ZN20QTextObjectInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextObjectInterface {
  pub fn intrinsicSize<T: QTextObjectInterface_intrinsicSize>(&mut self, value: T) -> QSizeF {
    return value.intrinsicSize(self);
    // return 1;
  }
}

pub trait QTextObjectInterface_intrinsicSize {
  fn intrinsicSize(self, rsthis: &mut QTextObjectInterface) -> QSizeF;
}

// proto:  QSizeF QTextObjectInterface::intrinsicSize(QTextDocument * doc, int posInDocument, const QTextFormat & format);
impl<'a> /*trait*/ QTextObjectInterface_intrinsicSize for (&'a mut QTextDocument, i32, &'a  QTextFormat) {
  fn intrinsicSize(self, rsthis: &mut QTextObjectInterface) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextObjectInterface13intrinsicSizeEP13QTextDocumentiRK11QTextFormat()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN20QTextObjectInterface13intrinsicSizeEP13QTextDocumentiRK11QTextFormat(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextObjectInterface {
  pub fn drawObject<T: QTextObjectInterface_drawObject>(&mut self, value: T)  {
     value.drawObject(self);
    // return 1;
  }
}

pub trait QTextObjectInterface_drawObject {
  fn drawObject(self, rsthis: &mut QTextObjectInterface) ;
}

// proto:  void QTextObjectInterface::drawObject(QPainter * painter, const QRectF & rect, QTextDocument * doc, int posInDocument, const QTextFormat & format);
impl<'a> /*trait*/ QTextObjectInterface_drawObject for (&'a mut QPainter, &'a  QRectF, &'a mut QTextDocument, i32, &'a  QTextFormat) {
  fn drawObject(self, rsthis: &mut QTextObjectInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextObjectInterface10drawObjectEP8QPainterRK6QRectFP13QTextDocumentiRK11QTextFormat()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.qclsinst  as *mut c_void;
     unsafe {_ZN20QTextObjectInterface10drawObjectEP8QPainterRK6QRectFP13QTextDocumentiRK11QTextFormat(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

