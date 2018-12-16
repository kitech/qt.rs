

// mod ::core::QTranslator
// package qtcore
// /usr/include/qt/QtCore/qtranslator.h
// #include <qtranslator.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 33
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
#[derive(Default)] // class sizeof(QTranslator)=16
pub struct QTranslator {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTranslator_ITF interface {
//    QObject_ITF
//    QTranslator_PTR() *QTranslator
//}
//func (ptr *QTranslator) QTranslator_PTR() *QTranslator { return ptr }

impl /*struct*/ QTranslator {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTranslator {
    return QTranslator{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTranslator {
//  type Target = QTranslatorBASE;
//
//  fn deref(&self) -> &QTranslatorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTranslatorBASE> for QTranslator {
//  fn as_ref(& self) -> & QTranslatorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qtranslator.h:56
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTranslator {
  pub fn metaObject_0<RetType, T: QTranslator_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTranslator_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTranslator) -> RetType;
}
impl<'a> /*trait*/ QTranslator_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTranslator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTranslator10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtranslator.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTranslator(QObject *)

/*
Constructs an empty message file object with parent parent that is not connected to any file.
*/
// QTranslator(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QTranslator {
  pub fn QTranslator_0<T: QTranslator_QTranslator_0>(value: T) -> QTranslator {
    let rsthis = value.QTranslator_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTranslator_QTranslator_0 {
  fn QTranslator_0(self) -> QTranslator;
}
// QTranslator(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTranslator_QTranslator_0 for (usize) {
  fn QTranslator_0(self) -> QTranslator {
    // unsafe{_ZN11QTranslatorC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTranslatorC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTranslator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtranslator.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTranslator()

/*

*/
pub fn DeleteQTranslator(this :*mut QTranslator) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QTranslatorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qtranslator.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString translate(const char *, const char *, const char *, int) const

/*
Returns the translation for the key (context, sourceText, disambiguation). If none is found, also tries (context, sourceText, ""). If that still fails, returns a null string.

Note: Incomplete translations may result in unexpected behavior: If no translation for (context, sourceText, "") is provided, the method might in this case actually return a translation for a different disambiguation.

If n is not -1, it is used to choose an appropriate form for the translation (e.g. "%n file found" vs. "%n files found").

If you need to programatically insert translations into a QTranslator, this function can be reimplemented.

See also load().
*/
impl /*struct*/ QTranslator {
  pub fn translate_0<RetType, T: QTranslator_translate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_0(self);
    // return 1;
  }
}
pub trait QTranslator_translate_0<RetType> {
  fn translate_0(self , rsthis: & QTranslator) -> RetType;
}
impl<'a> /*trait*/ QTranslator_translate_0<usize> for (usize,usize,usize,i32) {
  fn translate_0(self , rsthis: & QTranslator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTranslator9translateEPKcS1_S1_i", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtranslator.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if this translator is empty, otherwise returns false. This function works with stripped and unstripped translation files.
*/
impl /*struct*/ QTranslator {
  pub fn isEmpty_0<RetType, T: QTranslator_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QTranslator_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QTranslator) -> RetType;
}
impl<'a> /*trait*/ QTranslator_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QTranslator) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTranslator7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtranslator.h:66
// index:0
// Public Visibility=Default Availability=Available
// [1] bool load(const QString &, const QString &, const QString &, const QString &)

/*
Loads filename + suffix (".qm" if the suffix is not specified), which may be an absolute file name or relative to directory. Returns true if the translation is successfully loaded; otherwise returns false.

If directory is not specified, the current directory is used (i.e., as currentPath()).

The previous contents of this translator object are discarded.

If the file name does not exist, other file names are tried in the following order:
*/
impl /*struct*/ QTranslator {
  pub fn load_0<RetType, T: QTranslator_load_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.load_0(self);
    // return 1;
  }
}
pub trait QTranslator_load_0<RetType> {
  fn load_0(self , rsthis: & QTranslator) -> RetType;
}
impl<'a> /*trait*/ QTranslator_load_0<bool> for (usize,usize,usize,usize) {
  fn load_0(self , rsthis: & QTranslator) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTranslator4loadERK7QStringS2_S2_S2_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtranslator.h:70
// index:1
// Public Visibility=Default Availability=Available
// [1] bool load(const QLocale &, const QString &, const QString &, const QString &, const QString &)

/*
Loads filename + suffix (".qm" if the suffix is not specified), which may be an absolute file name or relative to directory. Returns true if the translation is successfully loaded; otherwise returns false.

If directory is not specified, the current directory is used (i.e., as currentPath()).

The previous contents of this translator object are discarded.

If the file name does not exist, other file names are tried in the following order:
*/
impl /*struct*/ QTranslator {
  pub fn load_1<RetType, T: QTranslator_load_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.load_1(self);
    // return 1;
  }
}
pub trait QTranslator_load_1<RetType> {
  fn load_1(self , rsthis: & QTranslator) -> RetType;
}
impl<'a> /*trait*/ QTranslator_load_1<bool> for (usize,usize,usize,usize,usize) {
  fn load_1(self , rsthis: & QTranslator) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTranslator4loadERK7QLocaleRK7QStringS5_S5_S5_", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtranslator.h:75
// index:2
// Public Visibility=Default Availability=Available
// [1] bool load(const uchar *, int, const QString &)

/*
Loads filename + suffix (".qm" if the suffix is not specified), which may be an absolute file name or relative to directory. Returns true if the translation is successfully loaded; otherwise returns false.

If directory is not specified, the current directory is used (i.e., as currentPath()).

The previous contents of this translator object are discarded.

If the file name does not exist, other file names are tried in the following order:
*/
impl /*struct*/ QTranslator {
  pub fn load_2<RetType, T: QTranslator_load_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.load_2(self);
    // return 1;
  }
}
pub trait QTranslator_load_2<RetType> {
  fn load_2(self , rsthis: & QTranslator) -> RetType;
}
impl<'a> /*trait*/ QTranslator_load_2<bool> for (usize,i32,usize) {
  fn load_2(self , rsthis: & QTranslator) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTranslator4loadEPKhiRK7QString", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
