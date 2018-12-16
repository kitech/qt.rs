

// mod ::widgets::QStyleOptionFocusRect
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
// extern C begin: 5
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
#[derive(Default)] // class sizeof(QStyleOptionFocusRect)=80
pub struct QStyleOptionFocusRect {
  qbase: QStyleOption,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionFocusRect_ITF interface {
//    QStyleOption_ITF
//    QStyleOptionFocusRect_PTR() *QStyleOptionFocusRect
//}
//func (ptr *QStyleOptionFocusRect) QStyleOptionFocusRect_PTR() *QStyleOptionFocusRect { return ptr }

impl /*struct*/ QStyleOptionFocusRect {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionFocusRect {
    return QStyleOptionFocusRect{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionFocusRect {
//  type Target = QStyleOptionFocusRectBASE;
//
//  fn deref(&self) -> &QStyleOptionFocusRectBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionFocusRectBASE> for QStyleOptionFocusRect {
//  fn as_ref(& self) -> & QStyleOptionFocusRectBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionFocusRect()

/*

*/
// QStyleOptionFocusRect() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionFocusRect {
  pub fn QStyleOptionFocusRect_0<T: QStyleOptionFocusRect_QStyleOptionFocusRect_0>(value: T) -> QStyleOptionFocusRect {
    let rsthis = value.QStyleOptionFocusRect_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionFocusRect_QStyleOptionFocusRect_0 {
  fn QStyleOptionFocusRect_0(self) -> QStyleOptionFocusRect;
}
// QStyleOptionFocusRect() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionFocusRect_QStyleOptionFocusRect_0 for () {
  fn QStyleOptionFocusRect_0(self) -> QStyleOptionFocusRect {
    // unsafe{_ZN21QStyleOptionFocusRectC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN21QStyleOptionFocusRectC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionFocusRect{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:123
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionFocusRect(int)

/*

*/
// QStyleOptionFocusRect(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionFocusRect {
  pub fn QStyleOptionFocusRect_1<T: QStyleOptionFocusRect_QStyleOptionFocusRect_1>(value: T) -> QStyleOptionFocusRect {
    let rsthis = value.QStyleOptionFocusRect_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionFocusRect_QStyleOptionFocusRect_1 {
  fn QStyleOptionFocusRect_1(self) -> QStyleOptionFocusRect;
}
// QStyleOptionFocusRect(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionFocusRect_QStyleOptionFocusRect_1 for (i32) {
  fn QStyleOptionFocusRect_1(self) -> QStyleOptionFocusRect {
    // unsafe{_ZN21QStyleOptionFocusRectC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN21QStyleOptionFocusRectC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionFocusRect{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionFocusRect(this :*mut QStyleOptionFocusRect) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN21QStyleOptionFocusRectD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionFocusRect__StyleOptionType = i32;
// 
pub const QStyleOptionFocusRect__Type :QStyleOptionFocusRect__StyleOptionType = 1;
pub fn QStyleOptionFocusRect_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionFocusRect__Type => // 1
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionFocusRect_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionFocusRect
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionFocusRect_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionFocusRect__StyleOptionVersion = i32;
// 1
pub const QStyleOptionFocusRect__Version :QStyleOptionFocusRect__StyleOptionVersion = 1;
pub fn QStyleOptionFocusRect_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionFocusRect__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionFocusRect_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionFocusRect
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionFocusRect_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
