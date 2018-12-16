

// mod ::gui::QPainter
// package qtgui
// /usr/include/qt/QtGui/qpainter.h
// #include <qpainter.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 16
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
#[derive(Default)] // class sizeof(QPainter)=8
pub struct QPainter {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPainter_ITF interface {
//    QPainter_PTR() *QPainter
//}
//func (ptr *QPainter) QPainter_PTR() *QPainter { return ptr }

impl /*struct*/ QPainter {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPainter {
    return QPainter{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPainter {
//  type Target = QPainterBASE;
//
//  fn deref(&self) -> &QPainterBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPainterBASE> for QPainter {
//  fn as_ref(& self) -> & QPainterBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpainter.h:124
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPainter()

/*
Constructs a painter.

See also begin() and end().
*/
// QPainter() ctx.fn_proto_cpp
impl /*struct*/ QPainter {
  pub fn QPainter_0<T: QPainter_QPainter_0>(value: T) -> QPainter {
    let rsthis = value.QPainter_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPainter_QPainter_0 {
  fn QPainter_0(self) -> QPainter;
}
// QPainter() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPainter_QPainter_0 for () {
  fn QPainter_0(self) -> QPainter {
    // unsafe{_ZN8QPainterC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QPainterC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPainter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:125
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QPainter(QPaintDevice *)

/*
Constructs a painter.

See also begin() and end().
*/
// QPainter(QPaintDevice *) ctx.fn_proto_cpp
impl /*struct*/ QPainter {
  pub fn QPainter_1<T: QPainter_QPainter_1>(value: T) -> QPainter {
    let rsthis = value.QPainter_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPainter_QPainter_1 {
  fn QPainter_1(self) -> QPainter;
}
// QPainter(QPaintDevice *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPainter_QPainter_1 for (usize) {
  fn QPainter_1(self) -> QPainter {
    // unsafe{_ZN8QPainterC2EP12QPaintDevice()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QPainterC2EP12QPaintDevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPainter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QPainter()

/*

*/
pub fn DeleteQPainter(this :*mut QPainter) {
    // let rv = qtrt::InvokeQtFunc6("_ZN8QPainterD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qpainter.h:128
// index:0
// Public Visibility=Default Availability=Available
// [8] QPaintDevice * device() const

/*
Returns the paint device on which this painter is currently painting, or 0 if the painter is not active.

See also isActive().
*/
impl /*struct*/ QPainter {
  pub fn device_0<RetType, T: QPainter_device_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.device_0(self);
    // return 1;
  }
}
pub trait QPainter_device_0<RetType> {
  fn device_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_device_0<usize> for () {
  fn device_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter6deviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:130
// index:0
// Public Visibility=Default Availability=Available
// [1] bool begin(QPaintDevice *)

/*
Begins painting the paint device and returns true if successful; otherwise returns false.

Notice that all painter settings (setPen(), setBrush() etc.) are reset to default values when begin() is called.

The errors that can occur are serious problems, such as these:


  painter->begin(0); // impossible - paint device cannot be 0

  QPixmap image(0, 0);
  painter->begin(&image); // impossible - image.isNull() == true;

  painter->begin(myWidget);
  painter2->begin(myWidget); // impossible - only one painter at a time



Note that most of the time, you can use one of the constructors instead of begin(), and that end() is automatically done at destruction.

Warning: A paint device can only be painted by one painter at a time.

Warning: Painting on a QImage with the format QImage::Format_Indexed8 is not supported.

See also end() and QPainter().
*/
impl /*struct*/ QPainter {
  pub fn begin_0<RetType, T: QPainter_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QPainter_begin_0<RetType> {
  fn begin_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_begin_0<bool> for (usize) {
  fn begin_0(self , rsthis: & QPainter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QPainter5beginEP12QPaintDevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:131
// index:0
// Public Visibility=Default Availability=Available
// [1] bool end()

/*
Ends painting. Any resources used while painting are released. You don't normally need to call this since it is called by the destructor.

Returns true if the painter is no longer active; otherwise returns false.

See also begin() and isActive().
*/
impl /*struct*/ QPainter {
  pub fn end_0<RetType, T: QPainter_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QPainter_end_0<RetType> {
  fn end_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_end_0<bool> for () {
  fn end_0(self , rsthis: & QPainter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QPainter3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:132
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isActive() const

/*
Returns true if begin() has been called and end() has not yet been called; otherwise returns false.

See also begin() and QPaintDevice::paintingActive().
*/
impl /*struct*/ QPainter {
  pub fn isActive_0<RetType, T: QPainter_isActive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isActive_0(self);
    // return 1;
  }
}
pub trait QPainter_isActive_0<RetType> {
  fn isActive_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_isActive_0<bool> for () {
  fn isActive_0(self , rsthis: & QPainter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter8isActiveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:134
// index:0
// Public Visibility=Default Availability=Available
// [-2] void initFrom(const QPaintDevice *)

/*

*/
impl /*struct*/ QPainter {
  pub fn initFrom_0<RetType, T: QPainter_initFrom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initFrom_0(self);
    // return 1;
  }
}
pub trait QPainter_initFrom_0<RetType> {
  fn initFrom_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_initFrom_0<(/*void*/)> for (usize) {
  fn initFrom_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8initFromEPK12QPaintDevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:180
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCompositionMode(QPainter::CompositionMode)

/*
Sets the composition mode to the given mode.

Warning: Only a QPainter operating on a QImage fully supports all composition modes. The RasterOp modes are supported for X11 as described in compositionMode().

See also compositionMode().
*/
impl /*struct*/ QPainter {
  pub fn setCompositionMode_0<RetType, T: QPainter_setCompositionMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCompositionMode_0(self);
    // return 1;
  }
}
pub trait QPainter_setCompositionMode_0<RetType> {
  fn setCompositionMode_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setCompositionMode_0<(/*void*/)> for (i32) {
  fn setCompositionMode_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter18setCompositionModeENS_15CompositionModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:181
// index:0
// Public Visibility=Default Availability=Available
// [4] QPainter::CompositionMode compositionMode() const

/*
Returns the current composition mode.

See also CompositionMode and setCompositionMode().
*/
impl /*struct*/ QPainter {
  pub fn compositionMode_0<RetType, T: QPainter_compositionMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.compositionMode_0(self);
    // return 1;
  }
}
pub trait QPainter_compositionMode_0<RetType> {
  fn compositionMode_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_compositionMode_0<i32> for () {
  fn compositionMode_0(self , rsthis: & QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter15compositionModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:183
// index:0
// Public Visibility=Default Availability=Available
// [16] const QFont & font() const

/*
Returns the currently set font used for drawing text.

See also setFont(), drawText(), and Settings.
*/
impl /*struct*/ QPainter {
  pub fn font_0<RetType, T: QPainter_font_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.font_0(self);
    // return 1;
  }
}
pub trait QPainter_font_0<RetType> {
  fn font_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_font_0<usize> for () {
  fn font_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter4fontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:184
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFont(const QFont &)

/*
Sets the painter's font to the given font.

This font is used by subsequent drawText() functions. The text color is the same as the pen color.

If you set a font that isn't available, Qt finds a close match. font() will return what you set using setFont() and fontInfo() returns the font actually being used (which may be the same).

See also font(), drawText(), and Settings.
*/
impl /*struct*/ QPainter {
  pub fn setFont_0<RetType, T: QPainter_setFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFont_0(self);
    // return 1;
  }
}
pub trait QPainter_setFont_0<RetType> {
  fn setFont_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setFont_0<(/*void*/)> for (usize) {
  fn setFont_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter7setFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:186
// index:0
// Public Visibility=Default Availability=Available
// [8] QFontMetrics fontMetrics() const

/*
Returns the font metrics for the painter if the painter is active. Otherwise, the return value is undefined.

See also font(), isActive(), and Settings.
*/
impl /*struct*/ QPainter {
  pub fn fontMetrics_0<RetType, T: QPainter_fontMetrics_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontMetrics_0(self);
    // return 1;
  }
}
pub trait QPainter_fontMetrics_0<RetType> {
  fn fontMetrics_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_fontMetrics_0<usize> for () {
  fn fontMetrics_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter11fontMetricsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:187
// index:0
// Public Visibility=Default Availability=Available
// [8] QFontInfo fontInfo() const

/*
Returns the font info for the painter if the painter is active. Otherwise, the return value is undefined.

See also font(), isActive(), and Settings.
*/
impl /*struct*/ QPainter {
  pub fn fontInfo_0<RetType, T: QPainter_fontInfo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontInfo_0(self);
    // return 1;
  }
}
pub trait QPainter_fontInfo_0<RetType> {
  fn fontInfo_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_fontInfo_0<usize> for () {
  fn fontInfo_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter8fontInfoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:189
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPen(const QColor &)

/*
Sets the painter's pen to be the given pen.

The pen defines how to draw lines and outlines, and it also defines the text color.

See also pen() and Settings.
*/
impl /*struct*/ QPainter {
  pub fn setPen_0<RetType, T: QPainter_setPen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPen_0(self);
    // return 1;
  }
}
pub trait QPainter_setPen_0<RetType> {
  fn setPen_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setPen_0<(/*void*/)> for (usize) {
  fn setPen_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter6setPenERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:190
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setPen(const QPen &)

/*
Sets the painter's pen to be the given pen.

The pen defines how to draw lines and outlines, and it also defines the text color.

See also pen() and Settings.
*/
impl /*struct*/ QPainter {
  pub fn setPen_1<RetType, T: QPainter_setPen_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPen_1(self);
    // return 1;
  }
}
pub trait QPainter_setPen_1<RetType> {
  fn setPen_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setPen_1<(/*void*/)> for (usize) {
  fn setPen_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter6setPenERK4QPen", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:191
// index:2
// Public Visibility=Default Availability=Available
// [-2] void setPen(Qt::PenStyle)

/*
Sets the painter's pen to be the given pen.

The pen defines how to draw lines and outlines, and it also defines the text color.

See also pen() and Settings.
*/
impl /*struct*/ QPainter {
  pub fn setPen_2<RetType, T: QPainter_setPen_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPen_2(self);
    // return 1;
  }
}
pub trait QPainter_setPen_2<RetType> {
  fn setPen_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setPen_2<(/*void*/)> for (i32) {
  fn setPen_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter6setPenEN2Qt8PenStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:192
// index:0
// Public Visibility=Default Availability=Available
// [8] const QPen & pen() const

/*
Returns the painter's current pen.

See also setPen() and Settings.
*/
impl /*struct*/ QPainter {
  pub fn pen_0<RetType, T: QPainter_pen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pen_0(self);
    // return 1;
  }
}
pub trait QPainter_pen_0<RetType> {
  fn pen_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_pen_0<usize> for () {
  fn pen_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter3penEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:194
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBrush(const QBrush &)

/*
Sets the painter's brush to the given brush.

The painter's brush defines how shapes are filled.

See also brush() and Settings.
*/
impl /*struct*/ QPainter {
  pub fn setBrush_0<RetType, T: QPainter_setBrush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBrush_0(self);
    // return 1;
  }
}
pub trait QPainter_setBrush_0<RetType> {
  fn setBrush_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setBrush_0<(/*void*/)> for (usize) {
  fn setBrush_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8setBrushERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:195
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setBrush(Qt::BrushStyle)

/*
Sets the painter's brush to the given brush.

The painter's brush defines how shapes are filled.

See also brush() and Settings.
*/
impl /*struct*/ QPainter {
  pub fn setBrush_1<RetType, T: QPainter_setBrush_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBrush_1(self);
    // return 1;
  }
}
pub trait QPainter_setBrush_1<RetType> {
  fn setBrush_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setBrush_1<(/*void*/)> for (i32) {
  fn setBrush_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8setBrushEN2Qt10BrushStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:196
// index:0
// Public Visibility=Default Availability=Available
// [8] const QBrush & brush() const

/*
Returns the painter's current brush.

See also QPainter::setBrush() and Settings.
*/
impl /*struct*/ QPainter {
  pub fn brush_0<RetType, T: QPainter_brush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.brush_0(self);
    // return 1;
  }
}
pub trait QPainter_brush_0<RetType> {
  fn brush_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_brush_0<usize> for () {
  fn brush_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter5brushEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:199
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBackgroundMode(Qt::BGMode)

/*
Sets the background mode of the painter to the given mode

Qt::TransparentMode (the default) draws stippled lines and text without setting the background pixels. Qt::OpaqueMode fills these space with the current background color.

Note that in order to draw a bitmap or pixmap transparently, you must use QPixmap::setMask().

See also backgroundMode(), setBackground(), and Settings.
*/
impl /*struct*/ QPainter {
  pub fn setBackgroundMode_0<RetType, T: QPainter_setBackgroundMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBackgroundMode_0(self);
    // return 1;
  }
}
pub trait QPainter_setBackgroundMode_0<RetType> {
  fn setBackgroundMode_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setBackgroundMode_0<(/*void*/)> for (i32) {
  fn setBackgroundMode_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter17setBackgroundModeEN2Qt6BGModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:200
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::BGMode backgroundMode() const

/*
Returns the current background mode.

See also setBackgroundMode() and Settings.
*/
impl /*struct*/ QPainter {
  pub fn backgroundMode_0<RetType, T: QPainter_backgroundMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.backgroundMode_0(self);
    // return 1;
  }
}
pub trait QPainter_backgroundMode_0<RetType> {
  fn backgroundMode_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_backgroundMode_0<i32> for () {
  fn backgroundMode_0(self , rsthis: & QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter14backgroundModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:202
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint brushOrigin() const

/*
Returns the currently set brush origin.

See also setBrushOrigin() and Settings.
*/
impl /*struct*/ QPainter {
  pub fn brushOrigin_0<RetType, T: QPainter_brushOrigin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.brushOrigin_0(self);
    // return 1;
  }
}
pub trait QPainter_brushOrigin_0<RetType> {
  fn brushOrigin_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_brushOrigin_0<usize> for () {
  fn brushOrigin_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter11brushOriginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:203
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBrushOrigin(int, int)

/*
Sets the brush origin to position.

The brush origin specifies the (0, 0) coordinate of the painter's brush.

Note that while the brushOrigin() was necessary to adopt the parent's background for a widget in Qt 3, this is no longer the case since the Qt 4 painter doesn't paint the background unless you explicitly tell it to do so by setting the widget's autoFillBackground property to true.

See also brushOrigin() and Settings.
*/
impl /*struct*/ QPainter {
  pub fn setBrushOrigin_0<RetType, T: QPainter_setBrushOrigin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBrushOrigin_0(self);
    // return 1;
  }
}
pub trait QPainter_setBrushOrigin_0<RetType> {
  fn setBrushOrigin_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setBrushOrigin_0<(/*void*/)> for (i32,i32) {
  fn setBrushOrigin_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter14setBrushOriginEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:204
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setBrushOrigin(const QPoint &)

/*
Sets the brush origin to position.

The brush origin specifies the (0, 0) coordinate of the painter's brush.

Note that while the brushOrigin() was necessary to adopt the parent's background for a widget in Qt 3, this is no longer the case since the Qt 4 painter doesn't paint the background unless you explicitly tell it to do so by setting the widget's autoFillBackground property to true.

See also brushOrigin() and Settings.
*/
impl /*struct*/ QPainter {
  pub fn setBrushOrigin_1<RetType, T: QPainter_setBrushOrigin_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBrushOrigin_1(self);
    // return 1;
  }
}
pub trait QPainter_setBrushOrigin_1<RetType> {
  fn setBrushOrigin_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setBrushOrigin_1<(/*void*/)> for (usize) {
  fn setBrushOrigin_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter14setBrushOriginERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:205
// index:2
// Public Visibility=Default Availability=Available
// [-2] void setBrushOrigin(const QPointF &)

/*
Sets the brush origin to position.

The brush origin specifies the (0, 0) coordinate of the painter's brush.

Note that while the brushOrigin() was necessary to adopt the parent's background for a widget in Qt 3, this is no longer the case since the Qt 4 painter doesn't paint the background unless you explicitly tell it to do so by setting the widget's autoFillBackground property to true.

See also brushOrigin() and Settings.
*/
impl /*struct*/ QPainter {
  pub fn setBrushOrigin_2<RetType, T: QPainter_setBrushOrigin_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBrushOrigin_2(self);
    // return 1;
  }
}
pub trait QPainter_setBrushOrigin_2<RetType> {
  fn setBrushOrigin_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setBrushOrigin_2<(/*void*/)> for (usize) {
  fn setBrushOrigin_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter14setBrushOriginERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:207
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBackground(const QBrush &)

/*
Sets the background brush of the painter to the given brush.

The background brush is the brush that is filled in when drawing opaque text, stippled lines and bitmaps. The background brush has no effect in transparent background mode (which is the default).

See also background(), setBackgroundMode(), and Settings.
*/
impl /*struct*/ QPainter {
  pub fn setBackground_0<RetType, T: QPainter_setBackground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBackground_0(self);
    // return 1;
  }
}
pub trait QPainter_setBackground_0<RetType> {
  fn setBackground_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setBackground_0<(/*void*/)> for (usize) {
  fn setBackground_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter13setBackgroundERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:208
// index:0
// Public Visibility=Default Availability=Available
// [8] const QBrush & background() const

/*
Returns the current background brush.

See also setBackground() and Settings.
*/
impl /*struct*/ QPainter {
  pub fn background_0<RetType, T: QPainter_background_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.background_0(self);
    // return 1;
  }
}
pub trait QPainter_background_0<RetType> {
  fn background_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_background_0<usize> for () {
  fn background_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter10backgroundEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:210
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal opacity() const

/*
Returns the opacity of the painter. The default value is 1.

This function was introduced in  Qt 4.2.

See also setOpacity().
*/
impl /*struct*/ QPainter {
  pub fn opacity_0<RetType, T: QPainter_opacity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.opacity_0(self);
    // return 1;
  }
}
pub trait QPainter_opacity_0<RetType> {
  fn opacity_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_opacity_0<f64> for () {
  fn opacity_0(self , rsthis: & QPainter) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter7opacityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:211
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOpacity(qreal)

/*
Sets the opacity of the painter to opacity. The value should be in the range 0.0 to 1.0, where 0.0 is fully transparent and 1.0 is fully opaque.

Opacity set on the painter will apply to all drawing operations individually.

This function was introduced in  Qt 4.2.

See also opacity().
*/
impl /*struct*/ QPainter {
  pub fn setOpacity_0<RetType, T: QPainter_setOpacity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOpacity_0(self);
    // return 1;
  }
}
pub trait QPainter_setOpacity_0<RetType> {
  fn setOpacity_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setOpacity_0<(/*void*/)> for (f64) {
  fn setOpacity_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter10setOpacityEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:214
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion clipRegion() const

/*
Returns the currently set clip region. Note that the clip region is given in logical coordinates.

Warning: QPainter does not store the combined clip explicitly as this is handled by the underlying QPaintEngine, so the path is recreated on demand and transformed to the current logical coordinate system. This is potentially an expensive operation.

See also setClipRegion(), clipPath(), and setClipping().
*/
impl /*struct*/ QPainter {
  pub fn clipRegion_0<RetType, T: QPainter_clipRegion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clipRegion_0(self);
    // return 1;
  }
}
pub trait QPainter_clipRegion_0<RetType> {
  fn clipRegion_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_clipRegion_0<usize> for () {
  fn clipRegion_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter10clipRegionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:215
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath clipPath() const

/*
Returns the current clip path in logical coordinates.

Warning: QPainter does not store the combined clip explicitly as this is handled by the underlying QPaintEngine, so the path is recreated on demand and transformed to the current logical coordinate system. This is potentially an expensive operation.

See also setClipPath(), clipRegion(), and setClipping().
*/
impl /*struct*/ QPainter {
  pub fn clipPath_0<RetType, T: QPainter_clipPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clipPath_0(self);
    // return 1;
  }
}
pub trait QPainter_clipPath_0<RetType> {
  fn clipPath_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_clipPath_0<usize> for () {
  fn clipPath_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter8clipPathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:217
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setClipRect(const QRectF &, Qt::ClipOperation)

/*
Enables clipping, and sets the clip region to the given rectangle using the given clip operation. The default operation is to replace the current clip rectangle.

Note that the clip rectangle is specified in logical (painter) coordinates.

See also clipRegion(), setClipping(), and Clipping.
*/
impl /*struct*/ QPainter {
  pub fn setClipRect_0<RetType, T: QPainter_setClipRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setClipRect_0(self);
    // return 1;
  }
}
pub trait QPainter_setClipRect_0<RetType> {
  fn setClipRect_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setClipRect_0<(/*void*/)> for (usize,i32) {
  fn setClipRect_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11setClipRectERK6QRectFN2Qt13ClipOperationE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:218
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setClipRect(const QRect &, Qt::ClipOperation)

/*
Enables clipping, and sets the clip region to the given rectangle using the given clip operation. The default operation is to replace the current clip rectangle.

Note that the clip rectangle is specified in logical (painter) coordinates.

See also clipRegion(), setClipping(), and Clipping.
*/
impl /*struct*/ QPainter {
  pub fn setClipRect_1<RetType, T: QPainter_setClipRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setClipRect_1(self);
    // return 1;
  }
}
pub trait QPainter_setClipRect_1<RetType> {
  fn setClipRect_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setClipRect_1<(/*void*/)> for (usize,i32) {
  fn setClipRect_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11setClipRectERK5QRectN2Qt13ClipOperationE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:219
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void setClipRect(int, int, int, int, Qt::ClipOperation)

/*
Enables clipping, and sets the clip region to the given rectangle using the given clip operation. The default operation is to replace the current clip rectangle.

Note that the clip rectangle is specified in logical (painter) coordinates.

See also clipRegion(), setClipping(), and Clipping.
*/
impl /*struct*/ QPainter {
  pub fn setClipRect_2<RetType, T: QPainter_setClipRect_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setClipRect_2(self);
    // return 1;
  }
}
pub trait QPainter_setClipRect_2<RetType> {
  fn setClipRect_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setClipRect_2<(/*void*/)> for (i32,i32,i32,i32,i32) {
  fn setClipRect_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11setClipRectEiiiiN2Qt13ClipOperationE", 5,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:221
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setClipRegion(const QRegion &, Qt::ClipOperation)

/*
Sets the clip region to the given region using the specified clip operation. The default clip operation is to replace the current clip region.

Note that the clip region is given in logical coordinates.

See also clipRegion(), setClipRect(), and Clipping.
*/
impl /*struct*/ QPainter {
  pub fn setClipRegion_0<RetType, T: QPainter_setClipRegion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setClipRegion_0(self);
    // return 1;
  }
}
pub trait QPainter_setClipRegion_0<RetType> {
  fn setClipRegion_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setClipRegion_0<(/*void*/)> for (usize,i32) {
  fn setClipRegion_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter13setClipRegionERK7QRegionN2Qt13ClipOperationE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:223
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setClipPath(const QPainterPath &, Qt::ClipOperation)

/*
Enables clipping, and sets the clip path for the painter to the given path, with the clip operation.

Note that the clip path is specified in logical (painter) coordinates.

See also clipPath(), clipRegion(), and Clipping.
*/
impl /*struct*/ QPainter {
  pub fn setClipPath_0<RetType, T: QPainter_setClipPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setClipPath_0(self);
    // return 1;
  }
}
pub trait QPainter_setClipPath_0<RetType> {
  fn setClipPath_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setClipPath_0<(/*void*/)> for (usize,i32) {
  fn setClipPath_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11setClipPathERK12QPainterPathN2Qt13ClipOperationE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:225
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setClipping(bool)

/*
Enables clipping if enable is true, or disables clipping if enable is false.

See also hasClipping() and Clipping.
*/
impl /*struct*/ QPainter {
  pub fn setClipping_0<RetType, T: QPainter_setClipping_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setClipping_0(self);
    // return 1;
  }
}
pub trait QPainter_setClipping_0<RetType> {
  fn setClipping_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setClipping_0<(/*void*/)> for (bool) {
  fn setClipping_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11setClippingEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:226
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasClipping() const

/*
Returns true if clipping has been set; otherwise returns false.

See also setClipping() and Clipping.
*/
impl /*struct*/ QPainter {
  pub fn hasClipping_0<RetType, T: QPainter_hasClipping_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasClipping_0(self);
    // return 1;
  }
}
pub trait QPainter_hasClipping_0<RetType> {
  fn hasClipping_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_hasClipping_0<bool> for () {
  fn hasClipping_0(self , rsthis: & QPainter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter11hasClippingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:228
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF clipBoundingRect() const

/*
Returns the bounding rectangle of the current clip if there is a clip; otherwise returns an empty rectangle. Note that the clip region is given in logical coordinates.

The bounding rectangle is not guaranteed to be tight.

This function was introduced in  Qt 4.8.

See also setClipRect(), setClipPath(), and setClipRegion().
*/
impl /*struct*/ QPainter {
  pub fn clipBoundingRect_0<RetType, T: QPainter_clipBoundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clipBoundingRect_0(self);
    // return 1;
  }
}
pub trait QPainter_clipBoundingRect_0<RetType> {
  fn clipBoundingRect_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_clipBoundingRect_0<usize> for () {
  fn clipBoundingRect_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter16clipBoundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:230
// index:0
// Public Visibility=Default Availability=Available
// [-2] void save()

/*
Saves the current painter state (pushes the state onto a stack). A save() must be followed by a corresponding restore(); the end() function unwinds the stack.

See also restore().
*/
impl /*struct*/ QPainter {
  pub fn save_0<RetType, T: QPainter_save_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.save_0(self);
    // return 1;
  }
}
pub trait QPainter_save_0<RetType> {
  fn save_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_save_0<(/*void*/)> for () {
  fn save_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QPainter4saveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:231
// index:0
// Public Visibility=Default Availability=Available
// [-2] void restore()

/*
Restores the current painter state (pops a saved state off the stack).

See also save().
*/
impl /*struct*/ QPainter {
  pub fn restore_0<RetType, T: QPainter_restore_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.restore_0(self);
    // return 1;
  }
}
pub trait QPainter_restore_0<RetType> {
  fn restore_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_restore_0<(/*void*/)> for () {
  fn restore_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QPainter7restoreEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:234
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMatrix(const QMatrix &, bool)

/*

*/
impl /*struct*/ QPainter {
  pub fn setMatrix_0<RetType, T: QPainter_setMatrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMatrix_0(self);
    // return 1;
  }
}
pub trait QPainter_setMatrix_0<RetType> {
  fn setMatrix_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setMatrix_0<(/*void*/)> for (usize,bool) {
  fn setMatrix_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9setMatrixERK7QMatrixb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:235
// index:0
// Public Visibility=Default Availability=Available
// [48] const QMatrix & matrix() const

/*

*/
impl /*struct*/ QPainter {
  pub fn matrix_0<RetType, T: QPainter_matrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.matrix_0(self);
    // return 1;
  }
}
pub trait QPainter_matrix_0<RetType> {
  fn matrix_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_matrix_0<usize> for () {
  fn matrix_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter6matrixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:236
// index:0
// Public Visibility=Default Availability=Available
// [48] const QMatrix & deviceMatrix() const

/*

*/
impl /*struct*/ QPainter {
  pub fn deviceMatrix_0<RetType, T: QPainter_deviceMatrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.deviceMatrix_0(self);
    // return 1;
  }
}
pub trait QPainter_deviceMatrix_0<RetType> {
  fn deviceMatrix_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_deviceMatrix_0<usize> for () {
  fn deviceMatrix_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter12deviceMatrixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:237
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resetMatrix()

/*

*/
impl /*struct*/ QPainter {
  pub fn resetMatrix_0<RetType, T: QPainter_resetMatrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetMatrix_0(self);
    // return 1;
  }
}
pub trait QPainter_resetMatrix_0<RetType> {
  fn resetMatrix_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_resetMatrix_0<(/*void*/)> for () {
  fn resetMatrix_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QPainter11resetMatrixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:239
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTransform(const QTransform &, bool)

/*
Sets the world transformation matrix. If combine is true, the specified transform is combined with the current matrix; otherwise it replaces the current matrix.

This function was introduced in  Qt 4.3.

See also transform() and setWorldTransform().
*/
impl /*struct*/ QPainter {
  pub fn setTransform_0<RetType, T: QPainter_setTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTransform_0(self);
    // return 1;
  }
}
pub trait QPainter_setTransform_0<RetType> {
  fn setTransform_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setTransform_0<(/*void*/)> for (usize,bool) {
  fn setTransform_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter12setTransformERK10QTransformb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:240
// index:0
// Public Visibility=Default Availability=Available
// [88] const QTransform & transform() const

/*
Returns the world transformation matrix.

See also setTransform() and worldTransform().
*/
impl /*struct*/ QPainter {
  pub fn transform_0<RetType, T: QPainter_transform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transform_0(self);
    // return 1;
  }
}
pub trait QPainter_transform_0<RetType> {
  fn transform_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_transform_0<usize> for () {
  fn transform_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter9transformEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:241
// index:0
// Public Visibility=Default Availability=Available
// [88] const QTransform & deviceTransform() const

/*
Returns the matrix that transforms from logical coordinates to device coordinates of the platform dependent paint device.

This function is only needed when using platform painting commands on the platform dependent handle (Qt::HANDLE), and the platform does not do transformations nativly.

The QPaintEngine::PaintEngineFeature enum can be queried to determine whether the platform performs the transformations or not.

See also worldTransform() and QPaintEngine::hasFeature().
*/
impl /*struct*/ QPainter {
  pub fn deviceTransform_0<RetType, T: QPainter_deviceTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.deviceTransform_0(self);
    // return 1;
  }
}
pub trait QPainter_deviceTransform_0<RetType> {
  fn deviceTransform_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_deviceTransform_0<usize> for () {
  fn deviceTransform_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter15deviceTransformEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:242
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resetTransform()

/*
Resets any transformations that were made using translate(), scale(), shear(), rotate(), setWorldTransform(), setViewport() and setWindow().

See also Coordinate Transformations.
*/
impl /*struct*/ QPainter {
  pub fn resetTransform_0<RetType, T: QPainter_resetTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetTransform_0(self);
    // return 1;
  }
}
pub trait QPainter_resetTransform_0<RetType> {
  fn resetTransform_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_resetTransform_0<(/*void*/)> for () {
  fn resetTransform_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QPainter14resetTransformEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:244
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWorldMatrix(const QMatrix &, bool)

/*

*/
impl /*struct*/ QPainter {
  pub fn setWorldMatrix_0<RetType, T: QPainter_setWorldMatrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWorldMatrix_0(self);
    // return 1;
  }
}
pub trait QPainter_setWorldMatrix_0<RetType> {
  fn setWorldMatrix_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setWorldMatrix_0<(/*void*/)> for (usize,bool) {
  fn setWorldMatrix_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter14setWorldMatrixERK7QMatrixb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:245
// index:0
// Public Visibility=Default Availability=Available
// [48] const QMatrix & worldMatrix() const

/*

*/
impl /*struct*/ QPainter {
  pub fn worldMatrix_0<RetType, T: QPainter_worldMatrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.worldMatrix_0(self);
    // return 1;
  }
}
pub trait QPainter_worldMatrix_0<RetType> {
  fn worldMatrix_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_worldMatrix_0<usize> for () {
  fn worldMatrix_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter11worldMatrixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:247
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWorldTransform(const QTransform &, bool)

/*
Sets the world transformation matrix. If combine is true, the specified matrix is combined with the current matrix; otherwise it replaces the current matrix.

See also worldTransform(), transform(), and setTransform().
*/
impl /*struct*/ QPainter {
  pub fn setWorldTransform_0<RetType, T: QPainter_setWorldTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWorldTransform_0(self);
    // return 1;
  }
}
pub trait QPainter_setWorldTransform_0<RetType> {
  fn setWorldTransform_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setWorldTransform_0<(/*void*/)> for (usize,bool) {
  fn setWorldTransform_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter17setWorldTransformERK10QTransformb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:248
// index:0
// Public Visibility=Default Availability=Available
// [88] const QTransform & worldTransform() const

/*
Returns the world transformation matrix.

See also setWorldTransform().
*/
impl /*struct*/ QPainter {
  pub fn worldTransform_0<RetType, T: QPainter_worldTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.worldTransform_0(self);
    // return 1;
  }
}
pub trait QPainter_worldTransform_0<RetType> {
  fn worldTransform_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_worldTransform_0<usize> for () {
  fn worldTransform_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter14worldTransformEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:250
// index:0
// Public Visibility=Default Availability=Available
// [48] QMatrix combinedMatrix() const

/*

*/
impl /*struct*/ QPainter {
  pub fn combinedMatrix_0<RetType, T: QPainter_combinedMatrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.combinedMatrix_0(self);
    // return 1;
  }
}
pub trait QPainter_combinedMatrix_0<RetType> {
  fn combinedMatrix_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_combinedMatrix_0<usize> for () {
  fn combinedMatrix_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter14combinedMatrixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:251
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform combinedTransform() const

/*
Returns the transformation matrix combining the current window/viewport and world transformation.

See also setWorldTransform(), setWindow(), and setViewport().
*/
impl /*struct*/ QPainter {
  pub fn combinedTransform_0<RetType, T: QPainter_combinedTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.combinedTransform_0(self);
    // return 1;
  }
}
pub trait QPainter_combinedTransform_0<RetType> {
  fn combinedTransform_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_combinedTransform_0<usize> for () {
  fn combinedTransform_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter17combinedTransformEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:253
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMatrixEnabled(bool)

/*

*/
impl /*struct*/ QPainter {
  pub fn setMatrixEnabled_0<RetType, T: QPainter_setMatrixEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMatrixEnabled_0(self);
    // return 1;
  }
}
pub trait QPainter_setMatrixEnabled_0<RetType> {
  fn setMatrixEnabled_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setMatrixEnabled_0<(/*void*/)> for (bool) {
  fn setMatrixEnabled_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter16setMatrixEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:254
// index:0
// Public Visibility=Default Availability=Available
// [1] bool matrixEnabled() const

/*

*/
impl /*struct*/ QPainter {
  pub fn matrixEnabled_0<RetType, T: QPainter_matrixEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.matrixEnabled_0(self);
    // return 1;
  }
}
pub trait QPainter_matrixEnabled_0<RetType> {
  fn matrixEnabled_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_matrixEnabled_0<bool> for () {
  fn matrixEnabled_0(self , rsthis: & QPainter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter13matrixEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:256
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWorldMatrixEnabled(bool)

/*
Enables transformations if enable is true, or disables transformations if enable is false. The world transformation matrix is not changed.

This function was introduced in  Qt 4.2.

See also worldMatrixEnabled(), worldTransform(), and Coordinate Transformations.
*/
impl /*struct*/ QPainter {
  pub fn setWorldMatrixEnabled_0<RetType, T: QPainter_setWorldMatrixEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWorldMatrixEnabled_0(self);
    // return 1;
  }
}
pub trait QPainter_setWorldMatrixEnabled_0<RetType> {
  fn setWorldMatrixEnabled_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setWorldMatrixEnabled_0<(/*void*/)> for (bool) {
  fn setWorldMatrixEnabled_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter21setWorldMatrixEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:257
// index:0
// Public Visibility=Default Availability=Available
// [1] bool worldMatrixEnabled() const

/*
Returns true if world transformation is enabled; otherwise returns false.

This function was introduced in  Qt 4.2.

See also setWorldMatrixEnabled(), worldTransform(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn worldMatrixEnabled_0<RetType, T: QPainter_worldMatrixEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.worldMatrixEnabled_0(self);
    // return 1;
  }
}
pub trait QPainter_worldMatrixEnabled_0<RetType> {
  fn worldMatrixEnabled_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_worldMatrixEnabled_0<bool> for () {
  fn worldMatrixEnabled_0(self , rsthis: & QPainter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter18worldMatrixEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:259
// index:0
// Public Visibility=Default Availability=Available
// [-2] void scale(qreal, qreal)

/*
Scales the coordinate system by (sx, sy).

See also setWorldTransform() and Coordinate Transformations.
*/
impl /*struct*/ QPainter {
  pub fn scale_0<RetType, T: QPainter_scale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scale_0(self);
    // return 1;
  }
}
pub trait QPainter_scale_0<RetType> {
  fn scale_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_scale_0<(/*void*/)> for (f64,f64) {
  fn scale_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter5scaleEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:260
// index:0
// Public Visibility=Default Availability=Available
// [-2] void shear(qreal, qreal)

/*
Shears the coordinate system by (sh, sv).

See also setWorldTransform() and Coordinate Transformations.
*/
impl /*struct*/ QPainter {
  pub fn shear_0<RetType, T: QPainter_shear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shear_0(self);
    // return 1;
  }
}
pub trait QPainter_shear_0<RetType> {
  fn shear_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_shear_0<(/*void*/)> for (f64,f64) {
  fn shear_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter5shearEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:261
// index:0
// Public Visibility=Default Availability=Available
// [-2] void rotate(qreal)

/*
Rotates the coordinate system clockwise. The given angle parameter is in degrees.

See also setWorldTransform() and Coordinate Transformations.
*/
impl /*struct*/ QPainter {
  pub fn rotate_0<RetType, T: QPainter_rotate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rotate_0(self);
    // return 1;
  }
}
pub trait QPainter_rotate_0<RetType> {
  fn rotate_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_rotate_0<(/*void*/)> for (f64) {
  fn rotate_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter6rotateEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:263
// index:0
// Public Visibility=Default Availability=Available
// [-2] void translate(const QPointF &)

/*
Translates the coordinate system by the given offset; i.e. the given offset is added to points.

See also setWorldTransform() and Coordinate Transformations.
*/
impl /*struct*/ QPainter {
  pub fn translate_0<RetType, T: QPainter_translate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_0(self);
    // return 1;
  }
}
pub trait QPainter_translate_0<RetType> {
  fn translate_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_translate_0<(/*void*/)> for (usize) {
  fn translate_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9translateERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:264
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void translate(const QPoint &)

/*
Translates the coordinate system by the given offset; i.e. the given offset is added to points.

See also setWorldTransform() and Coordinate Transformations.
*/
impl /*struct*/ QPainter {
  pub fn translate_1<RetType, T: QPainter_translate_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_1(self);
    // return 1;
  }
}
pub trait QPainter_translate_1<RetType> {
  fn translate_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_translate_1<(/*void*/)> for (usize) {
  fn translate_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9translateERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:265
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void translate(qreal, qreal)

/*
Translates the coordinate system by the given offset; i.e. the given offset is added to points.

See also setWorldTransform() and Coordinate Transformations.
*/
impl /*struct*/ QPainter {
  pub fn translate_2<RetType, T: QPainter_translate_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_2(self);
    // return 1;
  }
}
pub trait QPainter_translate_2<RetType> {
  fn translate_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_translate_2<(/*void*/)> for (f64,f64) {
  fn translate_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9translateEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:267
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect window() const

/*
Returns the window rectangle.

See also setWindow() and setViewTransformEnabled().
*/
impl /*struct*/ QPainter {
  pub fn window_0<RetType, T: QPainter_window_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.window_0(self);
    // return 1;
  }
}
pub trait QPainter_window_0<RetType> {
  fn window_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_window_0<usize> for () {
  fn window_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter6windowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:268
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindow(const QRect &)

/*
Sets the painter's window to the given rectangle, and enables view transformations.

The window rectangle is part of the view transformation. The window specifies the logical coordinate system. Its sister, the viewport(), specifies the device coordinate system.

The default window rectangle is the same as the device's rectangle.

See also window(), viewTransformEnabled(), and Window-Viewport Conversion.
*/
impl /*struct*/ QPainter {
  pub fn setWindow_0<RetType, T: QPainter_setWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindow_0(self);
    // return 1;
  }
}
pub trait QPainter_setWindow_0<RetType> {
  fn setWindow_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setWindow_0<(/*void*/)> for (usize) {
  fn setWindow_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9setWindowERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:269
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setWindow(int, int, int, int)

/*
Sets the painter's window to the given rectangle, and enables view transformations.

The window rectangle is part of the view transformation. The window specifies the logical coordinate system. Its sister, the viewport(), specifies the device coordinate system.

The default window rectangle is the same as the device's rectangle.

See also window(), viewTransformEnabled(), and Window-Viewport Conversion.
*/
impl /*struct*/ QPainter {
  pub fn setWindow_1<RetType, T: QPainter_setWindow_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindow_1(self);
    // return 1;
  }
}
pub trait QPainter_setWindow_1<RetType> {
  fn setWindow_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setWindow_1<(/*void*/)> for (i32,i32,i32,i32) {
  fn setWindow_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9setWindowEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:271
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect viewport() const

/*
Returns the viewport rectangle.

See also setViewport() and setViewTransformEnabled().
*/
impl /*struct*/ QPainter {
  pub fn viewport_0<RetType, T: QPainter_viewport_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewport_0(self);
    // return 1;
  }
}
pub trait QPainter_viewport_0<RetType> {
  fn viewport_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_viewport_0<usize> for () {
  fn viewport_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter8viewportEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:272
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setViewport(const QRect &)

/*
Sets the painter's viewport rectangle to the given rectangle, and enables view transformations.

The viewport rectangle is part of the view transformation. The viewport specifies the device coordinate system. Its sister, the window(), specifies the logical coordinate system.

The default viewport rectangle is the same as the device's rectangle.

See also viewport(), viewTransformEnabled(), and Window-Viewport Conversion.
*/
impl /*struct*/ QPainter {
  pub fn setViewport_0<RetType, T: QPainter_setViewport_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setViewport_0(self);
    // return 1;
  }
}
pub trait QPainter_setViewport_0<RetType> {
  fn setViewport_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setViewport_0<(/*void*/)> for (usize) {
  fn setViewport_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11setViewportERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:273
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setViewport(int, int, int, int)

/*
Sets the painter's viewport rectangle to the given rectangle, and enables view transformations.

The viewport rectangle is part of the view transformation. The viewport specifies the device coordinate system. Its sister, the window(), specifies the logical coordinate system.

The default viewport rectangle is the same as the device's rectangle.

See also viewport(), viewTransformEnabled(), and Window-Viewport Conversion.
*/
impl /*struct*/ QPainter {
  pub fn setViewport_1<RetType, T: QPainter_setViewport_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setViewport_1(self);
    // return 1;
  }
}
pub trait QPainter_setViewport_1<RetType> {
  fn setViewport_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setViewport_1<(/*void*/)> for (i32,i32,i32,i32) {
  fn setViewport_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11setViewportEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:275
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setViewTransformEnabled(bool)

/*
Enables view transformations if enable is true, or disables view transformations if enable is false.

See also viewTransformEnabled() and Window-Viewport Conversion.
*/
impl /*struct*/ QPainter {
  pub fn setViewTransformEnabled_0<RetType, T: QPainter_setViewTransformEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setViewTransformEnabled_0(self);
    // return 1;
  }
}
pub trait QPainter_setViewTransformEnabled_0<RetType> {
  fn setViewTransformEnabled_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setViewTransformEnabled_0<(/*void*/)> for (bool) {
  fn setViewTransformEnabled_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter23setViewTransformEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:276
// index:0
// Public Visibility=Default Availability=Available
// [1] bool viewTransformEnabled() const

/*
Returns true if view transformation is enabled; otherwise returns false.

See also setViewTransformEnabled() and worldTransform().
*/
impl /*struct*/ QPainter {
  pub fn viewTransformEnabled_0<RetType, T: QPainter_viewTransformEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewTransformEnabled_0(self);
    // return 1;
  }
}
pub trait QPainter_viewTransformEnabled_0<RetType> {
  fn viewTransformEnabled_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_viewTransformEnabled_0<bool> for () {
  fn viewTransformEnabled_0(self , rsthis: & QPainter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter20viewTransformEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:279
// index:0
// Public Visibility=Default Availability=Available
// [-2] void strokePath(const QPainterPath &, const QPen &)

/*
Draws the outline (strokes) the path path with the pen specified by pen

See also fillPath() and Drawing.
*/
impl /*struct*/ QPainter {
  pub fn strokePath_0<RetType, T: QPainter_strokePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.strokePath_0(self);
    // return 1;
  }
}
pub trait QPainter_strokePath_0<RetType> {
  fn strokePath_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_strokePath_0<(/*void*/)> for (usize,usize) {
  fn strokePath_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter10strokePathERK12QPainterPathRK4QPen", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:280
// index:0
// Public Visibility=Default Availability=Available
// [-2] void fillPath(const QPainterPath &, const QBrush &)

/*
Fills the given path using the given brush. The outline is not drawn.

Alternatively, you can specify a QColor instead of a QBrush; the QBrush constructor (taking a QColor argument) will automatically create a solid pattern brush.

See also drawPath().
*/
impl /*struct*/ QPainter {
  pub fn fillPath_0<RetType, T: QPainter_fillPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fillPath_0(self);
    // return 1;
  }
}
pub trait QPainter_fillPath_0<RetType> {
  fn fillPath_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_fillPath_0<(/*void*/)> for (usize,usize) {
  fn fillPath_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8fillPathERK12QPainterPathRK6QBrush", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:281
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawPath(const QPainterPath &)

/*
Draws the given painter path using the current pen for outline and the current brush for filling.


 
  QPainterPath path;
  path.moveTo(20, 80);
  path.lineTo(20, 30);
  path.cubicTo(80, 0, 50, 50, 80, 80);

  QPainter painter(this);
  painter.drawPath(path);





See also the Painter Paths example and the Vector Deformation example.
*/
impl /*struct*/ QPainter {
  pub fn drawPath_0<RetType, T: QPainter_drawPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPath_0(self);
    // return 1;
  }
}
pub trait QPainter_drawPath_0<RetType> {
  fn drawPath_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPath_0<(/*void*/)> for (usize) {
  fn drawPath_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8drawPathERK12QPainterPath", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:283
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void drawPoint(const QPointF &)

/*
Draws a single point at the given position using the current pen's color.

See also Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPoint_0<RetType, T: QPainter_drawPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPoint_0(self);
    // return 1;
  }
}
pub trait QPainter_drawPoint_0<RetType> {
  fn drawPoint_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPoint_0<(/*void*/)> for (usize) {
  fn drawPoint_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawPointERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:284
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawPoint(const QPoint &)

/*
Draws a single point at the given position using the current pen's color.

See also Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPoint_1<RetType, T: QPainter_drawPoint_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPoint_1(self);
    // return 1;
  }
}
pub trait QPainter_drawPoint_1<RetType> {
  fn drawPoint_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPoint_1<(/*void*/)> for (usize) {
  fn drawPoint_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawPointERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:285
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void drawPoint(int, int)

/*
Draws a single point at the given position using the current pen's color.

See also Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPoint_2<RetType, T: QPainter_drawPoint_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPoint_2(self);
    // return 1;
  }
}
pub trait QPainter_drawPoint_2<RetType> {
  fn drawPoint_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPoint_2<(/*void*/)> for (i32,i32) {
  fn drawPoint_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawPointEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:287
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawPoints(const QPointF *, int)

/*
Draws the first pointCount points in the array points using the current pen's color.

See also Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPoints_0<RetType, T: QPainter_drawPoints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPoints_0(self);
    // return 1;
  }
}
pub trait QPainter_drawPoints_0<RetType> {
  fn drawPoints_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPoints_0<(/*void*/)> for (usize,i32) {
  fn drawPoints_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter10drawPointsEPK7QPointFi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:288
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawPoints(const QPolygonF &)

/*
Draws the first pointCount points in the array points using the current pen's color.

See also Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPoints_1<RetType, T: QPainter_drawPoints_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPoints_1(self);
    // return 1;
  }
}
pub trait QPainter_drawPoints_1<RetType> {
  fn drawPoints_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPoints_1<(/*void*/)> for (usize) {
  fn drawPoints_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter10drawPointsERK9QPolygonF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:289
// index:2
// Public Visibility=Default Availability=Available
// [-2] void drawPoints(const QPoint *, int)

/*
Draws the first pointCount points in the array points using the current pen's color.

See also Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPoints_2<RetType, T: QPainter_drawPoints_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPoints_2(self);
    // return 1;
  }
}
pub trait QPainter_drawPoints_2<RetType> {
  fn drawPoints_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPoints_2<(/*void*/)> for (usize,i32) {
  fn drawPoints_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter10drawPointsEPK6QPointi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:290
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void drawPoints(const QPolygon &)

/*
Draws the first pointCount points in the array points using the current pen's color.

See also Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPoints_3<RetType, T: QPainter_drawPoints_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPoints_3(self);
    // return 1;
  }
}
pub trait QPainter_drawPoints_3<RetType> {
  fn drawPoints_3(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPoints_3<(/*void*/)> for (usize) {
  fn drawPoints_3(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter10drawPointsERK8QPolygon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:292
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void drawLine(const QLineF &)

/*
Draws a line defined by line.


 
  QLineF line(10.0, 80.0, 90.0, 20.0);

  QPainter(this);
  painter.drawLine(line);





See also drawLines(), drawPolyline(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawLine_0<RetType, T: QPainter_drawLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawLine_0(self);
    // return 1;
  }
}
pub trait QPainter_drawLine_0<RetType> {
  fn drawLine_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawLine_0<(/*void*/)> for (usize) {
  fn drawLine_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8drawLineERK6QLineF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:293
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawLine(const QLine &)

/*
Draws a line defined by line.


 
  QLineF line(10.0, 80.0, 90.0, 20.0);

  QPainter(this);
  painter.drawLine(line);





See also drawLines(), drawPolyline(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawLine_1<RetType, T: QPainter_drawLine_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawLine_1(self);
    // return 1;
  }
}
pub trait QPainter_drawLine_1<RetType> {
  fn drawLine_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawLine_1<(/*void*/)> for (usize) {
  fn drawLine_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8drawLineERK5QLine", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:294
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void drawLine(int, int, int, int)

/*
Draws a line defined by line.


 
  QLineF line(10.0, 80.0, 90.0, 20.0);

  QPainter(this);
  painter.drawLine(line);





See also drawLines(), drawPolyline(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawLine_2<RetType, T: QPainter_drawLine_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawLine_2(self);
    // return 1;
  }
}
pub trait QPainter_drawLine_2<RetType> {
  fn drawLine_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawLine_2<(/*void*/)> for (i32,i32,i32,i32) {
  fn drawLine_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8drawLineEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:295
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void drawLine(const QPoint &, const QPoint &)

/*
Draws a line defined by line.


 
  QLineF line(10.0, 80.0, 90.0, 20.0);

  QPainter(this);
  painter.drawLine(line);





See also drawLines(), drawPolyline(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawLine_3<RetType, T: QPainter_drawLine_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawLine_3(self);
    // return 1;
  }
}
pub trait QPainter_drawLine_3<RetType> {
  fn drawLine_3(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawLine_3<(/*void*/)> for (usize,usize) {
  fn drawLine_3(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8drawLineERK6QPointS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:296
// index:4
// Public inline Visibility=Default Availability=Available
// [-2] void drawLine(const QPointF &, const QPointF &)

/*
Draws a line defined by line.


 
  QLineF line(10.0, 80.0, 90.0, 20.0);

  QPainter(this);
  painter.drawLine(line);





See also drawLines(), drawPolyline(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawLine_4<RetType, T: QPainter_drawLine_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawLine_4(self);
    // return 1;
  }
}
pub trait QPainter_drawLine_4<RetType> {
  fn drawLine_4(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawLine_4<(/*void*/)> for (usize,usize) {
  fn drawLine_4(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8drawLineERK7QPointFS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:298
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawLines(const QLineF *, int)

/*
Draws the first lineCount lines in the array lines using the current pen.

See also drawLine() and drawPolyline().
*/
impl /*struct*/ QPainter {
  pub fn drawLines_0<RetType, T: QPainter_drawLines_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawLines_0(self);
    // return 1;
  }
}
pub trait QPainter_drawLines_0<RetType> {
  fn drawLines_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawLines_0<(/*void*/)> for (usize,i32) {
  fn drawLines_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawLinesEPK6QLineFi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:300
// index:1
// Public Visibility=Default Availability=Available
// [-2] void drawLines(const QPointF *, int)

/*
Draws the first lineCount lines in the array lines using the current pen.

See also drawLine() and drawPolyline().
*/
impl /*struct*/ QPainter {
  pub fn drawLines_1<RetType, T: QPainter_drawLines_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawLines_1(self);
    // return 1;
  }
}
pub trait QPainter_drawLines_1<RetType> {
  fn drawLines_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawLines_1<(/*void*/)> for (usize,i32) {
  fn drawLines_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawLinesEPK7QPointFi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:302
// index:2
// Public Visibility=Default Availability=Available
// [-2] void drawLines(const QLine *, int)

/*
Draws the first lineCount lines in the array lines using the current pen.

See also drawLine() and drawPolyline().
*/
impl /*struct*/ QPainter {
  pub fn drawLines_2<RetType, T: QPainter_drawLines_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawLines_2(self);
    // return 1;
  }
}
pub trait QPainter_drawLines_2<RetType> {
  fn drawLines_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawLines_2<(/*void*/)> for (usize,i32) {
  fn drawLines_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawLinesEPK5QLinei", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:304
// index:3
// Public Visibility=Default Availability=Available
// [-2] void drawLines(const QPoint *, int)

/*
Draws the first lineCount lines in the array lines using the current pen.

See also drawLine() and drawPolyline().
*/
impl /*struct*/ QPainter {
  pub fn drawLines_3<RetType, T: QPainter_drawLines_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawLines_3(self);
    // return 1;
  }
}
pub trait QPainter_drawLines_3<RetType> {
  fn drawLines_3(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawLines_3<(/*void*/)> for (usize,i32) {
  fn drawLines_3(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawLinesEPK6QPointi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:307
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void drawRect(const QRectF &)

/*
Draws the current rectangle with the current pen and brush.

A filled rectangle has a size of rectangle.size(). A stroked rectangle has a size of rectangle.size() plus the pen width.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);

  QPainter painter(this);
  painter.drawRect(rectangle);





See also drawRects(), drawPolygon(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawRect_0<RetType, T: QPainter_drawRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawRect_0(self);
    // return 1;
  }
}
pub trait QPainter_drawRect_0<RetType> {
  fn drawRect_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawRect_0<(/*void*/)> for (usize) {
  fn drawRect_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8drawRectERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:308
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawRect(int, int, int, int)

/*
Draws the current rectangle with the current pen and brush.

A filled rectangle has a size of rectangle.size(). A stroked rectangle has a size of rectangle.size() plus the pen width.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);

  QPainter painter(this);
  painter.drawRect(rectangle);





See also drawRects(), drawPolygon(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawRect_1<RetType, T: QPainter_drawRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawRect_1(self);
    // return 1;
  }
}
pub trait QPainter_drawRect_1<RetType> {
  fn drawRect_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawRect_1<(/*void*/)> for (i32,i32,i32,i32) {
  fn drawRect_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8drawRectEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:309
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void drawRect(const QRect &)

/*
Draws the current rectangle with the current pen and brush.

A filled rectangle has a size of rectangle.size(). A stroked rectangle has a size of rectangle.size() plus the pen width.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);

  QPainter painter(this);
  painter.drawRect(rectangle);





See also drawRects(), drawPolygon(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawRect_2<RetType, T: QPainter_drawRect_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawRect_2(self);
    // return 1;
  }
}
pub trait QPainter_drawRect_2<RetType> {
  fn drawRect_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawRect_2<(/*void*/)> for (usize) {
  fn drawRect_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8drawRectERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:311
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawRects(const QRectF *, int)

/*
Draws the first rectCount of the given rectangles using the current pen and brush.

See also drawRect().
*/
impl /*struct*/ QPainter {
  pub fn drawRects_0<RetType, T: QPainter_drawRects_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawRects_0(self);
    // return 1;
  }
}
pub trait QPainter_drawRects_0<RetType> {
  fn drawRects_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawRects_0<(/*void*/)> for (usize,i32) {
  fn drawRects_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawRectsEPK6QRectFi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:313
// index:1
// Public Visibility=Default Availability=Available
// [-2] void drawRects(const QRect *, int)

/*
Draws the first rectCount of the given rectangles using the current pen and brush.

See also drawRect().
*/
impl /*struct*/ QPainter {
  pub fn drawRects_1<RetType, T: QPainter_drawRects_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawRects_1(self);
    // return 1;
  }
}
pub trait QPainter_drawRects_1<RetType> {
  fn drawRects_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawRects_1<(/*void*/)> for (usize,i32) {
  fn drawRects_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawRectsEPK5QRecti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:316
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawEllipse(const QRectF &)

/*
Draws the ellipse defined by the given rectangle.

A filled ellipse has a size of rectangle.size(). A stroked ellipse has a size of rectangle.size() plus the pen width.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);

  QPainter painter(this);
  painter.drawEllipse(rectangle);





See also drawPie() and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawEllipse_0<RetType, T: QPainter_drawEllipse_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawEllipse_0(self);
    // return 1;
  }
}
pub trait QPainter_drawEllipse_0<RetType> {
  fn drawEllipse_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawEllipse_0<(/*void*/)> for (usize) {
  fn drawEllipse_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11drawEllipseERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:317
// index:1
// Public Visibility=Default Availability=Available
// [-2] void drawEllipse(const QRect &)

/*
Draws the ellipse defined by the given rectangle.

A filled ellipse has a size of rectangle.size(). A stroked ellipse has a size of rectangle.size() plus the pen width.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);

  QPainter painter(this);
  painter.drawEllipse(rectangle);





See also drawPie() and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawEllipse_1<RetType, T: QPainter_drawEllipse_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawEllipse_1(self);
    // return 1;
  }
}
pub trait QPainter_drawEllipse_1<RetType> {
  fn drawEllipse_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawEllipse_1<(/*void*/)> for (usize) {
  fn drawEllipse_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11drawEllipseERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:318
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void drawEllipse(int, int, int, int)

/*
Draws the ellipse defined by the given rectangle.

A filled ellipse has a size of rectangle.size(). A stroked ellipse has a size of rectangle.size() plus the pen width.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);

  QPainter painter(this);
  painter.drawEllipse(rectangle);





See also drawPie() and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawEllipse_2<RetType, T: QPainter_drawEllipse_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawEllipse_2(self);
    // return 1;
  }
}
pub trait QPainter_drawEllipse_2<RetType> {
  fn drawEllipse_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawEllipse_2<(/*void*/)> for (i32,i32,i32,i32) {
  fn drawEllipse_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11drawEllipseEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:320
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void drawEllipse(const QPointF &, qreal, qreal)

/*
Draws the ellipse defined by the given rectangle.

A filled ellipse has a size of rectangle.size(). A stroked ellipse has a size of rectangle.size() plus the pen width.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);

  QPainter painter(this);
  painter.drawEllipse(rectangle);





See also drawPie() and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawEllipse_3<RetType, T: QPainter_drawEllipse_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawEllipse_3(self);
    // return 1;
  }
}
pub trait QPainter_drawEllipse_3<RetType> {
  fn drawEllipse_3(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawEllipse_3<(/*void*/)> for (usize,f64,f64) {
  fn drawEllipse_3(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11drawEllipseERK7QPointFdd", 3,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:321
// index:4
// Public inline Visibility=Default Availability=Available
// [-2] void drawEllipse(const QPoint &, int, int)

/*
Draws the ellipse defined by the given rectangle.

A filled ellipse has a size of rectangle.size(). A stroked ellipse has a size of rectangle.size() plus the pen width.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);

  QPainter painter(this);
  painter.drawEllipse(rectangle);





See also drawPie() and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawEllipse_4<RetType, T: QPainter_drawEllipse_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawEllipse_4(self);
    // return 1;
  }
}
pub trait QPainter_drawEllipse_4<RetType> {
  fn drawEllipse_4(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawEllipse_4<(/*void*/)> for (usize,i32,i32) {
  fn drawEllipse_4(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11drawEllipseERK6QPointii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:323
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawPolyline(const QPointF *, int)

/*
Draws the polyline defined by the first pointCount points in points using the current pen.

Note that unlike the drawPolygon() function the last point is not connected to the first, neither is the polyline filled.


 
  static const QPointF points[3] = {
      QPointF(10.0, 80.0),
      QPointF(20.0, 10.0),
      QPointF(80.0, 30.0),
  };

  QPainter painter(this);
  painter.drawPolyline(points, 3);





See also drawLines(), drawPolygon(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPolyline_0<RetType, T: QPainter_drawPolyline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPolyline_0(self);
    // return 1;
  }
}
pub trait QPainter_drawPolyline_0<RetType> {
  fn drawPolyline_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPolyline_0<(/*void*/)> for (usize,i32) {
  fn drawPolyline_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter12drawPolylineEPK7QPointFi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:324
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawPolyline(const QPolygonF &)

/*
Draws the polyline defined by the first pointCount points in points using the current pen.

Note that unlike the drawPolygon() function the last point is not connected to the first, neither is the polyline filled.


 
  static const QPointF points[3] = {
      QPointF(10.0, 80.0),
      QPointF(20.0, 10.0),
      QPointF(80.0, 30.0),
  };

  QPainter painter(this);
  painter.drawPolyline(points, 3);





See also drawLines(), drawPolygon(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPolyline_1<RetType, T: QPainter_drawPolyline_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPolyline_1(self);
    // return 1;
  }
}
pub trait QPainter_drawPolyline_1<RetType> {
  fn drawPolyline_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPolyline_1<(/*void*/)> for (usize) {
  fn drawPolyline_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter12drawPolylineERK9QPolygonF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:325
// index:2
// Public Visibility=Default Availability=Available
// [-2] void drawPolyline(const QPoint *, int)

/*
Draws the polyline defined by the first pointCount points in points using the current pen.

Note that unlike the drawPolygon() function the last point is not connected to the first, neither is the polyline filled.


 
  static const QPointF points[3] = {
      QPointF(10.0, 80.0),
      QPointF(20.0, 10.0),
      QPointF(80.0, 30.0),
  };

  QPainter painter(this);
  painter.drawPolyline(points, 3);





See also drawLines(), drawPolygon(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPolyline_2<RetType, T: QPainter_drawPolyline_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPolyline_2(self);
    // return 1;
  }
}
pub trait QPainter_drawPolyline_2<RetType> {
  fn drawPolyline_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPolyline_2<(/*void*/)> for (usize,i32) {
  fn drawPolyline_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter12drawPolylineEPK6QPointi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:326
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void drawPolyline(const QPolygon &)

/*
Draws the polyline defined by the first pointCount points in points using the current pen.

Note that unlike the drawPolygon() function the last point is not connected to the first, neither is the polyline filled.


 
  static const QPointF points[3] = {
      QPointF(10.0, 80.0),
      QPointF(20.0, 10.0),
      QPointF(80.0, 30.0),
  };

  QPainter painter(this);
  painter.drawPolyline(points, 3);





See also drawLines(), drawPolygon(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPolyline_3<RetType, T: QPainter_drawPolyline_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPolyline_3(self);
    // return 1;
  }
}
pub trait QPainter_drawPolyline_3<RetType> {
  fn drawPolyline_3(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPolyline_3<(/*void*/)> for (usize) {
  fn drawPolyline_3(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter12drawPolylineERK8QPolygon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:328
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawPolygon(const QPointF *, int, Qt::FillRule)

/*
Draws the polygon defined by the first pointCount points in the array points using the current pen and brush.


 
  static const QPointF points[4] = {
      QPointF(10.0, 80.0),
      QPointF(20.0, 10.0),
      QPointF(80.0, 30.0),
      QPointF(90.0, 70.0)
  };

  QPainter painter(this);
  painter.drawPolygon(points, 4);





The first point is implicitly connected to the last point, and the polygon is filled with the current brush().

If fillRule is Qt::WindingFill, the polygon is filled using the winding fill algorithm. If fillRule is Qt::OddEvenFill, the polygon is filled using the odd-even fill algorithm. See Qt::FillRule for a more detailed description of these fill rules.

See also drawConvexPolygon(), drawPolyline(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPolygon_0<RetType, T: QPainter_drawPolygon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPolygon_0(self);
    // return 1;
  }
}
pub trait QPainter_drawPolygon_0<RetType> {
  fn drawPolygon_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPolygon_0<(/*void*/)> for (usize,i32,i32) {
  fn drawPolygon_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11drawPolygonEPK7QPointFiN2Qt8FillRuleE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:329
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawPolygon(const QPolygonF &, Qt::FillRule)

/*
Draws the polygon defined by the first pointCount points in the array points using the current pen and brush.


 
  static const QPointF points[4] = {
      QPointF(10.0, 80.0),
      QPointF(20.0, 10.0),
      QPointF(80.0, 30.0),
      QPointF(90.0, 70.0)
  };

  QPainter painter(this);
  painter.drawPolygon(points, 4);





The first point is implicitly connected to the last point, and the polygon is filled with the current brush().

If fillRule is Qt::WindingFill, the polygon is filled using the winding fill algorithm. If fillRule is Qt::OddEvenFill, the polygon is filled using the odd-even fill algorithm. See Qt::FillRule for a more detailed description of these fill rules.

See also drawConvexPolygon(), drawPolyline(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPolygon_1<RetType, T: QPainter_drawPolygon_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPolygon_1(self);
    // return 1;
  }
}
pub trait QPainter_drawPolygon_1<RetType> {
  fn drawPolygon_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPolygon_1<(/*void*/)> for (usize,i32) {
  fn drawPolygon_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11drawPolygonERK9QPolygonFN2Qt8FillRuleE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:330
// index:2
// Public Visibility=Default Availability=Available
// [-2] void drawPolygon(const QPoint *, int, Qt::FillRule)

/*
Draws the polygon defined by the first pointCount points in the array points using the current pen and brush.


 
  static const QPointF points[4] = {
      QPointF(10.0, 80.0),
      QPointF(20.0, 10.0),
      QPointF(80.0, 30.0),
      QPointF(90.0, 70.0)
  };

  QPainter painter(this);
  painter.drawPolygon(points, 4);





The first point is implicitly connected to the last point, and the polygon is filled with the current brush().

If fillRule is Qt::WindingFill, the polygon is filled using the winding fill algorithm. If fillRule is Qt::OddEvenFill, the polygon is filled using the odd-even fill algorithm. See Qt::FillRule for a more detailed description of these fill rules.

See also drawConvexPolygon(), drawPolyline(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPolygon_2<RetType, T: QPainter_drawPolygon_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPolygon_2(self);
    // return 1;
  }
}
pub trait QPainter_drawPolygon_2<RetType> {
  fn drawPolygon_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPolygon_2<(/*void*/)> for (usize,i32,i32) {
  fn drawPolygon_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11drawPolygonEPK6QPointiN2Qt8FillRuleE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:331
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void drawPolygon(const QPolygon &, Qt::FillRule)

/*
Draws the polygon defined by the first pointCount points in the array points using the current pen and brush.


 
  static const QPointF points[4] = {
      QPointF(10.0, 80.0),
      QPointF(20.0, 10.0),
      QPointF(80.0, 30.0),
      QPointF(90.0, 70.0)
  };

  QPainter painter(this);
  painter.drawPolygon(points, 4);





The first point is implicitly connected to the last point, and the polygon is filled with the current brush().

If fillRule is Qt::WindingFill, the polygon is filled using the winding fill algorithm. If fillRule is Qt::OddEvenFill, the polygon is filled using the odd-even fill algorithm. See Qt::FillRule for a more detailed description of these fill rules.

See also drawConvexPolygon(), drawPolyline(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPolygon_3<RetType, T: QPainter_drawPolygon_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPolygon_3(self);
    // return 1;
  }
}
pub trait QPainter_drawPolygon_3<RetType> {
  fn drawPolygon_3(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPolygon_3<(/*void*/)> for (usize,i32) {
  fn drawPolygon_3(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11drawPolygonERK8QPolygonN2Qt8FillRuleE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:333
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawConvexPolygon(const QPointF *, int)

/*
Draws the convex polygon defined by the first pointCount points in the array points using the current pen.


 
  static const QPointF points[4] = {
      QPointF(10.0, 80.0),
      QPointF(20.0, 10.0),
      QPointF(80.0, 30.0),
      QPointF(90.0, 70.0)
  };

  QPainter painter(this);
  painter.drawConvexPolygon(points, 4);





The first point is implicitly connected to the last point, and the polygon is filled with the current brush(). If the supplied polygon is not convex, i.e. it contains at least one angle larger than 180 degrees, the results are undefined.

On some platforms (e.g. X11), the drawConvexPolygon() function can be faster than the drawPolygon() function.

See also drawPolygon(), drawPolyline(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawConvexPolygon_0<RetType, T: QPainter_drawConvexPolygon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawConvexPolygon_0(self);
    // return 1;
  }
}
pub trait QPainter_drawConvexPolygon_0<RetType> {
  fn drawConvexPolygon_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawConvexPolygon_0<(/*void*/)> for (usize,i32) {
  fn drawConvexPolygon_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter17drawConvexPolygonEPK7QPointFi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:334
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawConvexPolygon(const QPolygonF &)

/*
Draws the convex polygon defined by the first pointCount points in the array points using the current pen.


 
  static const QPointF points[4] = {
      QPointF(10.0, 80.0),
      QPointF(20.0, 10.0),
      QPointF(80.0, 30.0),
      QPointF(90.0, 70.0)
  };

  QPainter painter(this);
  painter.drawConvexPolygon(points, 4);





The first point is implicitly connected to the last point, and the polygon is filled with the current brush(). If the supplied polygon is not convex, i.e. it contains at least one angle larger than 180 degrees, the results are undefined.

On some platforms (e.g. X11), the drawConvexPolygon() function can be faster than the drawPolygon() function.

See also drawPolygon(), drawPolyline(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawConvexPolygon_1<RetType, T: QPainter_drawConvexPolygon_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawConvexPolygon_1(self);
    // return 1;
  }
}
pub trait QPainter_drawConvexPolygon_1<RetType> {
  fn drawConvexPolygon_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawConvexPolygon_1<(/*void*/)> for (usize) {
  fn drawConvexPolygon_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter17drawConvexPolygonERK9QPolygonF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:335
// index:2
// Public Visibility=Default Availability=Available
// [-2] void drawConvexPolygon(const QPoint *, int)

/*
Draws the convex polygon defined by the first pointCount points in the array points using the current pen.


 
  static const QPointF points[4] = {
      QPointF(10.0, 80.0),
      QPointF(20.0, 10.0),
      QPointF(80.0, 30.0),
      QPointF(90.0, 70.0)
  };

  QPainter painter(this);
  painter.drawConvexPolygon(points, 4);





The first point is implicitly connected to the last point, and the polygon is filled with the current brush(). If the supplied polygon is not convex, i.e. it contains at least one angle larger than 180 degrees, the results are undefined.

On some platforms (e.g. X11), the drawConvexPolygon() function can be faster than the drawPolygon() function.

See also drawPolygon(), drawPolyline(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawConvexPolygon_2<RetType, T: QPainter_drawConvexPolygon_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawConvexPolygon_2(self);
    // return 1;
  }
}
pub trait QPainter_drawConvexPolygon_2<RetType> {
  fn drawConvexPolygon_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawConvexPolygon_2<(/*void*/)> for (usize,i32) {
  fn drawConvexPolygon_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter17drawConvexPolygonEPK6QPointi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:336
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void drawConvexPolygon(const QPolygon &)

/*
Draws the convex polygon defined by the first pointCount points in the array points using the current pen.


 
  static const QPointF points[4] = {
      QPointF(10.0, 80.0),
      QPointF(20.0, 10.0),
      QPointF(80.0, 30.0),
      QPointF(90.0, 70.0)
  };

  QPainter painter(this);
  painter.drawConvexPolygon(points, 4);





The first point is implicitly connected to the last point, and the polygon is filled with the current brush(). If the supplied polygon is not convex, i.e. it contains at least one angle larger than 180 degrees, the results are undefined.

On some platforms (e.g. X11), the drawConvexPolygon() function can be faster than the drawPolygon() function.

See also drawPolygon(), drawPolyline(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawConvexPolygon_3<RetType, T: QPainter_drawConvexPolygon_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawConvexPolygon_3(self);
    // return 1;
  }
}
pub trait QPainter_drawConvexPolygon_3<RetType> {
  fn drawConvexPolygon_3(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawConvexPolygon_3<(/*void*/)> for (usize) {
  fn drawConvexPolygon_3(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter17drawConvexPolygonERK8QPolygon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:338
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawArc(const QRectF &, int, int)

/*
Draws the arc defined by the given rectangle, startAngle and spanAngle.

The startAngle and spanAngle must be specified in 1/16th of a degree, i.e. a full circle equals 5760 (16 * 360). Positive values for the angles mean counter-clockwise while negative values mean the clockwise direction. Zero degrees is at the 3 o'clock position.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);
  int startAngle = 30 * 16;
  int spanAngle = 120 * 16;

  QPainter painter(this);
  painter.drawArc(rectangle, startAngle, spanAngle);





See also drawPie(), drawChord(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawArc_0<RetType, T: QPainter_drawArc_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawArc_0(self);
    // return 1;
  }
}
pub trait QPainter_drawArc_0<RetType> {
  fn drawArc_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawArc_0<(/*void*/)> for (usize,i32,i32) {
  fn drawArc_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter7drawArcERK6QRectFii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:339
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawArc(const QRect &, int, int)

/*
Draws the arc defined by the given rectangle, startAngle and spanAngle.

The startAngle and spanAngle must be specified in 1/16th of a degree, i.e. a full circle equals 5760 (16 * 360). Positive values for the angles mean counter-clockwise while negative values mean the clockwise direction. Zero degrees is at the 3 o'clock position.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);
  int startAngle = 30 * 16;
  int spanAngle = 120 * 16;

  QPainter painter(this);
  painter.drawArc(rectangle, startAngle, spanAngle);





See also drawPie(), drawChord(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawArc_1<RetType, T: QPainter_drawArc_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawArc_1(self);
    // return 1;
  }
}
pub trait QPainter_drawArc_1<RetType> {
  fn drawArc_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawArc_1<(/*void*/)> for (usize,i32,i32) {
  fn drawArc_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter7drawArcERK5QRectii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:340
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void drawArc(int, int, int, int, int, int)

/*
Draws the arc defined by the given rectangle, startAngle and spanAngle.

The startAngle and spanAngle must be specified in 1/16th of a degree, i.e. a full circle equals 5760 (16 * 360). Positive values for the angles mean counter-clockwise while negative values mean the clockwise direction. Zero degrees is at the 3 o'clock position.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);
  int startAngle = 30 * 16;
  int spanAngle = 120 * 16;

  QPainter painter(this);
  painter.drawArc(rectangle, startAngle, spanAngle);





See also drawPie(), drawChord(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawArc_2<RetType, T: QPainter_drawArc_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawArc_2(self);
    // return 1;
  }
}
pub trait QPainter_drawArc_2<RetType> {
  fn drawArc_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawArc_2<(/*void*/)> for (i32,i32,i32,i32,i32,i32) {
  fn drawArc_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter7drawArcEiiiiii", 6,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:342
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawPie(const QRectF &, int, int)

/*
Draws a pie defined by the given rectangle, startAngle and spanAngle.

The pie is filled with the current brush().

The startAngle and spanAngle must be specified in 1/16th of a degree, i.e. a full circle equals 5760 (16 * 360). Positive values for the angles mean counter-clockwise while negative values mean the clockwise direction. Zero degrees is at the 3 o'clock position.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);
  int startAngle = 30 * 16;
  int spanAngle = 120 * 16;

  QPainter painter(this);
  painter.drawPie(rectangle, startAngle, spanAngle);





See also drawEllipse(), drawChord(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPie_0<RetType, T: QPainter_drawPie_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPie_0(self);
    // return 1;
  }
}
pub trait QPainter_drawPie_0<RetType> {
  fn drawPie_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPie_0<(/*void*/)> for (usize,i32,i32) {
  fn drawPie_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter7drawPieERK6QRectFii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:343
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawPie(int, int, int, int, int, int)

/*
Draws a pie defined by the given rectangle, startAngle and spanAngle.

The pie is filled with the current brush().

The startAngle and spanAngle must be specified in 1/16th of a degree, i.e. a full circle equals 5760 (16 * 360). Positive values for the angles mean counter-clockwise while negative values mean the clockwise direction. Zero degrees is at the 3 o'clock position.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);
  int startAngle = 30 * 16;
  int spanAngle = 120 * 16;

  QPainter painter(this);
  painter.drawPie(rectangle, startAngle, spanAngle);





See also drawEllipse(), drawChord(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPie_1<RetType, T: QPainter_drawPie_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPie_1(self);
    // return 1;
  }
}
pub trait QPainter_drawPie_1<RetType> {
  fn drawPie_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPie_1<(/*void*/)> for (i32,i32,i32,i32,i32,i32) {
  fn drawPie_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter7drawPieEiiiiii", 6,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:344
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void drawPie(const QRect &, int, int)

/*
Draws a pie defined by the given rectangle, startAngle and spanAngle.

The pie is filled with the current brush().

The startAngle and spanAngle must be specified in 1/16th of a degree, i.e. a full circle equals 5760 (16 * 360). Positive values for the angles mean counter-clockwise while negative values mean the clockwise direction. Zero degrees is at the 3 o'clock position.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);
  int startAngle = 30 * 16;
  int spanAngle = 120 * 16;

  QPainter painter(this);
  painter.drawPie(rectangle, startAngle, spanAngle);





See also drawEllipse(), drawChord(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawPie_2<RetType, T: QPainter_drawPie_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPie_2(self);
    // return 1;
  }
}
pub trait QPainter_drawPie_2<RetType> {
  fn drawPie_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPie_2<(/*void*/)> for (usize,i32,i32) {
  fn drawPie_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter7drawPieERK5QRectii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:346
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawChord(const QRectF &, int, int)

/*
Draws the chord defined by the given rectangle, startAngle and spanAngle. The chord is filled with the current brush().

The startAngle and spanAngle must be specified in 1/16th of a degree, i.e. a full circle equals 5760 (16 * 360). Positive values for the angles mean counter-clockwise while negative values mean the clockwise direction. Zero degrees is at the 3 o'clock position.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);
  int startAngle = 30 * 16;
  int spanAngle = 120 * 16;

  QPainter painter(this);
  painter.drawChord(rect, startAngle, spanAngle);





See also drawArc(), drawPie(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawChord_0<RetType, T: QPainter_drawChord_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawChord_0(self);
    // return 1;
  }
}
pub trait QPainter_drawChord_0<RetType> {
  fn drawChord_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawChord_0<(/*void*/)> for (usize,i32,i32) {
  fn drawChord_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawChordERK6QRectFii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:347
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawChord(int, int, int, int, int, int)

/*
Draws the chord defined by the given rectangle, startAngle and spanAngle. The chord is filled with the current brush().

The startAngle and spanAngle must be specified in 1/16th of a degree, i.e. a full circle equals 5760 (16 * 360). Positive values for the angles mean counter-clockwise while negative values mean the clockwise direction. Zero degrees is at the 3 o'clock position.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);
  int startAngle = 30 * 16;
  int spanAngle = 120 * 16;

  QPainter painter(this);
  painter.drawChord(rect, startAngle, spanAngle);





See also drawArc(), drawPie(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawChord_1<RetType, T: QPainter_drawChord_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawChord_1(self);
    // return 1;
  }
}
pub trait QPainter_drawChord_1<RetType> {
  fn drawChord_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawChord_1<(/*void*/)> for (i32,i32,i32,i32,i32,i32) {
  fn drawChord_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawChordEiiiiii", 6,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:348
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void drawChord(const QRect &, int, int)

/*
Draws the chord defined by the given rectangle, startAngle and spanAngle. The chord is filled with the current brush().

The startAngle and spanAngle must be specified in 1/16th of a degree, i.e. a full circle equals 5760 (16 * 360). Positive values for the angles mean counter-clockwise while negative values mean the clockwise direction. Zero degrees is at the 3 o'clock position.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);
  int startAngle = 30 * 16;
  int spanAngle = 120 * 16;

  QPainter painter(this);
  painter.drawChord(rect, startAngle, spanAngle);





See also drawArc(), drawPie(), and Coordinate System.
*/
impl /*struct*/ QPainter {
  pub fn drawChord_2<RetType, T: QPainter_drawChord_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawChord_2(self);
    // return 1;
  }
}
pub trait QPainter_drawChord_2<RetType> {
  fn drawChord_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawChord_2<(/*void*/)> for (usize,i32,i32) {
  fn drawChord_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawChordERK5QRectii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:350
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawRoundedRect(const QRectF &, qreal, qreal, Qt::SizeMode)

/*
Draws the given rectangle rect with rounded corners.

The xRadius and yRadius arguments specify the radii of the ellipses defining the corners of the rounded rectangle. When mode is Qt::RelativeSize, xRadius and yRadius are specified in percentage of half the rectangle's width and height respectively, and should be in the range 0.0 to 100.0.

A filled rectangle has a size of rect.size(). A stroked rectangle has a size of rect.size() plus the pen width.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);

  QPainter painter(this);
  painter.drawRoundedRect(rectangle, 20.0, 15.0);





This function was introduced in  Qt 4.4.

See also drawRect() and QPen.
*/
impl /*struct*/ QPainter {
  pub fn drawRoundedRect_0<RetType, T: QPainter_drawRoundedRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawRoundedRect_0(self);
    // return 1;
  }
}
pub trait QPainter_drawRoundedRect_0<RetType> {
  fn drawRoundedRect_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawRoundedRect_0<(/*void*/)> for (usize,f64,f64,i32) {
  fn drawRoundedRect_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter15drawRoundedRectERK6QRectFddN2Qt8SizeModeE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:352
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawRoundedRect(int, int, int, int, qreal, qreal, Qt::SizeMode)

/*
Draws the given rectangle rect with rounded corners.

The xRadius and yRadius arguments specify the radii of the ellipses defining the corners of the rounded rectangle. When mode is Qt::RelativeSize, xRadius and yRadius are specified in percentage of half the rectangle's width and height respectively, and should be in the range 0.0 to 100.0.

A filled rectangle has a size of rect.size(). A stroked rectangle has a size of rect.size() plus the pen width.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);

  QPainter painter(this);
  painter.drawRoundedRect(rectangle, 20.0, 15.0);





This function was introduced in  Qt 4.4.

See also drawRect() and QPen.
*/
impl /*struct*/ QPainter {
  pub fn drawRoundedRect_1<RetType, T: QPainter_drawRoundedRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawRoundedRect_1(self);
    // return 1;
  }
}
pub trait QPainter_drawRoundedRect_1<RetType> {
  fn drawRoundedRect_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawRoundedRect_1<(/*void*/)> for (i32,i32,i32,i32,f64,f64,i32) {
  fn drawRoundedRect_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let arg5 = (&self.5) as *const f64 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter15drawRoundedRectEiiiiddN2Qt8SizeModeE", 7,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:354
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void drawRoundedRect(const QRect &, qreal, qreal, Qt::SizeMode)

/*
Draws the given rectangle rect with rounded corners.

The xRadius and yRadius arguments specify the radii of the ellipses defining the corners of the rounded rectangle. When mode is Qt::RelativeSize, xRadius and yRadius are specified in percentage of half the rectangle's width and height respectively, and should be in the range 0.0 to 100.0.

A filled rectangle has a size of rect.size(). A stroked rectangle has a size of rect.size() plus the pen width.


 
  QRectF rectangle(10.0, 20.0, 80.0, 60.0);

  QPainter painter(this);
  painter.drawRoundedRect(rectangle, 20.0, 15.0);





This function was introduced in  Qt 4.4.

See also drawRect() and QPen.
*/
impl /*struct*/ QPainter {
  pub fn drawRoundedRect_2<RetType, T: QPainter_drawRoundedRect_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawRoundedRect_2(self);
    // return 1;
  }
}
pub trait QPainter_drawRoundedRect_2<RetType> {
  fn drawRoundedRect_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawRoundedRect_2<(/*void*/)> for (usize,f64,f64,i32) {
  fn drawRoundedRect_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter15drawRoundedRectERK5QRectddN2Qt8SizeModeE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:357
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawRoundRect(const QRectF &, int, int)

/*

*/
impl /*struct*/ QPainter {
  pub fn drawRoundRect_0<RetType, T: QPainter_drawRoundRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawRoundRect_0(self);
    // return 1;
  }
}
pub trait QPainter_drawRoundRect_0<RetType> {
  fn drawRoundRect_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawRoundRect_0<(/*void*/)> for (usize,i32,i32) {
  fn drawRoundRect_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter13drawRoundRectERK6QRectFii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:358
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawRoundRect(int, int, int, int, int, int)

/*

*/
impl /*struct*/ QPainter {
  pub fn drawRoundRect_1<RetType, T: QPainter_drawRoundRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawRoundRect_1(self);
    // return 1;
  }
}
pub trait QPainter_drawRoundRect_1<RetType> {
  fn drawRoundRect_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawRoundRect_1<(/*void*/)> for (i32,i32,i32,i32,i32,i32) {
  fn drawRoundRect_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter13drawRoundRectEiiiiii", 6,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:359
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void drawRoundRect(const QRect &, int, int)

/*

*/
impl /*struct*/ QPainter {
  pub fn drawRoundRect_2<RetType, T: QPainter_drawRoundRect_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawRoundRect_2(self);
    // return 1;
  }
}
pub trait QPainter_drawRoundRect_2<RetType> {
  fn drawRoundRect_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawRoundRect_2<(/*void*/)> for (usize,i32,i32) {
  fn drawRoundRect_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter13drawRoundRectERK5QRectii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:361
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawTiledPixmap(const QRectF &, const QPixmap &, const QPointF &)

/*
Draws a tiled pixmap, inside the given rectangle with its origin at the given position.

Calling drawTiledPixmap() is similar to calling drawPixmap() several times to fill (tile) an area with a pixmap, but is potentially much more efficient depending on the underlying window system.

See also drawPixmap().
*/
impl /*struct*/ QPainter {
  pub fn drawTiledPixmap_0<RetType, T: QPainter_drawTiledPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawTiledPixmap_0(self);
    // return 1;
  }
}
pub trait QPainter_drawTiledPixmap_0<RetType> {
  fn drawTiledPixmap_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawTiledPixmap_0<(/*void*/)> for (usize,usize,usize) {
  fn drawTiledPixmap_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:362
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawTiledPixmap(int, int, int, int, const QPixmap &, int, int)

/*
Draws a tiled pixmap, inside the given rectangle with its origin at the given position.

Calling drawTiledPixmap() is similar to calling drawPixmap() several times to fill (tile) an area with a pixmap, but is potentially much more efficient depending on the underlying window system.

See also drawPixmap().
*/
impl /*struct*/ QPainter {
  pub fn drawTiledPixmap_1<RetType, T: QPainter_drawTiledPixmap_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawTiledPixmap_1(self);
    // return 1;
  }
}
pub trait QPainter_drawTiledPixmap_1<RetType> {
  fn drawTiledPixmap_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawTiledPixmap_1<(/*void*/)> for (i32,i32,i32,i32,usize,i32,i32) {
  fn drawTiledPixmap_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter15drawTiledPixmapEiiiiRK7QPixmapii", 7,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:363
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void drawTiledPixmap(const QRect &, const QPixmap &, const QPoint &)

/*
Draws a tiled pixmap, inside the given rectangle with its origin at the given position.

Calling drawTiledPixmap() is similar to calling drawPixmap() several times to fill (tile) an area with a pixmap, but is potentially much more efficient depending on the underlying window system.

See also drawPixmap().
*/
impl /*struct*/ QPainter {
  pub fn drawTiledPixmap_2<RetType, T: QPainter_drawTiledPixmap_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawTiledPixmap_2(self);
    // return 1;
  }
}
pub trait QPainter_drawTiledPixmap_2<RetType> {
  fn drawTiledPixmap_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawTiledPixmap_2<(/*void*/)> for (usize,usize,usize) {
  fn drawTiledPixmap_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter15drawTiledPixmapERK5QRectRK7QPixmapRK6QPoint", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:365
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawPicture(const QPointF &, const QPicture &)

/*
Replays the given picture at the given point.

The QPicture class is a paint device that records and replays QPainter commands. A picture serializes the painter commands to an IO device in a platform-independent format. Everything that can be painted on a widget or pixmap can also be stored in a picture.

This function does exactly the same as QPicture::play() when called with point = QPoint(0, 0).


 
  QPicture picture;
  QPointF point(10.0, 20.0)
  picture.load("drawing.pic");

  QPainter painter(this);
  painter.drawPicture(0, 0, picture);





See also QPicture::play().
*/
impl /*struct*/ QPainter {
  pub fn drawPicture_0<RetType, T: QPainter_drawPicture_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPicture_0(self);
    // return 1;
  }
}
pub trait QPainter_drawPicture_0<RetType> {
  fn drawPicture_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPicture_0<(/*void*/)> for (usize,usize) {
  fn drawPicture_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11drawPictureERK7QPointFRK8QPicture", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:366
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawPicture(int, int, const QPicture &)

/*
Replays the given picture at the given point.

The QPicture class is a paint device that records and replays QPainter commands. A picture serializes the painter commands to an IO device in a platform-independent format. Everything that can be painted on a widget or pixmap can also be stored in a picture.

This function does exactly the same as QPicture::play() when called with point = QPoint(0, 0).


 
  QPicture picture;
  QPointF point(10.0, 20.0)
  picture.load("drawing.pic");

  QPainter painter(this);
  painter.drawPicture(0, 0, picture);





See also QPicture::play().
*/
impl /*struct*/ QPainter {
  pub fn drawPicture_1<RetType, T: QPainter_drawPicture_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPicture_1(self);
    // return 1;
  }
}
pub trait QPainter_drawPicture_1<RetType> {
  fn drawPicture_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPicture_1<(/*void*/)> for (i32,i32,usize) {
  fn drawPicture_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11drawPictureEiiRK8QPicture", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:367
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void drawPicture(const QPoint &, const QPicture &)

/*
Replays the given picture at the given point.

The QPicture class is a paint device that records and replays QPainter commands. A picture serializes the painter commands to an IO device in a platform-independent format. Everything that can be painted on a widget or pixmap can also be stored in a picture.

This function does exactly the same as QPicture::play() when called with point = QPoint(0, 0).


 
  QPicture picture;
  QPointF point(10.0, 20.0)
  picture.load("drawing.pic");

  QPainter painter(this);
  painter.drawPicture(0, 0, picture);





See also QPicture::play().
*/
impl /*struct*/ QPainter {
  pub fn drawPicture_2<RetType, T: QPainter_drawPicture_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPicture_2(self);
    // return 1;
  }
}
pub trait QPainter_drawPicture_2<RetType> {
  fn drawPicture_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPicture_2<(/*void*/)> for (usize,usize) {
  fn drawPicture_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter11drawPictureERK6QPointRK8QPicture", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:370
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawPixmap(const QRectF &, const QPixmap &, const QRectF &)

/*
Draws the rectangular portion source of the given pixmap into the given target in the paint device.

Note: The pixmap is scaled to fit the rectangle, if both the pixmap and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QPixmap::devicePixelRatio().


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QPixmap pixmap(":myPixmap.png");

  QPainter(this);
  painter.drawPixmap(target, pixmap, source);





If pixmap is a QBitmap it is drawn with the bits that are "set" using the pens color. If backgroundMode is Qt::OpaqueMode, the "unset" bits are drawn using the color of the background brush; if backgroundMode is Qt::TransparentMode, the "unset" bits are transparent. Drawing bitmaps with gradient or texture colors is not supported.

See also drawImage() and QPixmap::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawPixmap_0<RetType, T: QPainter_drawPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPixmap_0(self);
    // return 1;
  }
}
pub trait QPainter_drawPixmap_0<RetType> {
  fn drawPixmap_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPixmap_0<(/*void*/)> for (usize,usize,usize) {
  fn drawPixmap_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter10drawPixmapERK6QRectFRK7QPixmapS2_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:371
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawPixmap(const QRect &, const QPixmap &, const QRect &)

/*
Draws the rectangular portion source of the given pixmap into the given target in the paint device.

Note: The pixmap is scaled to fit the rectangle, if both the pixmap and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QPixmap::devicePixelRatio().


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QPixmap pixmap(":myPixmap.png");

  QPainter(this);
  painter.drawPixmap(target, pixmap, source);





If pixmap is a QBitmap it is drawn with the bits that are "set" using the pens color. If backgroundMode is Qt::OpaqueMode, the "unset" bits are drawn using the color of the background brush; if backgroundMode is Qt::TransparentMode, the "unset" bits are transparent. Drawing bitmaps with gradient or texture colors is not supported.

See also drawImage() and QPixmap::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawPixmap_1<RetType, T: QPainter_drawPixmap_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPixmap_1(self);
    // return 1;
  }
}
pub trait QPainter_drawPixmap_1<RetType> {
  fn drawPixmap_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPixmap_1<(/*void*/)> for (usize,usize,usize) {
  fn drawPixmap_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter10drawPixmapERK5QRectRK7QPixmapS2_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:372
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void drawPixmap(int, int, int, int, const QPixmap &, int, int, int, int)

/*
Draws the rectangular portion source of the given pixmap into the given target in the paint device.

Note: The pixmap is scaled to fit the rectangle, if both the pixmap and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QPixmap::devicePixelRatio().


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QPixmap pixmap(":myPixmap.png");

  QPainter(this);
  painter.drawPixmap(target, pixmap, source);





If pixmap is a QBitmap it is drawn with the bits that are "set" using the pens color. If backgroundMode is Qt::OpaqueMode, the "unset" bits are drawn using the color of the background brush; if backgroundMode is Qt::TransparentMode, the "unset" bits are transparent. Drawing bitmaps with gradient or texture colors is not supported.

See also drawImage() and QPixmap::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawPixmap_2<RetType, T: QPainter_drawPixmap_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPixmap_2(self);
    // return 1;
  }
}
pub trait QPainter_drawPixmap_2<RetType> {
  fn drawPixmap_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPixmap_2<(/*void*/)> for (i32,i32,i32,i32,usize,i32,i32,i32,i32) {
  fn drawPixmap_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const i32 as usize;
    let arg8 = (&self.8) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter10drawPixmapEiiiiRK7QPixmapiiii", 9,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:374
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void drawPixmap(int, int, const QPixmap &, int, int, int, int)

/*
Draws the rectangular portion source of the given pixmap into the given target in the paint device.

Note: The pixmap is scaled to fit the rectangle, if both the pixmap and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QPixmap::devicePixelRatio().


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QPixmap pixmap(":myPixmap.png");

  QPainter(this);
  painter.drawPixmap(target, pixmap, source);





If pixmap is a QBitmap it is drawn with the bits that are "set" using the pens color. If backgroundMode is Qt::OpaqueMode, the "unset" bits are drawn using the color of the background brush; if backgroundMode is Qt::TransparentMode, the "unset" bits are transparent. Drawing bitmaps with gradient or texture colors is not supported.

See also drawImage() and QPixmap::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawPixmap_3<RetType, T: QPainter_drawPixmap_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPixmap_3(self);
    // return 1;
  }
}
pub trait QPainter_drawPixmap_3<RetType> {
  fn drawPixmap_3(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPixmap_3<(/*void*/)> for (i32,i32,usize,i32,i32,i32,i32) {
  fn drawPixmap_3(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter10drawPixmapEiiRK7QPixmapiiii", 7,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:376
// index:4
// Public inline Visibility=Default Availability=Available
// [-2] void drawPixmap(const QPointF &, const QPixmap &, const QRectF &)

/*
Draws the rectangular portion source of the given pixmap into the given target in the paint device.

Note: The pixmap is scaled to fit the rectangle, if both the pixmap and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QPixmap::devicePixelRatio().


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QPixmap pixmap(":myPixmap.png");

  QPainter(this);
  painter.drawPixmap(target, pixmap, source);





If pixmap is a QBitmap it is drawn with the bits that are "set" using the pens color. If backgroundMode is Qt::OpaqueMode, the "unset" bits are drawn using the color of the background brush; if backgroundMode is Qt::TransparentMode, the "unset" bits are transparent. Drawing bitmaps with gradient or texture colors is not supported.

See also drawImage() and QPixmap::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawPixmap_4<RetType, T: QPainter_drawPixmap_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPixmap_4(self);
    // return 1;
  }
}
pub trait QPainter_drawPixmap_4<RetType> {
  fn drawPixmap_4(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPixmap_4<(/*void*/)> for (usize,usize,usize) {
  fn drawPixmap_4(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter10drawPixmapERK7QPointFRK7QPixmapRK6QRectF", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:377
// index:5
// Public inline Visibility=Default Availability=Available
// [-2] void drawPixmap(const QPoint &, const QPixmap &, const QRect &)

/*
Draws the rectangular portion source of the given pixmap into the given target in the paint device.

Note: The pixmap is scaled to fit the rectangle, if both the pixmap and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QPixmap::devicePixelRatio().


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QPixmap pixmap(":myPixmap.png");

  QPainter(this);
  painter.drawPixmap(target, pixmap, source);





If pixmap is a QBitmap it is drawn with the bits that are "set" using the pens color. If backgroundMode is Qt::OpaqueMode, the "unset" bits are drawn using the color of the background brush; if backgroundMode is Qt::TransparentMode, the "unset" bits are transparent. Drawing bitmaps with gradient or texture colors is not supported.

See also drawImage() and QPixmap::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawPixmap_5<RetType, T: QPainter_drawPixmap_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPixmap_5(self);
    // return 1;
  }
}
pub trait QPainter_drawPixmap_5<RetType> {
  fn drawPixmap_5(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPixmap_5<(/*void*/)> for (usize,usize,usize) {
  fn drawPixmap_5(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter10drawPixmapERK6QPointRK7QPixmapRK5QRect", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:378
// index:6
// Public Visibility=Default Availability=Available
// [-2] void drawPixmap(const QPointF &, const QPixmap &)

/*
Draws the rectangular portion source of the given pixmap into the given target in the paint device.

Note: The pixmap is scaled to fit the rectangle, if both the pixmap and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QPixmap::devicePixelRatio().


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QPixmap pixmap(":myPixmap.png");

  QPainter(this);
  painter.drawPixmap(target, pixmap, source);





If pixmap is a QBitmap it is drawn with the bits that are "set" using the pens color. If backgroundMode is Qt::OpaqueMode, the "unset" bits are drawn using the color of the background brush; if backgroundMode is Qt::TransparentMode, the "unset" bits are transparent. Drawing bitmaps with gradient or texture colors is not supported.

See also drawImage() and QPixmap::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawPixmap_6<RetType, T: QPainter_drawPixmap_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPixmap_6(self);
    // return 1;
  }
}
pub trait QPainter_drawPixmap_6<RetType> {
  fn drawPixmap_6(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPixmap_6<(/*void*/)> for (usize,usize) {
  fn drawPixmap_6(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter10drawPixmapERK7QPointFRK7QPixmap", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:379
// index:7
// Public inline Visibility=Default Availability=Available
// [-2] void drawPixmap(const QPoint &, const QPixmap &)

/*
Draws the rectangular portion source of the given pixmap into the given target in the paint device.

Note: The pixmap is scaled to fit the rectangle, if both the pixmap and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QPixmap::devicePixelRatio().


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QPixmap pixmap(":myPixmap.png");

  QPainter(this);
  painter.drawPixmap(target, pixmap, source);





If pixmap is a QBitmap it is drawn with the bits that are "set" using the pens color. If backgroundMode is Qt::OpaqueMode, the "unset" bits are drawn using the color of the background brush; if backgroundMode is Qt::TransparentMode, the "unset" bits are transparent. Drawing bitmaps with gradient or texture colors is not supported.

See also drawImage() and QPixmap::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawPixmap_7<RetType, T: QPainter_drawPixmap_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPixmap_7(self);
    // return 1;
  }
}
pub trait QPainter_drawPixmap_7<RetType> {
  fn drawPixmap_7(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPixmap_7<(/*void*/)> for (usize,usize) {
  fn drawPixmap_7(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter10drawPixmapERK6QPointRK7QPixmap", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:380
// index:8
// Public inline Visibility=Default Availability=Available
// [-2] void drawPixmap(int, int, const QPixmap &)

/*
Draws the rectangular portion source of the given pixmap into the given target in the paint device.

Note: The pixmap is scaled to fit the rectangle, if both the pixmap and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QPixmap::devicePixelRatio().


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QPixmap pixmap(":myPixmap.png");

  QPainter(this);
  painter.drawPixmap(target, pixmap, source);





If pixmap is a QBitmap it is drawn with the bits that are "set" using the pens color. If backgroundMode is Qt::OpaqueMode, the "unset" bits are drawn using the color of the background brush; if backgroundMode is Qt::TransparentMode, the "unset" bits are transparent. Drawing bitmaps with gradient or texture colors is not supported.

See also drawImage() and QPixmap::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawPixmap_8<RetType, T: QPainter_drawPixmap_8<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPixmap_8(self);
    // return 1;
  }
}
pub trait QPainter_drawPixmap_8<RetType> {
  fn drawPixmap_8(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPixmap_8<(/*void*/)> for (i32,i32,usize) {
  fn drawPixmap_8(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter10drawPixmapEiiRK7QPixmap", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:381
// index:9
// Public inline Visibility=Default Availability=Available
// [-2] void drawPixmap(const QRect &, const QPixmap &)

/*
Draws the rectangular portion source of the given pixmap into the given target in the paint device.

Note: The pixmap is scaled to fit the rectangle, if both the pixmap and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QPixmap::devicePixelRatio().


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QPixmap pixmap(":myPixmap.png");

  QPainter(this);
  painter.drawPixmap(target, pixmap, source);





If pixmap is a QBitmap it is drawn with the bits that are "set" using the pens color. If backgroundMode is Qt::OpaqueMode, the "unset" bits are drawn using the color of the background brush; if backgroundMode is Qt::TransparentMode, the "unset" bits are transparent. Drawing bitmaps with gradient or texture colors is not supported.

See also drawImage() and QPixmap::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawPixmap_9<RetType, T: QPainter_drawPixmap_9<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPixmap_9(self);
    // return 1;
  }
}
pub trait QPainter_drawPixmap_9<RetType> {
  fn drawPixmap_9(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPixmap_9<(/*void*/)> for (usize,usize) {
  fn drawPixmap_9(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter10drawPixmapERK5QRectRK7QPixmap", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:382
// index:10
// Public inline Visibility=Default Availability=Available
// [-2] void drawPixmap(int, int, int, int, const QPixmap &)

/*
Draws the rectangular portion source of the given pixmap into the given target in the paint device.

Note: The pixmap is scaled to fit the rectangle, if both the pixmap and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QPixmap::devicePixelRatio().


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QPixmap pixmap(":myPixmap.png");

  QPainter(this);
  painter.drawPixmap(target, pixmap, source);





If pixmap is a QBitmap it is drawn with the bits that are "set" using the pens color. If backgroundMode is Qt::OpaqueMode, the "unset" bits are drawn using the color of the background brush; if backgroundMode is Qt::TransparentMode, the "unset" bits are transparent. Drawing bitmaps with gradient or texture colors is not supported.

See also drawImage() and QPixmap::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawPixmap_10<RetType, T: QPainter_drawPixmap_10<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPixmap_10(self);
    // return 1;
  }
}
pub trait QPainter_drawPixmap_10<RetType> {
  fn drawPixmap_10(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawPixmap_10<(/*void*/)> for (i32,i32,i32,i32,usize) {
  fn drawPixmap_10(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter10drawPixmapEiiiiRK7QPixmap", 5,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:387
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawImage(const QRectF &, const QImage &, const QRectF &, Qt::ImageConversionFlags)

/*
Draws the rectangular portion source of the given image into the target rectangle in the paint device.

Note: The image is scaled to fit the rectangle, if both the image and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QImage::devicePixelRatio().

If the image needs to be modified to fit in a lower-resolution result (e.g. converting from 32-bit to 8-bit), use the flags to specify how you would prefer this to happen.


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QImage image(":/images/myImage.png");

  QPainter painter(this);
  painter.drawImage(target, image, source);





See also drawPixmap() and QImage::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawImage_0<RetType, T: QPainter_drawImage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawImage_0(self);
    // return 1;
  }
}
pub trait QPainter_drawImage_0<RetType> {
  fn drawImage_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawImage_0<(/*void*/)> for (usize,usize,usize,i32) {
  fn drawImage_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawImageERK6QRectFRK6QImageS2_6QFlagsIN2Qt19ImageConversionFlagEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:389
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawImage(const QRect &, const QImage &, const QRect &, Qt::ImageConversionFlags)

/*
Draws the rectangular portion source of the given image into the target rectangle in the paint device.

Note: The image is scaled to fit the rectangle, if both the image and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QImage::devicePixelRatio().

If the image needs to be modified to fit in a lower-resolution result (e.g. converting from 32-bit to 8-bit), use the flags to specify how you would prefer this to happen.


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QImage image(":/images/myImage.png");

  QPainter painter(this);
  painter.drawImage(target, image, source);





See also drawPixmap() and QImage::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawImage_1<RetType, T: QPainter_drawImage_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawImage_1(self);
    // return 1;
  }
}
pub trait QPainter_drawImage_1<RetType> {
  fn drawImage_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawImage_1<(/*void*/)> for (usize,usize,usize,i32) {
  fn drawImage_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawImageERK5QRectRK6QImageS2_6QFlagsIN2Qt19ImageConversionFlagEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:391
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void drawImage(const QPointF &, const QImage &, const QRectF &, Qt::ImageConversionFlags)

/*
Draws the rectangular portion source of the given image into the target rectangle in the paint device.

Note: The image is scaled to fit the rectangle, if both the image and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QImage::devicePixelRatio().

If the image needs to be modified to fit in a lower-resolution result (e.g. converting from 32-bit to 8-bit), use the flags to specify how you would prefer this to happen.


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QImage image(":/images/myImage.png");

  QPainter painter(this);
  painter.drawImage(target, image, source);





See also drawPixmap() and QImage::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawImage_2<RetType, T: QPainter_drawImage_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawImage_2(self);
    // return 1;
  }
}
pub trait QPainter_drawImage_2<RetType> {
  fn drawImage_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawImage_2<(/*void*/)> for (usize,usize,usize,i32) {
  fn drawImage_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawImageERK7QPointFRK6QImageRK6QRectF6QFlagsIN2Qt19ImageConversionFlagEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:393
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void drawImage(const QPoint &, const QImage &, const QRect &, Qt::ImageConversionFlags)

/*
Draws the rectangular portion source of the given image into the target rectangle in the paint device.

Note: The image is scaled to fit the rectangle, if both the image and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QImage::devicePixelRatio().

If the image needs to be modified to fit in a lower-resolution result (e.g. converting from 32-bit to 8-bit), use the flags to specify how you would prefer this to happen.


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QImage image(":/images/myImage.png");

  QPainter painter(this);
  painter.drawImage(target, image, source);





See also drawPixmap() and QImage::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawImage_3<RetType, T: QPainter_drawImage_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawImage_3(self);
    // return 1;
  }
}
pub trait QPainter_drawImage_3<RetType> {
  fn drawImage_3(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawImage_3<(/*void*/)> for (usize,usize,usize,i32) {
  fn drawImage_3(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawImageERK6QPointRK6QImageRK5QRect6QFlagsIN2Qt19ImageConversionFlagEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:395
// index:4
// Public inline Visibility=Default Availability=Available
// [-2] void drawImage(const QRectF &, const QImage &)

/*
Draws the rectangular portion source of the given image into the target rectangle in the paint device.

Note: The image is scaled to fit the rectangle, if both the image and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QImage::devicePixelRatio().

If the image needs to be modified to fit in a lower-resolution result (e.g. converting from 32-bit to 8-bit), use the flags to specify how you would prefer this to happen.


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QImage image(":/images/myImage.png");

  QPainter painter(this);
  painter.drawImage(target, image, source);





See also drawPixmap() and QImage::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawImage_4<RetType, T: QPainter_drawImage_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawImage_4(self);
    // return 1;
  }
}
pub trait QPainter_drawImage_4<RetType> {
  fn drawImage_4(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawImage_4<(/*void*/)> for (usize,usize) {
  fn drawImage_4(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawImageERK6QRectFRK6QImage", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:396
// index:5
// Public inline Visibility=Default Availability=Available
// [-2] void drawImage(const QRect &, const QImage &)

/*
Draws the rectangular portion source of the given image into the target rectangle in the paint device.

Note: The image is scaled to fit the rectangle, if both the image and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QImage::devicePixelRatio().

If the image needs to be modified to fit in a lower-resolution result (e.g. converting from 32-bit to 8-bit), use the flags to specify how you would prefer this to happen.


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QImage image(":/images/myImage.png");

  QPainter painter(this);
  painter.drawImage(target, image, source);





See also drawPixmap() and QImage::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawImage_5<RetType, T: QPainter_drawImage_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawImage_5(self);
    // return 1;
  }
}
pub trait QPainter_drawImage_5<RetType> {
  fn drawImage_5(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawImage_5<(/*void*/)> for (usize,usize) {
  fn drawImage_5(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawImageERK5QRectRK6QImage", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:397
// index:6
// Public Visibility=Default Availability=Available
// [-2] void drawImage(const QPointF &, const QImage &)

/*
Draws the rectangular portion source of the given image into the target rectangle in the paint device.

Note: The image is scaled to fit the rectangle, if both the image and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QImage::devicePixelRatio().

If the image needs to be modified to fit in a lower-resolution result (e.g. converting from 32-bit to 8-bit), use the flags to specify how you would prefer this to happen.


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QImage image(":/images/myImage.png");

  QPainter painter(this);
  painter.drawImage(target, image, source);





See also drawPixmap() and QImage::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawImage_6<RetType, T: QPainter_drawImage_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawImage_6(self);
    // return 1;
  }
}
pub trait QPainter_drawImage_6<RetType> {
  fn drawImage_6(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawImage_6<(/*void*/)> for (usize,usize) {
  fn drawImage_6(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawImageERK7QPointFRK6QImage", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:398
// index:7
// Public inline Visibility=Default Availability=Available
// [-2] void drawImage(const QPoint &, const QImage &)

/*
Draws the rectangular portion source of the given image into the target rectangle in the paint device.

Note: The image is scaled to fit the rectangle, if both the image and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QImage::devicePixelRatio().

If the image needs to be modified to fit in a lower-resolution result (e.g. converting from 32-bit to 8-bit), use the flags to specify how you would prefer this to happen.


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QImage image(":/images/myImage.png");

  QPainter painter(this);
  painter.drawImage(target, image, source);





See also drawPixmap() and QImage::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawImage_7<RetType, T: QPainter_drawImage_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawImage_7(self);
    // return 1;
  }
}
pub trait QPainter_drawImage_7<RetType> {
  fn drawImage_7(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawImage_7<(/*void*/)> for (usize,usize) {
  fn drawImage_7(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawImageERK6QPointRK6QImage", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:399
// index:8
// Public inline Visibility=Default Availability=Available
// [-2] void drawImage(int, int, const QImage &, int, int, int, int, Qt::ImageConversionFlags)

/*
Draws the rectangular portion source of the given image into the target rectangle in the paint device.

Note: The image is scaled to fit the rectangle, if both the image and rectangle size disagree.

Note: See Drawing High Resolution Versions of Pixmaps and Images on how this is affected by QImage::devicePixelRatio().

If the image needs to be modified to fit in a lower-resolution result (e.g. converting from 32-bit to 8-bit), use the flags to specify how you would prefer this to happen.


 
  QRectF target(10.0, 20.0, 80.0, 60.0);
  QRectF source(0.0, 0.0, 70.0, 40.0);
  QImage image(":/images/myImage.png");

  QPainter painter(this);
  painter.drawImage(target, image, source);





See also drawPixmap() and QImage::devicePixelRatio().
*/
impl /*struct*/ QPainter {
  pub fn drawImage_8<RetType, T: QPainter_drawImage_8<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawImage_8(self);
    // return 1;
  }
}
pub trait QPainter_drawImage_8<RetType> {
  fn drawImage_8(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawImage_8<(/*void*/)> for (i32,i32,usize,i32,i32,i32,i32,i32) {
  fn drawImage_8(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9drawImageEiiRK6QImageiiii6QFlagsIN2Qt19ImageConversionFlagEE", 8,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:402
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLayoutDirection(Qt::LayoutDirection)

/*
Sets the layout direction used by the painter when drawing text, to the specified direction.

The default is Qt::LayoutDirectionAuto, which will implicitly determine the direction from the text drawn.

See also QTextOption::setTextDirection(), layoutDirection(), drawText(), and Settings.
*/
impl /*struct*/ QPainter {
  pub fn setLayoutDirection_0<RetType, T: QPainter_setLayoutDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLayoutDirection_0(self);
    // return 1;
  }
}
pub trait QPainter_setLayoutDirection_0<RetType> {
  fn setLayoutDirection_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setLayoutDirection_0<(/*void*/)> for (i32) {
  fn setLayoutDirection_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter18setLayoutDirectionEN2Qt15LayoutDirectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:403
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::LayoutDirection layoutDirection() const

/*
Returns the layout direction used by the painter when drawing text.

See also QTextOption::textDirection(), setLayoutDirection(), drawText(), and Settings.
*/
impl /*struct*/ QPainter {
  pub fn layoutDirection_0<RetType, T: QPainter_layoutDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.layoutDirection_0(self);
    // return 1;
  }
}
pub trait QPainter_layoutDirection_0<RetType> {
  fn layoutDirection_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_layoutDirection_0<i32> for () {
  fn layoutDirection_0(self , rsthis: & QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter15layoutDirectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:406
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawGlyphRun(const QPointF &, const QGlyphRun &)

/*
Draws the glyphs represented by glyphs at position. The position gives the edge of the baseline for the string of glyphs. The glyphs will be retrieved from the font selected on glyphs and at offsets given by the positions in glyphs.

This function was introduced in  Qt 4.8.

See also QGlyphRun::setRawFont(), QGlyphRun::setPositions(), and QGlyphRun::setGlyphIndexes().
*/
impl /*struct*/ QPainter {
  pub fn drawGlyphRun_0<RetType, T: QPainter_drawGlyphRun_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawGlyphRun_0(self);
    // return 1;
  }
}
pub trait QPainter_drawGlyphRun_0<RetType> {
  fn drawGlyphRun_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawGlyphRun_0<(/*void*/)> for (usize,usize) {
  fn drawGlyphRun_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter12drawGlyphRunERK7QPointFRK9QGlyphRun", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:409
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawStaticText(const QPointF &, const QStaticText &)

/*
Draws the given staticText at the given topLeftPosition.

The text will be drawn using the font and the transformation set on the painter. If the font and/or transformation set on the painter are different from the ones used to initialize the layout of the QStaticText, then the layout will have to be recalculated. Use QStaticText::prepare() to initialize staticText with the font and transformation with which it will later be drawn.

If topLeftPosition is not the same as when staticText was initialized, or when it was last drawn, then there will be a slight overhead when translating the text to its new position.

Note: If the painter's transformation is not affine, then staticText will be drawn using regular calls to drawText(), losing any potential for performance improvement.

Note: The y-position is used as the top of the font.

This function was introduced in  Qt 4.7.

See also QStaticText.
*/
impl /*struct*/ QPainter {
  pub fn drawStaticText_0<RetType, T: QPainter_drawStaticText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawStaticText_0(self);
    // return 1;
  }
}
pub trait QPainter_drawStaticText_0<RetType> {
  fn drawStaticText_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawStaticText_0<(/*void*/)> for (usize,usize) {
  fn drawStaticText_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter14drawStaticTextERK7QPointFRK11QStaticText", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:410
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawStaticText(const QPoint &, const QStaticText &)

/*
Draws the given staticText at the given topLeftPosition.

The text will be drawn using the font and the transformation set on the painter. If the font and/or transformation set on the painter are different from the ones used to initialize the layout of the QStaticText, then the layout will have to be recalculated. Use QStaticText::prepare() to initialize staticText with the font and transformation with which it will later be drawn.

If topLeftPosition is not the same as when staticText was initialized, or when it was last drawn, then there will be a slight overhead when translating the text to its new position.

Note: If the painter's transformation is not affine, then staticText will be drawn using regular calls to drawText(), losing any potential for performance improvement.

Note: The y-position is used as the top of the font.

This function was introduced in  Qt 4.7.

See also QStaticText.
*/
impl /*struct*/ QPainter {
  pub fn drawStaticText_1<RetType, T: QPainter_drawStaticText_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawStaticText_1(self);
    // return 1;
  }
}
pub trait QPainter_drawStaticText_1<RetType> {
  fn drawStaticText_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawStaticText_1<(/*void*/)> for (usize,usize) {
  fn drawStaticText_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter14drawStaticTextERK6QPointRK11QStaticText", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:411
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void drawStaticText(int, int, const QStaticText &)

/*
Draws the given staticText at the given topLeftPosition.

The text will be drawn using the font and the transformation set on the painter. If the font and/or transformation set on the painter are different from the ones used to initialize the layout of the QStaticText, then the layout will have to be recalculated. Use QStaticText::prepare() to initialize staticText with the font and transformation with which it will later be drawn.

If topLeftPosition is not the same as when staticText was initialized, or when it was last drawn, then there will be a slight overhead when translating the text to its new position.

Note: If the painter's transformation is not affine, then staticText will be drawn using regular calls to drawText(), losing any potential for performance improvement.

Note: The y-position is used as the top of the font.

This function was introduced in  Qt 4.7.

See also QStaticText.
*/
impl /*struct*/ QPainter {
  pub fn drawStaticText_2<RetType, T: QPainter_drawStaticText_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawStaticText_2(self);
    // return 1;
  }
}
pub trait QPainter_drawStaticText_2<RetType> {
  fn drawStaticText_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawStaticText_2<(/*void*/)> for (i32,i32,usize) {
  fn drawStaticText_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter14drawStaticTextEiiRK11QStaticText", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:413
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawText(const QPointF &, const QString &)

/*
Draws the given text with the currently defined text direction, beginning at the given position.

This function does not handle the newline character (\n), as it cannot break text into multiple lines, and it cannot display the newline character. Use the QPainter::drawText() overload that takes a rectangle instead if you want to draw multiple lines of text with the newline character, or if you want the text to be wrapped.

By default, QPainter draws text anti-aliased.

Note: The y-position is used as the baseline of the font.

See also setFont() and setPen().
*/
impl /*struct*/ QPainter {
  pub fn drawText_0<RetType, T: QPainter_drawText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawText_0(self);
    // return 1;
  }
}
pub trait QPainter_drawText_0<RetType> {
  fn drawText_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawText_0<(/*void*/)> for (usize,usize) {
  fn drawText_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8drawTextERK7QPointFRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:414
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawText(const QPoint &, const QString &)

/*
Draws the given text with the currently defined text direction, beginning at the given position.

This function does not handle the newline character (\n), as it cannot break text into multiple lines, and it cannot display the newline character. Use the QPainter::drawText() overload that takes a rectangle instead if you want to draw multiple lines of text with the newline character, or if you want the text to be wrapped.

By default, QPainter draws text anti-aliased.

Note: The y-position is used as the baseline of the font.

See also setFont() and setPen().
*/
impl /*struct*/ QPainter {
  pub fn drawText_1<RetType, T: QPainter_drawText_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawText_1(self);
    // return 1;
  }
}
pub trait QPainter_drawText_1<RetType> {
  fn drawText_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawText_1<(/*void*/)> for (usize,usize) {
  fn drawText_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8drawTextERK6QPointRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:415
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void drawText(int, int, const QString &)

/*
Draws the given text with the currently defined text direction, beginning at the given position.

This function does not handle the newline character (\n), as it cannot break text into multiple lines, and it cannot display the newline character. Use the QPainter::drawText() overload that takes a rectangle instead if you want to draw multiple lines of text with the newline character, or if you want the text to be wrapped.

By default, QPainter draws text anti-aliased.

Note: The y-position is used as the baseline of the font.

See also setFont() and setPen().
*/
impl /*struct*/ QPainter {
  pub fn drawText_2<RetType, T: QPainter_drawText_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawText_2(self);
    // return 1;
  }
}
pub trait QPainter_drawText_2<RetType> {
  fn drawText_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawText_2<(/*void*/)> for (i32,i32,usize) {
  fn drawText_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8drawTextEiiRK7QString", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:417
// index:3
// Public Visibility=Default Availability=Available
// [-2] void drawText(const QPointF &, const QString &, int, int)

/*
Draws the given text with the currently defined text direction, beginning at the given position.

This function does not handle the newline character (\n), as it cannot break text into multiple lines, and it cannot display the newline character. Use the QPainter::drawText() overload that takes a rectangle instead if you want to draw multiple lines of text with the newline character, or if you want the text to be wrapped.

By default, QPainter draws text anti-aliased.

Note: The y-position is used as the baseline of the font.

See also setFont() and setPen().
*/
impl /*struct*/ QPainter {
  pub fn drawText_3<RetType, T: QPainter_drawText_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawText_3(self);
    // return 1;
  }
}
pub trait QPainter_drawText_3<RetType> {
  fn drawText_3(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawText_3<(/*void*/)> for (usize,usize,i32,i32) {
  fn drawText_3(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8drawTextERK7QPointFRK7QStringii", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:419
// index:4
// Public Visibility=Default Availability=Available
// [-2] void drawText(const QRectF &, int, const QString &, QRectF *)

/*
Draws the given text with the currently defined text direction, beginning at the given position.

This function does not handle the newline character (\n), as it cannot break text into multiple lines, and it cannot display the newline character. Use the QPainter::drawText() overload that takes a rectangle instead if you want to draw multiple lines of text with the newline character, or if you want the text to be wrapped.

By default, QPainter draws text anti-aliased.

Note: The y-position is used as the baseline of the font.

See also setFont() and setPen().
*/
impl /*struct*/ QPainter {
  pub fn drawText_4<RetType, T: QPainter_drawText_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawText_4(self);
    // return 1;
  }
}
pub trait QPainter_drawText_4<RetType> {
  fn drawText_4(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawText_4<(/*void*/)> for (usize,i32,usize,usize) {
  fn drawText_4(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8drawTextERK6QRectFiRK7QStringPS0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:420
// index:5
// Public Visibility=Default Availability=Available
// [-2] void drawText(const QRect &, int, const QString &, QRect *)

/*
Draws the given text with the currently defined text direction, beginning at the given position.

This function does not handle the newline character (\n), as it cannot break text into multiple lines, and it cannot display the newline character. Use the QPainter::drawText() overload that takes a rectangle instead if you want to draw multiple lines of text with the newline character, or if you want the text to be wrapped.

By default, QPainter draws text anti-aliased.

Note: The y-position is used as the baseline of the font.

See also setFont() and setPen().
*/
impl /*struct*/ QPainter {
  pub fn drawText_5<RetType, T: QPainter_drawText_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawText_5(self);
    // return 1;
  }
}
pub trait QPainter_drawText_5<RetType> {
  fn drawText_5(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawText_5<(/*void*/)> for (usize,i32,usize,usize) {
  fn drawText_5(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8drawTextERK5QRectiRK7QStringPS0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:421
// index:6
// Public inline Visibility=Default Availability=Available
// [-2] void drawText(int, int, int, int, int, const QString &, QRect *)

/*
Draws the given text with the currently defined text direction, beginning at the given position.

This function does not handle the newline character (\n), as it cannot break text into multiple lines, and it cannot display the newline character. Use the QPainter::drawText() overload that takes a rectangle instead if you want to draw multiple lines of text with the newline character, or if you want the text to be wrapped.

By default, QPainter draws text anti-aliased.

Note: The y-position is used as the baseline of the font.

See also setFont() and setPen().
*/
impl /*struct*/ QPainter {
  pub fn drawText_6<RetType, T: QPainter_drawText_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawText_6(self);
    // return 1;
  }
}
pub trait QPainter_drawText_6<RetType> {
  fn drawText_6(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawText_6<(/*void*/)> for (i32,i32,i32,i32,i32,usize,usize) {
  fn drawText_6(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8drawTextEiiiiiRK7QStringP5QRect", 7,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:423
// index:7
// Public Visibility=Default Availability=Available
// [-2] void drawText(const QRectF &, const QString &, const QTextOption &)

/*
Draws the given text with the currently defined text direction, beginning at the given position.

This function does not handle the newline character (\n), as it cannot break text into multiple lines, and it cannot display the newline character. Use the QPainter::drawText() overload that takes a rectangle instead if you want to draw multiple lines of text with the newline character, or if you want the text to be wrapped.

By default, QPainter draws text anti-aliased.

Note: The y-position is used as the baseline of the font.

See also setFont() and setPen().
*/
impl /*struct*/ QPainter {
  pub fn drawText_7<RetType, T: QPainter_drawText_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawText_7(self);
    // return 1;
  }
}
pub trait QPainter_drawText_7<RetType> {
  fn drawText_7(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawText_7<(/*void*/)> for (usize,usize,usize) {
  fn drawText_7(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8drawTextERK6QRectFRK7QStringRK11QTextOption", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:425
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF boundingRect(const QRectF &, int, const QString &)

/*
Returns the bounding rectangle of the text as it will appear when drawn inside the given rectangle with the specified flags using the currently set font(); i.e the function tells you where the drawText() function will draw when given the same arguments.

If the text does not fit within the given rectangle using the specified flags, the function returns the required rectangle.

The flags argument is a bitwise OR of the following flags:


Qt::AlignLeft
Qt::AlignRight
Qt::AlignHCenter
Qt::AlignTop
Qt::AlignBottom
Qt::AlignVCenter
Qt::AlignCenter
Qt::TextSingleLine
Qt::TextExpandTabs
Qt::TextShowMnemonic
Qt::TextWordWrap
Qt::TextIncludeTrailingSpaces


If several of the horizontal or several of the vertical alignment flags are set, the resulting alignment is undefined.

See also drawText(), Qt::Alignment, and Qt::TextFlag.
*/
impl /*struct*/ QPainter {
  pub fn boundingRect_0<RetType, T: QPainter_boundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_0(self);
    // return 1;
  }
}
pub trait QPainter_boundingRect_0<RetType> {
  fn boundingRect_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_boundingRect_0<usize> for (usize,i32,usize) {
  fn boundingRect_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QPainter12boundingRectERK6QRectFiRK7QString", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:426
// index:1
// Public Visibility=Default Availability=Available
// [16] QRect boundingRect(const QRect &, int, const QString &)

/*
Returns the bounding rectangle of the text as it will appear when drawn inside the given rectangle with the specified flags using the currently set font(); i.e the function tells you where the drawText() function will draw when given the same arguments.

If the text does not fit within the given rectangle using the specified flags, the function returns the required rectangle.

The flags argument is a bitwise OR of the following flags:


Qt::AlignLeft
Qt::AlignRight
Qt::AlignHCenter
Qt::AlignTop
Qt::AlignBottom
Qt::AlignVCenter
Qt::AlignCenter
Qt::TextSingleLine
Qt::TextExpandTabs
Qt::TextShowMnemonic
Qt::TextWordWrap
Qt::TextIncludeTrailingSpaces


If several of the horizontal or several of the vertical alignment flags are set, the resulting alignment is undefined.

See also drawText(), Qt::Alignment, and Qt::TextFlag.
*/
impl /*struct*/ QPainter {
  pub fn boundingRect_1<RetType, T: QPainter_boundingRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_1(self);
    // return 1;
  }
}
pub trait QPainter_boundingRect_1<RetType> {
  fn boundingRect_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_boundingRect_1<usize> for (usize,i32,usize) {
  fn boundingRect_1(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QPainter12boundingRectERK5QRectiRK7QString", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:427
// index:2
// Public inline Visibility=Default Availability=Available
// [16] QRect boundingRect(int, int, int, int, int, const QString &)

/*
Returns the bounding rectangle of the text as it will appear when drawn inside the given rectangle with the specified flags using the currently set font(); i.e the function tells you where the drawText() function will draw when given the same arguments.

If the text does not fit within the given rectangle using the specified flags, the function returns the required rectangle.

The flags argument is a bitwise OR of the following flags:


Qt::AlignLeft
Qt::AlignRight
Qt::AlignHCenter
Qt::AlignTop
Qt::AlignBottom
Qt::AlignVCenter
Qt::AlignCenter
Qt::TextSingleLine
Qt::TextExpandTabs
Qt::TextShowMnemonic
Qt::TextWordWrap
Qt::TextIncludeTrailingSpaces


If several of the horizontal or several of the vertical alignment flags are set, the resulting alignment is undefined.

See also drawText(), Qt::Alignment, and Qt::TextFlag.
*/
impl /*struct*/ QPainter {
  pub fn boundingRect_2<RetType, T: QPainter_boundingRect_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_2(self);
    // return 1;
  }
}
pub trait QPainter_boundingRect_2<RetType> {
  fn boundingRect_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_boundingRect_2<usize> for (i32,i32,i32,i32,i32,usize) {
  fn boundingRect_2(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QPainter12boundingRectEiiiiiRK7QString", 6,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:429
// index:3
// Public Visibility=Default Availability=Available
// [32] QRectF boundingRect(const QRectF &, const QString &, const QTextOption &)

/*
Returns the bounding rectangle of the text as it will appear when drawn inside the given rectangle with the specified flags using the currently set font(); i.e the function tells you where the drawText() function will draw when given the same arguments.

If the text does not fit within the given rectangle using the specified flags, the function returns the required rectangle.

The flags argument is a bitwise OR of the following flags:


Qt::AlignLeft
Qt::AlignRight
Qt::AlignHCenter
Qt::AlignTop
Qt::AlignBottom
Qt::AlignVCenter
Qt::AlignCenter
Qt::TextSingleLine
Qt::TextExpandTabs
Qt::TextShowMnemonic
Qt::TextWordWrap
Qt::TextIncludeTrailingSpaces


If several of the horizontal or several of the vertical alignment flags are set, the resulting alignment is undefined.

See also drawText(), Qt::Alignment, and Qt::TextFlag.
*/
impl /*struct*/ QPainter {
  pub fn boundingRect_3<RetType, T: QPainter_boundingRect_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_3(self);
    // return 1;
  }
}
pub trait QPainter_boundingRect_3<RetType> {
  fn boundingRect_3(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_boundingRect_3<usize> for (usize,usize,usize) {
  fn boundingRect_3(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QPainter12boundingRectERK6QRectFRK7QStringRK11QTextOption", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:431
// index:0
// Public Visibility=Default Availability=Available
// [-2] void drawTextItem(const QPointF &, const QTextItem &)

/*

*/
impl /*struct*/ QPainter {
  pub fn drawTextItem_0<RetType, T: QPainter_drawTextItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawTextItem_0(self);
    // return 1;
  }
}
pub trait QPainter_drawTextItem_0<RetType> {
  fn drawTextItem_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawTextItem_0<(/*void*/)> for (usize,usize) {
  fn drawTextItem_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter12drawTextItemERK7QPointFRK9QTextItem", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:432
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void drawTextItem(int, int, const QTextItem &)

/*

*/
impl /*struct*/ QPainter {
  pub fn drawTextItem_1<RetType, T: QPainter_drawTextItem_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawTextItem_1(self);
    // return 1;
  }
}
pub trait QPainter_drawTextItem_1<RetType> {
  fn drawTextItem_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawTextItem_1<(/*void*/)> for (i32,i32,usize) {
  fn drawTextItem_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter12drawTextItemEiiRK9QTextItem", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:433
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void drawTextItem(const QPoint &, const QTextItem &)

/*

*/
impl /*struct*/ QPainter {
  pub fn drawTextItem_2<RetType, T: QPainter_drawTextItem_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawTextItem_2(self);
    // return 1;
  }
}
pub trait QPainter_drawTextItem_2<RetType> {
  fn drawTextItem_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_drawTextItem_2<(/*void*/)> for (usize,usize) {
  fn drawTextItem_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter12drawTextItemERK6QPointRK9QTextItem", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:435
// index:0
// Public Visibility=Default Availability=Available
// [-2] void fillRect(const QRectF &, const QBrush &)

/*
Fills the given rectangle with the brush specified.

Alternatively, you can specify a QColor instead of a QBrush; the QBrush constructor (taking a QColor argument) will automatically create a solid pattern brush.

See also drawRect().
*/
impl /*struct*/ QPainter {
  pub fn fillRect_0<RetType, T: QPainter_fillRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fillRect_0(self);
    // return 1;
  }
}
pub trait QPainter_fillRect_0<RetType> {
  fn fillRect_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_fillRect_0<(/*void*/)> for (usize,usize) {
  fn fillRect_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8fillRectERK6QRectFRK6QBrush", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:436
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void fillRect(int, int, int, int, const QBrush &)

/*
Fills the given rectangle with the brush specified.

Alternatively, you can specify a QColor instead of a QBrush; the QBrush constructor (taking a QColor argument) will automatically create a solid pattern brush.

See also drawRect().
*/
impl /*struct*/ QPainter {
  pub fn fillRect_1<RetType, T: QPainter_fillRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fillRect_1(self);
    // return 1;
  }
}
pub trait QPainter_fillRect_1<RetType> {
  fn fillRect_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_fillRect_1<(/*void*/)> for (i32,i32,i32,i32,usize) {
  fn fillRect_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8fillRectEiiiiRK6QBrush", 5,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:437
// index:2
// Public Visibility=Default Availability=Available
// [-2] void fillRect(const QRect &, const QBrush &)

/*
Fills the given rectangle with the brush specified.

Alternatively, you can specify a QColor instead of a QBrush; the QBrush constructor (taking a QColor argument) will automatically create a solid pattern brush.

See also drawRect().
*/
impl /*struct*/ QPainter {
  pub fn fillRect_2<RetType, T: QPainter_fillRect_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fillRect_2(self);
    // return 1;
  }
}
pub trait QPainter_fillRect_2<RetType> {
  fn fillRect_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_fillRect_2<(/*void*/)> for (usize,usize) {
  fn fillRect_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8fillRectERK5QRectRK6QBrush", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:439
// index:3
// Public Visibility=Default Availability=Available
// [-2] void fillRect(const QRectF &, const QColor &)

/*
Fills the given rectangle with the brush specified.

Alternatively, you can specify a QColor instead of a QBrush; the QBrush constructor (taking a QColor argument) will automatically create a solid pattern brush.

See also drawRect().
*/
impl /*struct*/ QPainter {
  pub fn fillRect_3<RetType, T: QPainter_fillRect_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fillRect_3(self);
    // return 1;
  }
}
pub trait QPainter_fillRect_3<RetType> {
  fn fillRect_3(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_fillRect_3<(/*void*/)> for (usize,usize) {
  fn fillRect_3(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8fillRectERK6QRectFRK6QColor", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:440
// index:4
// Public inline Visibility=Default Availability=Available
// [-2] void fillRect(int, int, int, int, const QColor &)

/*
Fills the given rectangle with the brush specified.

Alternatively, you can specify a QColor instead of a QBrush; the QBrush constructor (taking a QColor argument) will automatically create a solid pattern brush.

See also drawRect().
*/
impl /*struct*/ QPainter {
  pub fn fillRect_4<RetType, T: QPainter_fillRect_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fillRect_4(self);
    // return 1;
  }
}
pub trait QPainter_fillRect_4<RetType> {
  fn fillRect_4(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_fillRect_4<(/*void*/)> for (i32,i32,i32,i32,usize) {
  fn fillRect_4(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8fillRectEiiiiRK6QColor", 5,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:441
// index:5
// Public Visibility=Default Availability=Available
// [-2] void fillRect(const QRect &, const QColor &)

/*
Fills the given rectangle with the brush specified.

Alternatively, you can specify a QColor instead of a QBrush; the QBrush constructor (taking a QColor argument) will automatically create a solid pattern brush.

See also drawRect().
*/
impl /*struct*/ QPainter {
  pub fn fillRect_5<RetType, T: QPainter_fillRect_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fillRect_5(self);
    // return 1;
  }
}
pub trait QPainter_fillRect_5<RetType> {
  fn fillRect_5(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_fillRect_5<(/*void*/)> for (usize,usize) {
  fn fillRect_5(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8fillRectERK5QRectRK6QColor", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:443
// index:6
// Public inline Visibility=Default Availability=Available
// [-2] void fillRect(int, int, int, int, Qt::GlobalColor)

/*
Fills the given rectangle with the brush specified.

Alternatively, you can specify a QColor instead of a QBrush; the QBrush constructor (taking a QColor argument) will automatically create a solid pattern brush.

See also drawRect().
*/
impl /*struct*/ QPainter {
  pub fn fillRect_6<RetType, T: QPainter_fillRect_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fillRect_6(self);
    // return 1;
  }
}
pub trait QPainter_fillRect_6<RetType> {
  fn fillRect_6(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_fillRect_6<(/*void*/)> for (i32,i32,i32,i32,i32) {
  fn fillRect_6(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8fillRectEiiiiN2Qt11GlobalColorE", 5,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:444
// index:7
// Public inline Visibility=Default Availability=Available
// [-2] void fillRect(const QRect &, Qt::GlobalColor)

/*
Fills the given rectangle with the brush specified.

Alternatively, you can specify a QColor instead of a QBrush; the QBrush constructor (taking a QColor argument) will automatically create a solid pattern brush.

See also drawRect().
*/
impl /*struct*/ QPainter {
  pub fn fillRect_7<RetType, T: QPainter_fillRect_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fillRect_7(self);
    // return 1;
  }
}
pub trait QPainter_fillRect_7<RetType> {
  fn fillRect_7(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_fillRect_7<(/*void*/)> for (usize,i32) {
  fn fillRect_7(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8fillRectERK5QRectN2Qt11GlobalColorE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:445
// index:8
// Public inline Visibility=Default Availability=Available
// [-2] void fillRect(const QRectF &, Qt::GlobalColor)

/*
Fills the given rectangle with the brush specified.

Alternatively, you can specify a QColor instead of a QBrush; the QBrush constructor (taking a QColor argument) will automatically create a solid pattern brush.

See also drawRect().
*/
impl /*struct*/ QPainter {
  pub fn fillRect_8<RetType, T: QPainter_fillRect_8<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fillRect_8(self);
    // return 1;
  }
}
pub trait QPainter_fillRect_8<RetType> {
  fn fillRect_8(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_fillRect_8<(/*void*/)> for (usize,i32) {
  fn fillRect_8(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8fillRectERK6QRectFN2Qt11GlobalColorE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:447
// index:9
// Public inline Visibility=Default Availability=Available
// [-2] void fillRect(int, int, int, int, Qt::BrushStyle)

/*
Fills the given rectangle with the brush specified.

Alternatively, you can specify a QColor instead of a QBrush; the QBrush constructor (taking a QColor argument) will automatically create a solid pattern brush.

See also drawRect().
*/
impl /*struct*/ QPainter {
  pub fn fillRect_9<RetType, T: QPainter_fillRect_9<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fillRect_9(self);
    // return 1;
  }
}
pub trait QPainter_fillRect_9<RetType> {
  fn fillRect_9(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_fillRect_9<(/*void*/)> for (i32,i32,i32,i32,i32) {
  fn fillRect_9(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8fillRectEiiiiN2Qt10BrushStyleE", 5,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:448
// index:10
// Public inline Visibility=Default Availability=Available
// [-2] void fillRect(const QRect &, Qt::BrushStyle)

/*
Fills the given rectangle with the brush specified.

Alternatively, you can specify a QColor instead of a QBrush; the QBrush constructor (taking a QColor argument) will automatically create a solid pattern brush.

See also drawRect().
*/
impl /*struct*/ QPainter {
  pub fn fillRect_10<RetType, T: QPainter_fillRect_10<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fillRect_10(self);
    // return 1;
  }
}
pub trait QPainter_fillRect_10<RetType> {
  fn fillRect_10(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_fillRect_10<(/*void*/)> for (usize,i32) {
  fn fillRect_10(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8fillRectERK5QRectN2Qt10BrushStyleE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:449
// index:11
// Public inline Visibility=Default Availability=Available
// [-2] void fillRect(const QRectF &, Qt::BrushStyle)

/*
Fills the given rectangle with the brush specified.

Alternatively, you can specify a QColor instead of a QBrush; the QBrush constructor (taking a QColor argument) will automatically create a solid pattern brush.

See also drawRect().
*/
impl /*struct*/ QPainter {
  pub fn fillRect_11<RetType, T: QPainter_fillRect_11<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fillRect_11(self);
    // return 1;
  }
}
pub trait QPainter_fillRect_11<RetType> {
  fn fillRect_11(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_fillRect_11<(/*void*/)> for (usize,i32) {
  fn fillRect_11(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter8fillRectERK6QRectFN2Qt10BrushStyleE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:451
// index:0
// Public Visibility=Default Availability=Available
// [-2] void eraseRect(const QRectF &)

/*
Erases the area inside the given rectangle. Equivalent to calling


  fillRect(rectangle, background()).



See also fillRect().
*/
impl /*struct*/ QPainter {
  pub fn eraseRect_0<RetType, T: QPainter_eraseRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eraseRect_0(self);
    // return 1;
  }
}
pub trait QPainter_eraseRect_0<RetType> {
  fn eraseRect_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_eraseRect_0<(/*void*/)> for (usize) {
  fn eraseRect_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9eraseRectERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:452
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void eraseRect(int, int, int, int)

/*
Erases the area inside the given rectangle. Equivalent to calling


  fillRect(rectangle, background()).



See also fillRect().
*/
impl /*struct*/ QPainter {
  pub fn eraseRect_1<RetType, T: QPainter_eraseRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eraseRect_1(self);
    // return 1;
  }
}
pub trait QPainter_eraseRect_1<RetType> {
  fn eraseRect_1(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_eraseRect_1<(/*void*/)> for (i32,i32,i32,i32) {
  fn eraseRect_1(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9eraseRectEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:453
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void eraseRect(const QRect &)

/*
Erases the area inside the given rectangle. Equivalent to calling


  fillRect(rectangle, background()).



See also fillRect().
*/
impl /*struct*/ QPainter {
  pub fn eraseRect_2<RetType, T: QPainter_eraseRect_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eraseRect_2(self);
    // return 1;
  }
}
pub trait QPainter_eraseRect_2<RetType> {
  fn eraseRect_2(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_eraseRect_2<(/*void*/)> for (usize) {
  fn eraseRect_2(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter9eraseRectERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:455
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRenderHint(QPainter::RenderHint, bool)

/*
Sets the given render hint on the painter if on is true; otherwise clears the render hint.

See also setRenderHints(), renderHints(), and Rendering Quality.
*/
impl /*struct*/ QPainter {
  pub fn setRenderHint_0<RetType, T: QPainter_setRenderHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRenderHint_0(self);
    // return 1;
  }
}
pub trait QPainter_setRenderHint_0<RetType> {
  fn setRenderHint_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setRenderHint_0<(/*void*/)> for (i32,bool) {
  fn setRenderHint_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter13setRenderHintENS_10RenderHintEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:456
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRenderHints(QPainter::RenderHints, bool)

/*
Sets the given render hints on the painter if on is true; otherwise clears the render hints.

This function was introduced in  Qt 4.2.

See also setRenderHint(), renderHints(), and Rendering Quality.
*/
impl /*struct*/ QPainter {
  pub fn setRenderHints_0<RetType, T: QPainter_setRenderHints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRenderHints_0(self);
    // return 1;
  }
}
pub trait QPainter_setRenderHints_0<RetType> {
  fn setRenderHints_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_setRenderHints_0<(/*void*/)> for (i32,bool) {
  fn setRenderHints_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter14setRenderHintsE6QFlagsINS_10RenderHintEEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:457
// index:0
// Public Visibility=Default Availability=Available
// [4] QPainter::RenderHints renderHints() const

/*
Returns a flag that specifies the rendering hints that are set for this painter.

See also setRenderHints(), testRenderHint(), and Rendering Quality.
*/
impl /*struct*/ QPainter {
  pub fn renderHints_0<RetType, T: QPainter_renderHints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.renderHints_0(self);
    // return 1;
  }
}
pub trait QPainter_renderHints_0<RetType> {
  fn renderHints_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_renderHints_0<i32> for () {
  fn renderHints_0(self , rsthis: & QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter11renderHintsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:458
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool testRenderHint(QPainter::RenderHint) const

/*
Returns true if hint is set; otherwise returns false.

This function was introduced in  Qt 4.3.

See also renderHints() and setRenderHint().
*/
impl /*struct*/ QPainter {
  pub fn testRenderHint_0<RetType, T: QPainter_testRenderHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.testRenderHint_0(self);
    // return 1;
  }
}
pub trait QPainter_testRenderHint_0<RetType> {
  fn testRenderHint_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_testRenderHint_0<bool> for (i32) {
  fn testRenderHint_0(self , rsthis: & QPainter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter14testRenderHintENS_10RenderHintE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:460
// index:0
// Public Visibility=Default Availability=Available
// [8] QPaintEngine * paintEngine() const

/*
Returns the paint engine that the painter is currently operating on if the painter is active; otherwise 0.

See also isActive().
*/
impl /*struct*/ QPainter {
  pub fn paintEngine_0<RetType, T: QPainter_paintEngine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEngine_0(self);
    // return 1;
  }
}
pub trait QPainter_paintEngine_0<RetType> {
  fn paintEngine_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_paintEngine_0<usize> for () {
  fn paintEngine_0(self , rsthis: & QPainter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QPainter11paintEngineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:462
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setRedirected(const QPaintDevice *, QPaintDevice *, const QPoint &)

/*

*/
impl /*struct*/ QPainter {
  pub fn setRedirected_0<RetType, T: QPainter_setRedirected_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setRedirected_0();
    // return 1;
  }
}
pub trait QPainter_setRedirected_0<RetType> {
  fn setRedirected_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPainter_setRedirected_0<(/*void*/)> for (usize,usize,usize) {
  fn setRedirected_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter13setRedirectedEPK12QPaintDevicePS0_RK6QPoint", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:464
// index:0
// Public static Visibility=Default Availability=Available
// [8] QPaintDevice * redirected(const QPaintDevice *, QPoint *)

/*

*/
impl /*struct*/ QPainter {
  pub fn redirected_0<RetType, T: QPainter_redirected_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.redirected_0();
    // return 1;
  }
}
pub trait QPainter_redirected_0<RetType> {
  fn redirected_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPainter_redirected_0<usize> for (usize,usize) {
  fn redirected_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QPainter10redirectedEPK12QPaintDeviceP6QPoint", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainter.h:465
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void restoreRedirected(const QPaintDevice *)

/*

*/
impl /*struct*/ QPainter {
  pub fn restoreRedirected_0<RetType, T: QPainter_restoreRedirected_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.restoreRedirected_0();
    // return 1;
  }
}
pub trait QPainter_restoreRedirected_0<RetType> {
  fn restoreRedirected_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPainter_restoreRedirected_0<(/*void*/)> for (usize) {
  fn restoreRedirected_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QPainter17restoreRedirectedEPK12QPaintDevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:467
// index:0
// Public Visibility=Default Availability=Available
// [-2] void beginNativePainting()

/*
Flushes the painting pipeline and prepares for the user issuing commands directly to the underlying graphics context. Must be followed by a call to endNativePainting().

Note that only the states the underlying paint engine changes will be reset to their respective default states. The states we reset may change from release to release. The following states are currently reset in the OpenGL 2 engine:


blending is disabled
the depth, stencil and scissor tests are disabled
the active texture unit is reset to 0
the depth mask, depth function and the clear depth are reset to their default values
the stencil mask, stencil operation and stencil function are reset to their default values
the current color is reset to solid white


If, for example, the OpenGL polygon mode is changed by the user inside a beginNativePaint()/endNativePainting() block, it will not be reset to the default state by endNativePainting(). Here is an example that shows intermixing of painter commands and raw OpenGL commands:


  QPainter painter(this);
  painter.fillRect(0, 0, 128, 128, Qt::green);
  painter.beginNativePainting();

  glEnable(GL_SCISSOR_TEST);
  glScissor(0, 0, 64, 64);

  glClearColor(1, 0, 0, 1);
  glClear(GL_COLOR_BUFFER_BIT);

  glDisable(GL_SCISSOR_TEST);

  painter.endNativePainting();



This function was introduced in  Qt 4.6.

See also endNativePainting().
*/
impl /*struct*/ QPainter {
  pub fn beginNativePainting_0<RetType, T: QPainter_beginNativePainting_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beginNativePainting_0(self);
    // return 1;
  }
}
pub trait QPainter_beginNativePainting_0<RetType> {
  fn beginNativePainting_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_beginNativePainting_0<(/*void*/)> for () {
  fn beginNativePainting_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QPainter19beginNativePaintingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainter.h:468
// index:0
// Public Visibility=Default Availability=Available
// [-2] void endNativePainting()

/*
Restores the painter after manually issuing native painting commands. Lets the painter restore any native state that it relies on before calling any other painter commands.

This function was introduced in  Qt 4.6.

See also beginNativePainting().
*/
impl /*struct*/ QPainter {
  pub fn endNativePainting_0<RetType, T: QPainter_endNativePainting_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endNativePainting_0(self);
    // return 1;
  }
}
pub trait QPainter_endNativePainting_0<RetType> {
  fn endNativePainting_0(self , rsthis: & QPainter) -> RetType;
}
impl<'a> /*trait*/ QPainter_endNativePainting_0<(/*void*/)> for () {
  fn endNativePainting_0(self , rsthis: & QPainter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QPainter17endNativePaintingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QPainter__RenderHint = i32;
// 
pub const QPainter__Antialiasing :QPainter__RenderHint = 1;
// 
pub const QPainter__TextAntialiasing :QPainter__RenderHint = 2;
// 
pub const QPainter__SmoothPixmapTransform :QPainter__RenderHint = 4;
// 
pub const QPainter__HighQualityAntialiasing :QPainter__RenderHint = 8;
// 
pub const QPainter__NonCosmeticDefaultPen :QPainter__RenderHint = 16;
// 
pub const QPainter__Qt4CompatiblePainting :QPainter__RenderHint = 32;
pub fn QPainter_RenderHintItemName(val: i32) ->String {
  match val {
     QPainter__Antialiasing => // 1
     {return String::from("Antialiasing");}
     QPainter__TextAntialiasing => // 2
     {return String::from("TextAntialiasing");}
     QPainter__SmoothPixmapTransform => // 4
     {return String::from("SmoothPixmapTransform");}
     QPainter__HighQualityAntialiasing => // 8
     {return String::from("HighQualityAntialiasing");}
     QPainter__NonCosmeticDefaultPen => // 16
     {return String::from("NonCosmeticDefaultPen");}
     QPainter__Qt4CompatiblePainting => // 32
     {return String::from("Qt4CompatiblePainting");}
  _ => {return format!("{}", val);}
}
}
pub fn QPainter_RenderHintItemName_s(val: i32) ->String {
  //var nilthis *QPainter
  //return nilthis.RenderHintItemName(val);
  return QPainter_RenderHintItemName(val);
}


/*


*/
pub type QPainter__PixmapFragmentHint = i32;
// 
pub const QPainter__OpaqueHint :QPainter__PixmapFragmentHint = 1;
pub fn QPainter_PixmapFragmentHintItemName(val: i32) ->String {
  match val {
     QPainter__OpaqueHint => // 1
     {return String::from("OpaqueHint");}
  _ => {return format!("{}", val);}
}
}
pub fn QPainter_PixmapFragmentHintItemName_s(val: i32) ->String {
  //var nilthis *QPainter
  //return nilthis.PixmapFragmentHintItemName(val);
  return QPainter_PixmapFragmentHintItemName(val);
}


/*
Defines the modes supported for digital image compositing. Composition modes are used to specify how the pixels in one image, the source, are merged with the pixel in another image, the destination.

Please note that the bitwise raster operation modes, denoted with a RasterOp prefix, are only natively supported in the X11 and raster paint engines. This means that the only way to utilize these modes on the Mac is via a QImage. The RasterOp denoted blend modes are not supported for pens and brushes with alpha components. Also, turning on the QPainter::Antialiasing render hint will effectively disable the RasterOp modes.





The most common type is SourceOver (often referred to as just alpha blending) where the source pixel is blended on top of the destination pixel in such a way that the alpha component of the source defines the translucency of the pixel.

Several composition modes require an alpha channel in the source or target images to have an effect. For optimal performance the image format Format_ARGB32_Premultiplied is preferred.

When a composition mode is set it applies to all painting operator, pens, brushes, gradients and pixmap/image drawing.



See also compositionMode(), setCompositionMode(), Composition Modes, and Image Composition Example.

*/
pub type QPainter__CompositionMode = i32;
// This is the default mode. The alpha of the source is used to blend the pixel on top of the destination.
pub const QPainter__CompositionMode_SourceOver :QPainter__CompositionMode = 0;
// The alpha of the destination is used to blend it on top of the source pixels. This mode is the inverse of CompositionMode_SourceOver.
pub const QPainter__CompositionMode_DestinationOver :QPainter__CompositionMode = 1;
// The pixels in the destination are cleared (set to fully transparent) independent of the source.
pub const QPainter__CompositionMode_Clear :QPainter__CompositionMode = 2;
// The output is the source pixel. (This means a basic copy operation and is identical to SourceOver when the source pixel is opaque).
pub const QPainter__CompositionMode_Source :QPainter__CompositionMode = 3;
// The output is the destination pixel. This means that the blending has no effect. This mode is the inverse of CompositionMode_Source.
pub const QPainter__CompositionMode_Destination :QPainter__CompositionMode = 4;
// The output is the source, where the alpha is reduced by that of the destination.
pub const QPainter__CompositionMode_SourceIn :QPainter__CompositionMode = 5;
// The output is the destination, where the alpha is reduced by that of the source. This mode is the inverse of CompositionMode_SourceIn.
pub const QPainter__CompositionMode_DestinationIn :QPainter__CompositionMode = 6;
// The output is the source, where the alpha is reduced by the inverse of destination.
pub const QPainter__CompositionMode_SourceOut :QPainter__CompositionMode = 7;
// The output is the destination, where the alpha is reduced by the inverse of the source. This mode is the inverse of CompositionMode_SourceOut.
pub const QPainter__CompositionMode_DestinationOut :QPainter__CompositionMode = 8;
// The source pixel is blended on top of the destination, with the alpha of the source pixel reduced by the alpha of the destination pixel.
pub const QPainter__CompositionMode_SourceAtop :QPainter__CompositionMode = 9;
// 
pub const QPainter__CompositionMode_DestinationAtop :QPainter__CompositionMode = 10;
// 
pub const QPainter__CompositionMode_Xor :QPainter__CompositionMode = 11;
// 
pub const QPainter__CompositionMode_Plus :QPainter__CompositionMode = 12;
// 
pub const QPainter__CompositionMode_Multiply :QPainter__CompositionMode = 13;
// 
pub const QPainter__CompositionMode_Screen :QPainter__CompositionMode = 14;
// 
pub const QPainter__CompositionMode_Overlay :QPainter__CompositionMode = 15;
// 
pub const QPainter__CompositionMode_Darken :QPainter__CompositionMode = 16;
// 
pub const QPainter__CompositionMode_Lighten :QPainter__CompositionMode = 17;
// 
pub const QPainter__CompositionMode_ColorDodge :QPainter__CompositionMode = 18;
// 
pub const QPainter__CompositionMode_ColorBurn :QPainter__CompositionMode = 19;
// 
pub const QPainter__CompositionMode_HardLight :QPainter__CompositionMode = 20;
// 
pub const QPainter__CompositionMode_SoftLight :QPainter__CompositionMode = 21;
// 
pub const QPainter__CompositionMode_Difference :QPainter__CompositionMode = 22;
// 
pub const QPainter__CompositionMode_Exclusion :QPainter__CompositionMode = 23;
// 
pub const QPainter__RasterOp_SourceOrDestination :QPainter__CompositionMode = 24;
// 
pub const QPainter__RasterOp_SourceAndDestination :QPainter__CompositionMode = 25;
// 
pub const QPainter__RasterOp_SourceXorDestination :QPainter__CompositionMode = 26;
// 
pub const QPainter__RasterOp_NotSourceAndNotDestination :QPainter__CompositionMode = 27;
// 
pub const QPainter__RasterOp_NotSourceOrNotDestination :QPainter__CompositionMode = 28;
// 
pub const QPainter__RasterOp_NotSourceXorDestination :QPainter__CompositionMode = 29;
// 
pub const QPainter__RasterOp_NotSource :QPainter__CompositionMode = 30;
// 
pub const QPainter__RasterOp_NotSourceAndDestination :QPainter__CompositionMode = 31;
// 
pub const QPainter__RasterOp_SourceAndNotDestination :QPainter__CompositionMode = 32;
// 
pub const QPainter__RasterOp_NotSourceOrDestination :QPainter__CompositionMode = 33;
// 
pub const QPainter__RasterOp_SourceOrNotDestination :QPainter__CompositionMode = 34;
// 
pub const QPainter__RasterOp_ClearDestination :QPainter__CompositionMode = 35;
// 
pub const QPainter__RasterOp_SetDestination :QPainter__CompositionMode = 36;
// 
pub const QPainter__RasterOp_NotDestination :QPainter__CompositionMode = 37;
pub fn QPainter_CompositionModeItemName(val: i32) ->String {
  match val {
     QPainter__CompositionMode_SourceOver => // 0
     {return String::from("CompositionMode_SourceOver");}
     QPainter__CompositionMode_DestinationOver => // 1
     {return String::from("CompositionMode_DestinationOver");}
     QPainter__CompositionMode_Clear => // 2
     {return String::from("CompositionMode_Clear");}
     QPainter__CompositionMode_Source => // 3
     {return String::from("CompositionMode_Source");}
     QPainter__CompositionMode_Destination => // 4
     {return String::from("CompositionMode_Destination");}
     QPainter__CompositionMode_SourceIn => // 5
     {return String::from("CompositionMode_SourceIn");}
     QPainter__CompositionMode_DestinationIn => // 6
     {return String::from("CompositionMode_DestinationIn");}
     QPainter__CompositionMode_SourceOut => // 7
     {return String::from("CompositionMode_SourceOut");}
     QPainter__CompositionMode_DestinationOut => // 8
     {return String::from("CompositionMode_DestinationOut");}
     QPainter__CompositionMode_SourceAtop => // 9
     {return String::from("CompositionMode_SourceAtop");}
     QPainter__CompositionMode_DestinationAtop => // 10
     {return String::from("CompositionMode_DestinationAtop");}
     QPainter__CompositionMode_Xor => // 11
     {return String::from("CompositionMode_Xor");}
     QPainter__CompositionMode_Plus => // 12
     {return String::from("CompositionMode_Plus");}
     QPainter__CompositionMode_Multiply => // 13
     {return String::from("CompositionMode_Multiply");}
     QPainter__CompositionMode_Screen => // 14
     {return String::from("CompositionMode_Screen");}
     QPainter__CompositionMode_Overlay => // 15
     {return String::from("CompositionMode_Overlay");}
     QPainter__CompositionMode_Darken => // 16
     {return String::from("CompositionMode_Darken");}
     QPainter__CompositionMode_Lighten => // 17
     {return String::from("CompositionMode_Lighten");}
     QPainter__CompositionMode_ColorDodge => // 18
     {return String::from("CompositionMode_ColorDodge");}
     QPainter__CompositionMode_ColorBurn => // 19
     {return String::from("CompositionMode_ColorBurn");}
     QPainter__CompositionMode_HardLight => // 20
     {return String::from("CompositionMode_HardLight");}
     QPainter__CompositionMode_SoftLight => // 21
     {return String::from("CompositionMode_SoftLight");}
     QPainter__CompositionMode_Difference => // 22
     {return String::from("CompositionMode_Difference");}
     QPainter__CompositionMode_Exclusion => // 23
     {return String::from("CompositionMode_Exclusion");}
     QPainter__RasterOp_SourceOrDestination => // 24
     {return String::from("RasterOp_SourceOrDestination");}
     QPainter__RasterOp_SourceAndDestination => // 25
     {return String::from("RasterOp_SourceAndDestination");}
     QPainter__RasterOp_SourceXorDestination => // 26
     {return String::from("RasterOp_SourceXorDestination");}
     QPainter__RasterOp_NotSourceAndNotDestination => // 27
     {return String::from("RasterOp_NotSourceAndNotDestination");}
     QPainter__RasterOp_NotSourceOrNotDestination => // 28
     {return String::from("RasterOp_NotSourceOrNotDestination");}
     QPainter__RasterOp_NotSourceXorDestination => // 29
     {return String::from("RasterOp_NotSourceXorDestination");}
     QPainter__RasterOp_NotSource => // 30
     {return String::from("RasterOp_NotSource");}
     QPainter__RasterOp_NotSourceAndDestination => // 31
     {return String::from("RasterOp_NotSourceAndDestination");}
     QPainter__RasterOp_SourceAndNotDestination => // 32
     {return String::from("RasterOp_SourceAndNotDestination");}
     QPainter__RasterOp_NotSourceOrDestination => // 33
     {return String::from("RasterOp_NotSourceOrDestination");}
     QPainter__RasterOp_SourceOrNotDestination => // 34
     {return String::from("RasterOp_SourceOrNotDestination");}
     QPainter__RasterOp_ClearDestination => // 35
     {return String::from("RasterOp_ClearDestination");}
     QPainter__RasterOp_SetDestination => // 36
     {return String::from("RasterOp_SetDestination");}
     QPainter__RasterOp_NotDestination => // 37
     {return String::from("RasterOp_NotDestination");}
  _ => {return format!("{}", val);}
}
}
pub fn QPainter_CompositionModeItemName_s(val: i32) ->String {
  //var nilthis *QPainter
  //return nilthis.CompositionModeItemName(val);
  return QPainter_CompositionModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
