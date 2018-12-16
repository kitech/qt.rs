

// mod ::gui::QTextDocument
// package qtgui
// /usr/include/qt/QtGui/qtextdocument.h
// #include <qtextdocument.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 3
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

// QTextObject * createObject(const QTextFormat &)
// func (this *QTextDocument) InheritCreateObject(f func(f *QTextFormat) unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "createObject", f)
// }

// QVariant loadResource(int, const QUrl &)
// func (this *QTextDocument) InheritLoadResource(f func(type_ int, name *qtcore.QUrl) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "loadResource", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QTextDocument)=16
pub struct QTextDocument {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextDocument_ITF interface {
//    qtcore.QObject_ITF
//    QTextDocument_PTR() *QTextDocument
//}
//func (ptr *QTextDocument) QTextDocument_PTR() *QTextDocument { return ptr }

impl /*struct*/ QTextDocument {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextDocument {
    return QTextDocument{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextDocument {
//  type Target = QTextDocumentBASE;
//
//  fn deref(&self) -> &QTextDocumentBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextDocumentBASE> for QTextDocument {
//  fn as_ref(& self) -> & QTextDocumentBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextdocument.h:99
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTextDocument {
  pub fn metaObject_0<RetType, T: QTextDocument_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTextDocument_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextDocument(QObject *)

/*
Constructs an empty QTextDocument with the given parent.
*/
// QTextDocument(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QTextDocument {
  pub fn QTextDocument_0<T: QTextDocument_QTextDocument_0>(value: T) -> QTextDocument {
    let rsthis = value.QTextDocument_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDocument_QTextDocument_0 {
  fn QTextDocument_0(self) -> QTextDocument;
}
// QTextDocument(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextDocument_QTextDocument_0 for (usize) {
  fn QTextDocument_0(self) -> QTextDocument {
    // unsafe{_ZN13QTextDocumentC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QTextDocumentC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextDocument{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:120
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTextDocument(const QString &, QObject *)

/*
Constructs an empty QTextDocument with the given parent.
*/
// QTextDocument(const QString &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QTextDocument {
  pub fn QTextDocument_1<T: QTextDocument_QTextDocument_1>(value: T) -> QTextDocument {
    let rsthis = value.QTextDocument_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDocument_QTextDocument_1 {
  fn QTextDocument_1(self) -> QTextDocument;
}
// QTextDocument(const QString &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextDocument_QTextDocument_1 for (usize,usize) {
  fn QTextDocument_1(self) -> QTextDocument {
    // unsafe{_ZN13QTextDocumentC2ERK7QStringP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QTextDocumentC2ERK7QStringP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextDocument{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:121
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTextDocument()

/*

*/
pub fn DeleteQTextDocument(this :*mut QTextDocument) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QTextDocumentD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qtextdocument.h:123
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextDocument * clone(QObject *) const

/*
Creates a new QTextDocument that is a copy of this text document. parent is the parent of the returned text document.
*/
impl /*struct*/ QTextDocument {
  pub fn clone_0<RetType, T: QTextDocument_clone_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clone_0(self);
    // return 1;
  }
}
pub trait QTextDocument_clone_0<RetType> {
  fn clone_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_clone_0<usize> for (usize) {
  fn clone_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument5cloneEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:125
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if the document is empty; otherwise returns false.
*/
impl /*struct*/ QTextDocument {
  pub fn isEmpty_0<RetType, T: QTextDocument_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QTextDocument_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QTextDocument) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:126
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears the document.
*/
impl /*struct*/ QTextDocument {
  pub fn clear_0<RetType, T: QTextDocument_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QTextDocument_clear_0<RetType> {
  fn clear_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QTextDocument5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:128
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUndoRedoEnabled(bool)

/*

*/
impl /*struct*/ QTextDocument {
  pub fn setUndoRedoEnabled_0<RetType, T: QTextDocument_setUndoRedoEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUndoRedoEnabled_0(self);
    // return 1;
  }
}
pub trait QTextDocument_setUndoRedoEnabled_0<RetType> {
  fn setUndoRedoEnabled_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_setUndoRedoEnabled_0<(/*void*/)> for (bool) {
  fn setUndoRedoEnabled_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument18setUndoRedoEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:129
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isUndoRedoEnabled() const

/*

*/
impl /*struct*/ QTextDocument {
  pub fn isUndoRedoEnabled_0<RetType, T: QTextDocument_isUndoRedoEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isUndoRedoEnabled_0(self);
    // return 1;
  }
}
pub trait QTextDocument_isUndoRedoEnabled_0<RetType> {
  fn isUndoRedoEnabled_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_isUndoRedoEnabled_0<bool> for () {
  fn isUndoRedoEnabled_0(self , rsthis: & QTextDocument) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument17isUndoRedoEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:131
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isUndoAvailable() const

/*
Returns true if undo is available; otherwise returns false.

See also isRedoAvailable() and availableUndoSteps().
*/
impl /*struct*/ QTextDocument {
  pub fn isUndoAvailable_0<RetType, T: QTextDocument_isUndoAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isUndoAvailable_0(self);
    // return 1;
  }
}
pub trait QTextDocument_isUndoAvailable_0<RetType> {
  fn isUndoAvailable_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_isUndoAvailable_0<bool> for () {
  fn isUndoAvailable_0(self , rsthis: & QTextDocument) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument15isUndoAvailableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:132
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRedoAvailable() const

/*
Returns true if redo is available; otherwise returns false.

See also isUndoAvailable() and availableRedoSteps().
*/
impl /*struct*/ QTextDocument {
  pub fn isRedoAvailable_0<RetType, T: QTextDocument_isRedoAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRedoAvailable_0(self);
    // return 1;
  }
}
pub trait QTextDocument_isRedoAvailable_0<RetType> {
  fn isRedoAvailable_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_isRedoAvailable_0<bool> for () {
  fn isRedoAvailable_0(self , rsthis: & QTextDocument) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument15isRedoAvailableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:134
// index:0
// Public Visibility=Default Availability=Available
// [4] int availableUndoSteps() const

/*
Returns the number of available undo steps.

This function was introduced in  Qt 4.6.

See also isUndoAvailable().
*/
impl /*struct*/ QTextDocument {
  pub fn availableUndoSteps_0<RetType, T: QTextDocument_availableUndoSteps_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.availableUndoSteps_0(self);
    // return 1;
  }
}
pub trait QTextDocument_availableUndoSteps_0<RetType> {
  fn availableUndoSteps_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_availableUndoSteps_0<i32> for () {
  fn availableUndoSteps_0(self , rsthis: & QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument18availableUndoStepsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:135
// index:0
// Public Visibility=Default Availability=Available
// [4] int availableRedoSteps() const

/*
Returns the number of available redo steps.

This function was introduced in  Qt 4.6.

See also isRedoAvailable().
*/
impl /*struct*/ QTextDocument {
  pub fn availableRedoSteps_0<RetType, T: QTextDocument_availableRedoSteps_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.availableRedoSteps_0(self);
    // return 1;
  }
}
pub trait QTextDocument_availableRedoSteps_0<RetType> {
  fn availableRedoSteps_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_availableRedoSteps_0<i32> for () {
  fn availableRedoSteps_0(self , rsthis: & QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument18availableRedoStepsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:137
// index:0
// Public Visibility=Default Availability=Available
// [4] int revision() const

/*
Returns the document's revision (if undo is enabled).

The revision is guaranteed to increase when a document that is not modified is edited.

This function was introduced in  Qt 4.4.

See also QTextBlock::revision() and isModified().
*/
impl /*struct*/ QTextDocument {
  pub fn revision_0<RetType, T: QTextDocument_revision_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.revision_0(self);
    // return 1;
  }
}
pub trait QTextDocument_revision_0<RetType> {
  fn revision_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_revision_0<i32> for () {
  fn revision_0(self , rsthis: & QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument8revisionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:139
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDocumentLayout(QAbstractTextDocumentLayout *)

/*
Sets the document to use the given layout. The previous layout is deleted.

See also documentLayoutChanged().
*/
impl /*struct*/ QTextDocument {
  pub fn setDocumentLayout_0<RetType, T: QTextDocument_setDocumentLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDocumentLayout_0(self);
    // return 1;
  }
}
pub trait QTextDocument_setDocumentLayout_0<RetType> {
  fn setDocumentLayout_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_setDocumentLayout_0<(/*void*/)> for (usize) {
  fn setDocumentLayout_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument17setDocumentLayoutEP27QAbstractTextDocumentLayout", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:140
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractTextDocumentLayout * documentLayout() const

/*
Returns the document layout for this document.

See also setDocumentLayout().
*/
impl /*struct*/ QTextDocument {
  pub fn documentLayout_0<RetType, T: QTextDocument_documentLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.documentLayout_0(self);
    // return 1;
  }
}
pub trait QTextDocument_documentLayout_0<RetType> {
  fn documentLayout_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_documentLayout_0<usize> for () {
  fn documentLayout_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument14documentLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:146
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMetaInformation(QTextDocument::MetaInformation, const QString &)

/*
Sets the document's meta information of the type specified by info to the given string.

See also metaInformation().
*/
impl /*struct*/ QTextDocument {
  pub fn setMetaInformation_0<RetType, T: QTextDocument_setMetaInformation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMetaInformation_0(self);
    // return 1;
  }
}
pub trait QTextDocument_setMetaInformation_0<RetType> {
  fn setMetaInformation_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_setMetaInformation_0<(/*void*/)> for (i32,usize) {
  fn setMetaInformation_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument18setMetaInformationENS_15MetaInformationERK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:147
// index:0
// Public Visibility=Default Availability=Available
// [8] QString metaInformation(QTextDocument::MetaInformation) const

/*
Returns meta information about the document of the type specified by info.

See also setMetaInformation().
*/
impl /*struct*/ QTextDocument {
  pub fn metaInformation_0<RetType, T: QTextDocument_metaInformation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaInformation_0(self);
    // return 1;
  }
}
pub trait QTextDocument_metaInformation_0<RetType> {
  fn metaInformation_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_metaInformation_0<usize> for (i32) {
  fn metaInformation_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument15metaInformationENS_15MetaInformationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:150
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toHtml(const QByteArray &) const

/*
Returns a string containing an HTML representation of the document.

The encoding parameter specifies the value for the charset attribute in the html header. For example if 'utf-8' is specified then the beginning of the generated html will look like this:


  <html><head><meta http-equiv="Content-Type" content="text/html; charset=utf-8"></head><body>...



If no encoding is specified then no such meta information is generated.

If you later on convert the returned html string into a byte array for transmission over a network or when saving to disk you should specify the encoding you're going to use for the conversion to a byte array here.

See also Supported HTML Subset.
*/
impl /*struct*/ QTextDocument {
  pub fn toHtml_0<RetType, T: QTextDocument_toHtml_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toHtml_0(self);
    // return 1;
  }
}
pub trait QTextDocument_toHtml_0<RetType> {
  fn toHtml_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_toHtml_0<usize> for (usize) {
  fn toHtml_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument6toHtmlERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:151
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHtml(const QString &)

/*
Replaces the entire contents of the document with the given HTML-formatted text in the html string. The undo/redo history is reset when this function is called.

The HTML formatting is respected as much as possible; for example, "<b>bold</b> text" will produce text where the first word has a font weight that gives it a bold appearance: "bold text".

Note: It is the responsibility of the caller to make sure that the text is correctly decoded when a QString containing HTML is created and passed to setHtml().

See also setPlainText() and Supported HTML Subset.
*/
impl /*struct*/ QTextDocument {
  pub fn setHtml_0<RetType, T: QTextDocument_setHtml_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHtml_0(self);
    // return 1;
  }
}
pub trait QTextDocument_setHtml_0<RetType> {
  fn setHtml_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_setHtml_0<(/*void*/)> for (usize) {
  fn setHtml_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument7setHtmlERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:154
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toRawText() const

/*
Returns the raw text contained in the document without any formatting information. If you want formatting information use a QTextCursor instead.

This function was introduced in  Qt 5.9.

See also toPlainText().
*/
impl /*struct*/ QTextDocument {
  pub fn toRawText_0<RetType, T: QTextDocument_toRawText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toRawText_0(self);
    // return 1;
  }
}
pub trait QTextDocument_toRawText_0<RetType> {
  fn toRawText_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_toRawText_0<usize> for () {
  fn toRawText_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument9toRawTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:155
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toPlainText() const

/*
Returns the plain text contained in the document. If you want formatting information use a QTextCursor instead.

This function returns the same as toRawText(), but will replace some unicode characters with ASCII alternatives. In particular, no-break space (U+00A0) is replaced by a regular space (U+0020), and both paragraph (U+2029) and line (U+2028) separators are replaced by line feed (U+000A). If you need the precise contents of the document, use toRawText() instead.

Note: Embedded objects, such as images, are represented by a Unicode value U+FFFC (OBJECT REPLACEMENT CHARACTER).

See also toHtml().
*/
impl /*struct*/ QTextDocument {
  pub fn toPlainText_0<RetType, T: QTextDocument_toPlainText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPlainText_0(self);
    // return 1;
  }
}
pub trait QTextDocument_toPlainText_0<RetType> {
  fn toPlainText_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_toPlainText_0<usize> for () {
  fn toPlainText_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument11toPlainTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPlainText(const QString &)

/*
Replaces the entire contents of the document with the given plain text. The undo/redo history is reset when this function is called.

See also setHtml().
*/
impl /*struct*/ QTextDocument {
  pub fn setPlainText_0<RetType, T: QTextDocument_setPlainText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPlainText_0(self);
    // return 1;
  }
}
pub trait QTextDocument_setPlainText_0<RetType> {
  fn setPlainText_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_setPlainText_0<(/*void*/)> for (usize) {
  fn setPlainText_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument12setPlainTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:158
// index:0
// Public Visibility=Default Availability=Available
// [2] QChar characterAt(int) const

/*
Returns the character at position pos, or a null character if the position is out of range.

This function was introduced in  Qt 4.5.

See also characterCount().
*/
impl /*struct*/ QTextDocument {
  pub fn characterAt_0<RetType, T: QTextDocument_characterAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.characterAt_0(self);
    // return 1;
  }
}
pub trait QTextDocument_characterAt_0<RetType> {
  fn characterAt_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_characterAt_0<usize> for (i32) {
  fn characterAt_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument11characterAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:168
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextCursor find(const QString &, int, QTextDocument::FindFlags) const

/*
Finds the next occurrence of the string, subString, in the document. The search starts at the position of the given cursor, and proceeds forwards through the document unless specified otherwise in the search options. The options control the type of search performed.

Returns a cursor with the match selected if subString was found; otherwise returns a null cursor.

If the given cursor has a selection, the search begins after the selection; otherwise it begins at the cursor's position.

By default the search is case-sensitive, and can match text anywhere in the document.
*/
impl /*struct*/ QTextDocument {
  pub fn find_0<RetType, T: QTextDocument_find_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.find_0(self);
    // return 1;
  }
}
pub trait QTextDocument_find_0<RetType> {
  fn find_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_find_0<usize> for (usize,i32,i32) {
  fn find_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument4findERK7QStringi6QFlagsINS_8FindFlagEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:169
// index:1
// Public Visibility=Default Availability=Available
// [8] QTextCursor find(const QString &, const QTextCursor &, QTextDocument::FindFlags) const

/*
Finds the next occurrence of the string, subString, in the document. The search starts at the position of the given cursor, and proceeds forwards through the document unless specified otherwise in the search options. The options control the type of search performed.

Returns a cursor with the match selected if subString was found; otherwise returns a null cursor.

If the given cursor has a selection, the search begins after the selection; otherwise it begins at the cursor's position.

By default the search is case-sensitive, and can match text anywhere in the document.
*/
impl /*struct*/ QTextDocument {
  pub fn find_1<RetType, T: QTextDocument_find_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.find_1(self);
    // return 1;
  }
}
pub trait QTextDocument_find_1<RetType> {
  fn find_1(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_find_1<usize> for (usize,usize,i32) {
  fn find_1(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument4findERK7QStringRK11QTextCursor6QFlagsINS_8FindFlagEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:172
// index:2
// Public Visibility=Default Availability=Available
// [8] QTextCursor find(const QRegExp &, int, QTextDocument::FindFlags) const

/*
Finds the next occurrence of the string, subString, in the document. The search starts at the position of the given cursor, and proceeds forwards through the document unless specified otherwise in the search options. The options control the type of search performed.

Returns a cursor with the match selected if subString was found; otherwise returns a null cursor.

If the given cursor has a selection, the search begins after the selection; otherwise it begins at the cursor's position.

By default the search is case-sensitive, and can match text anywhere in the document.
*/
impl /*struct*/ QTextDocument {
  pub fn find_2<RetType, T: QTextDocument_find_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.find_2(self);
    // return 1;
  }
}
pub trait QTextDocument_find_2<RetType> {
  fn find_2(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_find_2<usize> for (usize,i32,i32) {
  fn find_2(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument4findERK7QRegExpi6QFlagsINS_8FindFlagEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:173
// index:3
// Public Visibility=Default Availability=Available
// [8] QTextCursor find(const QRegExp &, const QTextCursor &, QTextDocument::FindFlags) const

/*
Finds the next occurrence of the string, subString, in the document. The search starts at the position of the given cursor, and proceeds forwards through the document unless specified otherwise in the search options. The options control the type of search performed.

Returns a cursor with the match selected if subString was found; otherwise returns a null cursor.

If the given cursor has a selection, the search begins after the selection; otherwise it begins at the cursor's position.

By default the search is case-sensitive, and can match text anywhere in the document.
*/
impl /*struct*/ QTextDocument {
  pub fn find_3<RetType, T: QTextDocument_find_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.find_3(self);
    // return 1;
  }
}
pub trait QTextDocument_find_3<RetType> {
  fn find_3(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_find_3<usize> for (usize,usize,i32) {
  fn find_3(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument4findERK7QRegExpRK11QTextCursor6QFlagsINS_8FindFlagEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:177
// index:4
// Public Visibility=Default Availability=Available
// [8] QTextCursor find(const QRegularExpression &, int, QTextDocument::FindFlags) const

/*
Finds the next occurrence of the string, subString, in the document. The search starts at the position of the given cursor, and proceeds forwards through the document unless specified otherwise in the search options. The options control the type of search performed.

Returns a cursor with the match selected if subString was found; otherwise returns a null cursor.

If the given cursor has a selection, the search begins after the selection; otherwise it begins at the cursor's position.

By default the search is case-sensitive, and can match text anywhere in the document.
*/
impl /*struct*/ QTextDocument {
  pub fn find_4<RetType, T: QTextDocument_find_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.find_4(self);
    // return 1;
  }
}
pub trait QTextDocument_find_4<RetType> {
  fn find_4(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_find_4<usize> for (usize,i32,i32) {
  fn find_4(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument4findERK18QRegularExpressioni6QFlagsINS_8FindFlagEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:178
// index:5
// Public Visibility=Default Availability=Available
// [8] QTextCursor find(const QRegularExpression &, const QTextCursor &, QTextDocument::FindFlags) const

/*
Finds the next occurrence of the string, subString, in the document. The search starts at the position of the given cursor, and proceeds forwards through the document unless specified otherwise in the search options. The options control the type of search performed.

Returns a cursor with the match selected if subString was found; otherwise returns a null cursor.

If the given cursor has a selection, the search begins after the selection; otherwise it begins at the cursor's position.

By default the search is case-sensitive, and can match text anywhere in the document.
*/
impl /*struct*/ QTextDocument {
  pub fn find_5<RetType, T: QTextDocument_find_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.find_5(self);
    // return 1;
  }
}
pub trait QTextDocument_find_5<RetType> {
  fn find_5(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_find_5<usize> for (usize,usize,i32) {
  fn find_5(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument4findERK18QRegularExpressionRK11QTextCursor6QFlagsINS_8FindFlagEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:181
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextFrame * frameAt(int) const

/*

*/
impl /*struct*/ QTextDocument {
  pub fn frameAt_0<RetType, T: QTextDocument_frameAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frameAt_0(self);
    // return 1;
  }
}
pub trait QTextDocument_frameAt_0<RetType> {
  fn frameAt_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_frameAt_0<usize> for (i32) {
  fn frameAt_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument7frameAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:182
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextFrame * rootFrame() const

/*
Returns the document's root frame.
*/
impl /*struct*/ QTextDocument {
  pub fn rootFrame_0<RetType, T: QTextDocument_rootFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rootFrame_0(self);
    // return 1;
  }
}
pub trait QTextDocument_rootFrame_0<RetType> {
  fn rootFrame_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_rootFrame_0<usize> for () {
  fn rootFrame_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument9rootFrameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:184
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextObject * object(int) const

/*
Returns the text object associated with the given objectIndex.
*/
impl /*struct*/ QTextDocument {
  pub fn object_0<RetType, T: QTextDocument_object_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.object_0(self);
    // return 1;
  }
}
pub trait QTextDocument_object_0<RetType> {
  fn object_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_object_0<usize> for (i32) {
  fn object_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument6objectEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:185
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextObject * objectForFormat(const QTextFormat &) const

/*
Returns the text object associated with the format f.
*/
impl /*struct*/ QTextDocument {
  pub fn objectForFormat_0<RetType, T: QTextDocument_objectForFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.objectForFormat_0(self);
    // return 1;
  }
}
pub trait QTextDocument_objectForFormat_0<RetType> {
  fn objectForFormat_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_objectForFormat_0<usize> for (usize) {
  fn objectForFormat_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument15objectForFormatERK11QTextFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:187
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextBlock findBlock(int) const

/*
Returns the text block that contains the pos-th character.
*/
impl /*struct*/ QTextDocument {
  pub fn findBlock_0<RetType, T: QTextDocument_findBlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.findBlock_0(self);
    // return 1;
  }
}
pub trait QTextDocument_findBlock_0<RetType> {
  fn findBlock_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_findBlock_0<usize> for (i32) {
  fn findBlock_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument9findBlockEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:188
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextBlock findBlockByNumber(int) const

/*
Returns the text block with the specified blockNumber.

This function was introduced in  Qt 4.4.

See also QTextBlock::blockNumber().
*/
impl /*struct*/ QTextDocument {
  pub fn findBlockByNumber_0<RetType, T: QTextDocument_findBlockByNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.findBlockByNumber_0(self);
    // return 1;
  }
}
pub trait QTextDocument_findBlockByNumber_0<RetType> {
  fn findBlockByNumber_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_findBlockByNumber_0<usize> for (i32) {
  fn findBlockByNumber_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument17findBlockByNumberEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:189
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextBlock findBlockByLineNumber(int) const

/*
Returns the text block that contains the specified lineNumber.

This function was introduced in  Qt 4.5.

See also QTextBlock::firstLineNumber().
*/
impl /*struct*/ QTextDocument {
  pub fn findBlockByLineNumber_0<RetType, T: QTextDocument_findBlockByLineNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.findBlockByLineNumber_0(self);
    // return 1;
  }
}
pub trait QTextDocument_findBlockByLineNumber_0<RetType> {
  fn findBlockByLineNumber_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_findBlockByLineNumber_0<usize> for (i32) {
  fn findBlockByLineNumber_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument21findBlockByLineNumberEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:190
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextBlock begin() const

/*
Returns the document's first text block.

See also firstBlock().
*/
impl /*struct*/ QTextDocument {
  pub fn begin_0<RetType, T: QTextDocument_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QTextDocument_begin_0<RetType> {
  fn begin_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_begin_0<usize> for () {
  fn begin_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:191
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextBlock end() const

/*
This function returns a block to test for the end of the document while iterating over it.


      for (QTextBlock it = doc->begin(); it != doc->end(); it = it.next())
          cout << it.text().toStdString() << endl;



The block returned is invalid and represents the block after the last block in the document. You can use lastBlock() to retrieve the last valid block of the document.

See also lastBlock().
*/
impl /*struct*/ QTextDocument {
  pub fn end_0<RetType, T: QTextDocument_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QTextDocument_end_0<RetType> {
  fn end_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_end_0<usize> for () {
  fn end_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:193
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextBlock firstBlock() const

/*
Returns the document's first text block.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QTextDocument {
  pub fn firstBlock_0<RetType, T: QTextDocument_firstBlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.firstBlock_0(self);
    // return 1;
  }
}
pub trait QTextDocument_firstBlock_0<RetType> {
  fn firstBlock_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_firstBlock_0<usize> for () {
  fn firstBlock_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument10firstBlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:194
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextBlock lastBlock() const

/*
Returns the document's last (valid) text block.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QTextDocument {
  pub fn lastBlock_0<RetType, T: QTextDocument_lastBlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastBlock_0(self);
    // return 1;
  }
}
pub trait QTextDocument_lastBlock_0<RetType> {
  fn lastBlock_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_lastBlock_0<usize> for () {
  fn lastBlock_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument9lastBlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:196
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPageSize(const QSizeF &)

/*

*/
impl /*struct*/ QTextDocument {
  pub fn setPageSize_0<RetType, T: QTextDocument_setPageSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPageSize_0(self);
    // return 1;
  }
}
pub trait QTextDocument_setPageSize_0<RetType> {
  fn setPageSize_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_setPageSize_0<(/*void*/)> for (usize) {
  fn setPageSize_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument11setPageSizeERK6QSizeF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:197
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF pageSize() const

/*

*/
impl /*struct*/ QTextDocument {
  pub fn pageSize_0<RetType, T: QTextDocument_pageSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pageSize_0(self);
    // return 1;
  }
}
pub trait QTextDocument_pageSize_0<RetType> {
  fn pageSize_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_pageSize_0<usize> for () {
  fn pageSize_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument8pageSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:199
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultFont(const QFont &)

/*
Sets the default font to use in the document layout.

Note: Setter function for property defaultFont. 

See also defaultFont().
*/
impl /*struct*/ QTextDocument {
  pub fn setDefaultFont_0<RetType, T: QTextDocument_setDefaultFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultFont_0(self);
    // return 1;
  }
}
pub trait QTextDocument_setDefaultFont_0<RetType> {
  fn setDefaultFont_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_setDefaultFont_0<(/*void*/)> for (usize) {
  fn setDefaultFont_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument14setDefaultFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:200
// index:0
// Public Visibility=Default Availability=Available
// [16] QFont defaultFont() const

/*
Returns the default font to be used in the document layout.

Note: Getter function for property defaultFont. 

See also setDefaultFont().
*/
impl /*struct*/ QTextDocument {
  pub fn defaultFont_0<RetType, T: QTextDocument_defaultFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultFont_0(self);
    // return 1;
  }
}
pub trait QTextDocument_defaultFont_0<RetType> {
  fn defaultFont_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_defaultFont_0<usize> for () {
  fn defaultFont_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument11defaultFontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:202
// index:0
// Public Visibility=Default Availability=Available
// [4] int pageCount() const

/*
returns the number of pages in this document.
*/
impl /*struct*/ QTextDocument {
  pub fn pageCount_0<RetType, T: QTextDocument_pageCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pageCount_0(self);
    // return 1;
  }
}
pub trait QTextDocument_pageCount_0<RetType> {
  fn pageCount_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_pageCount_0<i32> for () {
  fn pageCount_0(self , rsthis: & QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument9pageCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:204
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isModified() const

/*

*/
impl /*struct*/ QTextDocument {
  pub fn isModified_0<RetType, T: QTextDocument_isModified_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isModified_0(self);
    // return 1;
  }
}
pub trait QTextDocument_isModified_0<RetType> {
  fn isModified_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_isModified_0<bool> for () {
  fn isModified_0(self , rsthis: & QTextDocument) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument10isModifiedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:206
// index:0
// Public Visibility=Default Availability=Available
// [-2] void print(QPagedPaintDevice *) const

/*
Prints the document to the given printer. The QPageablePaintDevice must be set up before being used with this function.

This is only a convenience method to print the whole document to the printer.

If the document is already paginated through a specified height in the pageSize() property it is printed as-is.

If the document is not paginated, like for example a document used in a QTextEdit, then a temporary copy of the document is created and the copy is broken into multiple pages according to the size of the paint device's paperRect(). By default a 2 cm margin is set around the document contents. In addition the current page number is printed at the bottom of each page.

See also QTextEdit::print().
*/
impl /*struct*/ QTextDocument {
  pub fn print_0<RetType, T: QTextDocument_print_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.print_0(self);
    // return 1;
  }
}
pub trait QTextDocument_print_0<RetType> {
  fn print_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_print_0<(/*void*/)> for (usize) {
  fn print_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK13QTextDocument5printEP17QPagedPaintDevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:216
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant resource(int, const QUrl &) const

/*
Returns data of the specified type from the resource with the given name.

This function is called by the rich text engine to request data that isn't directly stored by QTextDocument, but still associated with it. For example, images are referenced indirectly by the name attribute of a QTextImageFormat object.

Resources are cached internally in the document. If a resource can not be found in the cache, loadResource is called to try to load the resource. loadResource should then use addResource to add the resource to the cache.

See also QTextDocument::ResourceType.
*/
impl /*struct*/ QTextDocument {
  pub fn resource_0<RetType, T: QTextDocument_resource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resource_0(self);
    // return 1;
  }
}
pub trait QTextDocument_resource_0<RetType> {
  fn resource_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_resource_0<usize> for (i32,usize) {
  fn resource_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument8resourceEiRK4QUrl", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:217
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addResource(int, const QUrl &, const QVariant &)

/*
Adds the resource resource to the resource cache, using type and name as identifiers. type should be a value from QTextDocument::ResourceType.

For example, you can add an image as a resource in order to reference it from within the document:


      document->addResource(QTextDocument::ImageResource,
          QUrl("mydata://image.png"), QVariant(image));



The image can be inserted into the document using the QTextCursor API:


      QTextImageFormat imageFormat;
      imageFormat.setName("mydata://image.png");
      cursor.insertImage(imageFormat);



Alternatively, you can insert images using the HTML img tag:


      editor->append("<img src=\"mydata://image.png\" />");
*/
impl /*struct*/ QTextDocument {
  pub fn addResource_0<RetType, T: QTextDocument_addResource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addResource_0(self);
    // return 1;
  }
}
pub trait QTextDocument_addResource_0<RetType> {
  fn addResource_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_addResource_0<(/*void*/)> for (i32,usize,usize) {
  fn addResource_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument11addResourceEiRK4QUrlRK8QVariant", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:221
// index:0
// Public Visibility=Default Availability=Available
// [-2] void markContentsDirty(int, int)

/*
Marks the contents specified by the given position and length as "dirty", informing the document that it needs to be laid out again.
*/
impl /*struct*/ QTextDocument {
  pub fn markContentsDirty_0<RetType, T: QTextDocument_markContentsDirty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.markContentsDirty_0(self);
    // return 1;
  }
}
pub trait QTextDocument_markContentsDirty_0<RetType> {
  fn markContentsDirty_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_markContentsDirty_0<(/*void*/)> for (i32,i32) {
  fn markContentsDirty_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument17markContentsDirtyEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:223
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUseDesignMetrics(bool)

/*

*/
impl /*struct*/ QTextDocument {
  pub fn setUseDesignMetrics_0<RetType, T: QTextDocument_setUseDesignMetrics_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUseDesignMetrics_0(self);
    // return 1;
  }
}
pub trait QTextDocument_setUseDesignMetrics_0<RetType> {
  fn setUseDesignMetrics_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_setUseDesignMetrics_0<(/*void*/)> for (bool) {
  fn setUseDesignMetrics_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument19setUseDesignMetricsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:224
// index:0
// Public Visibility=Default Availability=Available
// [1] bool useDesignMetrics() const

/*

*/
impl /*struct*/ QTextDocument {
  pub fn useDesignMetrics_0<RetType, T: QTextDocument_useDesignMetrics_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.useDesignMetrics_0(self);
    // return 1;
  }
}
pub trait QTextDocument_useDesignMetrics_0<RetType> {
  fn useDesignMetrics_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_useDesignMetrics_0<bool> for () {
  fn useDesignMetrics_0(self , rsthis: & QTextDocument) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument16useDesignMetricsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:226
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawContents(QPainter *, const QRectF &)

/*
Draws the content of the document with painter p, clipped to rect. If rect is a null rectangle (default) then the document is painted unclipped.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QTextDocument {
  pub fn drawContents_0<RetType, T: QTextDocument_drawContents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawContents_0(self);
    // return 1;
  }
}
pub trait QTextDocument_drawContents_0<RetType> {
  fn drawContents_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_drawContents_0<(/*void*/)> for (usize,usize) {
  fn drawContents_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument12drawContentsEP8QPainterRK6QRectF", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:228
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextWidth(qreal)

/*

*/
impl /*struct*/ QTextDocument {
  pub fn setTextWidth_0<RetType, T: QTextDocument_setTextWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextWidth_0(self);
    // return 1;
  }
}
pub trait QTextDocument_setTextWidth_0<RetType> {
  fn setTextWidth_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_setTextWidth_0<(/*void*/)> for (f64) {
  fn setTextWidth_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument12setTextWidthEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:229
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal textWidth() const

/*

*/
impl /*struct*/ QTextDocument {
  pub fn textWidth_0<RetType, T: QTextDocument_textWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textWidth_0(self);
    // return 1;
  }
}
pub trait QTextDocument_textWidth_0<RetType> {
  fn textWidth_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_textWidth_0<f64> for () {
  fn textWidth_0(self , rsthis: & QTextDocument) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument9textWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:231
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal idealWidth() const

/*
Returns the ideal width of the text document. The ideal width is the actually used width of the document without optional alignments taken into account. It is always <= size().width().

This function was introduced in  Qt 4.2.

See also adjustSize() and textWidth.
*/
impl /*struct*/ QTextDocument {
  pub fn idealWidth_0<RetType, T: QTextDocument_idealWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.idealWidth_0(self);
    // return 1;
  }
}
pub trait QTextDocument_idealWidth_0<RetType> {
  fn idealWidth_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_idealWidth_0<f64> for () {
  fn idealWidth_0(self , rsthis: & QTextDocument) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument10idealWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:233
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal indentWidth() const

/*

*/
impl /*struct*/ QTextDocument {
  pub fn indentWidth_0<RetType, T: QTextDocument_indentWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indentWidth_0(self);
    // return 1;
  }
}
pub trait QTextDocument_indentWidth_0<RetType> {
  fn indentWidth_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_indentWidth_0<f64> for () {
  fn indentWidth_0(self , rsthis: & QTextDocument) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument11indentWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:234
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIndentWidth(qreal)

/*
Sets the width used for text list and text block indenting.

The indent properties of QTextListFormat and QTextBlockFormat specify multiples of this value. The default indent width is 40 .

This function was introduced in  Qt 4.4.

Note: Setter function for property indentWidth. 

See also indentWidth().
*/
impl /*struct*/ QTextDocument {
  pub fn setIndentWidth_0<RetType, T: QTextDocument_setIndentWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIndentWidth_0(self);
    // return 1;
  }
}
pub trait QTextDocument_setIndentWidth_0<RetType> {
  fn setIndentWidth_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_setIndentWidth_0<(/*void*/)> for (f64) {
  fn setIndentWidth_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument14setIndentWidthEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:236
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal documentMargin() const

/*

*/
impl /*struct*/ QTextDocument {
  pub fn documentMargin_0<RetType, T: QTextDocument_documentMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.documentMargin_0(self);
    // return 1;
  }
}
pub trait QTextDocument_documentMargin_0<RetType> {
  fn documentMargin_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_documentMargin_0<f64> for () {
  fn documentMargin_0(self , rsthis: & QTextDocument) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument14documentMarginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:237
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDocumentMargin(qreal)

/*

*/
impl /*struct*/ QTextDocument {
  pub fn setDocumentMargin_0<RetType, T: QTextDocument_setDocumentMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDocumentMargin_0(self);
    // return 1;
  }
}
pub trait QTextDocument_setDocumentMargin_0<RetType> {
  fn setDocumentMargin_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_setDocumentMargin_0<(/*void*/)> for (f64) {
  fn setDocumentMargin_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument17setDocumentMarginEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:239
// index:0
// Public Visibility=Default Availability=Available
// [-2] void adjustSize()

/*
Adjusts the document to a reasonable size.

This function was introduced in  Qt 4.2.

See also idealWidth(), textWidth, and size.
*/
impl /*struct*/ QTextDocument {
  pub fn adjustSize_0<RetType, T: QTextDocument_adjustSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.adjustSize_0(self);
    // return 1;
  }
}
pub trait QTextDocument_adjustSize_0<RetType> {
  fn adjustSize_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_adjustSize_0<(/*void*/)> for () {
  fn adjustSize_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QTextDocument10adjustSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:240
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF size() const

/*

*/
impl /*struct*/ QTextDocument {
  pub fn size_0<RetType, T: QTextDocument_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QTextDocument_size_0<RetType> {
  fn size_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_size_0<usize> for () {
  fn size_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:242
// index:0
// Public Visibility=Default Availability=Available
// [4] int blockCount() const

/*

*/
impl /*struct*/ QTextDocument {
  pub fn blockCount_0<RetType, T: QTextDocument_blockCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockCount_0(self);
    // return 1;
  }
}
pub trait QTextDocument_blockCount_0<RetType> {
  fn blockCount_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_blockCount_0<i32> for () {
  fn blockCount_0(self , rsthis: & QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument10blockCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:243
// index:0
// Public Visibility=Default Availability=Available
// [4] int lineCount() const

/*
Returns the number of lines of this document (if the layout supports this). Otherwise, this is identical to the number of blocks.

This function was introduced in  Qt 4.5.

See also blockCount() and characterCount().
*/
impl /*struct*/ QTextDocument {
  pub fn lineCount_0<RetType, T: QTextDocument_lineCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineCount_0(self);
    // return 1;
  }
}
pub trait QTextDocument_lineCount_0<RetType> {
  fn lineCount_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_lineCount_0<i32> for () {
  fn lineCount_0(self , rsthis: & QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument9lineCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:244
// index:0
// Public Visibility=Default Availability=Available
// [4] int characterCount() const

/*
Returns the number of characters of this document.

This function was introduced in  Qt 4.5.

See also blockCount() and characterAt().
*/
impl /*struct*/ QTextDocument {
  pub fn characterCount_0<RetType, T: QTextDocument_characterCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.characterCount_0(self);
    // return 1;
  }
}
pub trait QTextDocument_characterCount_0<RetType> {
  fn characterCount_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_characterCount_0<i32> for () {
  fn characterCount_0(self , rsthis: & QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument14characterCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:247
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultStyleSheet(const QString &)

/*

*/
impl /*struct*/ QTextDocument {
  pub fn setDefaultStyleSheet_0<RetType, T: QTextDocument_setDefaultStyleSheet_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultStyleSheet_0(self);
    // return 1;
  }
}
pub trait QTextDocument_setDefaultStyleSheet_0<RetType> {
  fn setDefaultStyleSheet_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_setDefaultStyleSheet_0<(/*void*/)> for (usize) {
  fn setDefaultStyleSheet_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument20setDefaultStyleSheetERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:248
// index:0
// Public Visibility=Default Availability=Available
// [8] QString defaultStyleSheet() const

/*

*/
impl /*struct*/ QTextDocument {
  pub fn defaultStyleSheet_0<RetType, T: QTextDocument_defaultStyleSheet_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultStyleSheet_0(self);
    // return 1;
  }
}
pub trait QTextDocument_defaultStyleSheet_0<RetType> {
  fn defaultStyleSheet_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_defaultStyleSheet_0<usize> for () {
  fn defaultStyleSheet_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument17defaultStyleSheetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:251
// index:0
// Public Visibility=Default Availability=Available
// [-2] void undo(QTextCursor *)

/*
Undoes the last editing operation on the document if undo is available. The provided cursor is positioned at the end of the location where the edition operation was undone.

See the Qt Undo Framework documentation for details.

This function was introduced in  Qt 4.2.

See also undoAvailable() and isUndoRedoEnabled().
*/
impl /*struct*/ QTextDocument {
  pub fn undo_0<RetType, T: QTextDocument_undo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undo_0(self);
    // return 1;
  }
}
pub trait QTextDocument_undo_0<RetType> {
  fn undo_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_undo_0<(/*void*/)> for (usize) {
  fn undo_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument4undoEP11QTextCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:286
// index:1
// Public Visibility=Default Availability=Available
// [-2] void undo()

/*
Undoes the last editing operation on the document if undo is available. The provided cursor is positioned at the end of the location where the edition operation was undone.

See the Qt Undo Framework documentation for details.

This function was introduced in  Qt 4.2.

See also undoAvailable() and isUndoRedoEnabled().
*/
impl /*struct*/ QTextDocument {
  pub fn undo_1<RetType, T: QTextDocument_undo_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undo_1(self);
    // return 1;
  }
}
pub trait QTextDocument_undo_1<RetType> {
  fn undo_1(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_undo_1<(/*void*/)> for () {
  fn undo_1(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QTextDocument4undoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:252
// index:0
// Public Visibility=Default Availability=Available
// [-2] void redo(QTextCursor *)

/*
Redoes the last editing operation on the document if redo is available.

The provided cursor is positioned at the end of the location where the edition operation was redone.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QTextDocument {
  pub fn redo_0<RetType, T: QTextDocument_redo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redo_0(self);
    // return 1;
  }
}
pub trait QTextDocument_redo_0<RetType> {
  fn redo_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_redo_0<(/*void*/)> for (usize) {
  fn redo_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument4redoEP11QTextCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:287
// index:1
// Public Visibility=Default Availability=Available
// [-2] void redo()

/*
Redoes the last editing operation on the document if redo is available.

The provided cursor is positioned at the end of the location where the edition operation was redone.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QTextDocument {
  pub fn redo_1<RetType, T: QTextDocument_redo_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redo_1(self);
    // return 1;
  }
}
pub trait QTextDocument_redo_1<RetType> {
  fn redo_1(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_redo_1<(/*void*/)> for () {
  fn redo_1(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QTextDocument4redoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:259
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearUndoRedoStacks(QTextDocument::Stacks)

/*
Clears the stacks specified by stacksToClear.

This method clears any commands on the undo stack, the redo stack, or both (the default). If commands are cleared, the appropriate signals are emitted, QTextDocument::undoAvailable() or QTextDocument::redoAvailable().

This function was introduced in  Qt 4.7.

See also QTextDocument::undoAvailable() and QTextDocument::redoAvailable().
*/
impl /*struct*/ QTextDocument {
  pub fn clearUndoRedoStacks_0<RetType, T: QTextDocument_clearUndoRedoStacks_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearUndoRedoStacks_0(self);
    // return 1;
  }
}
pub trait QTextDocument_clearUndoRedoStacks_0<RetType> {
  fn clearUndoRedoStacks_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_clearUndoRedoStacks_0<(/*void*/)> for (i32) {
  fn clearUndoRedoStacks_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument19clearUndoRedoStacksENS_6StacksE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:261
// index:0
// Public Visibility=Default Availability=Available
// [4] int maximumBlockCount() const

/*

*/
impl /*struct*/ QTextDocument {
  pub fn maximumBlockCount_0<RetType, T: QTextDocument_maximumBlockCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumBlockCount_0(self);
    // return 1;
  }
}
pub trait QTextDocument_maximumBlockCount_0<RetType> {
  fn maximumBlockCount_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_maximumBlockCount_0<i32> for () {
  fn maximumBlockCount_0(self , rsthis: & QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument17maximumBlockCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:262
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximumBlockCount(int)

/*

*/
impl /*struct*/ QTextDocument {
  pub fn setMaximumBlockCount_0<RetType, T: QTextDocument_setMaximumBlockCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumBlockCount_0(self);
    // return 1;
  }
}
pub trait QTextDocument_setMaximumBlockCount_0<RetType> {
  fn setMaximumBlockCount_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_setMaximumBlockCount_0<(/*void*/)> for (i32) {
  fn setMaximumBlockCount_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument20setMaximumBlockCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:264
// index:0
// Public Visibility=Default Availability=Available
// [32] QTextOption defaultTextOption() const

/*
The default text option is used on all QTextLayout objects in the document. This allows setting global properties for the document such as the default word wrap mode.

This function was introduced in  Qt 4.3.

Note: Getter function for property defaultTextOption. 

See also setDefaultTextOption().
*/
impl /*struct*/ QTextDocument {
  pub fn defaultTextOption_0<RetType, T: QTextDocument_defaultTextOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultTextOption_0(self);
    // return 1;
  }
}
pub trait QTextDocument_defaultTextOption_0<RetType> {
  fn defaultTextOption_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_defaultTextOption_0<usize> for () {
  fn defaultTextOption_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument17defaultTextOptionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:265
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultTextOption(const QTextOption &)

/*
Sets the default text option to option.

This function was introduced in  Qt 4.3.

Note: Setter function for property defaultTextOption. 

See also defaultTextOption().
*/
impl /*struct*/ QTextDocument {
  pub fn setDefaultTextOption_0<RetType, T: QTextDocument_setDefaultTextOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultTextOption_0(self);
    // return 1;
  }
}
pub trait QTextDocument_setDefaultTextOption_0<RetType> {
  fn setDefaultTextOption_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_setDefaultTextOption_0<(/*void*/)> for (usize) {
  fn setDefaultTextOption_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument20setDefaultTextOptionERK11QTextOption", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:267
// index:0
// Public Visibility=Default Availability=Available
// [8] QUrl baseUrl() const

/*

*/
impl /*struct*/ QTextDocument {
  pub fn baseUrl_0<RetType, T: QTextDocument_baseUrl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.baseUrl_0(self);
    // return 1;
  }
}
pub trait QTextDocument_baseUrl_0<RetType> {
  fn baseUrl_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_baseUrl_0<usize> for () {
  fn baseUrl_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument7baseUrlEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:268
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBaseUrl(const QUrl &)

/*

*/
impl /*struct*/ QTextDocument {
  pub fn setBaseUrl_0<RetType, T: QTextDocument_setBaseUrl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBaseUrl_0(self);
    // return 1;
  }
}
pub trait QTextDocument_setBaseUrl_0<RetType> {
  fn setBaseUrl_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_setBaseUrl_0<(/*void*/)> for (usize) {
  fn setBaseUrl_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument10setBaseUrlERK4QUrl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:270
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::CursorMoveStyle defaultCursorMoveStyle() const

/*
The default cursor movement style is used by all QTextCursor objects created from the document. The default is Qt::LogicalMoveStyle.

This function was introduced in  Qt 4.8.

See also setDefaultCursorMoveStyle().
*/
impl /*struct*/ QTextDocument {
  pub fn defaultCursorMoveStyle_0<RetType, T: QTextDocument_defaultCursorMoveStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultCursorMoveStyle_0(self);
    // return 1;
  }
}
pub trait QTextDocument_defaultCursorMoveStyle_0<RetType> {
  fn defaultCursorMoveStyle_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_defaultCursorMoveStyle_0<i32> for () {
  fn defaultCursorMoveStyle_0(self , rsthis: & QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTextDocument22defaultCursorMoveStyleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:271
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultCursorMoveStyle(Qt::CursorMoveStyle)

/*
Sets the default cursor movement style to the given style.

This function was introduced in  Qt 4.8.

See also defaultCursorMoveStyle().
*/
impl /*struct*/ QTextDocument {
  pub fn setDefaultCursorMoveStyle_0<RetType, T: QTextDocument_setDefaultCursorMoveStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultCursorMoveStyle_0(self);
    // return 1;
  }
}
pub trait QTextDocument_setDefaultCursorMoveStyle_0<RetType> {
  fn setDefaultCursorMoveStyle_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_setDefaultCursorMoveStyle_0<(/*void*/)> for (i32) {
  fn setDefaultCursorMoveStyle_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument25setDefaultCursorMoveStyleEN2Qt15CursorMoveStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:274
// index:0
// Public Visibility=Default Availability=Available
// [-2] void contentsChange(int, int, int)

/*
This signal is emitted whenever the document's content changes; for example, when text is inserted or deleted, or when formatting is applied.

Information is provided about the position of the character in the document where the change occurred, the number of characters removed (charsRemoved), and the number of characters added (charsAdded).

The signal is emitted before the document's layout manager is notified about the change. This hook allows you to implement syntax highlighting for the document.

See also QAbstractTextDocumentLayout::documentChanged() and contentsChanged().
*/
impl /*struct*/ QTextDocument {
  pub fn contentsChange_0<RetType, T: QTextDocument_contentsChange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contentsChange_0(self);
    // return 1;
  }
}
pub trait QTextDocument_contentsChange_0<RetType> {
  fn contentsChange_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_contentsChange_0<(/*void*/)> for (i32,i32,i32) {
  fn contentsChange_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument14contentsChangeEiii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:275
// index:0
// Public Visibility=Default Availability=Available
// [-2] void contentsChanged()

/*
This signal is emitted whenever the document's content changes; for example, when text is inserted or deleted, or when formatting is applied.

See also contentsChange().
*/
impl /*struct*/ QTextDocument {
  pub fn contentsChanged_0<RetType, T: QTextDocument_contentsChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contentsChanged_0(self);
    // return 1;
  }
}
pub trait QTextDocument_contentsChanged_0<RetType> {
  fn contentsChanged_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_contentsChanged_0<(/*void*/)> for () {
  fn contentsChanged_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QTextDocument15contentsChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:276
// index:0
// Public Visibility=Default Availability=Available
// [-2] void undoAvailable(bool)

/*
This signal is emitted whenever undo operations become available (available is true) or unavailable (available is false).

See the Qt Undo Framework documentation for details.

See also undo() and isUndoRedoEnabled().
*/
impl /*struct*/ QTextDocument {
  pub fn undoAvailable_0<RetType, T: QTextDocument_undoAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undoAvailable_0(self);
    // return 1;
  }
}
pub trait QTextDocument_undoAvailable_0<RetType> {
  fn undoAvailable_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_undoAvailable_0<(/*void*/)> for (bool) {
  fn undoAvailable_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument13undoAvailableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:277
// index:0
// Public Visibility=Default Availability=Available
// [-2] void redoAvailable(bool)

/*
This signal is emitted whenever redo operations become available (available is true) or unavailable (available is false).
*/
impl /*struct*/ QTextDocument {
  pub fn redoAvailable_0<RetType, T: QTextDocument_redoAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redoAvailable_0(self);
    // return 1;
  }
}
pub trait QTextDocument_redoAvailable_0<RetType> {
  fn redoAvailable_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_redoAvailable_0<(/*void*/)> for (bool) {
  fn redoAvailable_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument13redoAvailableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:278
// index:0
// Public Visibility=Default Availability=Available
// [-2] void undoCommandAdded()

/*
This signal is emitted every time a new level of undo is added to the QTextDocument.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QTextDocument {
  pub fn undoCommandAdded_0<RetType, T: QTextDocument_undoCommandAdded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undoCommandAdded_0(self);
    // return 1;
  }
}
pub trait QTextDocument_undoCommandAdded_0<RetType> {
  fn undoCommandAdded_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_undoCommandAdded_0<(/*void*/)> for () {
  fn undoCommandAdded_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QTextDocument16undoCommandAddedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:279
// index:0
// Public Visibility=Default Availability=Available
// [-2] void modificationChanged(bool)

/*
This signal is emitted whenever the content of the document changes in a way that affects the modification state. If changed is true, the document has been modified; otherwise it is false.

For example, calling setModified(false) on a document and then inserting text causes the signal to get emitted. If you undo that operation, causing the document to return to its original unmodified state, the signal will get emitted again.
*/
impl /*struct*/ QTextDocument {
  pub fn modificationChanged_0<RetType, T: QTextDocument_modificationChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modificationChanged_0(self);
    // return 1;
  }
}
pub trait QTextDocument_modificationChanged_0<RetType> {
  fn modificationChanged_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_modificationChanged_0<(/*void*/)> for (bool) {
  fn modificationChanged_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument19modificationChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:280
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cursorPositionChanged(const QTextCursor &)

/*
This signal is emitted whenever the position of a cursor changed due to an editing operation. The cursor that changed is passed in cursor. If the document is used with the QTextEdit class and you need a signal when the cursor is moved with the arrow keys you can use the cursorPositionChanged() signal in QTextEdit.
*/
impl /*struct*/ QTextDocument {
  pub fn cursorPositionChanged_0<RetType, T: QTextDocument_cursorPositionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorPositionChanged_0(self);
    // return 1;
  }
}
pub trait QTextDocument_cursorPositionChanged_0<RetType> {
  fn cursorPositionChanged_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_cursorPositionChanged_0<(/*void*/)> for (usize) {
  fn cursorPositionChanged_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument21cursorPositionChangedERK11QTextCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:281
// index:0
// Public Visibility=Default Availability=Available
// [-2] void blockCountChanged(int)

/*
This signal is emitted when the total number of text blocks in the document changes. The value passed in newBlockCount is the new total.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QTextDocument {
  pub fn blockCountChanged_0<RetType, T: QTextDocument_blockCountChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockCountChanged_0(self);
    // return 1;
  }
}
pub trait QTextDocument_blockCountChanged_0<RetType> {
  fn blockCountChanged_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_blockCountChanged_0<(/*void*/)> for (i32) {
  fn blockCountChanged_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument17blockCountChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:282
// index:0
// Public Visibility=Default Availability=Available
// [-2] void baseUrlChanged(const QUrl &)

/*

*/
impl /*struct*/ QTextDocument {
  pub fn baseUrlChanged_0<RetType, T: QTextDocument_baseUrlChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.baseUrlChanged_0(self);
    // return 1;
  }
}
pub trait QTextDocument_baseUrlChanged_0<RetType> {
  fn baseUrlChanged_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_baseUrlChanged_0<(/*void*/)> for (usize) {
  fn baseUrlChanged_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument14baseUrlChangedERK4QUrl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:283
// index:0
// Public Visibility=Default Availability=Available
// [-2] void documentLayoutChanged()

/*
This signal is emitted when a new document layout is set.

This function was introduced in  Qt 4.4.

See also setDocumentLayout().
*/
impl /*struct*/ QTextDocument {
  pub fn documentLayoutChanged_0<RetType, T: QTextDocument_documentLayoutChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.documentLayoutChanged_0(self);
    // return 1;
  }
}
pub trait QTextDocument_documentLayoutChanged_0<RetType> {
  fn documentLayoutChanged_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_documentLayoutChanged_0<(/*void*/)> for () {
  fn documentLayoutChanged_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QTextDocument21documentLayoutChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:288
// index:0
// Public Visibility=Default Availability=Available
// [-2] void appendUndoItem(QAbstractUndoItem *)

/*

*/
impl /*struct*/ QTextDocument {
  pub fn appendUndoItem_0<RetType, T: QTextDocument_appendUndoItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.appendUndoItem_0(self);
    // return 1;
  }
}
pub trait QTextDocument_appendUndoItem_0<RetType> {
  fn appendUndoItem_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_appendUndoItem_0<(/*void*/)> for (usize) {
  fn appendUndoItem_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument14appendUndoItemEP17QAbstractUndoItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:289
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModified(bool)

/*

*/
impl /*struct*/ QTextDocument {
  pub fn setModified_0<RetType, T: QTextDocument_setModified_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModified_0(self);
    // return 1;
  }
}
pub trait QTextDocument_setModified_0<RetType> {
  fn setModified_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_setModified_0<(/*void*/)> for (bool) {
  fn setModified_0(self , rsthis: & QTextDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QTextDocument11setModifiedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:292
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QTextObject * createObject(const QTextFormat &)

/*
Creates and returns a new document object (a QTextObject), based on the given format.

QTextObjects will always get created through this method, so you must reimplement it if you use custom text objects inside your document.
*/
impl /*struct*/ QTextDocument {
  pub fn createObject_0<RetType, T: QTextDocument_createObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createObject_0(self);
    // return 1;
  }
}
pub trait QTextDocument_createObject_0<RetType> {
  fn createObject_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_createObject_0<usize> for (usize) {
  fn createObject_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QTextDocument12createObjectERK11QTextFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocument.h:293
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QVariant loadResource(int, const QUrl &)

/*
Loads data of the specified type from the resource with the given name.

This function is called by the rich text engine to request data that isn't directly stored by QTextDocument, but still associated with it. For example, images are referenced indirectly by the name attribute of a QTextImageFormat object.

When called by Qt, type is one of the values of QTextDocument::ResourceType.

If the QTextDocument is a child object of a QObject that has an invokable loadResource method such as QTextEdit, QTextBrowser or a QTextDocument itself then the default implementation tries to retrieve the data from the parent.
*/
impl /*struct*/ QTextDocument {
  pub fn loadResource_0<RetType, T: QTextDocument_loadResource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loadResource_0(self);
    // return 1;
  }
}
pub trait QTextDocument_loadResource_0<RetType> {
  fn loadResource_0(self , rsthis: & QTextDocument) -> RetType;
}
impl<'a> /*trait*/ QTextDocument_loadResource_0<usize> for (i32,usize) {
  fn loadResource_0(self , rsthis: & QTextDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QTextDocument12loadResourceEiRK4QUrl", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum describes the different types of meta information that can be added to a document.



See also metaInformation() and setMetaInformation().

*/
pub type QTextDocument__MetaInformation = i32;
// The title of the document.
pub const QTextDocument__DocumentTitle :QTextDocument__MetaInformation = 0;
// The url of the document. The loadResource() function uses this url as the base when loading relative resources.
pub const QTextDocument__DocumentUrl :QTextDocument__MetaInformation = 1;
pub fn QTextDocument_MetaInformationItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QTextDocument", val);
}
pub fn QTextDocument_MetaInformationItemName_s(val: i32) ->String {
  //var nilthis *QTextDocument
  //return nilthis.MetaInformationItemName(val);
  return QTextDocument_MetaInformationItemName(val);
}


/*


*/
pub type QTextDocument__FindFlag = i32;
// 
pub const QTextDocument__FindBackward :QTextDocument__FindFlag = 1;
// 
pub const QTextDocument__FindCaseSensitively :QTextDocument__FindFlag = 2;
// 
pub const QTextDocument__FindWholeWords :QTextDocument__FindFlag = 4;
pub fn QTextDocument_FindFlagItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QTextDocument", val);
}
pub fn QTextDocument_FindFlagItemName_s(val: i32) ->String {
  //var nilthis *QTextDocument
  //return nilthis.FindFlagItemName(val);
  return QTextDocument_FindFlagItemName(val);
}


/*
This enum describes the types of resources that can be loaded by QTextDocument's loadResource() function.



See also loadResource().

*/
pub type QTextDocument__ResourceType = i32;
// The resource contains HTML.
pub const QTextDocument__HtmlResource :QTextDocument__ResourceType = 1;
// The resource contains image data. Currently supported data types are QVariant::Pixmap and QVariant::Image. If the corresponding variant is of type QVariant::ByteArray then Qt attempts to load the image using QImage::loadFromData. QVariant::Icon is currently not supported. The icon needs to be converted to one of the supported types first, for example using QIcon::pixmap.
pub const QTextDocument__ImageResource :QTextDocument__ResourceType = 2;
// The resource contains CSS.
pub const QTextDocument__StyleSheetResource :QTextDocument__ResourceType = 3;
// 
pub const QTextDocument__UserResource :QTextDocument__ResourceType = 100;
pub fn QTextDocument_ResourceTypeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QTextDocument", val);
}
pub fn QTextDocument_ResourceTypeItemName_s(val: i32) ->String {
  //var nilthis *QTextDocument
  //return nilthis.ResourceTypeItemName(val);
  return QTextDocument_ResourceTypeItemName(val);
}


/*
QTextDocument::UndoAndRedoStacksUndoStack | RedoStackBoth the undo and redo stacks.

*/
pub type QTextDocument__Stacks = i32;
// 
pub const QTextDocument__UndoStack :QTextDocument__Stacks = 1;
// 
pub const QTextDocument__RedoStack :QTextDocument__Stacks = 2;
// 
pub const QTextDocument__UndoAndRedoStacks :QTextDocument__Stacks = 3;
pub fn QTextDocument_StacksItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QTextDocument", val);
}
pub fn QTextDocument_StacksItemName_s(val: i32) ->String {
  //var nilthis *QTextDocument
  //return nilthis.StacksItemName(val);
  return QTextDocument_StacksItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
