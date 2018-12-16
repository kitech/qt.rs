

// mod ::widgets::QStyleOptionGroupBox
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
#[derive(Default)] // class sizeof(QStyleOptionGroupBox)=120
pub struct QStyleOptionGroupBox {
  qbase: QStyleOptionComplex,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionGroupBox_ITF interface {
//    QStyleOptionComplex_ITF
//    QStyleOptionGroupBox_PTR() *QStyleOptionGroupBox
//}
//func (ptr *QStyleOptionGroupBox) QStyleOptionGroupBox_PTR() *QStyleOptionGroupBox { return ptr }

impl /*struct*/ QStyleOptionGroupBox {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionGroupBox {
    return QStyleOptionGroupBox{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionGroupBox {
//  type Target = QStyleOptionGroupBoxBASE;
//
//  fn deref(&self) -> &QStyleOptionGroupBoxBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionGroupBoxBASE> for QStyleOptionGroupBox {
//  fn as_ref(& self) -> & QStyleOptionGroupBoxBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:639
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionGroupBox()

/*

*/
// QStyleOptionGroupBox() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionGroupBox {
  pub fn QStyleOptionGroupBox_0<T: QStyleOptionGroupBox_QStyleOptionGroupBox_0>(value: T) -> QStyleOptionGroupBox {
    let rsthis = value.QStyleOptionGroupBox_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionGroupBox_QStyleOptionGroupBox_0 {
  fn QStyleOptionGroupBox_0(self) -> QStyleOptionGroupBox;
}
// QStyleOptionGroupBox() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionGroupBox_QStyleOptionGroupBox_0 for () {
  fn QStyleOptionGroupBox_0(self) -> QStyleOptionGroupBox {
    // unsafe{_ZN20QStyleOptionGroupBoxC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QStyleOptionGroupBoxC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionGroupBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:642
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionGroupBox(int)

/*

*/
// QStyleOptionGroupBox(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionGroupBox {
  pub fn QStyleOptionGroupBox_1<T: QStyleOptionGroupBox_QStyleOptionGroupBox_1>(value: T) -> QStyleOptionGroupBox {
    let rsthis = value.QStyleOptionGroupBox_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionGroupBox_QStyleOptionGroupBox_1 {
  fn QStyleOptionGroupBox_1(self) -> QStyleOptionGroupBox;
}
// QStyleOptionGroupBox(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionGroupBox_QStyleOptionGroupBox_1 for (i32) {
  fn QStyleOptionGroupBox_1(self) -> QStyleOptionGroupBox {
    // unsafe{_ZN20QStyleOptionGroupBoxC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QStyleOptionGroupBoxC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionGroupBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionGroupBox(this :*mut QStyleOptionGroupBox) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN20QStyleOptionGroupBoxD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionGroupBox__StyleOptionType = i32;
// 
pub const QStyleOptionGroupBox__Type :QStyleOptionGroupBox__StyleOptionType = 983046;
pub fn QStyleOptionGroupBox_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionGroupBox__Type => // 983046
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionGroupBox_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionGroupBox
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionGroupBox_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionGroupBox__StyleOptionVersion = i32;
// 1
pub const QStyleOptionGroupBox__Version :QStyleOptionGroupBox__StyleOptionVersion = 1;
pub fn QStyleOptionGroupBox_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionGroupBox__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionGroupBox_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionGroupBox
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionGroupBox_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
