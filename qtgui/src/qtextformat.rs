

// mod ::gui::QTextFormat
// package qtgui
// /usr/include/qt/QtGui/qtextformat.h
// #include <qtextformat.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 7
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
#[derive(Default)] // class sizeof(QTextFormat)=16
pub struct QTextFormat {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextFormat_ITF interface {
//    QTextFormat_PTR() *QTextFormat
//}
//func (ptr *QTextFormat) QTextFormat_PTR() *QTextFormat { return ptr }

impl /*struct*/ QTextFormat {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextFormat {
    return QTextFormat{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextFormat {
//  type Target = QTextFormatBASE;
//
//  fn deref(&self) -> &QTextFormatBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextFormatBASE> for QTextFormat {
//  fn as_ref(& self) -> & QTextFormatBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextformat.h:288
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextFormat()

/*
Creates a new text format with an InvalidFormat.

See also FormatType.
*/
// QTextFormat() ctx.fn_proto_cpp
impl /*struct*/ QTextFormat {
  pub fn QTextFormat_0<T: QTextFormat_QTextFormat_0>(value: T) -> QTextFormat {
    let rsthis = value.QTextFormat_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextFormat_QTextFormat_0 {
  fn QTextFormat_0(self) -> QTextFormat;
}
// QTextFormat() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextFormat_QTextFormat_0 for () {
  fn QTextFormat_0(self) -> QTextFormat {
    // unsafe{_ZN11QTextFormatC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextFormatC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:290
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTextFormat(int)

/*
Creates a new text format with an InvalidFormat.

See also FormatType.
*/
// QTextFormat(int) ctx.fn_proto_cpp
impl /*struct*/ QTextFormat {
  pub fn QTextFormat_1<T: QTextFormat_QTextFormat_1>(value: T) -> QTextFormat {
    let rsthis = value.QTextFormat_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextFormat_QTextFormat_1 {
  fn QTextFormat_1(self) -> QTextFormat;
}
// QTextFormat(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextFormat_QTextFormat_1 for (i32) {
  fn QTextFormat_1(self) -> QTextFormat {
    // unsafe{_ZN11QTextFormatC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextFormatC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:293
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextFormat & operator=(const QTextFormat &)

/*

*/
impl /*struct*/ QTextFormat {
  pub fn operator_equal_0<RetType, T: QTextFormat_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QTextFormat_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QTextFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextFormataSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:294
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QTextFormat()

/*

*/
pub fn DeleteQTextFormat(this :*mut QTextFormat) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QTextFormatD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qtextformat.h:296
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QTextFormat &)

/*
Swaps this text format with other. This function is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QTextFormat {
  pub fn swap_0<RetType, T: QTextFormat_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QTextFormat_swap_0<RetType> {
  fn swap_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QTextFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextFormat4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:299
// index:0
// Public Visibility=Default Availability=Available
// [-2] void merge(const QTextFormat &)

/*
Merges the other format with this format; where there are conflicts the other format takes precedence.
*/
impl /*struct*/ QTextFormat {
  pub fn merge_0<RetType, T: QTextFormat_merge_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.merge_0(self);
    // return 1;
  }
}
pub trait QTextFormat_merge_0<RetType> {
  fn merge_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_merge_0<(/*void*/)> for (usize) {
  fn merge_0(self , rsthis: & QTextFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextFormat5mergeERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:301
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the format is valid (i.e. is not InvalidFormat); otherwise returns false.
*/
impl /*struct*/ QTextFormat {
  pub fn isValid_0<RetType, T: QTextFormat_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QTextFormat_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QTextFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:302
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if the format does not store any properties; false otherwise.

This function was introduced in  Qt 5.3.

See also propertyCount() and properties().
*/
impl /*struct*/ QTextFormat {
  pub fn isEmpty_0<RetType, T: QTextFormat_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QTextFormat_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QTextFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:304
// index:0
// Public Visibility=Default Availability=Available
// [4] int type() const

/*
Returns the type of this format.

See also FormatType.
*/
impl /*struct*/ QTextFormat {
  pub fn type__0<RetType, T: QTextFormat_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QTextFormat_type__0<RetType> {
  fn type__0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_type__0<i32> for () {
  fn type__0(self , rsthis: & QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:306
// index:0
// Public Visibility=Default Availability=Available
// [4] int objectIndex() const

/*
Returns the index of the format object, or -1 if the format object is invalid.

See also setObjectIndex().
*/
impl /*struct*/ QTextFormat {
  pub fn objectIndex_0<RetType, T: QTextFormat_objectIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.objectIndex_0(self);
    // return 1;
  }
}
pub trait QTextFormat_objectIndex_0<RetType> {
  fn objectIndex_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_objectIndex_0<i32> for () {
  fn objectIndex_0(self , rsthis: & QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat11objectIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:307
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setObjectIndex(int)

/*
Sets the format object's object index.

See also objectIndex().
*/
impl /*struct*/ QTextFormat {
  pub fn setObjectIndex_0<RetType, T: QTextFormat_setObjectIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setObjectIndex_0(self);
    // return 1;
  }
}
pub trait QTextFormat_setObjectIndex_0<RetType> {
  fn setObjectIndex_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_setObjectIndex_0<(/*void*/)> for (i32) {
  fn setObjectIndex_0(self , rsthis: & QTextFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextFormat14setObjectIndexEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:309
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant property(int) const

/*
Returns the property specified by the given propertyId.

See also setProperty() and Property.
*/
impl /*struct*/ QTextFormat {
  pub fn property_0<RetType, T: QTextFormat_property_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.property_0(self);
    // return 1;
  }
}
pub trait QTextFormat_property_0<RetType> {
  fn property_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_property_0<usize> for (i32) {
  fn property_0(self , rsthis: & QTextFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat8propertyEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:310
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setProperty(int, const QVariant &)

/*
Sets the property specified by the propertyId to the given value.

See also property() and Property.
*/
impl /*struct*/ QTextFormat {
  pub fn setProperty_0<RetType, T: QTextFormat_setProperty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setProperty_0(self);
    // return 1;
  }
}
pub trait QTextFormat_setProperty_0<RetType> {
  fn setProperty_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_setProperty_0<(/*void*/)> for (i32,usize) {
  fn setProperty_0(self , rsthis: & QTextFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextFormat11setPropertyEiRK8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:311
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearProperty(int)

/*
Clears the value of the property given by propertyId

See also Property.
*/
impl /*struct*/ QTextFormat {
  pub fn clearProperty_0<RetType, T: QTextFormat_clearProperty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearProperty_0(self);
    // return 1;
  }
}
pub trait QTextFormat_clearProperty_0<RetType> {
  fn clearProperty_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_clearProperty_0<(/*void*/)> for (i32) {
  fn clearProperty_0(self , rsthis: & QTextFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextFormat13clearPropertyEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:312
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasProperty(int) const

/*
Returns true if the text format has a property with the given propertyId; otherwise returns false.

See also properties() and Property.
*/
impl /*struct*/ QTextFormat {
  pub fn hasProperty_0<RetType, T: QTextFormat_hasProperty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasProperty_0(self);
    // return 1;
  }
}
pub trait QTextFormat_hasProperty_0<RetType> {
  fn hasProperty_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_hasProperty_0<bool> for (i32) {
  fn hasProperty_0(self , rsthis: & QTextFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat11hasPropertyEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:314
// index:0
// Public Visibility=Default Availability=Available
// [1] bool boolProperty(int) const

/*
Returns the value of the property specified by propertyId. If the property isn't of QTextFormat::Bool type, false is returned instead.

See also setProperty(), intProperty(), doubleProperty(), stringProperty(), colorProperty(), lengthProperty(), lengthVectorProperty(), and Property.
*/
impl /*struct*/ QTextFormat {
  pub fn boolProperty_0<RetType, T: QTextFormat_boolProperty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boolProperty_0(self);
    // return 1;
  }
}
pub trait QTextFormat_boolProperty_0<RetType> {
  fn boolProperty_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_boolProperty_0<bool> for (i32) {
  fn boolProperty_0(self , rsthis: & QTextFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat12boolPropertyEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:315
// index:0
// Public Visibility=Default Availability=Available
// [4] int intProperty(int) const

/*
Returns the value of the property specified by propertyId. If the property is not of QTextFormat::Integer type, 0 is returned instead.

See also setProperty(), boolProperty(), doubleProperty(), stringProperty(), colorProperty(), lengthProperty(), lengthVectorProperty(), and Property.
*/
impl /*struct*/ QTextFormat {
  pub fn intProperty_0<RetType, T: QTextFormat_intProperty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intProperty_0(self);
    // return 1;
  }
}
pub trait QTextFormat_intProperty_0<RetType> {
  fn intProperty_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_intProperty_0<i32> for (i32) {
  fn intProperty_0(self , rsthis: & QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat11intPropertyEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:316
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal doubleProperty(int) const

/*
Returns the value of the property specified by propertyId. If the property isn't of QVariant::Double or QMetaType::Float type, 0 is returned instead.

See also setProperty(), boolProperty(), intProperty(), stringProperty(), colorProperty(), lengthProperty(), lengthVectorProperty(), and Property.
*/
impl /*struct*/ QTextFormat {
  pub fn doubleProperty_0<RetType, T: QTextFormat_doubleProperty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doubleProperty_0(self);
    // return 1;
  }
}
pub trait QTextFormat_doubleProperty_0<RetType> {
  fn doubleProperty_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_doubleProperty_0<f64> for (i32) {
  fn doubleProperty_0(self , rsthis: & QTextFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat14doublePropertyEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:317
// index:0
// Public Visibility=Default Availability=Available
// [8] QString stringProperty(int) const

/*
Returns the value of the property given by propertyId; if the property isn't of QVariant::String type, an empty string is returned instead.

See also setProperty(), boolProperty(), intProperty(), doubleProperty(), colorProperty(), lengthProperty(), lengthVectorProperty(), and Property.
*/
impl /*struct*/ QTextFormat {
  pub fn stringProperty_0<RetType, T: QTextFormat_stringProperty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stringProperty_0(self);
    // return 1;
  }
}
pub trait QTextFormat_stringProperty_0<RetType> {
  fn stringProperty_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_stringProperty_0<usize> for (i32) {
  fn stringProperty_0(self , rsthis: & QTextFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat14stringPropertyEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:318
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor colorProperty(int) const

/*
Returns the value of the property given by propertyId; if the property isn't of QVariant::Color type, an invalid color is returned instead.

See also setProperty(), boolProperty(), intProperty(), doubleProperty(), stringProperty(), lengthProperty(), lengthVectorProperty(), and Property.
*/
impl /*struct*/ QTextFormat {
  pub fn colorProperty_0<RetType, T: QTextFormat_colorProperty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.colorProperty_0(self);
    // return 1;
  }
}
pub trait QTextFormat_colorProperty_0<RetType> {
  fn colorProperty_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_colorProperty_0<usize> for (i32) {
  fn colorProperty_0(self , rsthis: & QTextFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat13colorPropertyEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:319
// index:0
// Public Visibility=Default Availability=Available
// [8] QPen penProperty(int) const

/*
Returns the value of the property given by propertyId; if the property isn't of QVariant::Pen type, Qt::NoPen is returned instead.

See also setProperty(), boolProperty(), intProperty(), doubleProperty(), stringProperty(), lengthProperty(), lengthVectorProperty(), and Property.
*/
impl /*struct*/ QTextFormat {
  pub fn penProperty_0<RetType, T: QTextFormat_penProperty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.penProperty_0(self);
    // return 1;
  }
}
pub trait QTextFormat_penProperty_0<RetType> {
  fn penProperty_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_penProperty_0<usize> for (i32) {
  fn penProperty_0(self , rsthis: & QTextFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat11penPropertyEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:320
// index:0
// Public Visibility=Default Availability=Available
// [8] QBrush brushProperty(int) const

/*
Returns the value of the property given by propertyId; if the property isn't of QVariant::Brush type, Qt::NoBrush is returned instead.

See also setProperty(), boolProperty(), intProperty(), doubleProperty(), stringProperty(), lengthProperty(), lengthVectorProperty(), and Property.
*/
impl /*struct*/ QTextFormat {
  pub fn brushProperty_0<RetType, T: QTextFormat_brushProperty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.brushProperty_0(self);
    // return 1;
  }
}
pub trait QTextFormat_brushProperty_0<RetType> {
  fn brushProperty_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_brushProperty_0<usize> for (i32) {
  fn brushProperty_0(self , rsthis: & QTextFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat13brushPropertyEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:321
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextLength lengthProperty(int) const

/*
Returns the value of the property given by propertyId.

See also setProperty(), boolProperty(), intProperty(), doubleProperty(), stringProperty(), colorProperty(), lengthVectorProperty(), and Property.
*/
impl /*struct*/ QTextFormat {
  pub fn lengthProperty_0<RetType, T: QTextFormat_lengthProperty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lengthProperty_0(self);
    // return 1;
  }
}
pub trait QTextFormat_lengthProperty_0<RetType> {
  fn lengthProperty_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_lengthProperty_0<usize> for (i32) {
  fn lengthProperty_0(self , rsthis: & QTextFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat14lengthPropertyEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:327
// index:0
// Public Visibility=Default Availability=Available
// [4] int propertyCount() const

/*
Returns the number of properties stored in the format.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QTextFormat {
  pub fn propertyCount_0<RetType, T: QTextFormat_propertyCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.propertyCount_0(self);
    // return 1;
  }
}
pub trait QTextFormat_propertyCount_0<RetType> {
  fn propertyCount_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_propertyCount_0<i32> for () {
  fn propertyCount_0(self , rsthis: & QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat13propertyCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:329
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setObjectType(int)

/*
Sets the text format's object type to type.

See also ObjectTypes and objectType().
*/
impl /*struct*/ QTextFormat {
  pub fn setObjectType_0<RetType, T: QTextFormat_setObjectType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setObjectType_0(self);
    // return 1;
  }
}
pub trait QTextFormat_setObjectType_0<RetType> {
  fn setObjectType_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_setObjectType_0<(/*void*/)> for (i32) {
  fn setObjectType_0(self , rsthis: & QTextFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextFormat13setObjectTypeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:330
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int objectType() const

/*
Returns the text format's object type.

See also ObjectTypes and setObjectType().
*/
impl /*struct*/ QTextFormat {
  pub fn objectType_0<RetType, T: QTextFormat_objectType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.objectType_0(self);
    // return 1;
  }
}
pub trait QTextFormat_objectType_0<RetType> {
  fn objectType_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_objectType_0<i32> for () {
  fn objectType_0(self , rsthis: & QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat10objectTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:333
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isCharFormat() const

/*
Returns true if this text format is a CharFormat; otherwise returns false.
*/
impl /*struct*/ QTextFormat {
  pub fn isCharFormat_0<RetType, T: QTextFormat_isCharFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCharFormat_0(self);
    // return 1;
  }
}
pub trait QTextFormat_isCharFormat_0<RetType> {
  fn isCharFormat_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_isCharFormat_0<bool> for () {
  fn isCharFormat_0(self , rsthis: & QTextFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat12isCharFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:334
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isBlockFormat() const

/*
Returns true if this text format is a BlockFormat; otherwise returns false.
*/
impl /*struct*/ QTextFormat {
  pub fn isBlockFormat_0<RetType, T: QTextFormat_isBlockFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isBlockFormat_0(self);
    // return 1;
  }
}
pub trait QTextFormat_isBlockFormat_0<RetType> {
  fn isBlockFormat_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_isBlockFormat_0<bool> for () {
  fn isBlockFormat_0(self , rsthis: & QTextFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat13isBlockFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:335
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isListFormat() const

/*
Returns true if this text format is a ListFormat; otherwise returns false.
*/
impl /*struct*/ QTextFormat {
  pub fn isListFormat_0<RetType, T: QTextFormat_isListFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isListFormat_0(self);
    // return 1;
  }
}
pub trait QTextFormat_isListFormat_0<RetType> {
  fn isListFormat_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_isListFormat_0<bool> for () {
  fn isListFormat_0(self , rsthis: & QTextFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat12isListFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:336
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isFrameFormat() const

/*
Returns true if this text format is a FrameFormat; otherwise returns false.
*/
impl /*struct*/ QTextFormat {
  pub fn isFrameFormat_0<RetType, T: QTextFormat_isFrameFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFrameFormat_0(self);
    // return 1;
  }
}
pub trait QTextFormat_isFrameFormat_0<RetType> {
  fn isFrameFormat_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_isFrameFormat_0<bool> for () {
  fn isFrameFormat_0(self , rsthis: & QTextFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat13isFrameFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:337
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isImageFormat() const

/*
Returns true if this text format is an image format; otherwise returns false.
*/
impl /*struct*/ QTextFormat {
  pub fn isImageFormat_0<RetType, T: QTextFormat_isImageFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isImageFormat_0(self);
    // return 1;
  }
}
pub trait QTextFormat_isImageFormat_0<RetType> {
  fn isImageFormat_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_isImageFormat_0<bool> for () {
  fn isImageFormat_0(self , rsthis: & QTextFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat13isImageFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:338
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isTableFormat() const

/*
Returns true if this text format is a TableFormat; otherwise returns false.
*/
impl /*struct*/ QTextFormat {
  pub fn isTableFormat_0<RetType, T: QTextFormat_isTableFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTableFormat_0(self);
    // return 1;
  }
}
pub trait QTextFormat_isTableFormat_0<RetType> {
  fn isTableFormat_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_isTableFormat_0<bool> for () {
  fn isTableFormat_0(self , rsthis: & QTextFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat13isTableFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:339
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isTableCellFormat() const

/*
Returns true if this text format is a TableCellFormat; otherwise returns false.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QTextFormat {
  pub fn isTableCellFormat_0<RetType, T: QTextFormat_isTableCellFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTableCellFormat_0(self);
    // return 1;
  }
}
pub trait QTextFormat_isTableCellFormat_0<RetType> {
  fn isTableCellFormat_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_isTableCellFormat_0<bool> for () {
  fn isTableCellFormat_0(self , rsthis: & QTextFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat17isTableCellFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:341
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextBlockFormat toBlockFormat() const

/*
Returns this format as a block format.
*/
impl /*struct*/ QTextFormat {
  pub fn toBlockFormat_0<RetType, T: QTextFormat_toBlockFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toBlockFormat_0(self);
    // return 1;
  }
}
pub trait QTextFormat_toBlockFormat_0<RetType> {
  fn toBlockFormat_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_toBlockFormat_0<usize> for () {
  fn toBlockFormat_0(self , rsthis: & QTextFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat13toBlockFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:342
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextCharFormat toCharFormat() const

/*
Returns this format as a character format.
*/
impl /*struct*/ QTextFormat {
  pub fn toCharFormat_0<RetType, T: QTextFormat_toCharFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toCharFormat_0(self);
    // return 1;
  }
}
pub trait QTextFormat_toCharFormat_0<RetType> {
  fn toCharFormat_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_toCharFormat_0<usize> for () {
  fn toCharFormat_0(self , rsthis: & QTextFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat12toCharFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:343
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextListFormat toListFormat() const

/*
Returns this format as a list format.
*/
impl /*struct*/ QTextFormat {
  pub fn toListFormat_0<RetType, T: QTextFormat_toListFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toListFormat_0(self);
    // return 1;
  }
}
pub trait QTextFormat_toListFormat_0<RetType> {
  fn toListFormat_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_toListFormat_0<usize> for () {
  fn toListFormat_0(self , rsthis: & QTextFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat12toListFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:344
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextTableFormat toTableFormat() const

/*
Returns this format as a table format.
*/
impl /*struct*/ QTextFormat {
  pub fn toTableFormat_0<RetType, T: QTextFormat_toTableFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toTableFormat_0(self);
    // return 1;
  }
}
pub trait QTextFormat_toTableFormat_0<RetType> {
  fn toTableFormat_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_toTableFormat_0<usize> for () {
  fn toTableFormat_0(self , rsthis: & QTextFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat13toTableFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:345
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextFrameFormat toFrameFormat() const

/*
Returns this format as a frame format.
*/
impl /*struct*/ QTextFormat {
  pub fn toFrameFormat_0<RetType, T: QTextFormat_toFrameFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toFrameFormat_0(self);
    // return 1;
  }
}
pub trait QTextFormat_toFrameFormat_0<RetType> {
  fn toFrameFormat_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_toFrameFormat_0<usize> for () {
  fn toFrameFormat_0(self , rsthis: & QTextFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat13toFrameFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:346
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextImageFormat toImageFormat() const

/*
Returns this format as an image format.
*/
impl /*struct*/ QTextFormat {
  pub fn toImageFormat_0<RetType, T: QTextFormat_toImageFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toImageFormat_0(self);
    // return 1;
  }
}
pub trait QTextFormat_toImageFormat_0<RetType> {
  fn toImageFormat_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_toImageFormat_0<usize> for () {
  fn toImageFormat_0(self , rsthis: & QTextFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat13toImageFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:347
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextTableCellFormat toTableCellFormat() const

/*
Returns this format as a table cell format.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QTextFormat {
  pub fn toTableCellFormat_0<RetType, T: QTextFormat_toTableCellFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toTableCellFormat_0(self);
    // return 1;
  }
}
pub trait QTextFormat_toTableCellFormat_0<RetType> {
  fn toTableCellFormat_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_toTableCellFormat_0<usize> for () {
  fn toTableCellFormat_0(self , rsthis: & QTextFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat17toTableCellFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:349
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QTextFormat &) const

/*

*/
impl /*struct*/ QTextFormat {
  pub fn operator_equal_equal_0<RetType, T: QTextFormat_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QTextFormat_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QTextFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormateqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:350
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QTextFormat &) const

/*

*/
impl /*struct*/ QTextFormat {
  pub fn operator_not_equal_0<RetType, T: QTextFormat_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QTextFormat_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QTextFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormatneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:353
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setLayoutDirection(Qt::LayoutDirection)

/*
Sets the document's layout direction to the specified direction.

See also layoutDirection().
*/
impl /*struct*/ QTextFormat {
  pub fn setLayoutDirection_0<RetType, T: QTextFormat_setLayoutDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLayoutDirection_0(self);
    // return 1;
  }
}
pub trait QTextFormat_setLayoutDirection_0<RetType> {
  fn setLayoutDirection_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_setLayoutDirection_0<(/*void*/)> for (i32) {
  fn setLayoutDirection_0(self , rsthis: & QTextFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextFormat18setLayoutDirectionEN2Qt15LayoutDirectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:355
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::LayoutDirection layoutDirection() const

/*
Returns the document's layout direction.

See also setLayoutDirection().
*/
impl /*struct*/ QTextFormat {
  pub fn layoutDirection_0<RetType, T: QTextFormat_layoutDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.layoutDirection_0(self);
    // return 1;
  }
}
pub trait QTextFormat_layoutDirection_0<RetType> {
  fn layoutDirection_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_layoutDirection_0<i32> for () {
  fn layoutDirection_0(self , rsthis: & QTextFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat15layoutDirectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:358
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBackground(const QBrush &)

/*
Sets the brush use to paint the document's background to the brush specified.

See also background(), clearBackground(), and setForeground().
*/
impl /*struct*/ QTextFormat {
  pub fn setBackground_0<RetType, T: QTextFormat_setBackground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBackground_0(self);
    // return 1;
  }
}
pub trait QTextFormat_setBackground_0<RetType> {
  fn setBackground_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_setBackground_0<(/*void*/)> for (usize) {
  fn setBackground_0(self , rsthis: & QTextFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextFormat13setBackgroundERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:360
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QBrush background() const

/*
Returns the brush used to paint the document's background.

See also setBackground(), clearBackground(), and foreground().
*/
impl /*struct*/ QTextFormat {
  pub fn background_0<RetType, T: QTextFormat_background_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.background_0(self);
    // return 1;
  }
}
pub trait QTextFormat_background_0<RetType> {
  fn background_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_background_0<usize> for () {
  fn background_0(self , rsthis: & QTextFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat10backgroundEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:362
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void clearBackground()

/*
Clears the brush used to paint the document's background. The default brush will be used.

See also background(), setBackground(), and clearForeground().
*/
impl /*struct*/ QTextFormat {
  pub fn clearBackground_0<RetType, T: QTextFormat_clearBackground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearBackground_0(self);
    // return 1;
  }
}
pub trait QTextFormat_clearBackground_0<RetType> {
  fn clearBackground_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_clearBackground_0<(/*void*/)> for () {
  fn clearBackground_0(self , rsthis: & QTextFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextFormat15clearBackgroundEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:365
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setForeground(const QBrush &)

/*
Sets the foreground brush to the specified brush. The foreground brush is mostly used to render text.

See also foreground(), clearForeground(), and setBackground().
*/
impl /*struct*/ QTextFormat {
  pub fn setForeground_0<RetType, T: QTextFormat_setForeground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setForeground_0(self);
    // return 1;
  }
}
pub trait QTextFormat_setForeground_0<RetType> {
  fn setForeground_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_setForeground_0<(/*void*/)> for (usize) {
  fn setForeground_0(self , rsthis: & QTextFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextFormat13setForegroundERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:367
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QBrush foreground() const

/*
Returns the brush used to render foreground details, such as text, frame outlines, and table borders.

See also setForeground(), clearForeground(), and background().
*/
impl /*struct*/ QTextFormat {
  pub fn foreground_0<RetType, T: QTextFormat_foreground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.foreground_0(self);
    // return 1;
  }
}
pub trait QTextFormat_foreground_0<RetType> {
  fn foreground_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_foreground_0<usize> for () {
  fn foreground_0(self , rsthis: & QTextFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextFormat10foregroundEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:369
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void clearForeground()

/*
Clears the brush used to paint the document's foreground. The default brush will be used.

See also foreground(), setForeground(), and clearBackground().
*/
impl /*struct*/ QTextFormat {
  pub fn clearForeground_0<RetType, T: QTextFormat_clearForeground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearForeground_0(self);
    // return 1;
  }
}
pub trait QTextFormat_clearForeground_0<RetType> {
  fn clearForeground_0(self , rsthis: & QTextFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFormat_clearForeground_0<(/*void*/)> for () {
  fn clearForeground_0(self , rsthis: & QTextFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextFormat15clearForegroundEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum describes the text item a QTextFormat object is formatting.



See also QTextCharFormat, QTextBlockFormat, QTextListFormat, QTextTableFormat, and type().

*/
pub type QTextFormat__FormatType = i32;
// 
pub const QTextFormat__InvalidFormat :QTextFormat__FormatType = -1;
// The object formats a text block
pub const QTextFormat__BlockFormat :QTextFormat__FormatType = 1;
// The object formats a single character
pub const QTextFormat__CharFormat :QTextFormat__FormatType = 2;
// The object formats a list Unused Value, a table's FormatType is FrameFormat.
pub const QTextFormat__ListFormat :QTextFormat__FormatType = 3;
// 
pub const QTextFormat__TableFormat :QTextFormat__FormatType = 4;
// The object formats a frame
pub const QTextFormat__FrameFormat :QTextFormat__FormatType = 5;
// 
pub const QTextFormat__UserFormat :QTextFormat__FormatType = 100;
pub fn QTextFormat_FormatTypeItemName(val: i32) ->String {
  match val {
     QTextFormat__InvalidFormat => // -1
     {return String::from("InvalidFormat");}
     QTextFormat__BlockFormat => // 1
     {return String::from("BlockFormat");}
     QTextFormat__CharFormat => // 2
     {return String::from("CharFormat");}
     QTextFormat__ListFormat => // 3
     {return String::from("ListFormat");}
     QTextFormat__TableFormat => // 4
     {return String::from("TableFormat");}
     QTextFormat__FrameFormat => // 5
     {return String::from("FrameFormat");}
     QTextFormat__UserFormat => // 100
     {return String::from("UserFormat");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextFormat_FormatTypeItemName_s(val: i32) ->String {
  //var nilthis *QTextFormat
  //return nilthis.FormatTypeItemName(val);
  return QTextFormat_FormatTypeItemName(val);
}


/*
This enum describes the different properties a format can have.



Paragraph and character properties



Paragraph properties



Character properties

QTextFormat::FontCapitalizationFirstFontPropertySpecifies the capitalization type that is to be applied to the text.


List properties



Table and frame properties



Table cell properties

ConstantValue


Image properties

ConstantValue


Selection properties



Page break properties



See also property() and setProperty().

*/
pub type QTextFormat__Property = i32;
// 
pub const QTextFormat__ObjectIndex :QTextFormat__Property = 0;
// 
pub const QTextFormat__CssFloat :QTextFormat__Property = 2048;
// 
pub const QTextFormat__LayoutDirection :QTextFormat__Property = 2049;
// 
pub const QTextFormat__OutlinePen :QTextFormat__Property = 2064;
// 
pub const QTextFormat__BackgroundBrush :QTextFormat__Property = 2080;
// 
pub const QTextFormat__ForegroundBrush :QTextFormat__Property = 2081;
// 
pub const QTextFormat__BackgroundImageUrl :QTextFormat__Property = 2083;
// 
pub const QTextFormat__BlockAlignment :QTextFormat__Property = 4112;
// 
pub const QTextFormat__BlockTopMargin :QTextFormat__Property = 4144;
// 
pub const QTextFormat__BlockBottomMargin :QTextFormat__Property = 4145;
// 
pub const QTextFormat__BlockLeftMargin :QTextFormat__Property = 4146;
// 
pub const QTextFormat__BlockRightMargin :QTextFormat__Property = 4147;
// 
pub const QTextFormat__TextIndent :QTextFormat__Property = 4148;
// 
pub const QTextFormat__TabPositions :QTextFormat__Property = 4149;
// 
pub const QTextFormat__BlockIndent :QTextFormat__Property = 4160;
// 
pub const QTextFormat__LineHeight :QTextFormat__Property = 4168;
// 
pub const QTextFormat__LineHeightType :QTextFormat__Property = 4169;
// 
pub const QTextFormat__BlockNonBreakableLines :QTextFormat__Property = 4176;
// 
pub const QTextFormat__BlockTrailingHorizontalRulerWidth :QTextFormat__Property = 4192;
// 
pub const QTextFormat__FirstFontProperty :QTextFormat__Property = 8160;
// 
pub const QTextFormat__FontCapitalization :QTextFormat__Property = 8160;
// 
pub const QTextFormat__FontLetterSpacingType :QTextFormat__Property = 8243;
// 
pub const QTextFormat__FontLetterSpacing :QTextFormat__Property = 8161;
// 
pub const QTextFormat__FontWordSpacing :QTextFormat__Property = 8162;
// 
pub const QTextFormat__FontStretch :QTextFormat__Property = 8244;
// 
pub const QTextFormat__FontStyleHint :QTextFormat__Property = 8163;
// 
pub const QTextFormat__FontStyleStrategy :QTextFormat__Property = 8164;
// 
pub const QTextFormat__FontKerning :QTextFormat__Property = 8165;
// 
pub const QTextFormat__FontHintingPreference :QTextFormat__Property = 8166;
// 
pub const QTextFormat__FontFamily :QTextFormat__Property = 8192;
// 
pub const QTextFormat__FontPointSize :QTextFormat__Property = 8193;
// 
pub const QTextFormat__FontSizeAdjustment :QTextFormat__Property = 8194;
// 
pub const QTextFormat__FontSizeIncrement :QTextFormat__Property = 8194;
// 
pub const QTextFormat__FontWeight :QTextFormat__Property = 8195;
// 
pub const QTextFormat__FontItalic :QTextFormat__Property = 8196;
// 
pub const QTextFormat__FontUnderline :QTextFormat__Property = 8197;
// 
pub const QTextFormat__FontOverline :QTextFormat__Property = 8198;
// 
pub const QTextFormat__FontStrikeOut :QTextFormat__Property = 8199;
// 
pub const QTextFormat__FontFixedPitch :QTextFormat__Property = 8200;
// 
pub const QTextFormat__FontPixelSize :QTextFormat__Property = 8201;
// 
pub const QTextFormat__LastFontProperty :QTextFormat__Property = 8201;
// 
pub const QTextFormat__TextUnderlineColor :QTextFormat__Property = 8208;
// 
pub const QTextFormat__TextVerticalAlignment :QTextFormat__Property = 8225;
// 
pub const QTextFormat__TextOutline :QTextFormat__Property = 8226;
// 
pub const QTextFormat__TextUnderlineStyle :QTextFormat__Property = 8227;
// 
pub const QTextFormat__TextToolTip :QTextFormat__Property = 8228;
// 
pub const QTextFormat__IsAnchor :QTextFormat__Property = 8240;
// 
pub const QTextFormat__AnchorHref :QTextFormat__Property = 8241;
// 
pub const QTextFormat__AnchorName :QTextFormat__Property = 8242;
// 
pub const QTextFormat__ObjectType :QTextFormat__Property = 12032;
// 
pub const QTextFormat__ListStyle :QTextFormat__Property = 12288;
// 
pub const QTextFormat__ListIndent :QTextFormat__Property = 12289;
// 
pub const QTextFormat__ListNumberPrefix :QTextFormat__Property = 12290;
// 
pub const QTextFormat__ListNumberSuffix :QTextFormat__Property = 12291;
// 
pub const QTextFormat__FrameBorder :QTextFormat__Property = 16384;
// 
pub const QTextFormat__FrameMargin :QTextFormat__Property = 16385;
// 
pub const QTextFormat__FramePadding :QTextFormat__Property = 16386;
// 
pub const QTextFormat__FrameWidth :QTextFormat__Property = 16387;
// 
pub const QTextFormat__FrameHeight :QTextFormat__Property = 16388;
// 
pub const QTextFormat__FrameTopMargin :QTextFormat__Property = 16389;
// 
pub const QTextFormat__FrameBottomMargin :QTextFormat__Property = 16390;
// 
pub const QTextFormat__FrameLeftMargin :QTextFormat__Property = 16391;
// 
pub const QTextFormat__FrameRightMargin :QTextFormat__Property = 16392;
// 
pub const QTextFormat__FrameBorderBrush :QTextFormat__Property = 16393;
// 
pub const QTextFormat__FrameBorderStyle :QTextFormat__Property = 16400;
// 
pub const QTextFormat__TableColumns :QTextFormat__Property = 16640;
// 
pub const QTextFormat__TableColumnWidthConstraints :QTextFormat__Property = 16641;
// 
pub const QTextFormat__TableCellSpacing :QTextFormat__Property = 16642;
// 
pub const QTextFormat__TableCellPadding :QTextFormat__Property = 16643;
// 
pub const QTextFormat__TableHeaderRowCount :QTextFormat__Property = 16644;
// 
pub const QTextFormat__TableCellRowSpan :QTextFormat__Property = 18448;
// 
pub const QTextFormat__TableCellColumnSpan :QTextFormat__Property = 18449;
// 
pub const QTextFormat__TableCellTopPadding :QTextFormat__Property = 18450;
// 
pub const QTextFormat__TableCellBottomPadding :QTextFormat__Property = 18451;
// 
pub const QTextFormat__TableCellLeftPadding :QTextFormat__Property = 18452;
// 
pub const QTextFormat__TableCellRightPadding :QTextFormat__Property = 18453;
// 
pub const QTextFormat__ImageName :QTextFormat__Property = 20480;
// 
pub const QTextFormat__ImageWidth :QTextFormat__Property = 20496;
// 
pub const QTextFormat__ImageHeight :QTextFormat__Property = 20497;
// 
pub const QTextFormat__FullWidthSelection :QTextFormat__Property = 24576;
// 
pub const QTextFormat__PageBreakPolicy :QTextFormat__Property = 28672;
// 
pub const QTextFormat__UserProperty :QTextFormat__Property = 1048576;
pub fn QTextFormat_PropertyItemName(val: i32) ->String {
  match val {
     QTextFormat__ObjectIndex => // 0
     {return String::from("ObjectIndex");}
     QTextFormat__CssFloat => // 2048
     {return String::from("CssFloat");}
     QTextFormat__LayoutDirection => // 2049
     {return String::from("LayoutDirection");}
     QTextFormat__OutlinePen => // 2064
     {return String::from("OutlinePen");}
     QTextFormat__BackgroundBrush => // 2080
     {return String::from("BackgroundBrush");}
     QTextFormat__ForegroundBrush => // 2081
     {return String::from("ForegroundBrush");}
     QTextFormat__BackgroundImageUrl => // 2083
     {return String::from("BackgroundImageUrl");}
     QTextFormat__BlockAlignment => // 4112
     {return String::from("BlockAlignment");}
     QTextFormat__BlockTopMargin => // 4144
     {return String::from("BlockTopMargin");}
     QTextFormat__BlockBottomMargin => // 4145
     {return String::from("BlockBottomMargin");}
     QTextFormat__BlockLeftMargin => // 4146
     {return String::from("BlockLeftMargin");}
     QTextFormat__BlockRightMargin => // 4147
     {return String::from("BlockRightMargin");}
     QTextFormat__TextIndent => // 4148
     {return String::from("TextIndent");}
     QTextFormat__TabPositions => // 4149
     {return String::from("TabPositions");}
     QTextFormat__BlockIndent => // 4160
     {return String::from("BlockIndent");}
     QTextFormat__LineHeight => // 4168
     {return String::from("LineHeight");}
     QTextFormat__LineHeightType => // 4169
     {return String::from("LineHeightType");}
     QTextFormat__BlockNonBreakableLines => // 4176
     {return String::from("BlockNonBreakableLines");}
     QTextFormat__BlockTrailingHorizontalRulerWidth => // 4192
     {return String::from("BlockTrailingHorizontalRulerWidth");}
     QTextFormat__FirstFontProperty => // 8160
     {return String::from("FirstFontProperty,FontCapitalization");}
    // QTextFormat__FontCapitalization => // 8160
    // {return String::from("");}
     QTextFormat__FontLetterSpacingType => // 8243
     {return String::from("FontLetterSpacingType");}
     QTextFormat__FontLetterSpacing => // 8161
     {return String::from("FontLetterSpacing");}
     QTextFormat__FontWordSpacing => // 8162
     {return String::from("FontWordSpacing");}
     QTextFormat__FontStretch => // 8244
     {return String::from("FontStretch");}
     QTextFormat__FontStyleHint => // 8163
     {return String::from("FontStyleHint");}
     QTextFormat__FontStyleStrategy => // 8164
     {return String::from("FontStyleStrategy");}
     QTextFormat__FontKerning => // 8165
     {return String::from("FontKerning");}
     QTextFormat__FontHintingPreference => // 8166
     {return String::from("FontHintingPreference");}
     QTextFormat__FontFamily => // 8192
     {return String::from("FontFamily");}
     QTextFormat__FontPointSize => // 8193
     {return String::from("FontPointSize");}
     QTextFormat__FontSizeAdjustment => // 8194
     {return String::from("FontSizeAdjustment,FontSizeIncrement");}
    // QTextFormat__FontSizeIncrement => // 8194
    // {return String::from("");}
     QTextFormat__FontWeight => // 8195
     {return String::from("FontWeight");}
     QTextFormat__FontItalic => // 8196
     {return String::from("FontItalic");}
     QTextFormat__FontUnderline => // 8197
     {return String::from("FontUnderline");}
     QTextFormat__FontOverline => // 8198
     {return String::from("FontOverline");}
     QTextFormat__FontStrikeOut => // 8199
     {return String::from("FontStrikeOut");}
     QTextFormat__FontFixedPitch => // 8200
     {return String::from("FontFixedPitch");}
     QTextFormat__FontPixelSize => // 8201
     {return String::from("FontPixelSize,LastFontProperty");}
    // QTextFormat__LastFontProperty => // 8201
    // {return String::from("");}
     QTextFormat__TextUnderlineColor => // 8208
     {return String::from("TextUnderlineColor");}
     QTextFormat__TextVerticalAlignment => // 8225
     {return String::from("TextVerticalAlignment");}
     QTextFormat__TextOutline => // 8226
     {return String::from("TextOutline");}
     QTextFormat__TextUnderlineStyle => // 8227
     {return String::from("TextUnderlineStyle");}
     QTextFormat__TextToolTip => // 8228
     {return String::from("TextToolTip");}
     QTextFormat__IsAnchor => // 8240
     {return String::from("IsAnchor");}
     QTextFormat__AnchorHref => // 8241
     {return String::from("AnchorHref");}
     QTextFormat__AnchorName => // 8242
     {return String::from("AnchorName");}
     QTextFormat__ObjectType => // 12032
     {return String::from("ObjectType");}
     QTextFormat__ListStyle => // 12288
     {return String::from("ListStyle");}
     QTextFormat__ListIndent => // 12289
     {return String::from("ListIndent");}
     QTextFormat__ListNumberPrefix => // 12290
     {return String::from("ListNumberPrefix");}
     QTextFormat__ListNumberSuffix => // 12291
     {return String::from("ListNumberSuffix");}
     QTextFormat__FrameBorder => // 16384
     {return String::from("FrameBorder");}
     QTextFormat__FrameMargin => // 16385
     {return String::from("FrameMargin");}
     QTextFormat__FramePadding => // 16386
     {return String::from("FramePadding");}
     QTextFormat__FrameWidth => // 16387
     {return String::from("FrameWidth");}
     QTextFormat__FrameHeight => // 16388
     {return String::from("FrameHeight");}
     QTextFormat__FrameTopMargin => // 16389
     {return String::from("FrameTopMargin");}
     QTextFormat__FrameBottomMargin => // 16390
     {return String::from("FrameBottomMargin");}
     QTextFormat__FrameLeftMargin => // 16391
     {return String::from("FrameLeftMargin");}
     QTextFormat__FrameRightMargin => // 16392
     {return String::from("FrameRightMargin");}
     QTextFormat__FrameBorderBrush => // 16393
     {return String::from("FrameBorderBrush");}
     QTextFormat__FrameBorderStyle => // 16400
     {return String::from("FrameBorderStyle");}
     QTextFormat__TableColumns => // 16640
     {return String::from("TableColumns");}
     QTextFormat__TableColumnWidthConstraints => // 16641
     {return String::from("TableColumnWidthConstraints");}
     QTextFormat__TableCellSpacing => // 16642
     {return String::from("TableCellSpacing");}
     QTextFormat__TableCellPadding => // 16643
     {return String::from("TableCellPadding");}
     QTextFormat__TableHeaderRowCount => // 16644
     {return String::from("TableHeaderRowCount");}
     QTextFormat__TableCellRowSpan => // 18448
     {return String::from("TableCellRowSpan");}
     QTextFormat__TableCellColumnSpan => // 18449
     {return String::from("TableCellColumnSpan");}
     QTextFormat__TableCellTopPadding => // 18450
     {return String::from("TableCellTopPadding");}
     QTextFormat__TableCellBottomPadding => // 18451
     {return String::from("TableCellBottomPadding");}
     QTextFormat__TableCellLeftPadding => // 18452
     {return String::from("TableCellLeftPadding");}
     QTextFormat__TableCellRightPadding => // 18453
     {return String::from("TableCellRightPadding");}
     QTextFormat__ImageName => // 20480
     {return String::from("ImageName");}
     QTextFormat__ImageWidth => // 20496
     {return String::from("ImageWidth");}
     QTextFormat__ImageHeight => // 20497
     {return String::from("ImageHeight");}
     QTextFormat__FullWidthSelection => // 24576
     {return String::from("FullWidthSelection");}
     QTextFormat__PageBreakPolicy => // 28672
     {return String::from("PageBreakPolicy");}
     QTextFormat__UserProperty => // 1048576
     {return String::from("UserProperty");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextFormat_PropertyItemName_s(val: i32) ->String {
  //var nilthis *QTextFormat
  //return nilthis.PropertyItemName(val);
  return QTextFormat_PropertyItemName(val);
}


/*
This enum describes what kind of QTextObject this format is associated with.



See also QTextObject, QTextTable, and QTextObject::format().

*/
pub type QTextFormat__ObjectTypes = i32;
//  
pub const QTextFormat__NoObject :QTextFormat__ObjectTypes = 0;
//  
pub const QTextFormat__ImageObject :QTextFormat__ObjectTypes = 1;
//  
pub const QTextFormat__TableObject :QTextFormat__ObjectTypes = 2;
//  
pub const QTextFormat__TableCellObject :QTextFormat__ObjectTypes = 3;
// 
pub const QTextFormat__UserObject :QTextFormat__ObjectTypes = 4096;
pub fn QTextFormat_ObjectTypesItemName(val: i32) ->String {
  match val {
     QTextFormat__NoObject => // 0
     {return String::from("NoObject");}
     QTextFormat__ImageObject => // 1
     {return String::from("ImageObject");}
     QTextFormat__TableObject => // 2
     {return String::from("TableObject");}
     QTextFormat__TableCellObject => // 3
     {return String::from("TableCellObject");}
     QTextFormat__UserObject => // 4096
     {return String::from("UserObject");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextFormat_ObjectTypesItemName_s(val: i32) ->String {
  //var nilthis *QTextFormat
  //return nilthis.ObjectTypesItemName(val);
  return QTextFormat_ObjectTypesItemName(val);
}


/*


*/
pub type QTextFormat__PageBreakFlag = i32;
// 
pub const QTextFormat__PageBreak_Auto :QTextFormat__PageBreakFlag = 0;
// 
pub const QTextFormat__PageBreak_AlwaysBefore :QTextFormat__PageBreakFlag = 1;
// 
pub const QTextFormat__PageBreak_AlwaysAfter :QTextFormat__PageBreakFlag = 16;
pub fn QTextFormat_PageBreakFlagItemName(val: i32) ->String {
  match val {
     QTextFormat__PageBreak_Auto => // 0
     {return String::from("PageBreak_Auto");}
     QTextFormat__PageBreak_AlwaysBefore => // 1
     {return String::from("PageBreak_AlwaysBefore");}
     QTextFormat__PageBreak_AlwaysAfter => // 16
     {return String::from("PageBreak_AlwaysAfter");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextFormat_PageBreakFlagItemName_s(val: i32) ->String {
  //var nilthis *QTextFormat
  //return nilthis.PageBreakFlagItemName(val);
  return QTextFormat_PageBreakFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
