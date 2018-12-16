

// mod ::widgets::QStyleOptionSpinBox
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
#[derive(Default)] // class sizeof(QStyleOptionSpinBox)=88
pub struct QStyleOptionSpinBox {
  qbase: QStyleOptionComplex,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionSpinBox_ITF interface {
//    QStyleOptionComplex_ITF
//    QStyleOptionSpinBox_PTR() *QStyleOptionSpinBox
//}
//func (ptr *QStyleOptionSpinBox) QStyleOptionSpinBox_PTR() *QStyleOptionSpinBox { return ptr }

impl /*struct*/ QStyleOptionSpinBox {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionSpinBox {
    return QStyleOptionSpinBox{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionSpinBox {
//  type Target = QStyleOptionSpinBoxBASE;
//
//  fn deref(&self) -> &QStyleOptionSpinBoxBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionSpinBoxBASE> for QStyleOptionSpinBox {
//  fn as_ref(& self) -> & QStyleOptionSpinBoxBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:552
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionSpinBox()

/*

*/
// QStyleOptionSpinBox() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionSpinBox {
  pub fn QStyleOptionSpinBox_0<T: QStyleOptionSpinBox_QStyleOptionSpinBox_0>(value: T) -> QStyleOptionSpinBox {
    let rsthis = value.QStyleOptionSpinBox_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSpinBox_QStyleOptionSpinBox_0 {
  fn QStyleOptionSpinBox_0(self) -> QStyleOptionSpinBox;
}
// QStyleOptionSpinBox() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionSpinBox_QStyleOptionSpinBox_0 for () {
  fn QStyleOptionSpinBox_0(self) -> QStyleOptionSpinBox {
    // unsafe{_ZN19QStyleOptionSpinBoxC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QStyleOptionSpinBoxC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionSpinBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:556
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionSpinBox(int)

/*

*/
// QStyleOptionSpinBox(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionSpinBox {
  pub fn QStyleOptionSpinBox_1<T: QStyleOptionSpinBox_QStyleOptionSpinBox_1>(value: T) -> QStyleOptionSpinBox {
    let rsthis = value.QStyleOptionSpinBox_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSpinBox_QStyleOptionSpinBox_1 {
  fn QStyleOptionSpinBox_1(self) -> QStyleOptionSpinBox;
}
// QStyleOptionSpinBox(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionSpinBox_QStyleOptionSpinBox_1 for (i32) {
  fn QStyleOptionSpinBox_1(self) -> QStyleOptionSpinBox {
    // unsafe{_ZN19QStyleOptionSpinBoxC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QStyleOptionSpinBoxC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionSpinBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionSpinBox(this :*mut QStyleOptionSpinBox) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN19QStyleOptionSpinBoxD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionSpinBox__StyleOptionType = i32;
// 
pub const QStyleOptionSpinBox__Type :QStyleOptionSpinBox__StyleOptionType = 983042;
pub fn QStyleOptionSpinBox_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionSpinBox__Type => // 983042
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionSpinBox_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionSpinBox
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionSpinBox_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionSpinBox__StyleOptionVersion = i32;
// 1
pub const QStyleOptionSpinBox__Version :QStyleOptionSpinBox__StyleOptionVersion = 1;
pub fn QStyleOptionSpinBox_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionSpinBox__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionSpinBox_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionSpinBox
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionSpinBox_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
