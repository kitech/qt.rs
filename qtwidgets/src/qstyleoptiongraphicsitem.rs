

// mod ::widgets::QStyleOptionGraphicsItem
// package qtwidgets
// /usr/include/qt/QtWidgets/qstyleoption.h
// #include <qstyleoption.h>
// #include <QtWidgets>

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
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QStyleOptionGraphicsItem)=152
pub struct QStyleOptionGraphicsItem {
  qbase: QStyleOption,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionGraphicsItem_ITF interface {
//    QStyleOption_ITF
//    QStyleOptionGraphicsItem_PTR() *QStyleOptionGraphicsItem
//}
//func (ptr *QStyleOptionGraphicsItem) QStyleOptionGraphicsItem_PTR() *QStyleOptionGraphicsItem { return ptr }

impl /*struct*/ QStyleOptionGraphicsItem {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionGraphicsItem {
    return QStyleOptionGraphicsItem{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionGraphicsItem {
//  type Target = QStyleOptionGraphicsItemBASE;
//
//  fn deref(&self) -> &QStyleOptionGraphicsItemBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionGraphicsItemBASE> for QStyleOptionGraphicsItem {
//  fn as_ref(& self) -> & QStyleOptionGraphicsItemBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:669
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionGraphicsItem()

/*

*/
// QStyleOptionGraphicsItem() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionGraphicsItem {
  pub fn QStyleOptionGraphicsItem_0<T: QStyleOptionGraphicsItem_QStyleOptionGraphicsItem_0>(value: T) -> QStyleOptionGraphicsItem {
    let rsthis = value.QStyleOptionGraphicsItem_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionGraphicsItem_QStyleOptionGraphicsItem_0 {
  fn QStyleOptionGraphicsItem_0(self) -> QStyleOptionGraphicsItem;
}
// QStyleOptionGraphicsItem() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionGraphicsItem_QStyleOptionGraphicsItem_0 for () {
  fn QStyleOptionGraphicsItem_0(self) -> QStyleOptionGraphicsItem {
    // unsafe{_ZN24QStyleOptionGraphicsItemC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN24QStyleOptionGraphicsItemC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionGraphicsItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:673
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionGraphicsItem(int)

/*

*/
// QStyleOptionGraphicsItem(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionGraphicsItem {
  pub fn QStyleOptionGraphicsItem_1<T: QStyleOptionGraphicsItem_QStyleOptionGraphicsItem_1>(value: T) -> QStyleOptionGraphicsItem {
    let rsthis = value.QStyleOptionGraphicsItem_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionGraphicsItem_QStyleOptionGraphicsItem_1 {
  fn QStyleOptionGraphicsItem_1(self) -> QStyleOptionGraphicsItem;
}
// QStyleOptionGraphicsItem(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionGraphicsItem_QStyleOptionGraphicsItem_1 for (i32) {
  fn QStyleOptionGraphicsItem_1(self) -> QStyleOptionGraphicsItem {
    // unsafe{_ZN24QStyleOptionGraphicsItemC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN24QStyleOptionGraphicsItemC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionGraphicsItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:671
// index:0
// Public static Visibility=Default Availability=Available
// [8] qreal levelOfDetailFromTransform(const QTransform &)

/*

*/
impl /*struct*/ QStyleOptionGraphicsItem {
  pub fn levelOfDetailFromTransform_0<RetType, T: QStyleOptionGraphicsItem_levelOfDetailFromTransform_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.levelOfDetailFromTransform_0();
    // return 1;
  }
}
pub trait QStyleOptionGraphicsItem_levelOfDetailFromTransform_0<RetType> {
  fn levelOfDetailFromTransform_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStyleOptionGraphicsItem_levelOfDetailFromTransform_0<f64> for (usize) {
  fn levelOfDetailFromTransform_0(self ) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN24QStyleOptionGraphicsItem26levelOfDetailFromTransformERK10QTransform", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}


pub fn DeleteQStyleOptionGraphicsItem(this :*mut QStyleOptionGraphicsItem) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN24QStyleOptionGraphicsItemD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionGraphicsItem__StyleOptionType = i32;
// 
pub const QStyleOptionGraphicsItem__Type :QStyleOptionGraphicsItem__StyleOptionType = 15;
pub fn QStyleOptionGraphicsItem_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionGraphicsItem__Type => // 15
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionGraphicsItem_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionGraphicsItem
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionGraphicsItem_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionGraphicsItem__StyleOptionVersion = i32;
// 1
pub const QStyleOptionGraphicsItem__Version :QStyleOptionGraphicsItem__StyleOptionVersion = 1;
pub fn QStyleOptionGraphicsItem_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionGraphicsItem__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionGraphicsItem_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionGraphicsItem
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionGraphicsItem_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
