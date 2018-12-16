

// mod ::gui::QColor
// package qtgui
// /usr/include/qt/QtGui/qcolor.h
// #include <qcolor.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 24
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QColor)=16
pub struct QColor {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QColor_ITF interface {
//    QColor_PTR() *QColor
//}
//func (ptr *QColor) QColor_PTR() *QColor { return ptr }

impl /*struct*/ QColor {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QColor {
    return QColor{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QColor {
//  type Target = QColorBASE;
//
//  fn deref(&self) -> &QColorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QColorBASE> for QColor {
//  fn as_ref(& self) -> & QColorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qcolor.h:70
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QColor()

/*
Constructs an invalid color with the RGB value (0, 0, 0). An invalid color is a color that is not properly set up for the underlying window system.

The alpha value of an invalid color is unspecified.

See also isValid().
*/
// QColor() ctx.fn_proto_cpp
impl /*struct*/ QColor {
  pub fn QColor_0<T: QColor_QColor_0>(value: T) -> QColor {
    let rsthis = value.QColor_0();
    return rsthis;
    // return 1;
  }
}

pub trait QColor_QColor_0 {
  fn QColor_0(self) -> QColor;
}
// QColor() ctx.fn_proto_cpp
impl<'a> /*trait*/ QColor_QColor_0 for () {
  fn QColor_0(self) -> QColor {
    // unsafe{_ZN6QColorC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QColorC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QColor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:71
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QColor(Qt::GlobalColor)

/*
Constructs an invalid color with the RGB value (0, 0, 0). An invalid color is a color that is not properly set up for the underlying window system.

The alpha value of an invalid color is unspecified.

See also isValid().
*/
// QColor(Qt::GlobalColor) ctx.fn_proto_cpp
impl /*struct*/ QColor {
  pub fn QColor_1<T: QColor_QColor_1>(value: T) -> QColor {
    let rsthis = value.QColor_1();
    return rsthis;
    // return 1;
  }
}

pub trait QColor_QColor_1 {
  fn QColor_1(self) -> QColor;
}
// QColor(Qt::GlobalColor) ctx.fn_proto_cpp
impl<'a> /*trait*/ QColor_QColor_1 for (i32) {
  fn QColor_1(self) -> QColor {
    // unsafe{_ZN6QColorC2EN2Qt11GlobalColorE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QColorC2EN2Qt11GlobalColorE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QColor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:72
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QColor(int, int, int, int)

/*
Constructs an invalid color with the RGB value (0, 0, 0). An invalid color is a color that is not properly set up for the underlying window system.

The alpha value of an invalid color is unspecified.

See also isValid().
*/
// QColor(int, int, int, int) ctx.fn_proto_cpp
impl /*struct*/ QColor {
  pub fn QColor_2<T: QColor_QColor_2>(value: T) -> QColor {
    let rsthis = value.QColor_2();
    return rsthis;
    // return 1;
  }
}

pub trait QColor_QColor_2 {
  fn QColor_2(self) -> QColor;
}
// QColor(int, int, int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QColor_QColor_2 for (i32,i32,i32,i32) {
  fn QColor_2(self) -> QColor {
    // unsafe{_ZN6QColorC2Eiiii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QColorC2Eiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QColor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:73
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QColor(QRgb)

/*
Constructs an invalid color with the RGB value (0, 0, 0). An invalid color is a color that is not properly set up for the underlying window system.

The alpha value of an invalid color is unspecified.

See also isValid().
*/
// QColor(QRgb) ctx.fn_proto_cpp
impl /*struct*/ QColor {
  pub fn QColor_3<T: QColor_QColor_3>(value: T) -> QColor {
    let rsthis = value.QColor_3();
    return rsthis;
    // return 1;
  }
}

pub trait QColor_QColor_3 {
  fn QColor_3(self) -> QColor;
}
// QColor(QRgb) ctx.fn_proto_cpp
impl<'a> /*trait*/ QColor_QColor_3 for (u32) {
  fn QColor_3(self) -> QColor {
    // unsafe{_ZN6QColorC2Ej()};
    let arg0 = (&self/*.qclsinst*/) as *const u32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QColorC2Ej", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QColor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:74
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QColor(QRgba64)

/*
Constructs an invalid color with the RGB value (0, 0, 0). An invalid color is a color that is not properly set up for the underlying window system.

The alpha value of an invalid color is unspecified.

See also isValid().
*/
// QColor(QRgba64) ctx.fn_proto_cpp
impl /*struct*/ QColor {
  pub fn QColor_4<T: QColor_QColor_4>(value: T) -> QColor {
    let rsthis = value.QColor_4();
    return rsthis;
    // return 1;
  }
}

pub trait QColor_QColor_4 {
  fn QColor_4(self) -> QColor;
}
// QColor(QRgba64) ctx.fn_proto_cpp
impl<'a> /*trait*/ QColor_QColor_4 for (usize) {
  fn QColor_4(self) -> QColor {
    // unsafe{_ZN6QColorC2E7QRgba64()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QColorC2E7QRgba64", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QColor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:76
// index:5
// Public inline Visibility=Default Availability=Available
// [-2] void QColor(const QString &)

/*
Constructs an invalid color with the RGB value (0, 0, 0). An invalid color is a color that is not properly set up for the underlying window system.

The alpha value of an invalid color is unspecified.

See also isValid().
*/
// QColor(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QColor {
  pub fn QColor_5<T: QColor_QColor_5>(value: T) -> QColor {
    let rsthis = value.QColor_5();
    return rsthis;
    // return 1;
  }
}

pub trait QColor_QColor_5 {
  fn QColor_5(self) -> QColor;
}
// QColor(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QColor_QColor_5 for (usize) {
  fn QColor_5(self) -> QColor {
    // unsafe{_ZN6QColorC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QColorC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QColor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:78
// index:6
// Public inline Visibility=Default Availability=Available
// [-2] void QColor(QStringView)

/*
Constructs an invalid color with the RGB value (0, 0, 0). An invalid color is a color that is not properly set up for the underlying window system.

The alpha value of an invalid color is unspecified.

See also isValid().
*/
// QColor(QStringView) ctx.fn_proto_cpp
impl /*struct*/ QColor {
  pub fn QColor_6<T: QColor_QColor_6>(value: T) -> QColor {
    let rsthis = value.QColor_6();
    return rsthis;
    // return 1;
  }
}

pub trait QColor_QColor_6 {
  fn QColor_6(self) -> QColor;
}
// QColor(QStringView) ctx.fn_proto_cpp
impl<'a> /*trait*/ QColor_QColor_6 for (usize) {
  fn QColor_6(self) -> QColor {
    // unsafe{_ZN6QColorC2E11QStringView()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QColorC2E11QStringView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QColor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:79
// index:7
// Public inline Visibility=Default Availability=Available
// [-2] void QColor(const char *)

/*
Constructs an invalid color with the RGB value (0, 0, 0). An invalid color is a color that is not properly set up for the underlying window system.

The alpha value of an invalid color is unspecified.

See also isValid().
*/
// QColor(const char *) ctx.fn_proto_cpp
impl /*struct*/ QColor {
  pub fn QColor_7<T: QColor_QColor_7>(value: T) -> QColor {
    let rsthis = value.QColor_7();
    return rsthis;
    // return 1;
  }
}

pub trait QColor_QColor_7 {
  fn QColor_7(self) -> QColor;
}
// QColor(const char *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QColor_QColor_7 for (usize) {
  fn QColor_7(self) -> QColor {
    // unsafe{_ZN6QColorC2EPKc()};
    let arg0 = (self) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QColorC2EPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QColor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:80
// index:8
// Public inline Visibility=Default Availability=Available
// [-2] void QColor(QLatin1String)

/*
Constructs an invalid color with the RGB value (0, 0, 0). An invalid color is a color that is not properly set up for the underlying window system.

The alpha value of an invalid color is unspecified.

See also isValid().
*/
// QColor(QLatin1String) ctx.fn_proto_cpp
impl /*struct*/ QColor {
  pub fn QColor_8<T: QColor_QColor_8>(value: T) -> QColor {
    let rsthis = value.QColor_8();
    return rsthis;
    // return 1;
  }
}

pub trait QColor_QColor_8 {
  fn QColor_8(self) -> QColor;
}
// QColor(QLatin1String) ctx.fn_proto_cpp
impl<'a> /*trait*/ QColor_QColor_8 for (usize) {
  fn QColor_8(self) -> QColor {
    // unsafe{_ZN6QColorC2E13QLatin1String()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QColorC2E13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QColor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:81
// index:9
// Public Visibility=Default Availability=Available
// [-2] void QColor(QColor::Spec)

/*
Constructs an invalid color with the RGB value (0, 0, 0). An invalid color is a color that is not properly set up for the underlying window system.

The alpha value of an invalid color is unspecified.

See also isValid().
*/
// QColor(QColor::Spec) ctx.fn_proto_cpp
impl /*struct*/ QColor {
  pub fn QColor_9<T: QColor_QColor_9>(value: T) -> QColor {
    let rsthis = value.QColor_9();
    return rsthis;
    // return 1;
  }
}

pub trait QColor_QColor_9 {
  fn QColor_9(self) -> QColor;
}
// QColor(QColor::Spec) ctx.fn_proto_cpp
impl<'a> /*trait*/ QColor_QColor_9 for (i32) {
  fn QColor_9(self) -> QColor {
    // unsafe{_ZN6QColorC2ENS_4SpecE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QColorC2ENS_4SpecE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QColor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:87
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QColor & operator=(QColor &&)

/*

*/
impl /*struct*/ QColor {
  pub fn operator_equal_0<RetType, T: QColor_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QColor_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QColor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColoraSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:90
// index:1
// Public Visibility=Default Availability=Available
// [16] QColor & operator=(const QColor &)

/*

*/
impl /*struct*/ QColor {
  pub fn operator_equal_1<RetType, T: QColor_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QColor_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QColor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColoraSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:93
// index:2
// Public Visibility=Default Availability=Available
// [16] QColor & operator=(Qt::GlobalColor)

/*

*/
impl /*struct*/ QColor {
  pub fn operator_equal_2<RetType, T: QColor_operator_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_2(self);
    // return 1;
  }
}
pub trait QColor_operator_equal_2<RetType> {
  fn operator_equal_2(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_operator_equal_2<usize> for (i32) {
  fn operator_equal_2(self , rsthis: & QColor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColoraSEN2Qt11GlobalColorE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:95
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the color is valid; otherwise returns false.
*/
impl /*struct*/ QColor {
  pub fn isValid_0<RetType, T: QColor_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QColor_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QColor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:98
// index:0
// Public Visibility=Default Availability=Available
// [8] QString name() const

/*
Returns the name of the color in the format "#RRGGBB"; i.e. a "#" character followed by three two-digit hexadecimal numbers.

See also setNamedColor().
*/
impl /*struct*/ QColor {
  pub fn name_0<RetType, T: QColor_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QColor_name_0<RetType> {
  fn name_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_name_0<usize> for () {
  fn name_0(self , rsthis: & QColor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:99
// index:1
// Public Visibility=Default Availability=Available
// [8] QString name(QColor::NameFormat) const

/*
Returns the name of the color in the format "#RRGGBB"; i.e. a "#" character followed by three two-digit hexadecimal numbers.

See also setNamedColor().
*/
impl /*struct*/ QColor {
  pub fn name_1<RetType, T: QColor_name_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_1(self);
    // return 1;
  }
}
pub trait QColor_name_1<RetType> {
  fn name_1(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_name_1<usize> for (i32) {
  fn name_1(self , rsthis: & QColor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor4nameENS_10NameFormatE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNamedColor(const QString &)

/*
Sets the RGB value of this QColor to name, which may be in one of these formats:


#RGB (each of R, G, and B is a single hex digit)
#RRGGBB
#AARRGGBB (Since 5.2)
#RRRGGGBBB
#RRRRGGGGBBBB
A name from the list of colors defined in the list of SVG color keyword names provided by the World Wide Web Consortium; for example, "steelblue" or "gainsboro". These color names work on all platforms. Note that these color names are not the same as defined by the Qt::GlobalColor enums, e.g. "green" and Qt::green does not refer to the same color.
transparent - representing the absence of a color.


The color is invalid if name cannot be parsed.

See also QColor(), name(), and isValid().
*/
impl /*struct*/ QColor {
  pub fn setNamedColor_0<RetType, T: QColor_setNamedColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNamedColor_0(self);
    // return 1;
  }
}
pub trait QColor_setNamedColor_0<RetType> {
  fn setNamedColor_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setNamedColor_0<(/*void*/)> for (usize) {
  fn setNamedColor_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor13setNamedColorERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:104
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setNamedColor(QStringView)

/*
Sets the RGB value of this QColor to name, which may be in one of these formats:


#RGB (each of R, G, and B is a single hex digit)
#RRGGBB
#AARRGGBB (Since 5.2)
#RRRGGGBBB
#RRRRGGGGBBBB
A name from the list of colors defined in the list of SVG color keyword names provided by the World Wide Web Consortium; for example, "steelblue" or "gainsboro". These color names work on all platforms. Note that these color names are not the same as defined by the Qt::GlobalColor enums, e.g. "green" and Qt::green does not refer to the same color.
transparent - representing the absence of a color.


The color is invalid if name cannot be parsed.

See also QColor(), name(), and isValid().
*/
impl /*struct*/ QColor {
  pub fn setNamedColor_1<RetType, T: QColor_setNamedColor_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNamedColor_1(self);
    // return 1;
  }
}
pub trait QColor_setNamedColor_1<RetType> {
  fn setNamedColor_1(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setNamedColor_1<(/*void*/)> for (usize) {
  fn setNamedColor_1(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor13setNamedColorE11QStringView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:105
// index:2
// Public Visibility=Default Availability=Available
// [-2] void setNamedColor(QLatin1String)

/*
Sets the RGB value of this QColor to name, which may be in one of these formats:


#RGB (each of R, G, and B is a single hex digit)
#RRGGBB
#AARRGGBB (Since 5.2)
#RRRGGGBBB
#RRRRGGGGBBBB
A name from the list of colors defined in the list of SVG color keyword names provided by the World Wide Web Consortium; for example, "steelblue" or "gainsboro". These color names work on all platforms. Note that these color names are not the same as defined by the Qt::GlobalColor enums, e.g. "green" and Qt::green does not refer to the same color.
transparent - representing the absence of a color.


The color is invalid if name cannot be parsed.

See also QColor(), name(), and isValid().
*/
impl /*struct*/ QColor {
  pub fn setNamedColor_2<RetType, T: QColor_setNamedColor_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNamedColor_2(self);
    // return 1;
  }
}
pub trait QColor_setNamedColor_2<RetType> {
  fn setNamedColor_2(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setNamedColor_2<(/*void*/)> for (usize) {
  fn setNamedColor_2(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor13setNamedColorE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:107
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStringList colorNames()

/*
Returns a QStringList containing the color names Qt knows about.

See also Predefined Colors.
*/
impl /*struct*/ QColor {
  pub fn colorNames_0<RetType, T: QColor_colorNames_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.colorNames_0();
    // return 1;
  }
}
pub trait QColor_colorNames_0<RetType> {
  fn colorNames_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColor_colorNames_0<usize> for () {
  fn colorNames_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColor10colorNamesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:109
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QColor::Spec spec() const

/*
Returns how the color was specified.

See also Spec and convertTo().
*/
impl /*struct*/ QColor {
  pub fn spec_0<RetType, T: QColor_spec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.spec_0(self);
    // return 1;
  }
}
pub trait QColor_spec_0<RetType> {
  fn spec_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_spec_0<i32> for () {
  fn spec_0(self , rsthis: & QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor4specEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:112
// index:0
// Public Visibility=Default Availability=Available
// [4] int alpha() const

/*
Returns the alpha color component of this color.

See also setAlpha(), alphaF(), and Alpha-Blended Drawing.
*/
impl /*struct*/ QColor {
  pub fn alpha_0<RetType, T: QColor_alpha_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alpha_0(self);
    // return 1;
  }
}
pub trait QColor_alpha_0<RetType> {
  fn alpha_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_alpha_0<i32> for () {
  fn alpha_0(self , rsthis: & QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor5alphaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:113
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAlpha(int)

/*
Sets the alpha of this color to alpha. Integer alpha is specified in the range 0-255.

See also alpha(), alphaF(), and Alpha-Blended Drawing.
*/
impl /*struct*/ QColor {
  pub fn setAlpha_0<RetType, T: QColor_setAlpha_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlpha_0(self);
    // return 1;
  }
}
pub trait QColor_setAlpha_0<RetType> {
  fn setAlpha_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setAlpha_0<(/*void*/)> for (i32) {
  fn setAlpha_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor8setAlphaEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:115
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal alphaF() const

/*
Returns the alpha color component of this color.

See also setAlphaF(), alpha(), and Alpha-Blended Drawing.
*/
impl /*struct*/ QColor {
  pub fn alphaF_0<RetType, T: QColor_alphaF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alphaF_0(self);
    // return 1;
  }
}
pub trait QColor_alphaF_0<RetType> {
  fn alphaF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_alphaF_0<f64> for () {
  fn alphaF_0(self , rsthis: & QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor6alphaFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAlphaF(qreal)

/*
Sets the alpha of this color to alpha. qreal alpha is specified in the range 0.0-1.0.

See also alphaF(), alpha(), and Alpha-Blended Drawing.
*/
impl /*struct*/ QColor {
  pub fn setAlphaF_0<RetType, T: QColor_setAlphaF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlphaF_0(self);
    // return 1;
  }
}
pub trait QColor_setAlphaF_0<RetType> {
  fn setAlphaF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setAlphaF_0<(/*void*/)> for (f64) {
  fn setAlphaF_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor9setAlphaFEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:118
// index:0
// Public Visibility=Default Availability=Available
// [4] int red() const

/*
Returns the red color component of this color.

See also setRed(), redF(), and getRgb().
*/
impl /*struct*/ QColor {
  pub fn red_0<RetType, T: QColor_red_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.red_0(self);
    // return 1;
  }
}
pub trait QColor_red_0<RetType> {
  fn red_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_red_0<i32> for () {
  fn red_0(self , rsthis: & QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor3redEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:119
// index:0
// Public Visibility=Default Availability=Available
// [4] int green() const

/*
Returns the green color component of this color.

See also setGreen(), greenF(), and getRgb().
*/
impl /*struct*/ QColor {
  pub fn green_0<RetType, T: QColor_green_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.green_0(self);
    // return 1;
  }
}
pub trait QColor_green_0<RetType> {
  fn green_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_green_0<i32> for () {
  fn green_0(self , rsthis: & QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor5greenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:120
// index:0
// Public Visibility=Default Availability=Available
// [4] int blue() const

/*
Returns the blue color component of this color.

See also setBlue(), blueF(), and getRgb().
*/
impl /*struct*/ QColor {
  pub fn blue_0<RetType, T: QColor_blue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blue_0(self);
    // return 1;
  }
}
pub trait QColor_blue_0<RetType> {
  fn blue_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_blue_0<i32> for () {
  fn blue_0(self , rsthis: & QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor4blueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:121
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRed(int)

/*
Sets the red color component of this color to red. Integer components are specified in the range 0-255.

See also red(), redF(), and setRgb().
*/
impl /*struct*/ QColor {
  pub fn setRed_0<RetType, T: QColor_setRed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRed_0(self);
    // return 1;
  }
}
pub trait QColor_setRed_0<RetType> {
  fn setRed_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setRed_0<(/*void*/)> for (i32) {
  fn setRed_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor6setRedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:122
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGreen(int)

/*
Sets the green color component of this color to green. Integer components are specified in the range 0-255.

See also green(), greenF(), and setRgb().
*/
impl /*struct*/ QColor {
  pub fn setGreen_0<RetType, T: QColor_setGreen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGreen_0(self);
    // return 1;
  }
}
pub trait QColor_setGreen_0<RetType> {
  fn setGreen_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setGreen_0<(/*void*/)> for (i32) {
  fn setGreen_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor8setGreenEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:123
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBlue(int)

/*
Sets the blue color component of this color to blue. Integer components are specified in the range 0-255.

See also blue(), blueF(), and setRgb().
*/
impl /*struct*/ QColor {
  pub fn setBlue_0<RetType, T: QColor_setBlue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBlue_0(self);
    // return 1;
  }
}
pub trait QColor_setBlue_0<RetType> {
  fn setBlue_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setBlue_0<(/*void*/)> for (i32) {
  fn setBlue_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor7setBlueEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:125
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal redF() const

/*
Returns the red color component of this color.

See also setRedF(), red(), and getRgbF().
*/
impl /*struct*/ QColor {
  pub fn redF_0<RetType, T: QColor_redF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redF_0(self);
    // return 1;
  }
}
pub trait QColor_redF_0<RetType> {
  fn redF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_redF_0<f64> for () {
  fn redF_0(self , rsthis: & QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor4redFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:126
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal greenF() const

/*
Returns the green color component of this color.

See also setGreenF(), green(), and getRgbF().
*/
impl /*struct*/ QColor {
  pub fn greenF_0<RetType, T: QColor_greenF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.greenF_0(self);
    // return 1;
  }
}
pub trait QColor_greenF_0<RetType> {
  fn greenF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_greenF_0<f64> for () {
  fn greenF_0(self , rsthis: & QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor6greenFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:127
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal blueF() const

/*
Returns the blue color component of this color.

See also setBlueF(), blue(), and getRgbF().
*/
impl /*struct*/ QColor {
  pub fn blueF_0<RetType, T: QColor_blueF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blueF_0(self);
    // return 1;
  }
}
pub trait QColor_blueF_0<RetType> {
  fn blueF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_blueF_0<f64> for () {
  fn blueF_0(self , rsthis: & QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor5blueFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:128
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRedF(qreal)

/*
Sets the red color component of this color to red. Float components are specified in the range 0.0-1.0.

See also redF(), red(), and setRgbF().
*/
impl /*struct*/ QColor {
  pub fn setRedF_0<RetType, T: QColor_setRedF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRedF_0(self);
    // return 1;
  }
}
pub trait QColor_setRedF_0<RetType> {
  fn setRedF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setRedF_0<(/*void*/)> for (f64) {
  fn setRedF_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor7setRedFEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:129
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGreenF(qreal)

/*
Sets the green color component of this color to green. Float components are specified in the range 0.0-1.0.

See also greenF(), green(), and setRgbF().
*/
impl /*struct*/ QColor {
  pub fn setGreenF_0<RetType, T: QColor_setGreenF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGreenF_0(self);
    // return 1;
  }
}
pub trait QColor_setGreenF_0<RetType> {
  fn setGreenF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setGreenF_0<(/*void*/)> for (f64) {
  fn setGreenF_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor9setGreenFEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:130
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBlueF(qreal)

/*
Sets the blue color component of this color to blue. Float components are specified in the range 0.0-1.0.

See also blueF(), blue(), and setRgbF().
*/
impl /*struct*/ QColor {
  pub fn setBlueF_0<RetType, T: QColor_setBlueF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBlueF_0(self);
    // return 1;
  }
}
pub trait QColor_setBlueF_0<RetType> {
  fn setBlueF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setBlueF_0<(/*void*/)> for (f64) {
  fn setBlueF_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor8setBlueFEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:132
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getRgb(int *, int *, int *, int *) const

/*
Sets the contents pointed to by r, g, b, and a, to the red, green, blue, and alpha-channel (transparency) components of the color's RGB value.

These components can be retrieved individually using the red(), green(), blue() and alpha() functions.

See also rgb() and setRgb().
*/
impl /*struct*/ QColor {
  pub fn getRgb_0<RetType, T: QColor_getRgb_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getRgb_0(self);
    // return 1;
  }
}
pub trait QColor_getRgb_0<RetType> {
  fn getRgb_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_getRgb_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn getRgb_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK6QColor6getRgbEPiS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRgb(int, int, int, int)

/*
Sets the RGB value to r, g, b and the alpha value to a.

All the values must be in the range 0-255.

See also rgb(), getRgb(), and setRgbF().
*/
impl /*struct*/ QColor {
  pub fn setRgb_0<RetType, T: QColor_setRgb_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRgb_0(self);
    // return 1;
  }
}
pub trait QColor_setRgb_0<RetType> {
  fn setRgb_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setRgb_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn setRgb_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor6setRgbEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:145
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setRgb(QRgb)

/*
Sets the RGB value to r, g, b and the alpha value to a.

All the values must be in the range 0-255.

See also rgb(), getRgb(), and setRgbF().
*/
impl /*struct*/ QColor {
  pub fn setRgb_1<RetType, T: QColor_setRgb_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRgb_1(self);
    // return 1;
  }
}
pub trait QColor_setRgb_1<RetType> {
  fn setRgb_1(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setRgb_1<(/*void*/)> for (u32) {
  fn setRgb_1(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const u32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor6setRgbEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:135
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getRgbF(qreal *, qreal *, qreal *, qreal *) const

/*
Sets the contents pointed to by r, g, b, and a, to the red, green, blue, and alpha-channel (transparency) components of the color's RGB value.

These components can be retrieved individually using the redF(), greenF(), blueF() and alphaF() functions.

See also rgb() and setRgb().
*/
impl /*struct*/ QColor {
  pub fn getRgbF_0<RetType, T: QColor_getRgbF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getRgbF_0(self);
    // return 1;
  }
}
pub trait QColor_getRgbF_0<RetType> {
  fn getRgbF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_getRgbF_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn getRgbF_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK6QColor7getRgbFEPdS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:136
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRgbF(qreal, qreal, qreal, qreal)

/*
Sets the color channels of this color to r (red), g (green), b (blue) and a (alpha, transparency).

All values must be in the range 0.0-1.0.

See also rgb(), getRgbF(), and setRgb().
*/
impl /*struct*/ QColor {
  pub fn setRgbF_0<RetType, T: QColor_setRgbF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRgbF_0(self);
    // return 1;
  }
}
pub trait QColor_setRgbF_0<RetType> {
  fn setRgbF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setRgbF_0<(/*void*/)> for (f64,f64,f64,f64) {
  fn setRgbF_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor7setRgbFEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:138
// index:0
// Public Visibility=Default Availability=Available
// [8] QRgba64 rgba64() const

/*
Returns the RGB64 value of the color, including its alpha.

For an invalid color, the alpha value of the returned color is unspecified.

This function was introduced in  Qt 5.6.

See also setRgba64(), rgba(), and rgb().
*/
impl /*struct*/ QColor {
  pub fn rgba64_0<RetType, T: QColor_rgba64_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rgba64_0(self);
    // return 1;
  }
}
pub trait QColor_rgba64_0<RetType> {
  fn rgba64_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_rgba64_0<usize> for () {
  fn rgba64_0(self , rsthis: & QColor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor6rgba64Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:139
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRgba64(QRgba64)

/*
Sets the RGB64 value to rgba, including its alpha.

This function was introduced in  Qt 5.6.

See also setRgba() and rgba64().
*/
impl /*struct*/ QColor {
  pub fn setRgba64_0<RetType, T: QColor_setRgba64_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRgba64_0(self);
    // return 1;
  }
}
pub trait QColor_setRgba64_0<RetType> {
  fn setRgba64_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setRgba64_0<(/*void*/)> for (usize) {
  fn setRgba64_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor9setRgba64E7QRgba64", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:141
// index:0
// Public Visibility=Default Availability=Available
// [4] QRgb rgba() const

/*
Returns the RGB value of the color, including its alpha.

For an invalid color, the alpha value of the returned color is unspecified.

See also setRgba(), rgb(), and rgba64().
*/
impl /*struct*/ QColor {
  pub fn rgba_0<RetType, T: QColor_rgba_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rgba_0(self);
    // return 1;
  }
}
pub trait QColor_rgba_0<RetType> {
  fn rgba_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_rgba_0<u32> for () {
  fn rgba_0(self , rsthis: & QColor) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor4rgbaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:142
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRgba(QRgb)

/*
Sets the RGB value to rgba, including its alpha.

See also rgba(), rgb(), and setRgba64().
*/
impl /*struct*/ QColor {
  pub fn setRgba_0<RetType, T: QColor_setRgba_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRgba_0(self);
    // return 1;
  }
}
pub trait QColor_setRgba_0<RetType> {
  fn setRgba_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setRgba_0<(/*void*/)> for (u32) {
  fn setRgba_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const u32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor7setRgbaEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:144
// index:0
// Public Visibility=Default Availability=Available
// [4] QRgb rgb() const

/*
Returns the RGB value of the color. The alpha value is opaque.

See also setRgb(), getRgb(), and rgba().
*/
impl /*struct*/ QColor {
  pub fn rgb_0<RetType, T: QColor_rgb_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rgb_0(self);
    // return 1;
  }
}
pub trait QColor_rgb_0<RetType> {
  fn rgb_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_rgb_0<u32> for () {
  fn rgb_0(self , rsthis: & QColor) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor3rgbEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:147
// index:0
// Public Visibility=Default Availability=Available
// [4] int hue() const

/*
Returns the hue color component of this color.

The color is implicitly converted to HSV.

See also hsvHue(), hueF(), getHsv(), and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn hue_0<RetType, T: QColor_hue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hue_0(self);
    // return 1;
  }
}
pub trait QColor_hue_0<RetType> {
  fn hue_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_hue_0<i32> for () {
  fn hue_0(self , rsthis: & QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor3hueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:148
// index:0
// Public Visibility=Default Availability=Available
// [4] int saturation() const

/*
Returns the saturation color component of this color.

The color is implicitly converted to HSV.

See also hsvSaturation(), saturationF(), getHsv(), and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn saturation_0<RetType, T: QColor_saturation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.saturation_0(self);
    // return 1;
  }
}
pub trait QColor_saturation_0<RetType> {
  fn saturation_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_saturation_0<i32> for () {
  fn saturation_0(self , rsthis: & QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor10saturationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:149
// index:0
// Public Visibility=Default Availability=Available
// [4] int hsvHue() const

/*
Returns the hue color component of this color.

See also hueF(), getHsv(), and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn hsvHue_0<RetType, T: QColor_hsvHue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hsvHue_0(self);
    // return 1;
  }
}
pub trait QColor_hsvHue_0<RetType> {
  fn hsvHue_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_hsvHue_0<i32> for () {
  fn hsvHue_0(self , rsthis: & QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor6hsvHueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:150
// index:0
// Public Visibility=Default Availability=Available
// [4] int hsvSaturation() const

/*
Returns the saturation color component of this color.

See also saturationF(), getHsv(), and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn hsvSaturation_0<RetType, T: QColor_hsvSaturation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hsvSaturation_0(self);
    // return 1;
  }
}
pub trait QColor_hsvSaturation_0<RetType> {
  fn hsvSaturation_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_hsvSaturation_0<i32> for () {
  fn hsvSaturation_0(self , rsthis: & QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor13hsvSaturationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:151
// index:0
// Public Visibility=Default Availability=Available
// [4] int value() const

/*
Returns the value color component of this color.

See also valueF(), getHsv(), and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn value_0<RetType, T: QColor_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QColor_value_0<RetType> {
  fn value_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_value_0<i32> for () {
  fn value_0(self , rsthis: & QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor5valueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:153
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal hueF() const

/*
Returns the hue color component of this color.

The color is implicitly converted to HSV.

See also hsvHueF(), hue(), getHsvF(), and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn hueF_0<RetType, T: QColor_hueF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hueF_0(self);
    // return 1;
  }
}
pub trait QColor_hueF_0<RetType> {
  fn hueF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_hueF_0<f64> for () {
  fn hueF_0(self , rsthis: & QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor4hueFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:154
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal saturationF() const

/*
Returns the saturation color component of this color.

The color is implicitly converted to HSV.

See also hsvSaturationF(), saturation(), getHsvF(), and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn saturationF_0<RetType, T: QColor_saturationF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.saturationF_0(self);
    // return 1;
  }
}
pub trait QColor_saturationF_0<RetType> {
  fn saturationF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_saturationF_0<f64> for () {
  fn saturationF_0(self , rsthis: & QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor11saturationFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:155
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal hsvHueF() const

/*
Returns the hue color component of this color.

See also hue(), getHsvF(), and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn hsvHueF_0<RetType, T: QColor_hsvHueF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hsvHueF_0(self);
    // return 1;
  }
}
pub trait QColor_hsvHueF_0<RetType> {
  fn hsvHueF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_hsvHueF_0<f64> for () {
  fn hsvHueF_0(self , rsthis: & QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor7hsvHueFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:156
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal hsvSaturationF() const

/*
Returns the saturation color component of this color.

See also saturation(), getHsvF(), and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn hsvSaturationF_0<RetType, T: QColor_hsvSaturationF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hsvSaturationF_0(self);
    // return 1;
  }
}
pub trait QColor_hsvSaturationF_0<RetType> {
  fn hsvSaturationF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_hsvSaturationF_0<f64> for () {
  fn hsvSaturationF_0(self , rsthis: & QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor14hsvSaturationFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:157
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal valueF() const

/*
Returns the value color component of this color.

See also value(), getHsvF(), and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn valueF_0<RetType, T: QColor_valueF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valueF_0(self);
    // return 1;
  }
}
pub trait QColor_valueF_0<RetType> {
  fn valueF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_valueF_0<f64> for () {
  fn valueF_0(self , rsthis: & QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor6valueFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:159
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getHsv(int *, int *, int *, int *) const

/*
Sets the contents pointed to by h, s, v, and a, to the hue, saturation, value, and alpha-channel (transparency) components of the color's HSV value.

These components can be retrieved individually using the hue(), saturation(), value() and alpha() functions.

See also setHsv() and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn getHsv_0<RetType, T: QColor_getHsv_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getHsv_0(self);
    // return 1;
  }
}
pub trait QColor_getHsv_0<RetType> {
  fn getHsv_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_getHsv_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn getHsv_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK6QColor6getHsvEPiS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:160
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHsv(int, int, int, int)

/*
Sets a HSV color value; h is the hue, s is the saturation, v is the value and a is the alpha component of the HSV color.

The saturation, value and alpha-channel values must be in the range 0-255, and the hue value must be greater than -1.

See also getHsv(), setHsvF(), and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn setHsv_0<RetType, T: QColor_setHsv_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHsv_0(self);
    // return 1;
  }
}
pub trait QColor_setHsv_0<RetType> {
  fn setHsv_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setHsv_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn setHsv_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor6setHsvEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:162
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getHsvF(qreal *, qreal *, qreal *, qreal *) const

/*
Sets the contents pointed to by h, s, v, and a, to the hue, saturation, value, and alpha-channel (transparency) components of the color's HSV value.

These components can be retrieved individually using the hueF(), saturationF(), valueF() and alphaF() functions.

See also setHsv() and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn getHsvF_0<RetType, T: QColor_getHsvF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getHsvF_0(self);
    // return 1;
  }
}
pub trait QColor_getHsvF_0<RetType> {
  fn getHsvF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_getHsvF_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn getHsvF_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK6QColor7getHsvFEPdS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:163
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHsvF(qreal, qreal, qreal, qreal)

/*
Sets a HSV color value; h is the hue, s is the saturation, v is the value and a is the alpha component of the HSV color.

All the values must be in the range 0.0-1.0.

See also getHsvF(), setHsv(), and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn setHsvF_0<RetType, T: QColor_setHsvF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHsvF_0(self);
    // return 1;
  }
}
pub trait QColor_setHsvF_0<RetType> {
  fn setHsvF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setHsvF_0<(/*void*/)> for (f64,f64,f64,f64) {
  fn setHsvF_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor7setHsvFEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:165
// index:0
// Public Visibility=Default Availability=Available
// [4] int cyan() const

/*
Returns the cyan color component of this color.

See also cyanF(), getCmyk(), and The CMYK Color Model.
*/
impl /*struct*/ QColor {
  pub fn cyan_0<RetType, T: QColor_cyan_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cyan_0(self);
    // return 1;
  }
}
pub trait QColor_cyan_0<RetType> {
  fn cyan_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_cyan_0<i32> for () {
  fn cyan_0(self , rsthis: & QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor4cyanEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:166
// index:0
// Public Visibility=Default Availability=Available
// [4] int magenta() const

/*
Returns the magenta color component of this color.

See also magentaF(), getCmyk(), and The CMYK Color Model.
*/
impl /*struct*/ QColor {
  pub fn magenta_0<RetType, T: QColor_magenta_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.magenta_0(self);
    // return 1;
  }
}
pub trait QColor_magenta_0<RetType> {
  fn magenta_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_magenta_0<i32> for () {
  fn magenta_0(self , rsthis: & QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor7magentaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:167
// index:0
// Public Visibility=Default Availability=Available
// [4] int yellow() const

/*
Returns the yellow color component of this color.

See also yellowF(), getCmyk(), and The CMYK Color Model.
*/
impl /*struct*/ QColor {
  pub fn yellow_0<RetType, T: QColor_yellow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.yellow_0(self);
    // return 1;
  }
}
pub trait QColor_yellow_0<RetType> {
  fn yellow_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_yellow_0<i32> for () {
  fn yellow_0(self , rsthis: & QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor6yellowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:168
// index:0
// Public Visibility=Default Availability=Available
// [4] int black() const

/*
Returns the black color component of this color.

See also blackF(), getCmyk(), and The CMYK Color Model.
*/
impl /*struct*/ QColor {
  pub fn black_0<RetType, T: QColor_black_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.black_0(self);
    // return 1;
  }
}
pub trait QColor_black_0<RetType> {
  fn black_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_black_0<i32> for () {
  fn black_0(self , rsthis: & QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor5blackEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:170
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal cyanF() const

/*
Returns the cyan color component of this color.

See also cyan(), getCmykF(), and The CMYK Color Model.
*/
impl /*struct*/ QColor {
  pub fn cyanF_0<RetType, T: QColor_cyanF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cyanF_0(self);
    // return 1;
  }
}
pub trait QColor_cyanF_0<RetType> {
  fn cyanF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_cyanF_0<f64> for () {
  fn cyanF_0(self , rsthis: & QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor5cyanFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:171
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal magentaF() const

/*
Returns the magenta color component of this color.

See also magenta(), getCmykF(), and The CMYK Color Model.
*/
impl /*struct*/ QColor {
  pub fn magentaF_0<RetType, T: QColor_magentaF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.magentaF_0(self);
    // return 1;
  }
}
pub trait QColor_magentaF_0<RetType> {
  fn magentaF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_magentaF_0<f64> for () {
  fn magentaF_0(self , rsthis: & QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor8magentaFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:172
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal yellowF() const

/*
Returns the yellow color component of this color.

See also yellow(), getCmykF(), and The CMYK Color Model.
*/
impl /*struct*/ QColor {
  pub fn yellowF_0<RetType, T: QColor_yellowF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.yellowF_0(self);
    // return 1;
  }
}
pub trait QColor_yellowF_0<RetType> {
  fn yellowF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_yellowF_0<f64> for () {
  fn yellowF_0(self , rsthis: & QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor7yellowFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:173
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal blackF() const

/*
Returns the black color component of this color.

See also black(), getCmykF(), and The CMYK Color Model.
*/
impl /*struct*/ QColor {
  pub fn blackF_0<RetType, T: QColor_blackF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blackF_0(self);
    // return 1;
  }
}
pub trait QColor_blackF_0<RetType> {
  fn blackF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_blackF_0<f64> for () {
  fn blackF_0(self , rsthis: & QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor6blackFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:175
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getCmyk(int *, int *, int *, int *, int *)

/*
Sets the contents pointed to by c, m, y, k, and a, to the cyan, magenta, yellow, black, and alpha-channel (transparency) components of the color's CMYK value.

These components can be retrieved individually using the cyan(), magenta(), yellow(), black() and alpha() functions.

See also setCmyk() and The CMYK Color Model.
*/
impl /*struct*/ QColor {
  pub fn getCmyk_0<RetType, T: QColor_getCmyk_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getCmyk_0(self);
    // return 1;
  }
}
pub trait QColor_getCmyk_0<RetType> {
  fn getCmyk_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_getCmyk_0<(/*void*/)> for (usize,usize,usize,usize,usize) {
  fn getCmyk_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
    let arg4 = (&self.4) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor7getCmykEPiS0_S0_S0_S0_", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:176
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCmyk(int, int, int, int, int)

/*
Sets the color to CMYK values, c (cyan), m (magenta), y (yellow), k (black), and a (alpha-channel, i.e. transparency).

All the values must be in the range 0-255.

See also getCmyk(), setCmykF(), and The CMYK Color Model.
*/
impl /*struct*/ QColor {
  pub fn setCmyk_0<RetType, T: QColor_setCmyk_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCmyk_0(self);
    // return 1;
  }
}
pub trait QColor_setCmyk_0<RetType> {
  fn setCmyk_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setCmyk_0<(/*void*/)> for (i32,i32,i32,i32,i32) {
  fn setCmyk_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor7setCmykEiiiii", 5,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:178
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getCmykF(qreal *, qreal *, qreal *, qreal *, qreal *)

/*
Sets the contents pointed to by c, m, y, k, and a, to the cyan, magenta, yellow, black, and alpha-channel (transparency) components of the color's CMYK value.

These components can be retrieved individually using the cyanF(), magentaF(), yellowF(), blackF() and alphaF() functions.

See also setCmykF() and The CMYK Color Model.
*/
impl /*struct*/ QColor {
  pub fn getCmykF_0<RetType, T: QColor_getCmykF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getCmykF_0(self);
    // return 1;
  }
}
pub trait QColor_getCmykF_0<RetType> {
  fn getCmykF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_getCmykF_0<(/*void*/)> for (usize,usize,usize,usize,usize) {
  fn getCmykF_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
    let arg4 = (&self.4) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor8getCmykFEPdS0_S0_S0_S0_", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:179
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCmykF(qreal, qreal, qreal, qreal, qreal)

/*
This is an overloaded function.

Sets the color to CMYK values, c (cyan), m (magenta), y (yellow), k (black), and a (alpha-channel, i.e. transparency).

All the values must be in the range 0.0-1.0.

See also getCmykF(), setCmyk(), and The CMYK Color Model.
*/
impl /*struct*/ QColor {
  pub fn setCmykF_0<RetType, T: QColor_setCmykF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCmykF_0(self);
    // return 1;
  }
}
pub trait QColor_setCmykF_0<RetType> {
  fn setCmykF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setCmykF_0<(/*void*/)> for (f64,f64,f64,f64,f64) {
  fn setCmykF_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor8setCmykFEddddd", 5,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:181
// index:0
// Public Visibility=Default Availability=Available
// [4] int hslHue() const

/*
Returns the hue color component of this color.

This function was introduced in  Qt 4.6.

See also getHslF() and getHsl().
*/
impl /*struct*/ QColor {
  pub fn hslHue_0<RetType, T: QColor_hslHue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hslHue_0(self);
    // return 1;
  }
}
pub trait QColor_hslHue_0<RetType> {
  fn hslHue_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_hslHue_0<i32> for () {
  fn hslHue_0(self , rsthis: & QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor6hslHueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:182
// index:0
// Public Visibility=Default Availability=Available
// [4] int hslSaturation() const

/*
Returns the saturation color component of this color.

This function was introduced in  Qt 4.6.

See also saturationF(), getHsv(), and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn hslSaturation_0<RetType, T: QColor_hslSaturation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hslSaturation_0(self);
    // return 1;
  }
}
pub trait QColor_hslSaturation_0<RetType> {
  fn hslSaturation_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_hslSaturation_0<i32> for () {
  fn hslSaturation_0(self , rsthis: & QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor13hslSaturationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:183
// index:0
// Public Visibility=Default Availability=Available
// [4] int lightness() const

/*
Returns the lightness color component of this color.

This function was introduced in  Qt 4.6.

See also lightnessF() and getHsl().
*/
impl /*struct*/ QColor {
  pub fn lightness_0<RetType, T: QColor_lightness_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lightness_0(self);
    // return 1;
  }
}
pub trait QColor_lightness_0<RetType> {
  fn lightness_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_lightness_0<i32> for () {
  fn lightness_0(self , rsthis: & QColor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor9lightnessEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:185
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal hslHueF() const

/*
Returns the hue color component of this color.

This function was introduced in  Qt 4.6.

See also hue() and getHslF().
*/
impl /*struct*/ QColor {
  pub fn hslHueF_0<RetType, T: QColor_hslHueF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hslHueF_0(self);
    // return 1;
  }
}
pub trait QColor_hslHueF_0<RetType> {
  fn hslHueF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_hslHueF_0<f64> for () {
  fn hslHueF_0(self , rsthis: & QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor7hslHueFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:186
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal hslSaturationF() const

/*
Returns the saturation color component of this color.

This function was introduced in  Qt 4.6.

See also saturationF() and getHslF().
*/
impl /*struct*/ QColor {
  pub fn hslSaturationF_0<RetType, T: QColor_hslSaturationF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hslSaturationF_0(self);
    // return 1;
  }
}
pub trait QColor_hslSaturationF_0<RetType> {
  fn hslSaturationF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_hslSaturationF_0<f64> for () {
  fn hslSaturationF_0(self , rsthis: & QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor14hslSaturationFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:187
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal lightnessF() const

/*
Returns the lightness color component of this color.

This function was introduced in  Qt 4.6.

See also value() and getHslF().
*/
impl /*struct*/ QColor {
  pub fn lightnessF_0<RetType, T: QColor_lightnessF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lightnessF_0(self);
    // return 1;
  }
}
pub trait QColor_lightnessF_0<RetType> {
  fn lightnessF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_lightnessF_0<f64> for () {
  fn lightnessF_0(self , rsthis: & QColor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor10lightnessFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:189
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getHsl(int *, int *, int *, int *) const

/*
Sets the contents pointed to by h, s, l, and a, to the hue, saturation, lightness, and alpha-channel (transparency) components of the color's HSL value.

These components can be retrieved individually using the hslHue(), hslSaturation(), lightness() and alpha() functions.

This function was introduced in  Qt 4.6.

See also setHsl().
*/
impl /*struct*/ QColor {
  pub fn getHsl_0<RetType, T: QColor_getHsl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getHsl_0(self);
    // return 1;
  }
}
pub trait QColor_getHsl_0<RetType> {
  fn getHsl_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_getHsl_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn getHsl_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK6QColor6getHslEPiS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:190
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHsl(int, int, int, int)

/*
Sets a HSL color value; h is the hue, s is the saturation, l is the lightness and a is the alpha component of the HSL color.

The saturation, value and alpha-channel values must be in the range 0-255, and the hue value must be greater than -1.

This function was introduced in  Qt 4.6.

See also getHsl() and setHslF().
*/
impl /*struct*/ QColor {
  pub fn setHsl_0<RetType, T: QColor_setHsl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHsl_0(self);
    // return 1;
  }
}
pub trait QColor_setHsl_0<RetType> {
  fn setHsl_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setHsl_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn setHsl_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor6setHslEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:192
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getHslF(qreal *, qreal *, qreal *, qreal *) const

/*
Sets the contents pointed to by h, s, l, and a, to the hue, saturation, lightness, and alpha-channel (transparency) components of the color's HSL value.

These components can be retrieved individually using the hslHueF(), hslSaturationF(), lightnessF() and alphaF() functions.

This function was introduced in  Qt 4.6.

See also setHsl().
*/
impl /*struct*/ QColor {
  pub fn getHslF_0<RetType, T: QColor_getHslF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getHslF_0(self);
    // return 1;
  }
}
pub trait QColor_getHslF_0<RetType> {
  fn getHslF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_getHslF_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn getHslF_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK6QColor7getHslFEPdS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:193
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHslF(qreal, qreal, qreal, qreal)

/*
Sets a HSL color lightness; h is the hue, s is the saturation, l is the lightness and a is the alpha component of the HSL color.

All the values must be in the range 0.0-1.0.

This function was introduced in  Qt 4.6.

See also getHslF() and setHsl().
*/
impl /*struct*/ QColor {
  pub fn setHslF_0<RetType, T: QColor_setHslF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHslF_0(self);
    // return 1;
  }
}
pub trait QColor_setHslF_0<RetType> {
  fn setHslF_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_setHslF_0<(/*void*/)> for (f64,f64,f64,f64) {
  fn setHslF_0(self , rsthis: & QColor) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QColor7setHslFEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qcolor.h:195
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor toRgb() const

/*
Create and returns an RGB QColor based on this color.

See also fromRgb(), convertTo(), and isValid().
*/
impl /*struct*/ QColor {
  pub fn toRgb_0<RetType, T: QColor_toRgb_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toRgb_0(self);
    // return 1;
  }
}
pub trait QColor_toRgb_0<RetType> {
  fn toRgb_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_toRgb_0<usize> for () {
  fn toRgb_0(self , rsthis: & QColor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor5toRgbEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:196
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor toHsv() const

/*
Creates and returns an HSV QColor based on this color.

See also fromHsv(), convertTo(), isValid(), and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn toHsv_0<RetType, T: QColor_toHsv_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toHsv_0(self);
    // return 1;
  }
}
pub trait QColor_toHsv_0<RetType> {
  fn toHsv_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_toHsv_0<usize> for () {
  fn toHsv_0(self , rsthis: & QColor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor5toHsvEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:197
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor toCmyk() const

/*
Creates and returns a CMYK QColor based on this color.

See also fromCmyk(), convertTo(), isValid(), and The CMYK Color Model.
*/
impl /*struct*/ QColor {
  pub fn toCmyk_0<RetType, T: QColor_toCmyk_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toCmyk_0(self);
    // return 1;
  }
}
pub trait QColor_toCmyk_0<RetType> {
  fn toCmyk_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_toCmyk_0<usize> for () {
  fn toCmyk_0(self , rsthis: & QColor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor6toCmykEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:198
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor toHsl() const

/*
Creates and returns an HSL QColor based on this color.

See also fromHsl(), convertTo(), and isValid().
*/
impl /*struct*/ QColor {
  pub fn toHsl_0<RetType, T: QColor_toHsl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toHsl_0(self);
    // return 1;
  }
}
pub trait QColor_toHsl_0<RetType> {
  fn toHsl_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_toHsl_0<usize> for () {
  fn toHsl_0(self , rsthis: & QColor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor5toHslEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:200
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor convertTo(QColor::Spec) const

/*
Creates a copy of this color in the format specified by colorSpec.

See also spec(), toCmyk(), toHsv(), toRgb(), and isValid().
*/
impl /*struct*/ QColor {
  pub fn convertTo_0<RetType, T: QColor_convertTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.convertTo_0(self);
    // return 1;
  }
}
pub trait QColor_convertTo_0<RetType> {
  fn convertTo_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_convertTo_0<usize> for (i32) {
  fn convertTo_0(self , rsthis: & QColor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor9convertToENS_4SpecE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:202
// index:0
// Public static Visibility=Default Availability=Available
// [16] QColor fromRgb(QRgb)

/*
Static convenience function that returns a QColor constructed from the given QRgb value rgb.

The alpha component of rgb is ignored (i.e. it is automatically set to 255), use the fromRgba() function to include the alpha-channel specified by the given QRgb value.

See also fromRgba(), fromRgbF(), toRgb(), and isValid().
*/
impl /*struct*/ QColor {
  pub fn fromRgb_0<RetType, T: QColor_fromRgb_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRgb_0();
    // return 1;
  }
}
pub trait QColor_fromRgb_0<RetType> {
  fn fromRgb_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColor_fromRgb_0<usize> for (u32) {
  fn fromRgb_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColor7fromRgbEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:205
// index:1
// Public static Visibility=Default Availability=Available
// [16] QColor fromRgb(int, int, int, int)

/*
Static convenience function that returns a QColor constructed from the given QRgb value rgb.

The alpha component of rgb is ignored (i.e. it is automatically set to 255), use the fromRgba() function to include the alpha-channel specified by the given QRgb value.

See also fromRgba(), fromRgbF(), toRgb(), and isValid().
*/
impl /*struct*/ QColor {
  pub fn fromRgb_1<RetType, T: QColor_fromRgb_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRgb_1();
    // return 1;
  }
}
pub trait QColor_fromRgb_1<RetType> {
  fn fromRgb_1(self ) -> RetType;
}
impl<'a> /*trait*/ QColor_fromRgb_1<usize> for (i32,i32,i32,i32) {
  fn fromRgb_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColor7fromRgbEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:203
// index:0
// Public static Visibility=Default Availability=Available
// [16] QColor fromRgba(QRgb)

/*
Static convenience function that returns a QColor constructed from the given QRgb value rgba.

Unlike the fromRgb() function, the alpha-channel specified by the given QRgb value is included.

See also fromRgb(), fromRgba64(), and isValid().
*/
impl /*struct*/ QColor {
  pub fn fromRgba_0<RetType, T: QColor_fromRgba_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRgba_0();
    // return 1;
  }
}
pub trait QColor_fromRgba_0<RetType> {
  fn fromRgba_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColor_fromRgba_0<usize> for (u32) {
  fn fromRgba_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColor8fromRgbaEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:206
// index:0
// Public static Visibility=Default Availability=Available
// [16] QColor fromRgbF(qreal, qreal, qreal, qreal)

/*
Static convenience function that returns a QColor constructed from the RGB color values, r (red), g (green), b (blue), and a (alpha-channel, i.e. transparency).

All the values must be in the range 0.0-1.0.

See also fromRgb(), fromRgba64(), toRgb(), and isValid().
*/
impl /*struct*/ QColor {
  pub fn fromRgbF_0<RetType, T: QColor_fromRgbF_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRgbF_0();
    // return 1;
  }
}
pub trait QColor_fromRgbF_0<RetType> {
  fn fromRgbF_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColor_fromRgbF_0<usize> for (f64,f64,f64,f64) {
  fn fromRgbF_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColor8fromRgbFEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:208
// index:0
// Public static Visibility=Default Availability=Available
// [16] QColor fromRgba64(ushort, ushort, ushort, ushort)

/*
Static convenience function that returns a QColor constructed from the RGBA64 color values, r (red), g (green), b (blue), and a (alpha-channel, i.e. transparency).

This function was introduced in  Qt 5.6.

See also fromRgb(), fromRgbF(), toRgb(), and isValid().
*/
impl /*struct*/ QColor {
  pub fn fromRgba64_0<RetType, T: QColor_fromRgba64_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRgba64_0();
    // return 1;
  }
}
pub trait QColor_fromRgba64_0<RetType> {
  fn fromRgba64_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColor_fromRgba64_0<usize> for (u16,u16,u16,u16) {
  fn fromRgba64_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u16 as usize;
    let arg1 = (&self.1) as *const u16 as usize;
    let arg2 = (&self.2) as *const u16 as usize;
    let arg3 = (&self.3) as *const u16 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColor10fromRgba64Etttt", 4,qtrt::FFITY_UINT16,qtrt::FFITY_UINT16,qtrt::FFITY_UINT16,qtrt::FFITY_UINT16,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:209
// index:1
// Public static Visibility=Default Availability=Available
// [16] QColor fromRgba64(QRgba64)

/*
Static convenience function that returns a QColor constructed from the RGBA64 color values, r (red), g (green), b (blue), and a (alpha-channel, i.e. transparency).

This function was introduced in  Qt 5.6.

See also fromRgb(), fromRgbF(), toRgb(), and isValid().
*/
impl /*struct*/ QColor {
  pub fn fromRgba64_1<RetType, T: QColor_fromRgba64_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRgba64_1();
    // return 1;
  }
}
pub trait QColor_fromRgba64_1<RetType> {
  fn fromRgba64_1(self ) -> RetType;
}
impl<'a> /*trait*/ QColor_fromRgba64_1<usize> for (usize) {
  fn fromRgba64_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColor10fromRgba64E7QRgba64", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:211
// index:0
// Public static Visibility=Default Availability=Available
// [16] QColor fromHsv(int, int, int, int)

/*
Static convenience function that returns a QColor constructed from the HSV color values, h (hue), s (saturation), v (value), and a (alpha-channel, i.e. transparency).

The value of s, v, and a must all be in the range 0-255; the value of h must be in the range 0-359.

See also toHsv(), fromHsvF(), isValid(), and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn fromHsv_0<RetType, T: QColor_fromHsv_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromHsv_0();
    // return 1;
  }
}
pub trait QColor_fromHsv_0<RetType> {
  fn fromHsv_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColor_fromHsv_0<usize> for (i32,i32,i32,i32) {
  fn fromHsv_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColor7fromHsvEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:212
// index:0
// Public static Visibility=Default Availability=Available
// [16] QColor fromHsvF(qreal, qreal, qreal, qreal)

/*
This is an overloaded function.

Static convenience function that returns a QColor constructed from the HSV color values, h (hue), s (saturation), v (value), and a (alpha-channel, i.e. transparency).

All the values must be in the range 0.0-1.0.

See also toHsv(), fromHsv(), isValid(), and The HSV Color Model.
*/
impl /*struct*/ QColor {
  pub fn fromHsvF_0<RetType, T: QColor_fromHsvF_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromHsvF_0();
    // return 1;
  }
}
pub trait QColor_fromHsvF_0<RetType> {
  fn fromHsvF_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColor_fromHsvF_0<usize> for (f64,f64,f64,f64) {
  fn fromHsvF_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColor8fromHsvFEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:214
// index:0
// Public static Visibility=Default Availability=Available
// [16] QColor fromCmyk(int, int, int, int, int)

/*
Static convenience function that returns a QColor constructed from the given CMYK color values: c (cyan), m (magenta), y (yellow), k (black), and a (alpha-channel, i.e. transparency).

All the values must be in the range 0-255.

See also toCmyk(), fromCmykF(), isValid(), and The CMYK Color Model.
*/
impl /*struct*/ QColor {
  pub fn fromCmyk_0<RetType, T: QColor_fromCmyk_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromCmyk_0();
    // return 1;
  }
}
pub trait QColor_fromCmyk_0<RetType> {
  fn fromCmyk_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColor_fromCmyk_0<usize> for (i32,i32,i32,i32,i32) {
  fn fromCmyk_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColor8fromCmykEiiiii", 5,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:215
// index:0
// Public static Visibility=Default Availability=Available
// [16] QColor fromCmykF(qreal, qreal, qreal, qreal, qreal)

/*
This is an overloaded function.

Static convenience function that returns a QColor constructed from the given CMYK color values: c (cyan), m (magenta), y (yellow), k (black), and a (alpha-channel, i.e. transparency).

All the values must be in the range 0.0-1.0.

See also toCmyk(), fromCmyk(), isValid(), and The CMYK Color Model.
*/
impl /*struct*/ QColor {
  pub fn fromCmykF_0<RetType, T: QColor_fromCmykF_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromCmykF_0();
    // return 1;
  }
}
pub trait QColor_fromCmykF_0<RetType> {
  fn fromCmykF_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColor_fromCmykF_0<usize> for (f64,f64,f64,f64,f64) {
  fn fromCmykF_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColor9fromCmykFEddddd", 5,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:217
// index:0
// Public static Visibility=Default Availability=Available
// [16] QColor fromHsl(int, int, int, int)

/*
Static convenience function that returns a QColor constructed from the HSV color values, h (hue), s (saturation), l (lightness), and a (alpha-channel, i.e. transparency).

The value of s, l, and a must all be in the range 0-255; the value of h must be in the range 0-359.

This function was introduced in  Qt 4.6.

See also toHsl(), fromHslF(), and isValid().
*/
impl /*struct*/ QColor {
  pub fn fromHsl_0<RetType, T: QColor_fromHsl_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromHsl_0();
    // return 1;
  }
}
pub trait QColor_fromHsl_0<RetType> {
  fn fromHsl_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColor_fromHsl_0<usize> for (i32,i32,i32,i32) {
  fn fromHsl_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColor7fromHslEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:218
// index:0
// Public static Visibility=Default Availability=Available
// [16] QColor fromHslF(qreal, qreal, qreal, qreal)

/*
This is an overloaded function.

Static convenience function that returns a QColor constructed from the HSV color values, h (hue), s (saturation), l (lightness), and a (alpha-channel, i.e. transparency).

All the values must be in the range 0.0-1.0.

This function was introduced in  Qt 4.6.

See also toHsl(), fromHsl(), and isValid().
*/
impl /*struct*/ QColor {
  pub fn fromHslF_0<RetType, T: QColor_fromHslF_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromHslF_0();
    // return 1;
  }
}
pub trait QColor_fromHslF_0<RetType> {
  fn fromHslF_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColor_fromHslF_0<usize> for (f64,f64,f64,f64) {
  fn fromHslF_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColor8fromHslFEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:220
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor light(int) const

/*

*/
impl /*struct*/ QColor {
  pub fn light_0<RetType, T: QColor_light_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.light_0(self);
    // return 1;
  }
}
pub trait QColor_light_0<RetType> {
  fn light_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_light_0<usize> for (i32) {
  fn light_0(self , rsthis: & QColor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor5lightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:221
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor lighter(int) const

/*
Returns a lighter (or darker) color, but does not change this object.

If the factor is greater than 100, this functions returns a lighter color. Setting factor to 150 returns a color that is 50% brighter. If the factor is less than 100, the return color is darker, but we recommend using the darker() function for this purpose. If the factor is 0 or negative, the return value is unspecified.

The function converts the current RGB color to HSV, multiplies the value (V) component by factor and converts the color back to RGB.

This function was introduced in  Qt 4.3.

See also darker() and isValid().
*/
impl /*struct*/ QColor {
  pub fn lighter_0<RetType, T: QColor_lighter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lighter_0(self);
    // return 1;
  }
}
pub trait QColor_lighter_0<RetType> {
  fn lighter_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_lighter_0<usize> for (i32) {
  fn lighter_0(self , rsthis: & QColor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor7lighterEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:222
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor dark(int) const

/*

*/
impl /*struct*/ QColor {
  pub fn dark_0<RetType, T: QColor_dark_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dark_0(self);
    // return 1;
  }
}
pub trait QColor_dark_0<RetType> {
  fn dark_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_dark_0<usize> for (i32) {
  fn dark_0(self , rsthis: & QColor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor4darkEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:223
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor darker(int) const

/*
Returns a darker (or lighter) color, but does not change this object.

If the factor is greater than 100, this functions returns a darker color. Setting factor to 300 returns a color that has one-third the brightness. If the factor is less than 100, the return color is lighter, but we recommend using the lighter() function for this purpose. If the factor is 0 or negative, the return value is unspecified.

The function converts the current RGB color to HSV, divides the value (V) component by factor and converts the color back to RGB.

This function was introduced in  Qt 4.3.

See also lighter() and isValid().
*/
impl /*struct*/ QColor {
  pub fn darker_0<RetType, T: QColor_darker_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.darker_0(self);
    // return 1;
  }
}
pub trait QColor_darker_0<RetType> {
  fn darker_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_darker_0<usize> for (i32) {
  fn darker_0(self , rsthis: & QColor) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColor6darkerEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:225
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QColor &) const

/*

*/
impl /*struct*/ QColor {
  pub fn operator_equal_equal_0<RetType, T: QColor_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QColor_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QColor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColoreqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:226
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator!=(const QColor &) const

/*

*/
impl /*struct*/ QColor {
  pub fn operator_not_equal_0<RetType, T: QColor_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QColor_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QColor) -> RetType;
}
impl<'a> /*trait*/ QColor_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QColor) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QColorneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:231
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool isValidColor(const QString &)

/*
Returns true if the name is a valid color name and can be used to construct a valid QColor object, otherwise returns false.

It uses the same algorithm used in setNamedColor().

This function was introduced in  Qt 4.7.

See also setNamedColor().
*/
impl /*struct*/ QColor {
  pub fn isValidColor_0<RetType, T: QColor_isValidColor_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.isValidColor_0();
    // return 1;
  }
}
pub trait QColor_isValidColor_0<RetType> {
  fn isValidColor_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColor_isValidColor_0<bool> for (usize) {
  fn isValidColor_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColor12isValidColorERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:233
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool isValidColor(QStringView)

/*
Returns true if the name is a valid color name and can be used to construct a valid QColor object, otherwise returns false.

It uses the same algorithm used in setNamedColor().

This function was introduced in  Qt 4.7.

See also setNamedColor().
*/
impl /*struct*/ QColor {
  pub fn isValidColor_1<RetType, T: QColor_isValidColor_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isValidColor_1();
    // return 1;
  }
}
pub trait QColor_isValidColor_1<RetType> {
  fn isValidColor_1(self ) -> RetType;
}
impl<'a> /*trait*/ QColor_isValidColor_1<bool> for (usize) {
  fn isValidColor_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColor12isValidColorE11QStringView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qcolor.h:234
// index:2
// Public static Visibility=Default Availability=Available
// [1] bool isValidColor(QLatin1String)

/*
Returns true if the name is a valid color name and can be used to construct a valid QColor object, otherwise returns false.

It uses the same algorithm used in setNamedColor().

This function was introduced in  Qt 4.7.

See also setNamedColor().
*/
impl /*struct*/ QColor {
  pub fn isValidColor_2<RetType, T: QColor_isValidColor_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.isValidColor_2();
    // return 1;
  }
}
pub trait QColor_isValidColor_2<RetType> {
  fn isValidColor_2(self ) -> RetType;
}
impl<'a> /*trait*/ QColor_isValidColor_2<bool> for (usize) {
  fn isValidColor_2(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QColor12isValidColorE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


pub fn DeleteQColor(this :*mut QColor) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN6QColorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
The type of color specified, either RGB, HSV, CMYK or HSL.

ConstantValue
QColor::Rgb1
QColor::Hsv2
QColor::Cmyk3
QColor::Hsl4
QColor::Invalid0


See also spec() and convertTo().

*/
pub type QColor__Spec = i32;
// 
pub const QColor__Invalid :QColor__Spec = 0;
// 
pub const QColor__Rgb :QColor__Spec = 1;
// 
pub const QColor__Hsv :QColor__Spec = 2;
// 
pub const QColor__Cmyk :QColor__Spec = 3;
// 
pub const QColor__Hsl :QColor__Spec = 4;
pub fn QColor_SpecItemName(val: i32) ->String {
  match val {
     QColor__Invalid => // 0
     {return String::from("Invalid");}
     QColor__Rgb => // 1
     {return String::from("Rgb");}
     QColor__Hsv => // 2
     {return String::from("Hsv");}
     QColor__Cmyk => // 3
     {return String::from("Cmyk");}
     QColor__Hsl => // 4
     {return String::from("Hsl");}
  _ => {return format!("{}", val);}
}
}
pub fn QColor_SpecItemName_s(val: i32) ->String {
  //var nilthis *QColor
  //return nilthis.SpecItemName(val);
  return QColor_SpecItemName(val);
}


/*
How to format the output of the name() function



See also name().

*/
pub type QColor__NameFormat = i32;
// #RRGGBB A "#" character followed by three two-digit hexadecimal numbers (i.e. #RRGGBB).
pub const QColor__HexRgb :QColor__NameFormat = 0;
// #AARRGGBB A "#" character followed by four two-digit hexadecimal numbers (i.e. #AARRGGBB).
pub const QColor__HexArgb :QColor__NameFormat = 1;
pub fn QColor_NameFormatItemName(val: i32) ->String {
  match val {
     QColor__HexRgb => // 0
     {return String::from("HexRgb");}
     QColor__HexArgb => // 1
     {return String::from("HexArgb");}
  _ => {return format!("{}", val);}
}
}
pub fn QColor_NameFormatItemName_s(val: i32) ->String {
  //var nilthis *QColor
  //return nilthis.NameFormatItemName(val);
  return QColor_NameFormatItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
