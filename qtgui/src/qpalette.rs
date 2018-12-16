

// mod ::gui::QPalette
// package qtgui
// /usr/include/qt/QtGui/qpalette.h
// #include <qpalette.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 95
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
#[derive(Default)] // class sizeof(QPalette)=16
pub struct QPalette {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPalette_ITF interface {
//    QPalette_PTR() *QPalette
//}
//func (ptr *QPalette) QPalette_PTR() *QPalette { return ptr }

impl /*struct*/ QPalette {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPalette {
    return QPalette{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPalette {
//  type Target = QPaletteBASE;
//
//  fn deref(&self) -> &QPaletteBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPaletteBASE> for QPalette {
//  fn as_ref(& self) -> & QPaletteBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpalette.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPalette()

/*
Constructs a palette object that uses the application's default palette.

See also QApplication::setPalette() and QApplication::palette().
*/
// QPalette() ctx.fn_proto_cpp
impl /*struct*/ QPalette {
  pub fn QPalette_0<T: QPalette_QPalette_0>(value: T) -> QPalette {
    let rsthis = value.QPalette_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPalette_QPalette_0 {
  fn QPalette_0(self) -> QPalette;
}
// QPalette() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPalette_QPalette_0 for () {
  fn QPalette_0(self) -> QPalette {
    // unsafe{_ZN8QPaletteC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QPaletteC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPalette{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpalette.h:59
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QPalette(const QColor &)

/*
Constructs a palette object that uses the application's default palette.

See also QApplication::setPalette() and QApplication::palette().
*/
// QPalette(const QColor &) ctx.fn_proto_cpp
impl /*struct*/ QPalette {
  pub fn QPalette_1<T: QPalette_QPalette_1>(value: T) -> QPalette {
    let rsthis = value.QPalette_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPalette_QPalette_1 {
  fn QPalette_1(self) -> QPalette;
}
// QPalette(const QColor &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPalette_QPalette_1 for (usize) {
  fn QPalette_1(self) -> QPalette {
    // unsafe{_ZN8QPaletteC2ERK6QColor()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QPaletteC2ERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPalette{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpalette.h:60
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QPalette(Qt::GlobalColor)

/*
Constructs a palette object that uses the application's default palette.

See also QApplication::setPalette() and QApplication::palette().
*/
// QPalette(Qt::GlobalColor) ctx.fn_proto_cpp
impl /*struct*/ QPalette {
  pub fn QPalette_2<T: QPalette_QPalette_2>(value: T) -> QPalette {
    let rsthis = value.QPalette_2();
    return rsthis;
    // return 1;
  }
}

pub trait QPalette_QPalette_2 {
  fn QPalette_2(self) -> QPalette;
}
// QPalette(Qt::GlobalColor) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPalette_QPalette_2 for (i32) {
  fn QPalette_2(self) -> QPalette {
    // unsafe{_ZN8QPaletteC2EN2Qt11GlobalColorE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QPaletteC2EN2Qt11GlobalColorE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPalette{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpalette.h:61
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QPalette(const QColor &, const QColor &)

/*
Constructs a palette object that uses the application's default palette.

See also QApplication::setPalette() and QApplication::palette().
*/
// QPalette(const QColor &, const QColor &) ctx.fn_proto_cpp
impl /*struct*/ QPalette {
  pub fn QPalette_3<T: QPalette_QPalette_3>(value: T) -> QPalette {
    let rsthis = value.QPalette_3();
    return rsthis;
    // return 1;
  }
}

pub trait QPalette_QPalette_3 {
  fn QPalette_3(self) -> QPalette;
}
// QPalette(const QColor &, const QColor &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPalette_QPalette_3 for (usize,usize) {
  fn QPalette_3(self) -> QPalette {
    // unsafe{_ZN8QPaletteC2ERK6QColorS2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QPaletteC2ERK6QColorS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPalette{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpalette.h:62
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QPalette(const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &)

/*
Constructs a palette object that uses the application's default palette.

See also QApplication::setPalette() and QApplication::palette().
*/
// QPalette(const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &) ctx.fn_proto_cpp
impl /*struct*/ QPalette {
  pub fn QPalette_4<T: QPalette_QPalette_4>(value: T) -> QPalette {
    let rsthis = value.QPalette_4();
    return rsthis;
    // return 1;
  }
}

pub trait QPalette_QPalette_4 {
  fn QPalette_4(self) -> QPalette;
}
// QPalette(const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPalette_QPalette_4 for (usize,usize,usize,usize,usize,usize,usize,usize,usize) {
  fn QPalette_4(self) -> QPalette {
    // unsafe{_ZN8QPaletteC2ERK6QBrushS2_S2_S2_S2_S2_S2_S2_S2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
    let arg7 = (&self.7/*.qclsinst*/) as *const usize as usize;
    let arg8 = (&self.8/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QPaletteC2ERK6QBrushS2_S2_S2_S2_S2_S2_S2_S2_", 9,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,0,0,0,0,0,0,0);
    let rsthis = QPalette{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpalette.h:65
// index:5
// Public Visibility=Default Availability=Available
// [-2] void QPalette(const QColor &, const QColor &, const QColor &, const QColor &, const QColor &, const QColor &, const QColor &)

/*
Constructs a palette object that uses the application's default palette.

See also QApplication::setPalette() and QApplication::palette().
*/
// QPalette(const QColor &, const QColor &, const QColor &, const QColor &, const QColor &, const QColor &, const QColor &) ctx.fn_proto_cpp
impl /*struct*/ QPalette {
  pub fn QPalette_5<T: QPalette_QPalette_5>(value: T) -> QPalette {
    let rsthis = value.QPalette_5();
    return rsthis;
    // return 1;
  }
}

pub trait QPalette_QPalette_5 {
  fn QPalette_5(self) -> QPalette;
}
// QPalette(const QColor &, const QColor &, const QColor &, const QColor &, const QColor &, const QColor &, const QColor &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPalette_QPalette_5 for (usize,usize,usize,usize,usize,usize,usize) {
  fn QPalette_5(self) -> QPalette {
    // unsafe{_ZN8QPaletteC2ERK6QColorS2_S2_S2_S2_S2_S2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QPaletteC2ERK6QColorS2_S2_S2_S2_S2_S2_", 7,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    let rsthis = QPalette{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpalette.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QPalette()

/*

*/
pub fn DeleteQPalette(this :*mut QPalette) {
    // let rv = qtrt::InvokeQtFunc6("_ZN8QPaletteD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qpalette.h:69
// index:0
// Public Visibility=Default Availability=Available
// [16] QPalette & operator=(const QPalette &)

/*

*/
impl /*struct*/ QPalette {
  pub fn operator_equal_0<RetType, T: QPalette_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QPalette_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QPaletteaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:74
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QPalette & operator=(QPalette &&)

/*

*/
impl /*struct*/ QPalette {
  pub fn operator_equal_1<RetType, T: QPalette_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QPalette_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QPaletteaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QPalette &)

/*
Swaps this palette instance with other. This function is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QPalette {
  pub fn swap_0<RetType, T: QPalette_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QPalette_swap_0<RetType> {
  fn swap_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QPalette) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPalette4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpalette.h:104
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QPalette::ColorGroup currentColorGroup() const

/*
Returns the palette's current color group.

See also setCurrentColorGroup().
*/
impl /*struct*/ QPalette {
  pub fn currentColorGroup_0<RetType, T: QPalette_currentColorGroup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentColorGroup_0(self);
    // return 1;
  }
}
pub trait QPalette_currentColorGroup_0<RetType> {
  fn currentColorGroup_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_currentColorGroup_0<i32> for () {
  fn currentColorGroup_0(self , rsthis: & QPalette) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette17currentColorGroupEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:105
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setCurrentColorGroup(QPalette::ColorGroup)

/*
Set the palette's current color group to cg.

See also currentColorGroup().
*/
impl /*struct*/ QPalette {
  pub fn setCurrentColorGroup_0<RetType, T: QPalette_setCurrentColorGroup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentColorGroup_0(self);
    // return 1;
  }
}
pub trait QPalette_setCurrentColorGroup_0<RetType> {
  fn setCurrentColorGroup_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_setCurrentColorGroup_0<(/*void*/)> for (i32) {
  fn setCurrentColorGroup_0(self , rsthis: & QPalette) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPalette20setCurrentColorGroupENS_10ColorGroupE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpalette.h:107
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QColor & color(QPalette::ColorGroup, QPalette::ColorRole) const

/*
Returns the color in the specified color group, used for the given color role.

See also brush(), setColor(), and ColorRole.
*/
impl /*struct*/ QPalette {
  pub fn color_0<RetType, T: QPalette_color_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.color_0(self);
    // return 1;
  }
}
pub trait QPalette_color_0<RetType> {
  fn color_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_color_0<usize> for (i32,i32) {
  fn color_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette5colorENS_10ColorGroupENS_9ColorRoleE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:121
// index:1
// Public inline Visibility=Default Availability=Available
// [16] const QColor & color(QPalette::ColorRole) const

/*
Returns the color in the specified color group, used for the given color role.

See also brush(), setColor(), and ColorRole.
*/
impl /*struct*/ QPalette {
  pub fn color_1<RetType, T: QPalette_color_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.color_1(self);
    // return 1;
  }
}
pub trait QPalette_color_1<RetType> {
  fn color_1(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_color_1<usize> for (i32) {
  fn color_1(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette5colorENS_9ColorRoleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:109
// index:0
// Public Visibility=Default Availability=Available
// [8] const QBrush & brush(QPalette::ColorGroup, QPalette::ColorRole) const

/*
Returns the brush in the specified color group, used for the given color role.

See also color(), setBrush(), and ColorRole.
*/
impl /*struct*/ QPalette {
  pub fn brush_0<RetType, T: QPalette_brush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.brush_0(self);
    // return 1;
  }
}
pub trait QPalette_brush_0<RetType> {
  fn brush_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_brush_0<usize> for (i32,i32) {
  fn brush_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette5brushENS_10ColorGroupENS_9ColorRoleE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:122
// index:1
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & brush(QPalette::ColorRole) const

/*
Returns the brush in the specified color group, used for the given color role.

See also color(), setBrush(), and ColorRole.
*/
impl /*struct*/ QPalette {
  pub fn brush_1<RetType, T: QPalette_brush_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.brush_1(self);
    // return 1;
  }
}
pub trait QPalette_brush_1<RetType> {
  fn brush_1(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_brush_1<usize> for (i32) {
  fn brush_1(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette5brushENS_9ColorRoleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:110
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setColor(QPalette::ColorGroup, QPalette::ColorRole, const QColor &)

/*
Sets the color in the specified color group, used for the given color role, to the specified solid color.

See also setBrush(), color(), and ColorRole.
*/
impl /*struct*/ QPalette {
  pub fn setColor_0<RetType, T: QPalette_setColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColor_0(self);
    // return 1;
  }
}
pub trait QPalette_setColor_0<RetType> {
  fn setColor_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_setColor_0<(/*void*/)> for (i32,i32,usize) {
  fn setColor_0(self , rsthis: & QPalette) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPalette8setColorENS_10ColorGroupENS_9ColorRoleERK6QColor", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpalette.h:111
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setColor(QPalette::ColorRole, const QColor &)

/*
Sets the color in the specified color group, used for the given color role, to the specified solid color.

See also setBrush(), color(), and ColorRole.
*/
impl /*struct*/ QPalette {
  pub fn setColor_1<RetType, T: QPalette_setColor_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColor_1(self);
    // return 1;
  }
}
pub trait QPalette_setColor_1<RetType> {
  fn setColor_1(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_setColor_1<(/*void*/)> for (i32,usize) {
  fn setColor_1(self , rsthis: & QPalette) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPalette8setColorENS_9ColorRoleERK6QColor", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpalette.h:112
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBrush(QPalette::ColorRole, const QBrush &)

/*
Sets the brush for the given color role to the specified brush for all groups in the palette.

See also brush(), setColor(), and ColorRole.
*/
impl /*struct*/ QPalette {
  pub fn setBrush_0<RetType, T: QPalette_setBrush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBrush_0(self);
    // return 1;
  }
}
pub trait QPalette_setBrush_0<RetType> {
  fn setBrush_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_setBrush_0<(/*void*/)> for (i32,usize) {
  fn setBrush_0(self , rsthis: & QPalette) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPalette8setBrushENS_9ColorRoleERK6QBrush", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpalette.h:114
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setBrush(QPalette::ColorGroup, QPalette::ColorRole, const QBrush &)

/*
Sets the brush for the given color role to the specified brush for all groups in the palette.

See also brush(), setColor(), and ColorRole.
*/
impl /*struct*/ QPalette {
  pub fn setBrush_1<RetType, T: QPalette_setBrush_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBrush_1(self);
    // return 1;
  }
}
pub trait QPalette_setBrush_1<RetType> {
  fn setBrush_1(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_setBrush_1<(/*void*/)> for (i32,i32,usize) {
  fn setBrush_1(self , rsthis: & QPalette) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPalette8setBrushENS_10ColorGroupENS_9ColorRoleERK6QBrush", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpalette.h:113
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isBrushSet(QPalette::ColorGroup, QPalette::ColorRole) const

/*
Returns true if the ColorGroup cg and ColorRole cr has been set previously on this palette; otherwise returns false.

This function was introduced in  Qt 4.2.

See also setBrush().
*/
impl /*struct*/ QPalette {
  pub fn isBrushSet_0<RetType, T: QPalette_isBrushSet_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isBrushSet_0(self);
    // return 1;
  }
}
pub trait QPalette_isBrushSet_0<RetType> {
  fn isBrushSet_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_isBrushSet_0<bool> for (i32,i32) {
  fn isBrushSet_0(self , rsthis: & QPalette) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette10isBrushSetENS_10ColorGroupENS_9ColorRoleE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColorGroup(QPalette::ColorGroup, const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &, const QBrush &)

/*
Sets a the group at cg. You can pass either brushes, pixmaps or plain colors for windowText, button, light, dark, mid, text, bright_text, base and window.

See also QBrush.
*/
impl /*struct*/ QPalette {
  pub fn setColorGroup_0<RetType, T: QPalette_setColorGroup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColorGroup_0(self);
    // return 1;
  }
}
pub trait QPalette_setColorGroup_0<RetType> {
  fn setColorGroup_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_setColorGroup_0<(/*void*/)> for (i32,usize,usize,usize,usize,usize,usize,usize,usize,usize) {
  fn setColorGroup_0(self , rsthis: & QPalette) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
    let arg7 = (&self.7/*.qclsinst*/) as *const usize as usize;
    let arg8 = (&self.8/*.qclsinst*/) as *const usize as usize;
    let arg9 = (&self.9/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPalette13setColorGroupENS_10ColorGroupERK6QBrushS3_S3_S3_S3_S3_S3_S3_S3_", 10,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpalette.h:119
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEqual(QPalette::ColorGroup, QPalette::ColorGroup) const

/*
Returns true (usually quickly) if color group cg1 is equal to cg2; otherwise returns false.
*/
impl /*struct*/ QPalette {
  pub fn isEqual_0<RetType, T: QPalette_isEqual_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEqual_0(self);
    // return 1;
  }
}
pub trait QPalette_isEqual_0<RetType> {
  fn isEqual_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_isEqual_0<bool> for (i32,i32) {
  fn isEqual_0(self , rsthis: & QPalette) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette7isEqualENS_10ColorGroupES0_", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:123
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & foreground() const

/*

*/
impl /*struct*/ QPalette {
  pub fn foreground_0<RetType, T: QPalette_foreground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.foreground_0(self);
    // return 1;
  }
}
pub trait QPalette_foreground_0<RetType> {
  fn foreground_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_foreground_0<usize> for () {
  fn foreground_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette10foregroundEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:124
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & windowText() const

/*
Returns the window text (general foreground) brush of the current color group.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn windowText_0<RetType, T: QPalette_windowText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowText_0(self);
    // return 1;
  }
}
pub trait QPalette_windowText_0<RetType> {
  fn windowText_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_windowText_0<usize> for () {
  fn windowText_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette10windowTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:125
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & button() const

/*
Returns the button brush of the current color group.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn button_0<RetType, T: QPalette_button_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.button_0(self);
    // return 1;
  }
}
pub trait QPalette_button_0<RetType> {
  fn button_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_button_0<usize> for () {
  fn button_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette6buttonEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:126
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & light() const

/*
Returns the light brush of the current color group.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn light_0<RetType, T: QPalette_light_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.light_0(self);
    // return 1;
  }
}
pub trait QPalette_light_0<RetType> {
  fn light_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_light_0<usize> for () {
  fn light_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette5lightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:127
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & dark() const

/*
Returns the dark brush of the current color group.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn dark_0<RetType, T: QPalette_dark_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dark_0(self);
    // return 1;
  }
}
pub trait QPalette_dark_0<RetType> {
  fn dark_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_dark_0<usize> for () {
  fn dark_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette4darkEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:128
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & mid() const

/*
Returns the mid brush of the current color group.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn mid_0<RetType, T: QPalette_mid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mid_0(self);
    // return 1;
  }
}
pub trait QPalette_mid_0<RetType> {
  fn mid_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_mid_0<usize> for () {
  fn mid_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette3midEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:129
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & text() const

/*
Returns the text foreground brush of the current color group.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn text_0<RetType, T: QPalette_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QPalette_text_0<RetType> {
  fn text_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_text_0<usize> for () {
  fn text_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:130
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & base() const

/*
Returns the base brush of the current color group.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn base_0<RetType, T: QPalette_base_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.base_0(self);
    // return 1;
  }
}
pub trait QPalette_base_0<RetType> {
  fn base_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_base_0<usize> for () {
  fn base_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette4baseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:131
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & alternateBase() const

/*
Returns the alternate base brush of the current color group.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn alternateBase_0<RetType, T: QPalette_alternateBase_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alternateBase_0(self);
    // return 1;
  }
}
pub trait QPalette_alternateBase_0<RetType> {
  fn alternateBase_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_alternateBase_0<usize> for () {
  fn alternateBase_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette13alternateBaseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:132
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & toolTipBase() const

/*
Returns the tool tip base brush of the current color group. This brush is used by QToolTip and QWhatsThis.

Note: Tool tips use the Inactive color group of QPalette, because tool tips are not active windows.

This function was introduced in  Qt 4.4.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn toolTipBase_0<RetType, T: QPalette_toolTipBase_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolTipBase_0(self);
    // return 1;
  }
}
pub trait QPalette_toolTipBase_0<RetType> {
  fn toolTipBase_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_toolTipBase_0<usize> for () {
  fn toolTipBase_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette11toolTipBaseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:133
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & toolTipText() const

/*
Returns the tool tip text brush of the current color group. This brush is used by QToolTip and QWhatsThis.

Note: Tool tips use the Inactive color group of QPalette, because tool tips are not active windows.

This function was introduced in  Qt 4.4.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn toolTipText_0<RetType, T: QPalette_toolTipText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolTipText_0(self);
    // return 1;
  }
}
pub trait QPalette_toolTipText_0<RetType> {
  fn toolTipText_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_toolTipText_0<usize> for () {
  fn toolTipText_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette11toolTipTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:134
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & background() const

/*

*/
impl /*struct*/ QPalette {
  pub fn background_0<RetType, T: QPalette_background_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.background_0(self);
    // return 1;
  }
}
pub trait QPalette_background_0<RetType> {
  fn background_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_background_0<usize> for () {
  fn background_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette10backgroundEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:135
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & window() const

/*
Returns the window (general background) brush of the current color group.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn window_0<RetType, T: QPalette_window_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.window_0(self);
    // return 1;
  }
}
pub trait QPalette_window_0<RetType> {
  fn window_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_window_0<usize> for () {
  fn window_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette6windowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:136
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & midlight() const

/*
Returns the midlight brush of the current color group.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn midlight_0<RetType, T: QPalette_midlight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.midlight_0(self);
    // return 1;
  }
}
pub trait QPalette_midlight_0<RetType> {
  fn midlight_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_midlight_0<usize> for () {
  fn midlight_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette8midlightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:137
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & brightText() const

/*
Returns the bright text foreground brush of the current color group.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn brightText_0<RetType, T: QPalette_brightText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.brightText_0(self);
    // return 1;
  }
}
pub trait QPalette_brightText_0<RetType> {
  fn brightText_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_brightText_0<usize> for () {
  fn brightText_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette10brightTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:138
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & buttonText() const

/*
Returns the button text foreground brush of the current color group.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn buttonText_0<RetType, T: QPalette_buttonText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonText_0(self);
    // return 1;
  }
}
pub trait QPalette_buttonText_0<RetType> {
  fn buttonText_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_buttonText_0<usize> for () {
  fn buttonText_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette10buttonTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:139
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & shadow() const

/*
Returns the shadow brush of the current color group.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn shadow_0<RetType, T: QPalette_shadow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shadow_0(self);
    // return 1;
  }
}
pub trait QPalette_shadow_0<RetType> {
  fn shadow_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_shadow_0<usize> for () {
  fn shadow_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette6shadowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:140
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & highlight() const

/*
Returns the highlight brush of the current color group.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn highlight_0<RetType, T: QPalette_highlight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.highlight_0(self);
    // return 1;
  }
}
pub trait QPalette_highlight_0<RetType> {
  fn highlight_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_highlight_0<usize> for () {
  fn highlight_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette9highlightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:141
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & highlightedText() const

/*
Returns the highlighted text brush of the current color group.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn highlightedText_0<RetType, T: QPalette_highlightedText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.highlightedText_0(self);
    // return 1;
  }
}
pub trait QPalette_highlightedText_0<RetType> {
  fn highlightedText_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_highlightedText_0<usize> for () {
  fn highlightedText_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette15highlightedTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:142
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & link() const

/*
Returns the unvisited link text brush of the current color group.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn link_0<RetType, T: QPalette_link_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.link_0(self);
    // return 1;
  }
}
pub trait QPalette_link_0<RetType> {
  fn link_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_link_0<usize> for () {
  fn link_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette4linkEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:143
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QBrush & linkVisited() const

/*
Returns the visited link text brush of the current color group.

See also ColorRole and brush().
*/
impl /*struct*/ QPalette {
  pub fn linkVisited_0<RetType, T: QPalette_linkVisited_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.linkVisited_0(self);
    // return 1;
  }
}
pub trait QPalette_linkVisited_0<RetType> {
  fn linkVisited_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_linkVisited_0<usize> for () {
  fn linkVisited_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette11linkVisitedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:145
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QPalette &) const

/*

*/
impl /*struct*/ QPalette {
  pub fn operator_equal_equal_0<RetType, T: QPalette_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QPalette_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QPalette) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPaletteeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:146
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QPalette &) const

/*

*/
impl /*struct*/ QPalette {
  pub fn operator_not_equal_0<RetType, T: QPalette_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QPalette_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QPalette) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPaletteneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:147
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isCopyOf(const QPalette &) const

/*
Returns true if this palette and p are copies of each other, i.e. one of them was created as a copy of the other and neither was subsequently modified; otherwise returns false. This is much stricter than equality.

See also operator=() and operator==().
*/
impl /*struct*/ QPalette {
  pub fn isCopyOf_0<RetType, T: QPalette_isCopyOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCopyOf_0(self);
    // return 1;
  }
}
pub trait QPalette_isCopyOf_0<RetType> {
  fn isCopyOf_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_isCopyOf_0<bool> for (usize) {
  fn isCopyOf_0(self , rsthis: & QPalette) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette8isCopyOfERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:152
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 cacheKey() const

/*
Returns a number that identifies the contents of this QPalette object. Distinct QPalette objects can have the same key if they refer to the same contents.

The cacheKey() will change when the palette is altered.
*/
impl /*struct*/ QPalette {
  pub fn cacheKey_0<RetType, T: QPalette_cacheKey_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cacheKey_0(self);
    // return 1;
  }
}
pub trait QPalette_cacheKey_0<RetType> {
  fn cacheKey_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_cacheKey_0<i64> for () {
  fn cacheKey_0(self , rsthis: & QPalette) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette8cacheKeyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:154
// index:0
// Public Visibility=Default Availability=Available
// [16] QPalette resolve(const QPalette &) const

/*
Returns a new QPalette that has attributes copied from other.
*/
impl /*struct*/ QPalette {
  pub fn resolve_0<RetType, T: QPalette_resolve_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resolve_0(self);
    // return 1;
  }
}
pub trait QPalette_resolve_0<RetType> {
  fn resolve_0(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_resolve_0<usize> for (usize) {
  fn resolve_0(self , rsthis: & QPalette) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette7resolveERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:155
// index:1
// Public inline Visibility=Default Availability=Available
// [4] uint resolve() const

/*
Returns a new QPalette that has attributes copied from other.
*/
impl /*struct*/ QPalette {
  pub fn resolve_1<RetType, T: QPalette_resolve_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resolve_1(self);
    // return 1;
  }
}
pub trait QPalette_resolve_1<RetType> {
  fn resolve_1(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_resolve_1<u32> for () {
  fn resolve_1(self , rsthis: & QPalette) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPalette7resolveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpalette.h:156
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void resolve(uint)

/*
Returns a new QPalette that has attributes copied from other.
*/
impl /*struct*/ QPalette {
  pub fn resolve_2<RetType, T: QPalette_resolve_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resolve_2(self);
    // return 1;
  }
}
pub trait QPalette_resolve_2<RetType> {
  fn resolve_2(self , rsthis: & QPalette) -> RetType;
}
impl<'a> /*trait*/ QPalette_resolve_2<(/*void*/)> for (u32) {
  fn resolve_2(self , rsthis: & QPalette) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPalette7resolveEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
QPalette::NormalActivesynonym for Active

*/
pub type QPalette__ColorGroup = i32;
//  
pub const QPalette__Active :QPalette__ColorGroup = 0;
//  
pub const QPalette__Disabled :QPalette__ColorGroup = 1;
//  
pub const QPalette__Inactive :QPalette__ColorGroup = 2;
// 
pub const QPalette__NColorGroups :QPalette__ColorGroup = 3;
// 
pub const QPalette__Current :QPalette__ColorGroup = 4;
// 
pub const QPalette__All :QPalette__ColorGroup = 5;
// 
pub const QPalette__Normal :QPalette__ColorGroup = 0;
pub fn QPalette_ColorGroupItemName(val: i32) ->String {
  match val {
     QPalette__Active => // 0
     {return String::from("Active,Normal");}
     QPalette__Disabled => // 1
     {return String::from("Disabled");}
     QPalette__Inactive => // 2
     {return String::from("Inactive");}
     QPalette__NColorGroups => // 3
     {return String::from("NColorGroups");}
     QPalette__Current => // 4
     {return String::from("Current");}
     QPalette__All => // 5
     {return String::from("All");}
    // QPalette__Normal => // 0
    // {return String::from("");}
  _ => {return format!("{}", val);}
}
}
pub fn QPalette_ColorGroupItemName_s(val: i32) ->String {
  //var nilthis *QPalette
  //return nilthis.ColorGroupItemName(val);
  return QPalette_ColorGroupItemName(val);
}


/*
The ColorRole enum defines the different symbolic color roles used in current GUIs.

The central roles are:

QPalette::BackgroundWindowThis value is obsolete. Use Window instead.
QPalette::ForegroundWindowTextThis value is obsolete. Use WindowText instead.


There are some color roles used mostly for 3D bevel and shadow effects. All of these are normally derived from Window, and used in ways that depend on that relationship. For example, buttons depend on it to make the bevels look attractive, and Motif scroll bars depend on Mid to be slightly different from Window.



Selected (marked) items have two roles:



There are two color roles related to hyperlinks:



Note that we do not use the Link and LinkVisited roles when rendering rich text in Qt, and that we recommend that you use CSS and the QTextDocument::setDefaultStyleSheet() function to alter the appearance of links. For example:


      QTextBrowser browser;
      QColor linkColor(Qt::red);
      browser.document()->setDefaultStyleSheet(sheet);




*/
pub type QPalette__ColorRole = i32;
// A general foreground color.
pub const QPalette__WindowText :QPalette__ColorRole = 0;
// The general button background color. This background can be different from Window as some styles require a different background color for buttons.
pub const QPalette__Button :QPalette__ColorRole = 1;
// Lighter than Button color.
pub const QPalette__Light :QPalette__ColorRole = 2;
// Between Button and Light.
pub const QPalette__Midlight :QPalette__ColorRole = 3;
// Darker than Button.
pub const QPalette__Dark :QPalette__ColorRole = 4;
// Between Button and Dark.
pub const QPalette__Mid :QPalette__ColorRole = 5;
// The foreground color used with Base. This is usually the same as the WindowText, in which case it must provide good contrast with Window and Base.
pub const QPalette__Text :QPalette__ColorRole = 6;
// A text color that is very different from WindowText, and contrasts well with e.g. Dark. Typically used for text that needs to be drawn where Text or WindowText would give poor contrast, such as on pressed push buttons. Note that text colors can be used for things other than just words; text colors are usually used for text, but it's quite common to use the text color roles for lines, icons, etc.
pub const QPalette__BrightText :QPalette__ColorRole = 7;
// A foreground color used with the Button color.
pub const QPalette__ButtonText :QPalette__ColorRole = 8;
// Used mostly as the background color for text entry widgets, but can also be used for other painting - such as the background of combobox drop down lists and toolbar handles. It is usually white or another light color.
pub const QPalette__Base :QPalette__ColorRole = 9;
// 
pub const QPalette__Window :QPalette__ColorRole = 10;
// 
pub const QPalette__Shadow :QPalette__ColorRole = 11;
// 
pub const QPalette__Highlight :QPalette__ColorRole = 12;
// 
pub const QPalette__HighlightedText :QPalette__ColorRole = 13;
// 
pub const QPalette__Link :QPalette__ColorRole = 14;
// 
pub const QPalette__LinkVisited :QPalette__ColorRole = 15;
// 
pub const QPalette__AlternateBase :QPalette__ColorRole = 16;
// 
pub const QPalette__NoRole :QPalette__ColorRole = 17;
// 
pub const QPalette__ToolTipBase :QPalette__ColorRole = 18;
// 
pub const QPalette__ToolTipText :QPalette__ColorRole = 19;
// 
pub const QPalette__NColorRoles :QPalette__ColorRole = 20;
// 
pub const QPalette__Foreground :QPalette__ColorRole = 0;
// 
pub const QPalette__Background :QPalette__ColorRole = 10;
pub fn QPalette_ColorRoleItemName(val: i32) ->String {
  match val {
     QPalette__WindowText => // 0
     {return String::from("WindowText,Foreground");}
     QPalette__Button => // 1
     {return String::from("Button");}
     QPalette__Light => // 2
     {return String::from("Light");}
     QPalette__Midlight => // 3
     {return String::from("Midlight");}
     QPalette__Dark => // 4
     {return String::from("Dark");}
     QPalette__Mid => // 5
     {return String::from("Mid");}
     QPalette__Text => // 6
     {return String::from("Text");}
     QPalette__BrightText => // 7
     {return String::from("BrightText");}
     QPalette__ButtonText => // 8
     {return String::from("ButtonText");}
     QPalette__Base => // 9
     {return String::from("Base");}
     QPalette__Window => // 10
     {return String::from("Window,Background");}
     QPalette__Shadow => // 11
     {return String::from("Shadow");}
     QPalette__Highlight => // 12
     {return String::from("Highlight");}
     QPalette__HighlightedText => // 13
     {return String::from("HighlightedText");}
     QPalette__Link => // 14
     {return String::from("Link");}
     QPalette__LinkVisited => // 15
     {return String::from("LinkVisited");}
     QPalette__AlternateBase => // 16
     {return String::from("AlternateBase");}
     QPalette__NoRole => // 17
     {return String::from("NoRole");}
     QPalette__ToolTipBase => // 18
     {return String::from("ToolTipBase");}
     QPalette__ToolTipText => // 19
     {return String::from("ToolTipText");}
     QPalette__NColorRoles => // 20
     {return String::from("NColorRoles");}
    // QPalette__Foreground => // 0
    // {return String::from("");}
    // QPalette__Background => // 10
    // {return String::from("");}
  _ => {return format!("{}", val);}
}
}
pub fn QPalette_ColorRoleItemName_s(val: i32) ->String {
  //var nilthis *QPalette
  //return nilthis.ColorRoleItemName(val);
  return QPalette_ColorRoleItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
