

// mod ::widgets::QStyleOptionProgressBar
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
#[derive(Default)] // class sizeof(QStyleOptionProgressBar)=104
pub struct QStyleOptionProgressBar {
  qbase: QStyleOption,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionProgressBar_ITF interface {
//    QStyleOption_ITF
//    QStyleOptionProgressBar_PTR() *QStyleOptionProgressBar
//}
//func (ptr *QStyleOptionProgressBar) QStyleOptionProgressBar_PTR() *QStyleOptionProgressBar { return ptr }

impl /*struct*/ QStyleOptionProgressBar {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionProgressBar {
    return QStyleOptionProgressBar{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionProgressBar {
//  type Target = QStyleOptionProgressBarBASE;
//
//  fn deref(&self) -> &QStyleOptionProgressBarBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionProgressBarBASE> for QStyleOptionProgressBar {
//  fn as_ref(& self) -> & QStyleOptionProgressBarBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:342
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionProgressBar()

/*

*/
// QStyleOptionProgressBar() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionProgressBar {
  pub fn QStyleOptionProgressBar_0<T: QStyleOptionProgressBar_QStyleOptionProgressBar_0>(value: T) -> QStyleOptionProgressBar {
    let rsthis = value.QStyleOptionProgressBar_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionProgressBar_QStyleOptionProgressBar_0 {
  fn QStyleOptionProgressBar_0(self) -> QStyleOptionProgressBar;
}
// QStyleOptionProgressBar() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionProgressBar_QStyleOptionProgressBar_0 for () {
  fn QStyleOptionProgressBar_0(self) -> QStyleOptionProgressBar {
    // unsafe{_ZN23QStyleOptionProgressBarC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN23QStyleOptionProgressBarC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionProgressBar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:346
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionProgressBar(int)

/*

*/
// QStyleOptionProgressBar(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionProgressBar {
  pub fn QStyleOptionProgressBar_1<T: QStyleOptionProgressBar_QStyleOptionProgressBar_1>(value: T) -> QStyleOptionProgressBar {
    let rsthis = value.QStyleOptionProgressBar_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionProgressBar_QStyleOptionProgressBar_1 {
  fn QStyleOptionProgressBar_1(self) -> QStyleOptionProgressBar;
}
// QStyleOptionProgressBar(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionProgressBar_QStyleOptionProgressBar_1 for (i32) {
  fn QStyleOptionProgressBar_1(self) -> QStyleOptionProgressBar {
    // unsafe{_ZN23QStyleOptionProgressBarC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN23QStyleOptionProgressBarC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionProgressBar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionProgressBar(this :*mut QStyleOptionProgressBar) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN23QStyleOptionProgressBarD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionProgressBar__StyleOptionType = i32;
// 
pub const QStyleOptionProgressBar__Type :QStyleOptionProgressBar__StyleOptionType = 6;
pub fn QStyleOptionProgressBar_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionProgressBar__Type => // 6
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionProgressBar_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionProgressBar
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionProgressBar_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionProgressBar__StyleOptionVersion = i32;
// 1
pub const QStyleOptionProgressBar__Version :QStyleOptionProgressBar__StyleOptionVersion = 2;
pub fn QStyleOptionProgressBar_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionProgressBar__Version => // 2
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionProgressBar_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionProgressBar
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionProgressBar_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
