

// mod ::core::QXmlStreamAttributes
// package qtcore
// /usr/include/qt/QtCore/qxmlstream.h
// #include <qxmlstream.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 14
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
#[derive(Default)] // class sizeof(QXmlStreamAttributes)=8
pub struct QXmlStreamAttributes {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QXmlStreamAttributes_ITF interface {
//    QXmlStreamAttributes_PTR() *QXmlStreamAttributes
//}
//func (ptr *QXmlStreamAttributes) QXmlStreamAttributes_PTR() *QXmlStreamAttributes { return ptr }

impl /*struct*/ QXmlStreamAttributes {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QXmlStreamAttributes {
    return QXmlStreamAttributes{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QXmlStreamAttributes {
//  type Target = QXmlStreamAttributesBASE;
//
//  fn deref(&self) -> &QXmlStreamAttributesBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QXmlStreamAttributesBASE> for QXmlStreamAttributes {
//  fn as_ref(& self) -> & QXmlStreamAttributesBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qxmlstream.h:164
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QXmlStreamAttributes()

/*

*/
// QXmlStreamAttributes() ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamAttributes {
  pub fn QXmlStreamAttributes_0<T: QXmlStreamAttributes_QXmlStreamAttributes_0>(value: T) -> QXmlStreamAttributes {
    let rsthis = value.QXmlStreamAttributes_0();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamAttributes_QXmlStreamAttributes_0 {
  fn QXmlStreamAttributes_0(self) -> QXmlStreamAttributes;
}
// QXmlStreamAttributes() ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamAttributes_QXmlStreamAttributes_0 for () {
  fn QXmlStreamAttributes_0(self) -> QXmlStreamAttributes {
    // unsafe{_ZN20QXmlStreamAttributesC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QXmlStreamAttributesC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamAttributes{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:165
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef value(const QString &, const QString &) const

/*

*/
impl /*struct*/ QXmlStreamAttributes {
  pub fn value_0<RetType, T: QXmlStreamAttributes_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QXmlStreamAttributes_value_0<RetType> {
  fn value_0(self , rsthis: & QXmlStreamAttributes) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttributes_value_0<usize> for (usize,usize) {
  fn value_0(self , rsthis: & QXmlStreamAttributes) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QXmlStreamAttributes5valueERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:166
// index:1
// Public Visibility=Default Availability=Available
// [16] QStringRef value(const QString &, QLatin1String) const

/*

*/
impl /*struct*/ QXmlStreamAttributes {
  pub fn value_1<RetType, T: QXmlStreamAttributes_value_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_1(self);
    // return 1;
  }
}
pub trait QXmlStreamAttributes_value_1<RetType> {
  fn value_1(self , rsthis: & QXmlStreamAttributes) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttributes_value_1<usize> for (usize,usize) {
  fn value_1(self , rsthis: & QXmlStreamAttributes) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QXmlStreamAttributes5valueERK7QString13QLatin1String", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:167
// index:2
// Public Visibility=Default Availability=Available
// [16] QStringRef value(QLatin1String, QLatin1String) const

/*

*/
impl /*struct*/ QXmlStreamAttributes {
  pub fn value_2<RetType, T: QXmlStreamAttributes_value_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_2(self);
    // return 1;
  }
}
pub trait QXmlStreamAttributes_value_2<RetType> {
  fn value_2(self , rsthis: & QXmlStreamAttributes) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttributes_value_2<usize> for (usize,usize) {
  fn value_2(self , rsthis: & QXmlStreamAttributes) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QXmlStreamAttributes5valueE13QLatin1StringS0_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:168
// index:3
// Public Visibility=Default Availability=Available
// [16] QStringRef value(const QString &) const

/*

*/
impl /*struct*/ QXmlStreamAttributes {
  pub fn value_3<RetType, T: QXmlStreamAttributes_value_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_3(self);
    // return 1;
  }
}
pub trait QXmlStreamAttributes_value_3<RetType> {
  fn value_3(self , rsthis: & QXmlStreamAttributes) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttributes_value_3<usize> for (usize) {
  fn value_3(self , rsthis: & QXmlStreamAttributes) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QXmlStreamAttributes5valueERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:169
// index:4
// Public Visibility=Default Availability=Available
// [16] QStringRef value(QLatin1String) const

/*

*/
impl /*struct*/ QXmlStreamAttributes {
  pub fn value_4<RetType, T: QXmlStreamAttributes_value_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_4(self);
    // return 1;
  }
}
pub trait QXmlStreamAttributes_value_4<RetType> {
  fn value_4(self , rsthis: & QXmlStreamAttributes) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttributes_value_4<usize> for (usize) {
  fn value_4(self , rsthis: & QXmlStreamAttributes) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QXmlStreamAttributes5valueE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:173
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool hasAttribute(const QString &) const

/*

*/
impl /*struct*/ QXmlStreamAttributes {
  pub fn hasAttribute_0<RetType, T: QXmlStreamAttributes_hasAttribute_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasAttribute_0(self);
    // return 1;
  }
}
pub trait QXmlStreamAttributes_hasAttribute_0<RetType> {
  fn hasAttribute_0(self , rsthis: & QXmlStreamAttributes) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttributes_hasAttribute_0<bool> for (usize) {
  fn hasAttribute_0(self , rsthis: & QXmlStreamAttributes) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QXmlStreamAttributes12hasAttributeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:178
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool hasAttribute(QLatin1String) const

/*

*/
impl /*struct*/ QXmlStreamAttributes {
  pub fn hasAttribute_1<RetType, T: QXmlStreamAttributes_hasAttribute_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasAttribute_1(self);
    // return 1;
  }
}
pub trait QXmlStreamAttributes_hasAttribute_1<RetType> {
  fn hasAttribute_1(self , rsthis: & QXmlStreamAttributes) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttributes_hasAttribute_1<bool> for (usize) {
  fn hasAttribute_1(self , rsthis: & QXmlStreamAttributes) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QXmlStreamAttributes12hasAttributeE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:183
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool hasAttribute(const QString &, const QString &) const

/*

*/
impl /*struct*/ QXmlStreamAttributes {
  pub fn hasAttribute_2<RetType, T: QXmlStreamAttributes_hasAttribute_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasAttribute_2(self);
    // return 1;
  }
}
pub trait QXmlStreamAttributes_hasAttribute_2<RetType> {
  fn hasAttribute_2(self , rsthis: & QXmlStreamAttributes) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttributes_hasAttribute_2<bool> for (usize,usize) {
  fn hasAttribute_2(self , rsthis: & QXmlStreamAttributes) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QXmlStreamAttributes12hasAttributeERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


pub fn DeleteQXmlStreamAttributes(this :*mut QXmlStreamAttributes) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN20QXmlStreamAttributesD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
