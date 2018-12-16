

// mod ::core::QFileSelector
// package qtcore
// /usr/include/qt/QtCore/qfileselector.h
// #include <qfileselector.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 2
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QFileSelector)=16
pub struct QFileSelector {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFileSelector_ITF interface {
//    QObject_ITF
//    QFileSelector_PTR() *QFileSelector
//}
//func (ptr *QFileSelector) QFileSelector_PTR() *QFileSelector { return ptr }

impl /*struct*/ QFileSelector {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFileSelector {
    return QFileSelector{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFileSelector {
//  type Target = QFileSelectorBASE;
//
//  fn deref(&self) -> &QFileSelectorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFileSelectorBASE> for QFileSelector {
//  fn as_ref(& self) -> & QFileSelectorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qfileselector.h:51
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QFileSelector {
  pub fn metaObject_0<RetType, T: QFileSelector_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QFileSelector_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QFileSelector) -> RetType;
}
impl<'a> /*trait*/ QFileSelector_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QFileSelector) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFileSelector10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileselector.h:53
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFileSelector(QObject *)

/*
Create a QFileSelector instance. This instance will have the same static selectors as other QFileSelector instances, but its own set of extra selectors.

If supplied, it will have the given QObject parent.
*/
// QFileSelector(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QFileSelector {
  pub fn QFileSelector_0<T: QFileSelector_QFileSelector_0>(value: T) -> QFileSelector {
    let rsthis = value.QFileSelector_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFileSelector_QFileSelector_0 {
  fn QFileSelector_0(self) -> QFileSelector;
}
// QFileSelector(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFileSelector_QFileSelector_0 for (usize) {
  fn QFileSelector_0(self) -> QFileSelector {
    // unsafe{_ZN13QFileSelectorC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QFileSelectorC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFileSelector{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfileselector.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QFileSelector()

/*

*/
pub fn DeleteQFileSelector(this :*mut QFileSelector) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QFileSelectorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qfileselector.h:56
// index:0
// Public Visibility=Default Availability=Available
// [8] QString select(const QString &) const

/*
This function returns the selected version of the path, based on the conditions at runtime. If no selectable files are present, returns the original filePath.

If the original file does not exist, the original filePath is returned. This means that you must have a base file to fall back on, you cannot have only files in selectable sub-directories.

See the class overview for the selection algorithm.
*/
impl /*struct*/ QFileSelector {
  pub fn select__0<RetType, T: QFileSelector_select__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.select__0(self);
    // return 1;
  }
}
pub trait QFileSelector_select__0<RetType> {
  fn select__0(self , rsthis: & QFileSelector) -> RetType;
}
impl<'a> /*trait*/ QFileSelector_select__0<usize> for (usize) {
  fn select__0(self , rsthis: & QFileSelector) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFileSelector6selectERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileselector.h:57
// index:1
// Public Visibility=Default Availability=Available
// [8] QUrl select(const QUrl &) const

/*
This function returns the selected version of the path, based on the conditions at runtime. If no selectable files are present, returns the original filePath.

If the original file does not exist, the original filePath is returned. This means that you must have a base file to fall back on, you cannot have only files in selectable sub-directories.

See the class overview for the selection algorithm.
*/
impl /*struct*/ QFileSelector {
  pub fn select__1<RetType, T: QFileSelector_select__1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.select__1(self);
    // return 1;
  }
}
pub trait QFileSelector_select__1<RetType> {
  fn select__1(self , rsthis: & QFileSelector) -> RetType;
}
impl<'a> /*trait*/ QFileSelector_select__1<usize> for (usize) {
  fn select__1(self , rsthis: & QFileSelector) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFileSelector6selectERK4QUrl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileselector.h:59
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList extraSelectors() const

/*
Returns the list of extra selectors which have been added programmatically to this instance.

See also setExtraSelectors().
*/
impl /*struct*/ QFileSelector {
  pub fn extraSelectors_0<RetType, T: QFileSelector_extraSelectors_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.extraSelectors_0(self);
    // return 1;
  }
}
pub trait QFileSelector_extraSelectors_0<RetType> {
  fn extraSelectors_0(self , rsthis: & QFileSelector) -> RetType;
}
impl<'a> /*trait*/ QFileSelector_extraSelectors_0<usize> for () {
  fn extraSelectors_0(self , rsthis: & QFileSelector) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFileSelector14extraSelectorsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileselector.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setExtraSelectors(const QStringList &)

/*
Sets the list of extra selectors which have been added programmatically to this instance.

These selectors have priority over any which have been automatically picked up.

See also extraSelectors().
*/
impl /*struct*/ QFileSelector {
  pub fn setExtraSelectors_0<RetType, T: QFileSelector_setExtraSelectors_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setExtraSelectors_0(self);
    // return 1;
  }
}
pub trait QFileSelector_setExtraSelectors_0<RetType> {
  fn setExtraSelectors_0(self , rsthis: & QFileSelector) -> RetType;
}
impl<'a> /*trait*/ QFileSelector_setExtraSelectors_0<(/*void*/)> for (usize) {
  fn setExtraSelectors_0(self , rsthis: & QFileSelector) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QFileSelector17setExtraSelectorsERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfileselector.h:62
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList allSelectors() const

/*
Returns the complete, ordered list of selectors used by this instance
*/
impl /*struct*/ QFileSelector {
  pub fn allSelectors_0<RetType, T: QFileSelector_allSelectors_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.allSelectors_0(self);
    // return 1;
  }
}
pub trait QFileSelector_allSelectors_0<RetType> {
  fn allSelectors_0(self , rsthis: & QFileSelector) -> RetType;
}
impl<'a> /*trait*/ QFileSelector_allSelectors_0<usize> for () {
  fn allSelectors_0(self , rsthis: & QFileSelector) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFileSelector12allSelectorsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
