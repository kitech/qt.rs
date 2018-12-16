

// mod ::gui::QPagedPaintDevice
// package qtgui
// /usr/include/qt/QtGui/qpagedpaintdevice.h
// #include <qpagedpaintdevice.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 36
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

// QPageLayout devicePageLayout()
// func (this *QPagedPaintDevice) InheritDevicePageLayout(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "devicePageLayout", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QPagedPaintDevice)=32
pub struct QPagedPaintDevice {
  qbase: QPaintDevice,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPagedPaintDevice_ITF interface {
//    QPaintDevice_ITF
//    QPagedPaintDevice_PTR() *QPagedPaintDevice
//}
//func (ptr *QPagedPaintDevice) QPagedPaintDevice_PTR() *QPagedPaintDevice { return ptr }

impl /*struct*/ QPagedPaintDevice {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPagedPaintDevice {
    return QPagedPaintDevice{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPagedPaintDevice {
//  type Target = QPagedPaintDeviceBASE;
//
//  fn deref(&self) -> &QPagedPaintDeviceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPagedPaintDeviceBASE> for QPagedPaintDevice {
//  fn as_ref(& self) -> & QPagedPaintDeviceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpagedpaintdevice.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPagedPaintDevice()

/*
Constructs a new paged paint device.
*/
// QPagedPaintDevice() ctx.fn_proto_cpp
impl /*struct*/ QPagedPaintDevice {
  pub fn QPagedPaintDevice_0<T: QPagedPaintDevice_QPagedPaintDevice_0>(value: T) -> QPagedPaintDevice {
    let rsthis = value.QPagedPaintDevice_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPagedPaintDevice_QPagedPaintDevice_0 {
  fn QPagedPaintDevice_0(self) -> QPagedPaintDevice;
}
// QPagedPaintDevice() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPagedPaintDevice_QPagedPaintDevice_0 for () {
  fn QPagedPaintDevice_0(self) -> QPagedPaintDevice {
    // unsafe{_ZN17QPagedPaintDeviceC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QPagedPaintDeviceC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPagedPaintDevice{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpagedpaintdevice.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QPagedPaintDevice()

/*

*/
pub fn DeleteQPagedPaintDevice(this :*mut QPagedPaintDevice) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QPagedPaintDeviceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qpagedpaintdevice.h:61
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool newPage()

/*
Starts a new page. Returns true on success.
*/
impl /*struct*/ QPagedPaintDevice {
  pub fn newPage_0<RetType, T: QPagedPaintDevice_newPage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.newPage_0(self);
    // return 1;
  }
}
pub trait QPagedPaintDevice_newPage_0<RetType> {
  fn newPage_0(self , rsthis: & QPagedPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPagedPaintDevice_newPage_0<bool> for () {
  fn newPage_0(self , rsthis: & QPagedPaintDevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QPagedPaintDevice7newPageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagedpaintdevice.h:219
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setPageLayout(const QPageLayout &)

/*
Sets the page layout to newPageLayout.

You should call this before calling QPainter::begin(), or immediately before calling newPage() to apply the new page layout to a new page. You should not call any painting methods between a call to setPageLayout() and newPage() as the wrong paint metrics may be used.

Returns true if the page layout was successfully set to newPageLayout.

This function was introduced in  Qt 5.3.

See also pageLayout().
*/
impl /*struct*/ QPagedPaintDevice {
  pub fn setPageLayout_0<RetType, T: QPagedPaintDevice_setPageLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPageLayout_0(self);
    // return 1;
  }
}
pub trait QPagedPaintDevice_setPageLayout_0<RetType> {
  fn setPageLayout_0(self , rsthis: & QPagedPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPagedPaintDevice_setPageLayout_0<bool> for (usize) {
  fn setPageLayout_0(self , rsthis: & QPagedPaintDevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QPagedPaintDevice13setPageLayoutERK11QPageLayout", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagedpaintdevice.h:220
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setPageSize(const QPageSize &)

/*
Sets the page size to pageSize.

To get the current QPageSize use pageLayout().pageSize().

You should call this before calling QPainter::begin(), or immediately before calling newPage() to apply the new page size to a new page. You should not call any painting methods between a call to setPageSize() and newPage() as the wrong paint metrics may be used.

Returns true if the page size was successfully set to pageSize.

This function was introduced in  Qt 5.3.

See also pageSize() and pageLayout().
*/
impl /*struct*/ QPagedPaintDevice {
  pub fn setPageSize_0<RetType, T: QPagedPaintDevice_setPageSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPageSize_0(self);
    // return 1;
  }
}
pub trait QPagedPaintDevice_setPageSize_0<RetType> {
  fn setPageSize_0(self , rsthis: & QPagedPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPagedPaintDevice_setPageSize_0<bool> for (usize) {
  fn setPageSize_0(self , rsthis: & QPagedPaintDevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QPagedPaintDevice11setPageSizeERK9QPageSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagedpaintdevice.h:226
// index:1
// Public virtual Visibility=Default Availability=Available
// [-2] void setPageSize(QPagedPaintDevice::PageSize)

/*
Sets the page size to pageSize.

To get the current QPageSize use pageLayout().pageSize().

You should call this before calling QPainter::begin(), or immediately before calling newPage() to apply the new page size to a new page. You should not call any painting methods between a call to setPageSize() and newPage() as the wrong paint metrics may be used.

Returns true if the page size was successfully set to pageSize.

This function was introduced in  Qt 5.3.

See also pageSize() and pageLayout().
*/
impl /*struct*/ QPagedPaintDevice {
  pub fn setPageSize_1<RetType, T: QPagedPaintDevice_setPageSize_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPageSize_1(self);
    // return 1;
  }
}
pub trait QPagedPaintDevice_setPageSize_1<RetType> {
  fn setPageSize_1(self , rsthis: & QPagedPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPagedPaintDevice_setPageSize_1<(/*void*/)> for (i32) {
  fn setPageSize_1(self , rsthis: & QPagedPaintDevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QPagedPaintDevice11setPageSizeENS_8PageSizeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpagedpaintdevice.h:221
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setPageOrientation(QPageLayout::Orientation)

/*
Sets the page orientation.

The page orientation is used to define the orientation of the page size when obtaining the page rect.

You should call this before calling QPainter::begin(), or immediately before calling newPage() to apply the new orientation to a new page. You should not call any painting methods between a call to setPageOrientation() and newPage() as the wrong paint metrics may be used.

To get the current QPageLayout::Orientation use pageLayout().pageOrientation().

Returns true if the page orientation was successfully set to orientation.

This function was introduced in  Qt 5.3.

See also pageLayout().
*/
impl /*struct*/ QPagedPaintDevice {
  pub fn setPageOrientation_0<RetType, T: QPagedPaintDevice_setPageOrientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPageOrientation_0(self);
    // return 1;
  }
}
pub trait QPagedPaintDevice_setPageOrientation_0<RetType> {
  fn setPageOrientation_0(self , rsthis: & QPagedPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPagedPaintDevice_setPageOrientation_0<bool> for (i32) {
  fn setPageOrientation_0(self , rsthis: & QPagedPaintDevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QPagedPaintDevice18setPageOrientationEN11QPageLayout11OrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagedpaintdevice.h:222
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setPageMargins(const QMarginsF &)

/*
Set the page margins in the current page layout units.

You should call this before calling QPainter::begin(), or immediately before calling newPage() to apply the new margins to a new page. You should not call any painting methods between a call to setPageMargins() and newPage() as the wrong paint metrics may be used.

To get the current page margins use pageLayout().pageMargins().

Returns true if the page margins were successfully set to margins.

This function was introduced in  Qt 5.3.

See also pageLayout().
*/
impl /*struct*/ QPagedPaintDevice {
  pub fn setPageMargins_0<RetType, T: QPagedPaintDevice_setPageMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPageMargins_0(self);
    // return 1;
  }
}
pub trait QPagedPaintDevice_setPageMargins_0<RetType> {
  fn setPageMargins_0(self , rsthis: & QPagedPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPagedPaintDevice_setPageMargins_0<bool> for (usize) {
  fn setPageMargins_0(self , rsthis: & QPagedPaintDevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QPagedPaintDevice14setPageMarginsERK9QMarginsF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagedpaintdevice.h:223
// index:1
// Public Visibility=Default Availability=Available
// [1] bool setPageMargins(const QMarginsF &, QPageLayout::Unit)

/*
Set the page margins in the current page layout units.

You should call this before calling QPainter::begin(), or immediately before calling newPage() to apply the new margins to a new page. You should not call any painting methods between a call to setPageMargins() and newPage() as the wrong paint metrics may be used.

To get the current page margins use pageLayout().pageMargins().

Returns true if the page margins were successfully set to margins.

This function was introduced in  Qt 5.3.

See also pageLayout().
*/
impl /*struct*/ QPagedPaintDevice {
  pub fn setPageMargins_1<RetType, T: QPagedPaintDevice_setPageMargins_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPageMargins_1(self);
    // return 1;
  }
}
pub trait QPagedPaintDevice_setPageMargins_1<RetType> {
  fn setPageMargins_1(self , rsthis: & QPagedPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPagedPaintDevice_setPageMargins_1<bool> for (usize,i32) {
  fn setPageMargins_1(self , rsthis: & QPagedPaintDevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QPagedPaintDevice14setPageMarginsERK9QMarginsFN11QPageLayout4UnitE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagedpaintdevice.h:224
// index:0
// Public Visibility=Default Availability=Available
// [8] QPageLayout pageLayout() const

/*
Returns the current page layout. Use this method to access the current QPageSize, QPageLayout::Orientation, QMarginsF, fullRect() and paintRect().

Note that you cannot use the setters on the returned object, you must either call the individual QPagedPaintDevice setters or use setPageLayout().

This function was introduced in  Qt 5.3.

See also setPageLayout(), setPageSize(), setPageOrientation(), and setPageMargins().
*/
impl /*struct*/ QPagedPaintDevice {
  pub fn pageLayout_0<RetType, T: QPagedPaintDevice_pageLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pageLayout_0(self);
    // return 1;
  }
}
pub trait QPagedPaintDevice_pageLayout_0<RetType> {
  fn pageLayout_0(self , rsthis: & QPagedPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPagedPaintDevice_pageLayout_0<usize> for () {
  fn pageLayout_0(self , rsthis: & QPagedPaintDevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QPagedPaintDevice10pageLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagedpaintdevice.h:227
// index:0
// Public Visibility=Default Availability=Available
// [4] QPagedPaintDevice::PageSize pageSize() const

/*
Returns the currently used page size.

See also setPageSize().
*/
impl /*struct*/ QPagedPaintDevice {
  pub fn pageSize_0<RetType, T: QPagedPaintDevice_pageSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pageSize_0(self);
    // return 1;
  }
}
pub trait QPagedPaintDevice_pageSize_0<RetType> {
  fn pageSize_0(self , rsthis: & QPagedPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPagedPaintDevice_pageSize_0<i32> for () {
  fn pageSize_0(self , rsthis: & QPagedPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QPagedPaintDevice8pageSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagedpaintdevice.h:229
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setPageSizeMM(const QSizeF &)

/*
Sets the page size to size. size is specified in millimeters.

If the size matches a standard QPagedPaintDevice::PageSize then that page size will be used, otherwise QPagedPaintDevice::Custom will be set.

See also pageSizeMM().
*/
impl /*struct*/ QPagedPaintDevice {
  pub fn setPageSizeMM_0<RetType, T: QPagedPaintDevice_setPageSizeMM_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPageSizeMM_0(self);
    // return 1;
  }
}
pub trait QPagedPaintDevice_setPageSizeMM_0<RetType> {
  fn setPageSizeMM_0(self , rsthis: & QPagedPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPagedPaintDevice_setPageSizeMM_0<(/*void*/)> for (usize) {
  fn setPageSizeMM_0(self , rsthis: & QPagedPaintDevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QPagedPaintDevice13setPageSizeMMERK6QSizeF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpagedpaintdevice.h:230
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF pageSizeMM() const

/*
Returns the page size in millimeters.

See also setPageSizeMM().
*/
impl /*struct*/ QPagedPaintDevice {
  pub fn pageSizeMM_0<RetType, T: QPagedPaintDevice_pageSizeMM_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pageSizeMM_0(self);
    // return 1;
  }
}
pub trait QPagedPaintDevice_pageSizeMM_0<RetType> {
  fn pageSizeMM_0(self , rsthis: & QPagedPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPagedPaintDevice_pageSizeMM_0<usize> for () {
  fn pageSizeMM_0(self , rsthis: & QPagedPaintDevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QPagedPaintDevice10pageSizeMMEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagedpaintdevice.h:241
// index:0
// Public Visibility=Default Availability=Available
// [32] QPagedPaintDevice::Margins margins() const

/*
Returns the current margins of the paint device. The default is 0.

Margins are specified in millimeters.

See also setMargins().
*/
impl /*struct*/ QPagedPaintDevice {
  pub fn margins_0<RetType, T: QPagedPaintDevice_margins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.margins_0(self);
    // return 1;
  }
}
pub trait QPagedPaintDevice_margins_0<RetType> {
  fn margins_0(self , rsthis: & QPagedPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPagedPaintDevice_margins_0<usize> for () {
  fn margins_0(self , rsthis: & QPagedPaintDevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QPagedPaintDevice7marginsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagedpaintdevice.h:246
// index:0
// Protected Visibility=Default Availability=Available
// [8] QPageLayout devicePageLayout() const

/*

*/
impl /*struct*/ QPagedPaintDevice {
  pub fn devicePageLayout_0<RetType, T: QPagedPaintDevice_devicePageLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.devicePageLayout_0(self);
    // return 1;
  }
}
pub trait QPagedPaintDevice_devicePageLayout_0<RetType> {
  fn devicePageLayout_0(self , rsthis: & QPagedPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPagedPaintDevice_devicePageLayout_0<usize> for () {
  fn devicePageLayout_0(self , rsthis: & QPagedPaintDevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QPagedPaintDevice16devicePageLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpagedpaintdevice.h:247
// index:1
// Protected Visibility=Default Availability=Available
// [8] QPageLayout & devicePageLayout()

/*

*/
impl /*struct*/ QPagedPaintDevice {
  pub fn devicePageLayout_1<RetType, T: QPagedPaintDevice_devicePageLayout_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.devicePageLayout_1(self);
    // return 1;
  }
}
pub trait QPagedPaintDevice_devicePageLayout_1<RetType> {
  fn devicePageLayout_1(self , rsthis: & QPagedPaintDevice) -> RetType;
}
impl<'a> /*trait*/ QPagedPaintDevice_devicePageLayout_1<usize> for () {
  fn devicePageLayout_1(self , rsthis: & QPagedPaintDevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QPagedPaintDevice16devicePageLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum type lists the available page sizes as defined in the Postscript PPD standard. These values are duplicated in QPageSize and QPrinter.

The defined sizes are:

QPagedPaintDevice::AnsiALetter= Letter
QPagedPaintDevice::AnsiBLedger= Ledger
QPagedPaintDevice::EnvelopeDLDLE= DLE


Due to historic reasons QPageSize::Executive is not the same as the standard Postscript and Windows Executive size, use QPageSize::ExecutiveStandard instead.

The Postscript standard size QPageSize::Folio is different to the Windows DMPAPER_FOLIO size, use the Postscript standard size QPageSize::FanFoldGermanLegal if needed.

*/
pub type QPagedPaintDevice__PageSize = i32;
// 
pub const QPagedPaintDevice__A4 :QPagedPaintDevice__PageSize = 0;
// 
pub const QPagedPaintDevice__B5 :QPagedPaintDevice__PageSize = 1;
// 
pub const QPagedPaintDevice__Letter :QPagedPaintDevice__PageSize = 2;
// 
pub const QPagedPaintDevice__Legal :QPagedPaintDevice__PageSize = 3;
// 
pub const QPagedPaintDevice__Executive :QPagedPaintDevice__PageSize = 4;
// 
pub const QPagedPaintDevice__A0 :QPagedPaintDevice__PageSize = 5;
// 
pub const QPagedPaintDevice__A1 :QPagedPaintDevice__PageSize = 6;
// 
pub const QPagedPaintDevice__A2 :QPagedPaintDevice__PageSize = 7;
// 
pub const QPagedPaintDevice__A3 :QPagedPaintDevice__PageSize = 8;
// 
pub const QPagedPaintDevice__A5 :QPagedPaintDevice__PageSize = 9;
// 
pub const QPagedPaintDevice__A6 :QPagedPaintDevice__PageSize = 10;
// 
pub const QPagedPaintDevice__A7 :QPagedPaintDevice__PageSize = 11;
// 
pub const QPagedPaintDevice__A8 :QPagedPaintDevice__PageSize = 12;
// 
pub const QPagedPaintDevice__A9 :QPagedPaintDevice__PageSize = 13;
// 
pub const QPagedPaintDevice__B0 :QPagedPaintDevice__PageSize = 14;
// 
pub const QPagedPaintDevice__B1 :QPagedPaintDevice__PageSize = 15;
// 
pub const QPagedPaintDevice__B10 :QPagedPaintDevice__PageSize = 16;
// 
pub const QPagedPaintDevice__B2 :QPagedPaintDevice__PageSize = 17;
// 
pub const QPagedPaintDevice__B3 :QPagedPaintDevice__PageSize = 18;
// 
pub const QPagedPaintDevice__B4 :QPagedPaintDevice__PageSize = 19;
// 
pub const QPagedPaintDevice__B6 :QPagedPaintDevice__PageSize = 20;
// 
pub const QPagedPaintDevice__B7 :QPagedPaintDevice__PageSize = 21;
// 
pub const QPagedPaintDevice__B8 :QPagedPaintDevice__PageSize = 22;
// 
pub const QPagedPaintDevice__B9 :QPagedPaintDevice__PageSize = 23;
// 
pub const QPagedPaintDevice__C5E :QPagedPaintDevice__PageSize = 24;
// 
pub const QPagedPaintDevice__Comm10E :QPagedPaintDevice__PageSize = 25;
// 
pub const QPagedPaintDevice__DLE :QPagedPaintDevice__PageSize = 26;
// 
pub const QPagedPaintDevice__Folio :QPagedPaintDevice__PageSize = 27;
// 
pub const QPagedPaintDevice__Ledger :QPagedPaintDevice__PageSize = 28;
// 
pub const QPagedPaintDevice__Tabloid :QPagedPaintDevice__PageSize = 29;
// 
pub const QPagedPaintDevice__Custom :QPagedPaintDevice__PageSize = 30;
// 
pub const QPagedPaintDevice__A10 :QPagedPaintDevice__PageSize = 31;
// 
pub const QPagedPaintDevice__A3Extra :QPagedPaintDevice__PageSize = 32;
// 
pub const QPagedPaintDevice__A4Extra :QPagedPaintDevice__PageSize = 33;
// 
pub const QPagedPaintDevice__A4Plus :QPagedPaintDevice__PageSize = 34;
// 
pub const QPagedPaintDevice__A4Small :QPagedPaintDevice__PageSize = 35;
// 
pub const QPagedPaintDevice__A5Extra :QPagedPaintDevice__PageSize = 36;
// 
pub const QPagedPaintDevice__B5Extra :QPagedPaintDevice__PageSize = 37;
// 
pub const QPagedPaintDevice__JisB0 :QPagedPaintDevice__PageSize = 38;
// 
pub const QPagedPaintDevice__JisB1 :QPagedPaintDevice__PageSize = 39;
// 
pub const QPagedPaintDevice__JisB2 :QPagedPaintDevice__PageSize = 40;
// 
pub const QPagedPaintDevice__JisB3 :QPagedPaintDevice__PageSize = 41;
// 
pub const QPagedPaintDevice__JisB4 :QPagedPaintDevice__PageSize = 42;
// 
pub const QPagedPaintDevice__JisB5 :QPagedPaintDevice__PageSize = 43;
// 
pub const QPagedPaintDevice__JisB6 :QPagedPaintDevice__PageSize = 44;
// 
pub const QPagedPaintDevice__JisB7 :QPagedPaintDevice__PageSize = 45;
// 
pub const QPagedPaintDevice__JisB8 :QPagedPaintDevice__PageSize = 46;
// 
pub const QPagedPaintDevice__JisB9 :QPagedPaintDevice__PageSize = 47;
// 
pub const QPagedPaintDevice__JisB10 :QPagedPaintDevice__PageSize = 48;
// 
pub const QPagedPaintDevice__AnsiC :QPagedPaintDevice__PageSize = 49;
// 
pub const QPagedPaintDevice__AnsiD :QPagedPaintDevice__PageSize = 50;
// 
pub const QPagedPaintDevice__AnsiE :QPagedPaintDevice__PageSize = 51;
// 
pub const QPagedPaintDevice__LegalExtra :QPagedPaintDevice__PageSize = 52;
// 
pub const QPagedPaintDevice__LetterExtra :QPagedPaintDevice__PageSize = 53;
// 
pub const QPagedPaintDevice__LetterPlus :QPagedPaintDevice__PageSize = 54;
// 
pub const QPagedPaintDevice__LetterSmall :QPagedPaintDevice__PageSize = 55;
// 
pub const QPagedPaintDevice__TabloidExtra :QPagedPaintDevice__PageSize = 56;
// 
pub const QPagedPaintDevice__ArchA :QPagedPaintDevice__PageSize = 57;
// 
pub const QPagedPaintDevice__ArchB :QPagedPaintDevice__PageSize = 58;
// 
pub const QPagedPaintDevice__ArchC :QPagedPaintDevice__PageSize = 59;
// 
pub const QPagedPaintDevice__ArchD :QPagedPaintDevice__PageSize = 60;
// 
pub const QPagedPaintDevice__ArchE :QPagedPaintDevice__PageSize = 61;
// 
pub const QPagedPaintDevice__Imperial7x9 :QPagedPaintDevice__PageSize = 62;
// 
pub const QPagedPaintDevice__Imperial8x10 :QPagedPaintDevice__PageSize = 63;
// 
pub const QPagedPaintDevice__Imperial9x11 :QPagedPaintDevice__PageSize = 64;
// 
pub const QPagedPaintDevice__Imperial9x12 :QPagedPaintDevice__PageSize = 65;
// 
pub const QPagedPaintDevice__Imperial10x11 :QPagedPaintDevice__PageSize = 66;
// 
pub const QPagedPaintDevice__Imperial10x13 :QPagedPaintDevice__PageSize = 67;
// 
pub const QPagedPaintDevice__Imperial10x14 :QPagedPaintDevice__PageSize = 68;
// 
pub const QPagedPaintDevice__Imperial12x11 :QPagedPaintDevice__PageSize = 69;
// 
pub const QPagedPaintDevice__Imperial15x11 :QPagedPaintDevice__PageSize = 70;
// 
pub const QPagedPaintDevice__ExecutiveStandard :QPagedPaintDevice__PageSize = 71;
// 
pub const QPagedPaintDevice__Note :QPagedPaintDevice__PageSize = 72;
// 
pub const QPagedPaintDevice__Quarto :QPagedPaintDevice__PageSize = 73;
// 
pub const QPagedPaintDevice__Statement :QPagedPaintDevice__PageSize = 74;
// 
pub const QPagedPaintDevice__SuperA :QPagedPaintDevice__PageSize = 75;
// 
pub const QPagedPaintDevice__SuperB :QPagedPaintDevice__PageSize = 76;
// 
pub const QPagedPaintDevice__Postcard :QPagedPaintDevice__PageSize = 77;
// 
pub const QPagedPaintDevice__DoublePostcard :QPagedPaintDevice__PageSize = 78;
// 
pub const QPagedPaintDevice__Prc16K :QPagedPaintDevice__PageSize = 79;
// 
pub const QPagedPaintDevice__Prc32K :QPagedPaintDevice__PageSize = 80;
// 
pub const QPagedPaintDevice__Prc32KBig :QPagedPaintDevice__PageSize = 81;
// 
pub const QPagedPaintDevice__FanFoldUS :QPagedPaintDevice__PageSize = 82;
// 
pub const QPagedPaintDevice__FanFoldGerman :QPagedPaintDevice__PageSize = 83;
// 
pub const QPagedPaintDevice__FanFoldGermanLegal :QPagedPaintDevice__PageSize = 84;
// 
pub const QPagedPaintDevice__EnvelopeB4 :QPagedPaintDevice__PageSize = 85;
// 
pub const QPagedPaintDevice__EnvelopeB5 :QPagedPaintDevice__PageSize = 86;
// 
pub const QPagedPaintDevice__EnvelopeB6 :QPagedPaintDevice__PageSize = 87;
// 
pub const QPagedPaintDevice__EnvelopeC0 :QPagedPaintDevice__PageSize = 88;
// 
pub const QPagedPaintDevice__EnvelopeC1 :QPagedPaintDevice__PageSize = 89;
// 
pub const QPagedPaintDevice__EnvelopeC2 :QPagedPaintDevice__PageSize = 90;
// 
pub const QPagedPaintDevice__EnvelopeC3 :QPagedPaintDevice__PageSize = 91;
// 
pub const QPagedPaintDevice__EnvelopeC4 :QPagedPaintDevice__PageSize = 92;
// 
pub const QPagedPaintDevice__EnvelopeC6 :QPagedPaintDevice__PageSize = 93;
// 
pub const QPagedPaintDevice__EnvelopeC65 :QPagedPaintDevice__PageSize = 94;
// 
pub const QPagedPaintDevice__EnvelopeC7 :QPagedPaintDevice__PageSize = 95;
// 
pub const QPagedPaintDevice__Envelope9 :QPagedPaintDevice__PageSize = 96;
// 
pub const QPagedPaintDevice__Envelope11 :QPagedPaintDevice__PageSize = 97;
// 
pub const QPagedPaintDevice__Envelope12 :QPagedPaintDevice__PageSize = 98;
// 
pub const QPagedPaintDevice__Envelope14 :QPagedPaintDevice__PageSize = 99;
// 
pub const QPagedPaintDevice__EnvelopeMonarch :QPagedPaintDevice__PageSize = 100;
// 
pub const QPagedPaintDevice__EnvelopePersonal :QPagedPaintDevice__PageSize = 101;
// 
pub const QPagedPaintDevice__EnvelopeChou3 :QPagedPaintDevice__PageSize = 102;
// 
pub const QPagedPaintDevice__EnvelopeChou4 :QPagedPaintDevice__PageSize = 103;
// 
pub const QPagedPaintDevice__EnvelopeInvite :QPagedPaintDevice__PageSize = 104;
// 
pub const QPagedPaintDevice__EnvelopeItalian :QPagedPaintDevice__PageSize = 105;
// 
pub const QPagedPaintDevice__EnvelopeKaku2 :QPagedPaintDevice__PageSize = 106;
// 
pub const QPagedPaintDevice__EnvelopeKaku3 :QPagedPaintDevice__PageSize = 107;
// 
pub const QPagedPaintDevice__EnvelopePrc1 :QPagedPaintDevice__PageSize = 108;
// 
pub const QPagedPaintDevice__EnvelopePrc2 :QPagedPaintDevice__PageSize = 109;
// 
pub const QPagedPaintDevice__EnvelopePrc3 :QPagedPaintDevice__PageSize = 110;
// 
pub const QPagedPaintDevice__EnvelopePrc4 :QPagedPaintDevice__PageSize = 111;
// 
pub const QPagedPaintDevice__EnvelopePrc5 :QPagedPaintDevice__PageSize = 112;
// 
pub const QPagedPaintDevice__EnvelopePrc6 :QPagedPaintDevice__PageSize = 113;
// 
pub const QPagedPaintDevice__EnvelopePrc7 :QPagedPaintDevice__PageSize = 114;
// 
pub const QPagedPaintDevice__EnvelopePrc8 :QPagedPaintDevice__PageSize = 115;
// 
pub const QPagedPaintDevice__EnvelopePrc9 :QPagedPaintDevice__PageSize = 116;
// 
pub const QPagedPaintDevice__EnvelopePrc10 :QPagedPaintDevice__PageSize = 117;
// 
pub const QPagedPaintDevice__EnvelopeYou4 :QPagedPaintDevice__PageSize = 118;
// 
pub const QPagedPaintDevice__LastPageSize :QPagedPaintDevice__PageSize = 118;
// 
pub const QPagedPaintDevice__NPageSize :QPagedPaintDevice__PageSize = 118;
// 
pub const QPagedPaintDevice__NPaperSize :QPagedPaintDevice__PageSize = 118;
// 
pub const QPagedPaintDevice__AnsiA :QPagedPaintDevice__PageSize = 2;
// 
pub const QPagedPaintDevice__AnsiB :QPagedPaintDevice__PageSize = 28;
// 
pub const QPagedPaintDevice__EnvelopeC5 :QPagedPaintDevice__PageSize = 24;
// 
pub const QPagedPaintDevice__EnvelopeDL :QPagedPaintDevice__PageSize = 26;
// 
pub const QPagedPaintDevice__Envelope10 :QPagedPaintDevice__PageSize = 25;
pub fn QPagedPaintDevice_PageSizeItemName(val: i32) ->String {
  match val {
     QPagedPaintDevice__A4 => // 0
     {return String::from("A4");}
     QPagedPaintDevice__B5 => // 1
     {return String::from("B5");}
     QPagedPaintDevice__Letter => // 2
     {return String::from("Letter,AnsiA");}
     QPagedPaintDevice__Legal => // 3
     {return String::from("Legal");}
     QPagedPaintDevice__Executive => // 4
     {return String::from("Executive");}
     QPagedPaintDevice__A0 => // 5
     {return String::from("A0");}
     QPagedPaintDevice__A1 => // 6
     {return String::from("A1");}
     QPagedPaintDevice__A2 => // 7
     {return String::from("A2");}
     QPagedPaintDevice__A3 => // 8
     {return String::from("A3");}
     QPagedPaintDevice__A5 => // 9
     {return String::from("A5");}
     QPagedPaintDevice__A6 => // 10
     {return String::from("A6");}
     QPagedPaintDevice__A7 => // 11
     {return String::from("A7");}
     QPagedPaintDevice__A8 => // 12
     {return String::from("A8");}
     QPagedPaintDevice__A9 => // 13
     {return String::from("A9");}
     QPagedPaintDevice__B0 => // 14
     {return String::from("B0");}
     QPagedPaintDevice__B1 => // 15
     {return String::from("B1");}
     QPagedPaintDevice__B10 => // 16
     {return String::from("B10");}
     QPagedPaintDevice__B2 => // 17
     {return String::from("B2");}
     QPagedPaintDevice__B3 => // 18
     {return String::from("B3");}
     QPagedPaintDevice__B4 => // 19
     {return String::from("B4");}
     QPagedPaintDevice__B6 => // 20
     {return String::from("B6");}
     QPagedPaintDevice__B7 => // 21
     {return String::from("B7");}
     QPagedPaintDevice__B8 => // 22
     {return String::from("B8");}
     QPagedPaintDevice__B9 => // 23
     {return String::from("B9");}
     QPagedPaintDevice__C5E => // 24
     {return String::from("C5E,EnvelopeC5");}
     QPagedPaintDevice__Comm10E => // 25
     {return String::from("Comm10E,Envelope10");}
     QPagedPaintDevice__DLE => // 26
     {return String::from("DLE,EnvelopeDL");}
     QPagedPaintDevice__Folio => // 27
     {return String::from("Folio");}
     QPagedPaintDevice__Ledger => // 28
     {return String::from("Ledger,AnsiB");}
     QPagedPaintDevice__Tabloid => // 29
     {return String::from("Tabloid");}
     QPagedPaintDevice__Custom => // 30
     {return String::from("Custom");}
     QPagedPaintDevice__A10 => // 31
     {return String::from("A10");}
     QPagedPaintDevice__A3Extra => // 32
     {return String::from("A3Extra");}
     QPagedPaintDevice__A4Extra => // 33
     {return String::from("A4Extra");}
     QPagedPaintDevice__A4Plus => // 34
     {return String::from("A4Plus");}
     QPagedPaintDevice__A4Small => // 35
     {return String::from("A4Small");}
     QPagedPaintDevice__A5Extra => // 36
     {return String::from("A5Extra");}
     QPagedPaintDevice__B5Extra => // 37
     {return String::from("B5Extra");}
     QPagedPaintDevice__JisB0 => // 38
     {return String::from("JisB0");}
     QPagedPaintDevice__JisB1 => // 39
     {return String::from("JisB1");}
     QPagedPaintDevice__JisB2 => // 40
     {return String::from("JisB2");}
     QPagedPaintDevice__JisB3 => // 41
     {return String::from("JisB3");}
     QPagedPaintDevice__JisB4 => // 42
     {return String::from("JisB4");}
     QPagedPaintDevice__JisB5 => // 43
     {return String::from("JisB5");}
     QPagedPaintDevice__JisB6 => // 44
     {return String::from("JisB6");}
     QPagedPaintDevice__JisB7 => // 45
     {return String::from("JisB7");}
     QPagedPaintDevice__JisB8 => // 46
     {return String::from("JisB8");}
     QPagedPaintDevice__JisB9 => // 47
     {return String::from("JisB9");}
     QPagedPaintDevice__JisB10 => // 48
     {return String::from("JisB10");}
     QPagedPaintDevice__AnsiC => // 49
     {return String::from("AnsiC");}
     QPagedPaintDevice__AnsiD => // 50
     {return String::from("AnsiD");}
     QPagedPaintDevice__AnsiE => // 51
     {return String::from("AnsiE");}
     QPagedPaintDevice__LegalExtra => // 52
     {return String::from("LegalExtra");}
     QPagedPaintDevice__LetterExtra => // 53
     {return String::from("LetterExtra");}
     QPagedPaintDevice__LetterPlus => // 54
     {return String::from("LetterPlus");}
     QPagedPaintDevice__LetterSmall => // 55
     {return String::from("LetterSmall");}
     QPagedPaintDevice__TabloidExtra => // 56
     {return String::from("TabloidExtra");}
     QPagedPaintDevice__ArchA => // 57
     {return String::from("ArchA");}
     QPagedPaintDevice__ArchB => // 58
     {return String::from("ArchB");}
     QPagedPaintDevice__ArchC => // 59
     {return String::from("ArchC");}
     QPagedPaintDevice__ArchD => // 60
     {return String::from("ArchD");}
     QPagedPaintDevice__ArchE => // 61
     {return String::from("ArchE");}
     QPagedPaintDevice__Imperial7x9 => // 62
     {return String::from("Imperial7x9");}
     QPagedPaintDevice__Imperial8x10 => // 63
     {return String::from("Imperial8x10");}
     QPagedPaintDevice__Imperial9x11 => // 64
     {return String::from("Imperial9x11");}
     QPagedPaintDevice__Imperial9x12 => // 65
     {return String::from("Imperial9x12");}
     QPagedPaintDevice__Imperial10x11 => // 66
     {return String::from("Imperial10x11");}
     QPagedPaintDevice__Imperial10x13 => // 67
     {return String::from("Imperial10x13");}
     QPagedPaintDevice__Imperial10x14 => // 68
     {return String::from("Imperial10x14");}
     QPagedPaintDevice__Imperial12x11 => // 69
     {return String::from("Imperial12x11");}
     QPagedPaintDevice__Imperial15x11 => // 70
     {return String::from("Imperial15x11");}
     QPagedPaintDevice__ExecutiveStandard => // 71
     {return String::from("ExecutiveStandard");}
     QPagedPaintDevice__Note => // 72
     {return String::from("Note");}
     QPagedPaintDevice__Quarto => // 73
     {return String::from("Quarto");}
     QPagedPaintDevice__Statement => // 74
     {return String::from("Statement");}
     QPagedPaintDevice__SuperA => // 75
     {return String::from("SuperA");}
     QPagedPaintDevice__SuperB => // 76
     {return String::from("SuperB");}
     QPagedPaintDevice__Postcard => // 77
     {return String::from("Postcard");}
     QPagedPaintDevice__DoublePostcard => // 78
     {return String::from("DoublePostcard");}
     QPagedPaintDevice__Prc16K => // 79
     {return String::from("Prc16K");}
     QPagedPaintDevice__Prc32K => // 80
     {return String::from("Prc32K");}
     QPagedPaintDevice__Prc32KBig => // 81
     {return String::from("Prc32KBig");}
     QPagedPaintDevice__FanFoldUS => // 82
     {return String::from("FanFoldUS");}
     QPagedPaintDevice__FanFoldGerman => // 83
     {return String::from("FanFoldGerman");}
     QPagedPaintDevice__FanFoldGermanLegal => // 84
     {return String::from("FanFoldGermanLegal");}
     QPagedPaintDevice__EnvelopeB4 => // 85
     {return String::from("EnvelopeB4");}
     QPagedPaintDevice__EnvelopeB5 => // 86
     {return String::from("EnvelopeB5");}
     QPagedPaintDevice__EnvelopeB6 => // 87
     {return String::from("EnvelopeB6");}
     QPagedPaintDevice__EnvelopeC0 => // 88
     {return String::from("EnvelopeC0");}
     QPagedPaintDevice__EnvelopeC1 => // 89
     {return String::from("EnvelopeC1");}
     QPagedPaintDevice__EnvelopeC2 => // 90
     {return String::from("EnvelopeC2");}
     QPagedPaintDevice__EnvelopeC3 => // 91
     {return String::from("EnvelopeC3");}
     QPagedPaintDevice__EnvelopeC4 => // 92
     {return String::from("EnvelopeC4");}
     QPagedPaintDevice__EnvelopeC6 => // 93
     {return String::from("EnvelopeC6");}
     QPagedPaintDevice__EnvelopeC65 => // 94
     {return String::from("EnvelopeC65");}
     QPagedPaintDevice__EnvelopeC7 => // 95
     {return String::from("EnvelopeC7");}
     QPagedPaintDevice__Envelope9 => // 96
     {return String::from("Envelope9");}
     QPagedPaintDevice__Envelope11 => // 97
     {return String::from("Envelope11");}
     QPagedPaintDevice__Envelope12 => // 98
     {return String::from("Envelope12");}
     QPagedPaintDevice__Envelope14 => // 99
     {return String::from("Envelope14");}
     QPagedPaintDevice__EnvelopeMonarch => // 100
     {return String::from("EnvelopeMonarch");}
     QPagedPaintDevice__EnvelopePersonal => // 101
     {return String::from("EnvelopePersonal");}
     QPagedPaintDevice__EnvelopeChou3 => // 102
     {return String::from("EnvelopeChou3");}
     QPagedPaintDevice__EnvelopeChou4 => // 103
     {return String::from("EnvelopeChou4");}
     QPagedPaintDevice__EnvelopeInvite => // 104
     {return String::from("EnvelopeInvite");}
     QPagedPaintDevice__EnvelopeItalian => // 105
     {return String::from("EnvelopeItalian");}
     QPagedPaintDevice__EnvelopeKaku2 => // 106
     {return String::from("EnvelopeKaku2");}
     QPagedPaintDevice__EnvelopeKaku3 => // 107
     {return String::from("EnvelopeKaku3");}
     QPagedPaintDevice__EnvelopePrc1 => // 108
     {return String::from("EnvelopePrc1");}
     QPagedPaintDevice__EnvelopePrc2 => // 109
     {return String::from("EnvelopePrc2");}
     QPagedPaintDevice__EnvelopePrc3 => // 110
     {return String::from("EnvelopePrc3");}
     QPagedPaintDevice__EnvelopePrc4 => // 111
     {return String::from("EnvelopePrc4");}
     QPagedPaintDevice__EnvelopePrc5 => // 112
     {return String::from("EnvelopePrc5");}
     QPagedPaintDevice__EnvelopePrc6 => // 113
     {return String::from("EnvelopePrc6");}
     QPagedPaintDevice__EnvelopePrc7 => // 114
     {return String::from("EnvelopePrc7");}
     QPagedPaintDevice__EnvelopePrc8 => // 115
     {return String::from("EnvelopePrc8");}
     QPagedPaintDevice__EnvelopePrc9 => // 116
     {return String::from("EnvelopePrc9");}
     QPagedPaintDevice__EnvelopePrc10 => // 117
     {return String::from("EnvelopePrc10");}
     QPagedPaintDevice__EnvelopeYou4 => // 118
     {return String::from("EnvelopeYou4,LastPageSize,NPageSize,NPaperSize");}
    // QPagedPaintDevice__LastPageSize => // 118
    // {return String::from("");}
    // QPagedPaintDevice__NPageSize => // 118
    // {return String::from("");}
    // QPagedPaintDevice__NPaperSize => // 118
    // {return String::from("");}
    // QPagedPaintDevice__AnsiA => // 2
    // {return String::from("");}
    // QPagedPaintDevice__AnsiB => // 28
    // {return String::from("");}
    // QPagedPaintDevice__EnvelopeC5 => // 24
    // {return String::from("");}
    // QPagedPaintDevice__EnvelopeDL => // 26
    // {return String::from("");}
    // QPagedPaintDevice__Envelope10 => // 25
    // {return String::from("");}
  _ => {return format!("{}", val);}
}
}
pub fn QPagedPaintDevice_PageSizeItemName_s(val: i32) ->String {
  //var nilthis *QPagedPaintDevice
  //return nilthis.PageSizeItemName(val);
  return QPagedPaintDevice_PageSizeItemName(val);
}


/*
The PdfVersion enum describes the version of the PDF file that is produced by QPrinter or QPdfWriter.



This enum was introduced or modified in  Qt 5.10.

*/
pub type QPagedPaintDevice__PdfVersion = i32;
// 
pub const QPagedPaintDevice__PdfVersion_1_4 :QPagedPaintDevice__PdfVersion = 0;
// 
pub const QPagedPaintDevice__PdfVersion_A1b :QPagedPaintDevice__PdfVersion = 1;
pub fn QPagedPaintDevice_PdfVersionItemName(val: i32) ->String {
  match val {
     QPagedPaintDevice__PdfVersion_1_4 => // 0
     {return String::from("PdfVersion_1_4");}
     QPagedPaintDevice__PdfVersion_A1b => // 1
     {return String::from("PdfVersion_A1b");}
  _ => {return format!("{}", val);}
}
}
pub fn QPagedPaintDevice_PdfVersionItemName_s(val: i32) ->String {
  //var nilthis *QPagedPaintDevice
  //return nilthis.PdfVersionItemName(val);
  return QPagedPaintDevice_PdfVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
