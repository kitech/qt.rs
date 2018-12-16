

// mod ::widgets::QStyleOptionTabWidgetFrame
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
#[derive(Default)] // class sizeof(QStyleOptionTabWidgetFrame)=136
pub struct QStyleOptionTabWidgetFrame {
  qbase: QStyleOption,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionTabWidgetFrame_ITF interface {
//    QStyleOption_ITF
//    QStyleOptionTabWidgetFrame_PTR() *QStyleOptionTabWidgetFrame
//}
//func (ptr *QStyleOptionTabWidgetFrame) QStyleOptionTabWidgetFrame_PTR() *QStyleOptionTabWidgetFrame { return ptr }

impl /*struct*/ QStyleOptionTabWidgetFrame {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionTabWidgetFrame {
    return QStyleOptionTabWidgetFrame{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionTabWidgetFrame {
//  type Target = QStyleOptionTabWidgetFrameBASE;
//
//  fn deref(&self) -> &QStyleOptionTabWidgetFrameBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionTabWidgetFrameBASE> for QStyleOptionTabWidgetFrame {
//  fn as_ref(& self) -> & QStyleOptionTabWidgetFrameBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:171
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionTabWidgetFrame()

/*

*/
// QStyleOptionTabWidgetFrame() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionTabWidgetFrame {
  pub fn QStyleOptionTabWidgetFrame_0<T: QStyleOptionTabWidgetFrame_QStyleOptionTabWidgetFrame_0>(value: T) -> QStyleOptionTabWidgetFrame {
    let rsthis = value.QStyleOptionTabWidgetFrame_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTabWidgetFrame_QStyleOptionTabWidgetFrame_0 {
  fn QStyleOptionTabWidgetFrame_0(self) -> QStyleOptionTabWidgetFrame;
}
// QStyleOptionTabWidgetFrame() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionTabWidgetFrame_QStyleOptionTabWidgetFrame_0 for () {
  fn QStyleOptionTabWidgetFrame_0(self) -> QStyleOptionTabWidgetFrame {
    // unsafe{_ZN26QStyleOptionTabWidgetFrameC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN26QStyleOptionTabWidgetFrameC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionTabWidgetFrame{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:176
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionTabWidgetFrame(int)

/*

*/
// QStyleOptionTabWidgetFrame(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionTabWidgetFrame {
  pub fn QStyleOptionTabWidgetFrame_1<T: QStyleOptionTabWidgetFrame_QStyleOptionTabWidgetFrame_1>(value: T) -> QStyleOptionTabWidgetFrame {
    let rsthis = value.QStyleOptionTabWidgetFrame_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTabWidgetFrame_QStyleOptionTabWidgetFrame_1 {
  fn QStyleOptionTabWidgetFrame_1(self) -> QStyleOptionTabWidgetFrame;
}
// QStyleOptionTabWidgetFrame(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionTabWidgetFrame_QStyleOptionTabWidgetFrame_1 for (i32) {
  fn QStyleOptionTabWidgetFrame_1(self) -> QStyleOptionTabWidgetFrame {
    // unsafe{_ZN26QStyleOptionTabWidgetFrameC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN26QStyleOptionTabWidgetFrameC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionTabWidgetFrame{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionTabWidgetFrame(this :*mut QStyleOptionTabWidgetFrame) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN26QStyleOptionTabWidgetFrameD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionTabWidgetFrame__StyleOptionType = i32;
// 
pub const QStyleOptionTabWidgetFrame__Type :QStyleOptionTabWidgetFrame__StyleOptionType = 11;
pub fn QStyleOptionTabWidgetFrame_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionTabWidgetFrame__Type => // 11
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionTabWidgetFrame_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionTabWidgetFrame
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionTabWidgetFrame_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionTabWidgetFrame__StyleOptionVersion = i32;
// 1
pub const QStyleOptionTabWidgetFrame__Version :QStyleOptionTabWidgetFrame__StyleOptionVersion = 2;
pub fn QStyleOptionTabWidgetFrame_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionTabWidgetFrame__Version => // 2
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionTabWidgetFrame_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionTabWidgetFrame
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionTabWidgetFrame_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
