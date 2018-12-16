

// mod ::widgets::QStyleOptionSlider
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
// extern C begin: 1
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
#[derive(Default)] // class sizeof(QStyleOptionSlider)=128
pub struct QStyleOptionSlider {
  qbase: QStyleOptionComplex,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStyleOptionSlider_ITF interface {
//    QStyleOptionComplex_ITF
//    QStyleOptionSlider_PTR() *QStyleOptionSlider
//}
//func (ptr *QStyleOptionSlider) QStyleOptionSlider_PTR() *QStyleOptionSlider { return ptr }

impl /*struct*/ QStyleOptionSlider {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStyleOptionSlider {
    return QStyleOptionSlider{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStyleOptionSlider {
//  type Target = QStyleOptionSliderBASE;
//
//  fn deref(&self) -> &QStyleOptionSliderBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStyleOptionSliderBASE> for QStyleOptionSlider {
//  fn as_ref(& self) -> & QStyleOptionSliderBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qstyleoption.h:533
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStyleOptionSlider()

/*

*/
// QStyleOptionSlider() ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionSlider {
  pub fn QStyleOptionSlider_0<T: QStyleOptionSlider_QStyleOptionSlider_0>(value: T) -> QStyleOptionSlider {
    let rsthis = value.QStyleOptionSlider_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSlider_QStyleOptionSlider_0 {
  fn QStyleOptionSlider_0(self) -> QStyleOptionSlider;
}
// QStyleOptionSlider() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionSlider_QStyleOptionSlider_0 for () {
  fn QStyleOptionSlider_0(self) -> QStyleOptionSlider {
    // unsafe{_ZN18QStyleOptionSliderC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QStyleOptionSliderC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionSlider{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qstyleoption.h:537
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QStyleOptionSlider(int)

/*

*/
// QStyleOptionSlider(int) ctx.fn_proto_cpp
impl /*struct*/ QStyleOptionSlider {
  pub fn QStyleOptionSlider_1<T: QStyleOptionSlider_QStyleOptionSlider_1>(value: T) -> QStyleOptionSlider {
    let rsthis = value.QStyleOptionSlider_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSlider_QStyleOptionSlider_1 {
  fn QStyleOptionSlider_1(self) -> QStyleOptionSlider;
}
// QStyleOptionSlider(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStyleOptionSlider_QStyleOptionSlider_1 for (i32) {
  fn QStyleOptionSlider_1(self) -> QStyleOptionSlider {
    // unsafe{_ZN18QStyleOptionSliderC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QStyleOptionSliderC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStyleOptionSlider{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQStyleOptionSlider(this :*mut QStyleOptionSlider) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN18QStyleOptionSliderD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum is used to hold information about the type of the style option, and is defined for each QStyleOption subclass.

QStyleOption::TypeSO_DefaultThe type of style option provided (SO_Default for this class).


The type is used internally by QStyleOption, its subclasses, and qstyleoption_cast() to determine the type of style option. In general you do not need to worry about this unless you want to create your own QStyleOption subclass and your own styles.

See also StyleOptionVersion.

*/
pub type QStyleOptionSlider__StyleOptionType = i32;
// 
pub const QStyleOptionSlider__Type :QStyleOptionSlider__StyleOptionType = 983041;
pub fn QStyleOptionSlider_StyleOptionTypeItemName(val: i32) ->String {
  match val {
     QStyleOptionSlider__Type => // 983041
     {return String::from("Type");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionSlider_StyleOptionTypeItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionSlider
  //return nilthis.StyleOptionTypeItemName(val);
  return QStyleOptionSlider_StyleOptionTypeItemName(val);
}


/*
This enum is used to hold information about the version of the style option, and is defined for each QStyleOption subclass.



The version is used by QStyleOption subclasses to implement extensions without breaking compatibility. If you use qstyleoption_cast(), you normally do not need to check it.

See also StyleOptionType.

*/
pub type QStyleOptionSlider__StyleOptionVersion = i32;
// 1
pub const QStyleOptionSlider__Version :QStyleOptionSlider__StyleOptionVersion = 1;
pub fn QStyleOptionSlider_StyleOptionVersionItemName(val: i32) ->String {
  match val {
     QStyleOptionSlider__Version => // 1
     {return String::from("Version");}
  _ => {return format!("{}", val);}
}
}
pub fn QStyleOptionSlider_StyleOptionVersionItemName_s(val: i32) ->String {
  //var nilthis *QStyleOptionSlider
  //return nilthis.StyleOptionVersionItemName(val);
  return QStyleOptionSlider_StyleOptionVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
