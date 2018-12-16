

// mod ::core::QLoggingCategory
// package qtcore
// /usr/include/qt/QtCore/qloggingcategory.h
// #include <qloggingcategory.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 11
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QLoggingCategory)=24
pub struct QLoggingCategory {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QLoggingCategory_ITF interface {
//    QLoggingCategory_PTR() *QLoggingCategory
//}
//func (ptr *QLoggingCategory) QLoggingCategory_PTR() *QLoggingCategory { return ptr }

impl /*struct*/ QLoggingCategory {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QLoggingCategory {
    return QLoggingCategory{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QLoggingCategory {
//  type Target = QLoggingCategoryBASE;
//
//  fn deref(&self) -> &QLoggingCategoryBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QLoggingCategoryBASE> for QLoggingCategory {
//  fn as_ref(& self) -> & QLoggingCategoryBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qloggingcategory.h:53
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QLoggingCategory(const char *)

/*
Constructs a QLoggingCategory object with the provided category name. All message types for this category are enabled by default.

If category is 0, the category name is changed to "default".
*/
// QLoggingCategory(const char *) ctx.fn_proto_cpp
impl /*struct*/ QLoggingCategory {
  pub fn QLoggingCategory_0<T: QLoggingCategory_QLoggingCategory_0>(value: T) -> QLoggingCategory {
    let rsthis = value.QLoggingCategory_0();
    return rsthis;
    // return 1;
  }
}

pub trait QLoggingCategory_QLoggingCategory_0 {
  fn QLoggingCategory_0(self) -> QLoggingCategory;
}
// QLoggingCategory(const char *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLoggingCategory_QLoggingCategory_0 for (usize) {
  fn QLoggingCategory_0(self) -> QLoggingCategory {
    // unsafe{_ZN16QLoggingCategoryC2EPKc()};
    let arg0 = (self) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QLoggingCategoryC2EPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLoggingCategory{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qloggingcategory.h:54
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QLoggingCategory(const char *, QtMsgType)

/*
Constructs a QLoggingCategory object with the provided category name. All message types for this category are enabled by default.

If category is 0, the category name is changed to "default".
*/
// QLoggingCategory(const char *, QtMsgType) ctx.fn_proto_cpp
impl /*struct*/ QLoggingCategory {
  pub fn QLoggingCategory_1<T: QLoggingCategory_QLoggingCategory_1>(value: T) -> QLoggingCategory {
    let rsthis = value.QLoggingCategory_1();
    return rsthis;
    // return 1;
  }
}

pub trait QLoggingCategory_QLoggingCategory_1 {
  fn QLoggingCategory_1(self) -> QLoggingCategory;
}
// QLoggingCategory(const char *, QtMsgType) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLoggingCategory_QLoggingCategory_1 for (usize,i32) {
  fn QLoggingCategory_1(self) -> QLoggingCategory {
    // unsafe{_ZN16QLoggingCategoryC2EPKc9QtMsgType()};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QLoggingCategoryC2EPKc9QtMsgType", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLoggingCategory{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qloggingcategory.h:55
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QLoggingCategory()

/*

*/
pub fn DeleteQLoggingCategory(this :*mut QLoggingCategory) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QLoggingCategoryD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qloggingcategory.h:57
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEnabled(QtMsgType) const

/*
Returns true if a message of type msgtype for the category should be shown. Returns false otherwise.
*/
impl /*struct*/ QLoggingCategory {
  pub fn isEnabled_0<RetType, T: QLoggingCategory_isEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEnabled_0(self);
    // return 1;
  }
}
pub trait QLoggingCategory_isEnabled_0<RetType> {
  fn isEnabled_0(self , rsthis: & QLoggingCategory) -> RetType;
}
impl<'a> /*trait*/ QLoggingCategory_isEnabled_0<bool> for (i32) {
  fn isEnabled_0(self , rsthis: & QLoggingCategory) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QLoggingCategory9isEnabledE9QtMsgType", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qloggingcategory.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEnabled(QtMsgType, bool)

/*
Changes the message type type for the category to enable.

This method is meant to be used only from inside a filter installed by installFilter(). See Configuring Categories for an overview on how to configure categories globally.

Note: QtFatalMsg cannot be changed. It will always remain true.

See also isEnabled().
*/
impl /*struct*/ QLoggingCategory {
  pub fn setEnabled_0<RetType, T: QLoggingCategory_setEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEnabled_0(self);
    // return 1;
  }
}
pub trait QLoggingCategory_setEnabled_0<RetType> {
  fn setEnabled_0(self , rsthis: & QLoggingCategory) -> RetType;
}
impl<'a> /*trait*/ QLoggingCategory_setEnabled_0<(/*void*/)> for (i32,bool) {
  fn setEnabled_0(self , rsthis: & QLoggingCategory) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QLoggingCategory10setEnabledE9QtMsgTypeb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qloggingcategory.h:61
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isDebugEnabled() const

/*
Returns true if debug messages should be shown for this category. Returns false otherwise.

Note: The qCDebug() macro already does this check before executing any code. However, calling this method may be useful to avoid expensive generation of data that is only used for debug output.
*/
impl /*struct*/ QLoggingCategory {
  pub fn isDebugEnabled_0<RetType, T: QLoggingCategory_isDebugEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDebugEnabled_0(self);
    // return 1;
  }
}
pub trait QLoggingCategory_isDebugEnabled_0<RetType> {
  fn isDebugEnabled_0(self , rsthis: & QLoggingCategory) -> RetType;
}
impl<'a> /*trait*/ QLoggingCategory_isDebugEnabled_0<bool> for () {
  fn isDebugEnabled_0(self , rsthis: & QLoggingCategory) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QLoggingCategory14isDebugEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qloggingcategory.h:62
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isInfoEnabled() const

/*
Returns true if informational messages should be shown for this category. Returns false otherwise.

Note: The qCInfo() macro already does this check before executing any code. However, calling this method may be useful to avoid expensive generation of data that is only used for debug output.

This function was introduced in  Qt 5.5.
*/
impl /*struct*/ QLoggingCategory {
  pub fn isInfoEnabled_0<RetType, T: QLoggingCategory_isInfoEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isInfoEnabled_0(self);
    // return 1;
  }
}
pub trait QLoggingCategory_isInfoEnabled_0<RetType> {
  fn isInfoEnabled_0(self , rsthis: & QLoggingCategory) -> RetType;
}
impl<'a> /*trait*/ QLoggingCategory_isInfoEnabled_0<bool> for () {
  fn isInfoEnabled_0(self , rsthis: & QLoggingCategory) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QLoggingCategory13isInfoEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qloggingcategory.h:63
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isWarningEnabled() const

/*
Returns true if warning messages should be shown for this category. Returns false otherwise.

Note: The qCWarning() macro already does this check before executing any code. However, calling this method may be useful to avoid expensive generation of data that is only used for debug output.
*/
impl /*struct*/ QLoggingCategory {
  pub fn isWarningEnabled_0<RetType, T: QLoggingCategory_isWarningEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isWarningEnabled_0(self);
    // return 1;
  }
}
pub trait QLoggingCategory_isWarningEnabled_0<RetType> {
  fn isWarningEnabled_0(self , rsthis: & QLoggingCategory) -> RetType;
}
impl<'a> /*trait*/ QLoggingCategory_isWarningEnabled_0<bool> for () {
  fn isWarningEnabled_0(self , rsthis: & QLoggingCategory) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QLoggingCategory16isWarningEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qloggingcategory.h:64
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isCriticalEnabled() const

/*
Returns true if critical messages should be shown for this category. Returns false otherwise.

Note: The qCCritical() macro already does this check before executing any code. However, calling this method may be useful to avoid expensive generation of data that is only used for debug output.
*/
impl /*struct*/ QLoggingCategory {
  pub fn isCriticalEnabled_0<RetType, T: QLoggingCategory_isCriticalEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCriticalEnabled_0(self);
    // return 1;
  }
}
pub trait QLoggingCategory_isCriticalEnabled_0<RetType> {
  fn isCriticalEnabled_0(self , rsthis: & QLoggingCategory) -> RetType;
}
impl<'a> /*trait*/ QLoggingCategory_isCriticalEnabled_0<bool> for () {
  fn isCriticalEnabled_0(self , rsthis: & QLoggingCategory) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QLoggingCategory17isCriticalEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qloggingcategory.h:71
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const char * categoryName() const

/*
Returns the name of the category.
*/
impl /*struct*/ QLoggingCategory {
  pub fn categoryName_0<RetType, T: QLoggingCategory_categoryName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.categoryName_0(self);
    // return 1;
  }
}
pub trait QLoggingCategory_categoryName_0<RetType> {
  fn categoryName_0(self , rsthis: & QLoggingCategory) -> RetType;
}
impl<'a> /*trait*/ QLoggingCategory_categoryName_0<usize> for () {
  fn categoryName_0(self , rsthis: & QLoggingCategory) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QLoggingCategory12categoryNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qloggingcategory.h:77
// index:0
// Public static Visibility=Default Availability=Available
// [8] QLoggingCategory * defaultCategory()

/*
Returns a pointer to the global category "default" that is used e.g. by qDebug(), qInfo(), qWarning(), qCritical(), qFatal().

Note: The returned pointer may be null during destruction of static objects.

Note: Ownership of the category is not transferred, do not delete the returned pointer.
*/
impl /*struct*/ QLoggingCategory {
  pub fn defaultCategory_0<RetType, T: QLoggingCategory_defaultCategory_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.defaultCategory_0();
    // return 1;
  }
}
pub trait QLoggingCategory_defaultCategory_0<RetType> {
  fn defaultCategory_0(self ) -> RetType;
}
impl<'a> /*trait*/ QLoggingCategory_defaultCategory_0<usize> for () {
  fn defaultCategory_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QLoggingCategory15defaultCategoryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qloggingcategory.h:82
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setFilterRules(const QString &)

/*
Configures which categories and message types should be enabled through a a set of rules.

Example:


      QLoggingCategory::setFilterRules(QStringLiteral("driver.usb.debug=true"));



Note: The rules might be ignored if a custom category filter is installed with installFilter(), or if the user defined QT_LOGGING_CONF or QT_LOGGING_RULES environment variable.
*/
impl /*struct*/ QLoggingCategory {
  pub fn setFilterRules_0<RetType, T: QLoggingCategory_setFilterRules_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setFilterRules_0();
    // return 1;
  }
}
pub trait QLoggingCategory_setFilterRules_0<RetType> {
  fn setFilterRules_0(self ) -> RetType;
}
impl<'a> /*trait*/ QLoggingCategory_setFilterRules_0<(/*void*/)> for (usize) {
  fn setFilterRules_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QLoggingCategory14setFilterRulesERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QLoggingCategory__ = i32;
// 
pub const QLoggingCategory__DebugShift :QLoggingCategory__ = 0;
// 
pub const QLoggingCategory__WarningShift :QLoggingCategory__ = 8;
// 
pub const QLoggingCategory__CriticalShift :QLoggingCategory__ = 16;
// 
pub const QLoggingCategory__InfoShift :QLoggingCategory__ = 24;
pub fn QLoggingCategory_ItemName(val: i32) ->String {
  match val {
     QLoggingCategory__DebugShift => // 0
     {return String::from("DebugShift");}
     QLoggingCategory__WarningShift => // 8
     {return String::from("WarningShift");}
     QLoggingCategory__CriticalShift => // 16
     {return String::from("CriticalShift");}
     QLoggingCategory__InfoShift => // 24
     {return String::from("InfoShift");}
  _ => {return format!("{}", val);}
}
}
pub fn QLoggingCategory_ItemName_s(val: i32) ->String {
  //var nilthis *QLoggingCategory
  //return nilthis.ItemName(val);
  return QLoggingCategory_ItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
