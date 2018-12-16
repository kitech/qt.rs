

// mod ::widgets::QStyleOptionFrame
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
#[derive(Default)] // class sizeof(QStyleOptionFrame)=80
pub struct QStyleOptionFrame {
  qbase: QStyleOption,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionFrame_ITF interface {
//    QStyleOption_ITF
//    QStyleOptionFrame_PTR() *QStyleOptionFrame
//}
//func (ptr *QStyleOptionFrame) QStyleOptionFrame_PTR() *QStyleOptionFrame { return ptr }

impl /*struct*/ QStyleOptionFrame {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionFrame {
    return QStyleOptionFrame{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionFrame {
//  type Target = QStyleOptionFrameBASE;
//
//  fn deref(&self) -> &QStyleOptionFrameBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionFrameBASE> for QStyleOptionFrame {
//  fn as_ref(& self) -> & QStyleOptionFrameBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionFrame()

/*

*/
// QStyleOptionFrame() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionFrame {
  pub fn QStyleOptionFrame_0<T: QStyleOptionFrame_QStyleOptionFrame_0>(value: T) -> QStyleOptionFrame {
    let rsthis = value.QStyleOptionFrame_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionFrame_QStyleOptionFrame_0 {
  fn QStyleOptionFrame_0(self) -> QStyleOptionFrame;
}
// QStyleOptionFrame() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionFrame_QStyleOptionFrame_0 for () {
  fn QStyleOptionFrame_0(self) -> QStyleOptionFrame {
    // unsafe{_ZN17QStyleOptionFrameC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QStyleOptionFrameC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionFrame{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:147
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionFrame(int)

/*

*/
// QStyleOptionFrame(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionFrame {
  pub fn QStyleOptionFrame_1<T: QStyleOptionFrame_QStyleOptionFrame_1>(value: T) -> QStyleOptionFrame {
    let rsthis = value.QStyleOptionFrame_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionFrame_QStyleOptionFrame_1 {
  fn QStyleOptionFrame_1(self) -> QStyleOptionFrame;
}
// QStyleOptionFrame(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionFrame_QStyleOptionFrame_1 for (i32) {
  fn QStyleOptionFrame_1(self) -> QStyleOptionFrame {
    // unsafe{_ZN17QStyleOptionFrameC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QStyleOptionFrameC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionFrame{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionFrame(this :*mut QStyleOptionFrame) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN17QStyleOptionFrameD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionFrame__StyleOptionType = i32;
// 
pub const QStyleOptionFrame__Type :QStyleOptionFrame__StyleOptionType = 5;
pub fn QStyleOptionFrame_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionFrame__Type => // 5
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionFrame_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionFrame
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionFrame_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionFrame__StyleOptionVersion = i32;
// 1
pub const QStyleOptionFrame__Version :QStyleOptionFrame__StyleOptionVersion = 3;
pub fn QStyleOptionFrame_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionFrame__Version => // 3
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionFrame_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionFrame
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionFrame_StyleOptionVersionItemName(val);
}


/*


*/
pub type QStyleOptionFrame__FrameFeature = i32;
// 
pub const QStyleOptionFrame__None :QStyleOptionFrame__FrameFeature = 0;
// 
pub const QStyleOptionFrame__Flat :QStyleOptionFrame__FrameFeature = 1;
// 
pub const QStyleOptionFrame__Rounded :QStyleOptionFrame__FrameFeature = 2;
pub fn QStyleOptionFrame_FrameFeatureItemName(val: i32) ->String {
  match val {
     QStyleOptionFrame__None => // 0
     {return String::from("None");}
     QStyleOptionFrame__Flat => // 1
     {return String::from("Flat");}
     QStyleOptionFrame__Rounded => // 2
     {return String::from("Rounded");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionFrame_FrameFeatureItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionFrame
  //return nilthis.FrameFeatureItemName(val);
  return QStyleOptionFrame_FrameFeatureItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
