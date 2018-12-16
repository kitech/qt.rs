

// mod ::gui::QPdfWriter
// package qtgui
// /usr/include/qt/QtGui/qpdfwriter.h
// #include <qpdfwriter.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 19
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

// QPaintEngine * paintEngine()
// func (this *QPdfWriter) InheritPaintEngine(f func() unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "paintEngine", f)
// }

// int metric(QPaintDevice::PaintDeviceMetric)
// func (this *QPdfWriter) InheritMetric(f func(id int) int) {
//  qtrt.SetAllInheritCallback(this, "metric", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QPdfWriter)=48
pub struct QPdfWriter {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPdfWriter_ITF interface {
//    qtcore.QObject_ITF
//    QPagedPaintDevice_ITF
//    QPdfWriter_PTR() *QPdfWriter
//}
//func (ptr *QPdfWriter) QPdfWriter_PTR() *QPdfWriter { return ptr }

impl /*struct*/ QPdfWriter {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPdfWriter {
    return QPdfWriter{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPdfWriter {
//  type Target = QPdfWriterBASE;
//
//  fn deref(&self) -> &QPdfWriterBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPdfWriterBASE> for QPdfWriter {
//  fn as_ref(& self) -> & QPdfWriterBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpdfwriter.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QPdfWriter {
  pub fn metaObject_0<RetType, T: QPdfWriter_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QPdfWriter_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QPdfWriter) -> RetType;
}
impl<'a> /*trait*/ QPdfWriter_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QPdfWriter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QPdfWriter10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpdfwriter.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPdfWriter(const QString &)

/*
Constructs a PDF writer that will write the pdf to filename.
*/
// QPdfWriter(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QPdfWriter {
  pub fn QPdfWriter_0<T: QPdfWriter_QPdfWriter_0>(value: T) -> QPdfWriter {
    let rsthis = value.QPdfWriter_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPdfWriter_QPdfWriter_0 {
  fn QPdfWriter_0(self) -> QPdfWriter;
}
// QPdfWriter(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPdfWriter_QPdfWriter_0 for (usize) {
  fn QPdfWriter_0(self) -> QPdfWriter {
    // unsafe{_ZN10QPdfWriterC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QPdfWriterC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPdfWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpdfwriter.h:61
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QPdfWriter(QIODevice *)

/*
Constructs a PDF writer that will write the pdf to filename.
*/
// QPdfWriter(QIODevice *) ctx.fn_proto_cpp
impl /*struct*/ QPdfWriter {
  pub fn QPdfWriter_1<T: QPdfWriter_QPdfWriter_1>(value: T) -> QPdfWriter {
    let rsthis = value.QPdfWriter_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPdfWriter_QPdfWriter_1 {
  fn QPdfWriter_1(self) -> QPdfWriter;
}
// QPdfWriter(QIODevice *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPdfWriter_QPdfWriter_1 for (usize) {
  fn QPdfWriter_1(self) -> QPdfWriter {
    // unsafe{_ZN10QPdfWriterC2EP9QIODevice()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QPdfWriterC2EP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPdfWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpdfwriter.h:62
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QPdfWriter()

/*

*/
pub fn DeleteQPdfWriter(this :*mut QPdfWriter) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QPdfWriterD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qpdfwriter.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPdfVersion(QPagedPaintDevice::PdfVersion)

/*
Sets the PDF version for this writer to version.

If version is the same value as currently set then no change will be made.

This function was introduced in  Qt 5.10.

See also pdfVersion().
*/
impl /*struct*/ QPdfWriter {
  pub fn setPdfVersion_0<RetType, T: QPdfWriter_setPdfVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPdfVersion_0(self);
    // return 1;
  }
}
pub trait QPdfWriter_setPdfVersion_0<RetType> {
  fn setPdfVersion_0(self , rsthis: & QPdfWriter) -> RetType;
}
impl<'a> /*trait*/ QPdfWriter_setPdfVersion_0<(/*void*/)> for (i32) {
  fn setPdfVersion_0(self , rsthis: & QPdfWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QPdfWriter13setPdfVersionEN17QPagedPaintDevice10PdfVersionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpdfwriter.h:65
// index:0
// Public Visibility=Default Availability=Available
// [4] QPagedPaintDevice::PdfVersion pdfVersion() const

/*
Returns the PDF version for this writer. The default is PdfVersion_1_4.

This function was introduced in  Qt 5.10.

See also setPdfVersion().
*/
impl /*struct*/ QPdfWriter {
  pub fn pdfVersion_0<RetType, T: QPdfWriter_pdfVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pdfVersion_0(self);
    // return 1;
  }
}
pub trait QPdfWriter_pdfVersion_0<RetType> {
  fn pdfVersion_0(self , rsthis: & QPdfWriter) -> RetType;
}
impl<'a> /*trait*/ QPdfWriter_pdfVersion_0<i32> for () {
  fn pdfVersion_0(self , rsthis: & QPdfWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QPdfWriter10pdfVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpdfwriter.h:67
// index:0
// Public Visibility=Default Availability=Available
// [8] QString title() const

/*
Returns the title of the document.

See also setTitle().
*/
impl /*struct*/ QPdfWriter {
  pub fn title_0<RetType, T: QPdfWriter_title_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.title_0(self);
    // return 1;
  }
}
pub trait QPdfWriter_title_0<RetType> {
  fn title_0(self , rsthis: & QPdfWriter) -> RetType;
}
impl<'a> /*trait*/ QPdfWriter_title_0<usize> for () {
  fn title_0(self , rsthis: & QPdfWriter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QPdfWriter5titleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpdfwriter.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTitle(const QString &)

/*
Sets the title of the document being created to title.

See also title().
*/
impl /*struct*/ QPdfWriter {
  pub fn setTitle_0<RetType, T: QPdfWriter_setTitle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTitle_0(self);
    // return 1;
  }
}
pub trait QPdfWriter_setTitle_0<RetType> {
  fn setTitle_0(self , rsthis: & QPdfWriter) -> RetType;
}
impl<'a> /*trait*/ QPdfWriter_setTitle_0<(/*void*/)> for (usize) {
  fn setTitle_0(self , rsthis: & QPdfWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QPdfWriter8setTitleERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpdfwriter.h:70
// index:0
// Public Visibility=Default Availability=Available
// [8] QString creator() const

/*
Returns the creator of the document.

See also setCreator().
*/
impl /*struct*/ QPdfWriter {
  pub fn creator_0<RetType, T: QPdfWriter_creator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.creator_0(self);
    // return 1;
  }
}
pub trait QPdfWriter_creator_0<RetType> {
  fn creator_0(self , rsthis: & QPdfWriter) -> RetType;
}
impl<'a> /*trait*/ QPdfWriter_creator_0<usize> for () {
  fn creator_0(self , rsthis: & QPdfWriter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QPdfWriter7creatorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpdfwriter.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCreator(const QString &)

/*
Sets the creator of the document to creator.

See also creator().
*/
impl /*struct*/ QPdfWriter {
  pub fn setCreator_0<RetType, T: QPdfWriter_setCreator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCreator_0(self);
    // return 1;
  }
}
pub trait QPdfWriter_setCreator_0<RetType> {
  fn setCreator_0(self , rsthis: & QPdfWriter) -> RetType;
}
impl<'a> /*trait*/ QPdfWriter_setCreator_0<(/*void*/)> for (usize) {
  fn setCreator_0(self , rsthis: & QPdfWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QPdfWriter10setCreatorERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpdfwriter.h:73
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool newPage()

/*
Reimplemented from QPagedPaintDevice::newPage().
*/
impl /*struct*/ QPdfWriter {
  pub fn newPage_0<RetType, T: QPdfWriter_newPage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.newPage_0(self);
    // return 1;
  }
}
pub trait QPdfWriter_newPage_0<RetType> {
  fn newPage_0(self , rsthis: & QPdfWriter) -> RetType;
}
impl<'a> /*trait*/ QPdfWriter_newPage_0<bool> for () {
  fn newPage_0(self , rsthis: & QPdfWriter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QPdfWriter7newPageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpdfwriter.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setResolution(int)

/*
Sets the PDF resolution in DPI.

This setting affects the coordinate system as returned by, for example QPainter::viewport().

This function was introduced in  Qt 5.3.

See also resolution().
*/
impl /*struct*/ QPdfWriter {
  pub fn setResolution_0<RetType, T: QPdfWriter_setResolution_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setResolution_0(self);
    // return 1;
  }
}
pub trait QPdfWriter_setResolution_0<RetType> {
  fn setResolution_0(self , rsthis: & QPdfWriter) -> RetType;
}
impl<'a> /*trait*/ QPdfWriter_setResolution_0<(/*void*/)> for (i32) {
  fn setResolution_0(self , rsthis: & QPdfWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QPdfWriter13setResolutionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpdfwriter.h:76
// index:0
// Public Visibility=Default Availability=Available
// [4] int resolution() const

/*
Returns the resolution of the PDF in DPI.

This function was introduced in  Qt 5.3.

See also setResolution().
*/
impl /*struct*/ QPdfWriter {
  pub fn resolution_0<RetType, T: QPdfWriter_resolution_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resolution_0(self);
    // return 1;
  }
}
pub trait QPdfWriter_resolution_0<RetType> {
  fn resolution_0(self , rsthis: & QPdfWriter) -> RetType;
}
impl<'a> /*trait*/ QPdfWriter_resolution_0<i32> for () {
  fn resolution_0(self , rsthis: & QPdfWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QPdfWriter10resolutionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpdfwriter.h:89
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setPageSize(QPagedPaintDevice::PageSize)

/*
Sets the PDF page size to pageSize.

To get the current QPageSize use pageLayout().pageSize().

You should call this before calling QPainter::begin(), or immediately before calling newPage() to apply the new page size to a new page. You should not call any painting methods between a call to setPageSize() and newPage() as the wrong paint metrics may be used.

Returns true if the page size was successfully set to pageSize.

This function was introduced in  Qt 5.3.

See also pageLayout().
*/
impl /*struct*/ QPdfWriter {
  pub fn setPageSize_0<RetType, T: QPdfWriter_setPageSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPageSize_0(self);
    // return 1;
  }
}
pub trait QPdfWriter_setPageSize_0<RetType> {
  fn setPageSize_0(self , rsthis: & QPdfWriter) -> RetType;
}
impl<'a> /*trait*/ QPdfWriter_setPageSize_0<(/*void*/)> for (i32) {
  fn setPageSize_0(self , rsthis: & QPdfWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QPdfWriter11setPageSizeEN17QPagedPaintDevice8PageSizeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpdfwriter.h:90
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setPageSizeMM(const QSizeF &)

/*

*/
impl /*struct*/ QPdfWriter {
  pub fn setPageSizeMM_0<RetType, T: QPdfWriter_setPageSizeMM_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPageSizeMM_0(self);
    // return 1;
  }
}
pub trait QPdfWriter_setPageSizeMM_0<RetType> {
  fn setPageSizeMM_0(self , rsthis: & QPdfWriter) -> RetType;
}
impl<'a> /*trait*/ QPdfWriter_setPageSizeMM_0<(/*void*/)> for (usize) {
  fn setPageSizeMM_0(self , rsthis: & QPdfWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QPdfWriter13setPageSizeMMERK6QSizeF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpdfwriter.h:95
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QPaintEngine * paintEngine() const

/*
Reimplemented from QPaintDevice::paintEngine().
*/
impl /*struct*/ QPdfWriter {
  pub fn paintEngine_0<RetType, T: QPdfWriter_paintEngine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEngine_0(self);
    // return 1;
  }
}
pub trait QPdfWriter_paintEngine_0<RetType> {
  fn paintEngine_0(self , rsthis: & QPdfWriter) -> RetType;
}
impl<'a> /*trait*/ QPdfWriter_paintEngine_0<usize> for () {
  fn paintEngine_0(self , rsthis: & QPdfWriter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QPdfWriter11paintEngineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpdfwriter.h:96
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int metric(QPaintDevice::PaintDeviceMetric) const

/*

*/
impl /*struct*/ QPdfWriter {
  pub fn metric_0<RetType, T: QPdfWriter_metric_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metric_0(self);
    // return 1;
  }
}
pub trait QPdfWriter_metric_0<RetType> {
  fn metric_0(self , rsthis: & QPdfWriter) -> RetType;
}
impl<'a> /*trait*/ QPdfWriter_metric_0<i32> for (i32) {
  fn metric_0(self , rsthis: & QPdfWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QPdfWriter6metricEN12QPaintDevice17PaintDeviceMetricE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
