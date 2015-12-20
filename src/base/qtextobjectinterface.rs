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
  // proto:  void QTextObjectInterface::~QTextObjectInterface();
  fn _ZN20QTextObjectInterfaceD0Ev(qthis: *mut c_void);
  // proto:  QSizeF QTextObjectInterface::intrinsicSize(QTextDocument * doc, int posInDocument, const QTextFormat & format);
  fn _ZN20QTextObjectInterface13intrinsicSizeEP13QTextDocumentiRK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QTextObjectInterface::drawObject(QPainter * painter, const QRectF & rect, QTextDocument * doc, int posInDocument, const QTextFormat & format);
  fn _ZN20QTextObjectInterface10drawObjectEP8QPainterRK6QRectFP13QTextDocumentiRK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: c_int, arg4: *mut c_void);
}

// body block begin
// class sizeof(QTextObjectInterface)=8
pub struct QTextObjectInterface {
  pub qclsinst: *mut c_void,
}

  // proto:  void QTextObjectInterface::~QTextObjectInterface();
impl /*struct*/ QTextObjectInterface {
  pub fn FreeQTextObjectInterface<RetType, T: QTextObjectInterface_FreeQTextObjectInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTextObjectInterface(self);
    // return 1;
  }
}

pub trait QTextObjectInterface_FreeQTextObjectInterface<RetType> {
  fn FreeQTextObjectInterface(self , rsthis: &mut QTextObjectInterface) -> RetType;
}

  // proto:  void QTextObjectInterface::~QTextObjectInterface();
impl<'a> /*trait*/ QTextObjectInterface_FreeQTextObjectInterface<()> for () {
  fn FreeQTextObjectInterface(self , rsthis: &mut QTextObjectInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextObjectInterfaceD0Ev()};
     unsafe {_ZN20QTextObjectInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSizeF QTextObjectInterface::intrinsicSize(QTextDocument * doc, int posInDocument, const QTextFormat & format);
impl /*struct*/ QTextObjectInterface {
  pub fn intrinsicSize<RetType, T: QTextObjectInterface_intrinsicSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intrinsicSize(self);
    // return 1;
  }
}

pub trait QTextObjectInterface_intrinsicSize<RetType> {
  fn intrinsicSize(self , rsthis: &mut QTextObjectInterface) -> RetType;
}

  // proto:  QSizeF QTextObjectInterface::intrinsicSize(QTextDocument * doc, int posInDocument, const QTextFormat & format);
impl<'a> /*trait*/ QTextObjectInterface_intrinsicSize<QSizeF> for (QTextDocument, i32, QTextFormat) {
  fn intrinsicSize(self , rsthis: &mut QTextObjectInterface) -> QSizeF {
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

  // proto:  void QTextObjectInterface::drawObject(QPainter * painter, const QRectF & rect, QTextDocument * doc, int posInDocument, const QTextFormat & format);
impl /*struct*/ QTextObjectInterface {
  pub fn drawObject<RetType, T: QTextObjectInterface_drawObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.drawObject(self);
    // return 1;
  }
}

pub trait QTextObjectInterface_drawObject<RetType> {
  fn drawObject(self , rsthis: &mut QTextObjectInterface) -> RetType;
}

  // proto:  void QTextObjectInterface::drawObject(QPainter * painter, const QRectF & rect, QTextDocument * doc, int posInDocument, const QTextFormat & format);
impl<'a> /*trait*/ QTextObjectInterface_drawObject<()> for (QPainter, QRectF, QTextDocument, i32, QTextFormat) {
  fn drawObject(self , rsthis: &mut QTextObjectInterface) -> () {
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

