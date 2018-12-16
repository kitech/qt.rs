

// mod ::gui::QTextObjectInterface
// package qtgui
// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h
// #include <qabstracttextdocumentlayout.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 27
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QTextObjectInterface)=8
pub struct QTextObjectInterface {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextObjectInterface_ITF interface {
//    QTextObjectInterface_PTR() *QTextObjectInterface
//}
//func (ptr *QTextObjectInterface) QTextObjectInterface_PTR() *QTextObjectInterface { return ptr }

impl /*struct*/ QTextObjectInterface {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextObjectInterface {
    return QTextObjectInterface{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextObjectInterface {
//  type Target = QTextObjectInterfaceBASE;
//
//  fn deref(&self) -> &QTextObjectInterfaceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextObjectInterfaceBASE> for QTextObjectInterface {
//  fn as_ref(& self) -> & QTextObjectInterfaceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:141
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTextObjectInterface()

/*

*/
pub fn DeleteQTextObjectInterface(this :*mut QTextObjectInterface) {
    // let rv = qtrt::InvokeQtFunc6("_ZN20QTextObjectInterfaceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:142
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [16] QSizeF intrinsicSize(QTextDocument *, int, const QTextFormat &)

/*

*/
impl /*struct*/ QTextObjectInterface {
  pub fn intrinsicSize_0<RetType, T: QTextObjectInterface_intrinsicSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intrinsicSize_0(self);
    // return 1;
  }
}
pub trait QTextObjectInterface_intrinsicSize_0<RetType> {
  fn intrinsicSize_0(self , rsthis: & QTextObjectInterface) -> RetType;
}
impl<'a> /*trait*/ QTextObjectInterface_intrinsicSize_0<usize> for (usize,i32,usize) {
  fn intrinsicSize_0(self , rsthis: & QTextObjectInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QTextObjectInterface13intrinsicSizeEP13QTextDocumentiRK11QTextFormat", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:143
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void drawObject(QPainter *, const QRectF &, QTextDocument *, int, const QTextFormat &)

/*

*/
impl /*struct*/ QTextObjectInterface {
  pub fn drawObject_0<RetType, T: QTextObjectInterface_drawObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawObject_0(self);
    // return 1;
  }
}
pub trait QTextObjectInterface_drawObject_0<RetType> {
  fn drawObject_0(self , rsthis: & QTextObjectInterface) -> RetType;
}
impl<'a> /*trait*/ QTextObjectInterface_drawObject_0<(/*void*/)> for (usize,usize,usize,i32,usize) {
  fn drawObject_0(self , rsthis: & QTextObjectInterface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QTextObjectInterface10drawObjectEP8QPainterRK6QRectFP13QTextDocumentiRK11QTextFormat", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
