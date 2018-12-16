

// mod ::widgets::QStyleOptionTitleBar
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
#[derive(Default)] // class sizeof(QStyleOptionTitleBar)=96
pub struct QStyleOptionTitleBar {
  qbase: QStyleOptionComplex,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionTitleBar_ITF interface {
//    QStyleOptionComplex_ITF
//    QStyleOptionTitleBar_PTR() *QStyleOptionTitleBar
//}
//func (ptr *QStyleOptionTitleBar) QStyleOptionTitleBar_PTR() *QStyleOptionTitleBar { return ptr }

impl /*struct*/ QStyleOptionTitleBar {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionTitleBar {
    return QStyleOptionTitleBar{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionTitleBar {
//  type Target = QStyleOptionTitleBarBASE;
//
//  fn deref(&self) -> &QStyleOptionTitleBarBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionTitleBarBASE> for QStyleOptionTitleBar {
//  fn as_ref(& self) -> & QStyleOptionTitleBarBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:619
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionTitleBar()

/*

*/
// QStyleOptionTitleBar() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionTitleBar {
  pub fn QStyleOptionTitleBar_0<T: QStyleOptionTitleBar_QStyleOptionTitleBar_0>(value: T) -> QStyleOptionTitleBar {
    let rsthis = value.QStyleOptionTitleBar_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTitleBar_QStyleOptionTitleBar_0 {
  fn QStyleOptionTitleBar_0(self) -> QStyleOptionTitleBar;
}
// QStyleOptionTitleBar() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionTitleBar_QStyleOptionTitleBar_0 for () {
  fn QStyleOptionTitleBar_0(self) -> QStyleOptionTitleBar {
    // unsafe{_ZN20QStyleOptionTitleBarC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QStyleOptionTitleBarC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionTitleBar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:623
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionTitleBar(int)

/*

*/
// QStyleOptionTitleBar(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionTitleBar {
  pub fn QStyleOptionTitleBar_1<T: QStyleOptionTitleBar_QStyleOptionTitleBar_1>(value: T) -> QStyleOptionTitleBar {
    let rsthis = value.QStyleOptionTitleBar_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTitleBar_QStyleOptionTitleBar_1 {
  fn QStyleOptionTitleBar_1(self) -> QStyleOptionTitleBar;
}
// QStyleOptionTitleBar(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionTitleBar_QStyleOptionTitleBar_1 for (i32) {
  fn QStyleOptionTitleBar_1(self) -> QStyleOptionTitleBar {
    // unsafe{_ZN20QStyleOptionTitleBarC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QStyleOptionTitleBarC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionTitleBar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionTitleBar(this :*mut QStyleOptionTitleBar) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN20QStyleOptionTitleBarD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionTitleBar__StyleOptionType = i32;
// 
pub const QStyleOptionTitleBar__Type :QStyleOptionTitleBar__StyleOptionType = 983045;
pub fn QStyleOptionTitleBar_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionTitleBar__Type => // 983045
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionTitleBar_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionTitleBar
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionTitleBar_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionTitleBar__StyleOptionVersion = i32;
// 1
pub const QStyleOptionTitleBar__Version :QStyleOptionTitleBar__StyleOptionVersion = 1;
pub fn QStyleOptionTitleBar_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionTitleBar__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionTitleBar_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionTitleBar
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionTitleBar_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
