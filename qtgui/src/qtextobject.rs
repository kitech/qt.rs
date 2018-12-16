

// mod ::gui::QTextObject
// package qtgui
// /usr/include/qt/QtGui/qtextobject.h
// #include <qtextobject.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 39
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

// void setFormat(const QTextFormat &)
// func (this *QTextObject) InheritSetFormat(f func(format *QTextFormat) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setFormat", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QTextObject)=16
pub struct QTextObject {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextObject_ITF interface {
//    qtcore.QObject_ITF
//    QTextObject_PTR() *QTextObject
//}
//func (ptr *QTextObject) QTextObject_PTR() *QTextObject { return ptr }

impl /*struct*/ QTextObject {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextObject {
    return QTextObject{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextObject {
//  type Target = QTextObjectBASE;
//
//  fn deref(&self) -> &QTextObjectBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextObjectBASE> for QTextObject {
//  fn as_ref(& self) -> & QTextObjectBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextobject.h:62
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTextObject {
  pub fn metaObject_0<RetType, T: QTextObject_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTextObject_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTextObject) -> RetType;
}
impl<'a> /*trait*/ QTextObject_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTextObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextObject10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:65
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void QTextObject(QTextDocument *)

/*
Creates a new QTextObject for the given document.

Warning: This function should never be called directly, but only from QTextDocument::createObject().
*/
// QTextObject(QTextDocument *) ctx.fn_proto_cpp
impl /*struct*/ QTextObject {
  pub fn QTextObject_0<T: QTextObject_QTextObject_0>(value: T) -> QTextObject {
    let rsthis = value.QTextObject_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextObject_QTextObject_0 {
  fn QTextObject_0(self) -> QTextObject;
}
// QTextObject(QTextDocument *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextObject_QTextObject_0 for (usize) {
  fn QTextObject_0(self) -> QTextObject {
    // unsafe{_ZN11QTextObjectC2EP13QTextDocument()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextObjectC2EP13QTextDocument", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextObject{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:66
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void ~QTextObject()

/*

*/
pub fn DeleteQTextObject(this :*mut QTextObject) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QTextObjectD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qtextobject.h:68
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setFormat(const QTextFormat &)

/*
Sets the text object's format.

See also format().
*/
impl /*struct*/ QTextObject {
  pub fn setFormat_0<RetType, T: QTextObject_setFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormat_0(self);
    // return 1;
  }
}
pub trait QTextObject_setFormat_0<RetType> {
  fn setFormat_0(self , rsthis: & QTextObject) -> RetType;
}
impl<'a> /*trait*/ QTextObject_setFormat_0<(/*void*/)> for (usize) {
  fn setFormat_0(self , rsthis: & QTextObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextObject9setFormatERK11QTextFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:71
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextFormat format() const

/*
Returns the text object's format.

See also setFormat() and document().
*/
impl /*struct*/ QTextObject {
  pub fn format_0<RetType, T: QTextObject_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QTextObject_format_0<RetType> {
  fn format_0(self , rsthis: & QTextObject) -> RetType;
}
impl<'a> /*trait*/ QTextObject_format_0<usize> for () {
  fn format_0(self , rsthis: & QTextObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextObject6formatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:72
// index:0
// Public Visibility=Default Availability=Available
// [4] int formatIndex() const

/*
Returns the index of the object's format in the document's internal list of formats.

See also QTextDocument::allFormats().
*/
impl /*struct*/ QTextObject {
  pub fn formatIndex_0<RetType, T: QTextObject_formatIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.formatIndex_0(self);
    // return 1;
  }
}
pub trait QTextObject_formatIndex_0<RetType> {
  fn formatIndex_0(self , rsthis: & QTextObject) -> RetType;
}
impl<'a> /*trait*/ QTextObject_formatIndex_0<i32> for () {
  fn formatIndex_0(self , rsthis: & QTextObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextObject11formatIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:74
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextDocument * document() const

/*
Returns the document this object belongs to.

See also format().
*/
impl /*struct*/ QTextObject {
  pub fn document_0<RetType, T: QTextObject_document_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.document_0(self);
    // return 1;
  }
}
pub trait QTextObject_document_0<RetType> {
  fn document_0(self , rsthis: & QTextObject) -> RetType;
}
impl<'a> /*trait*/ QTextObject_document_0<usize> for () {
  fn document_0(self , rsthis: & QTextObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextObject8documentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:76
// index:0
// Public Visibility=Default Availability=Available
// [4] int objectIndex() const

/*
Returns the object index of this object. This can be used together with QTextFormat::setObjectIndex().
*/
impl /*struct*/ QTextObject {
  pub fn objectIndex_0<RetType, T: QTextObject_objectIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.objectIndex_0(self);
    // return 1;
  }
}
pub trait QTextObject_objectIndex_0<RetType> {
  fn objectIndex_0(self , rsthis: & QTextObject) -> RetType;
}
impl<'a> /*trait*/ QTextObject_objectIndex_0<i32> for () {
  fn objectIndex_0(self , rsthis: & QTextObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextObject11objectIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
