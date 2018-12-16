

// mod ::gui::QClipboard
// package qtgui
// /usr/include/qt/QtGui/qclipboard.h
// #include <qclipboard.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 15
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
#[derive(Default)] // class sizeof(QClipboard)=16
pub struct QClipboard {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QClipboard_ITF interface {
//    qtcore.QObject_ITF
//    QClipboard_PTR() *QClipboard
//}
//func (ptr *QClipboard) QClipboard_PTR() *QClipboard { return ptr }

impl /*struct*/ QClipboard {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QClipboard {
    return QClipboard{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QClipboard {
//  type Target = QClipboardBASE;
//
//  fn deref(&self) -> &QClipboardBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QClipboardBASE> for QClipboard {
//  fn as_ref(& self) -> & QClipboardBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qclipboard.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QClipboard {
  pub fn metaObject_0<RetType, T: QClipboard_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QClipboard_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QClipboard) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QClipboard10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear(QClipboard::Mode)

/*
Clear the clipboard contents.

The mode argument is used to control which part of the system clipboard is used. If mode is QClipboard::Clipboard, this function clears the global clipboard contents. If mode is QClipboard::Selection, this function clears the global mouse selection contents. If mode is QClipboard::FindBuffer, this function clears the search string buffer.

See also QClipboard::Mode and supportsSelection().
*/
impl /*struct*/ QClipboard {
  pub fn clear_0<RetType, T: QClipboard_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QClipboard_clear_0<RetType> {
  fn clear_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_clear_0<(/*void*/)> for (i32) {
  fn clear_0(self , rsthis: & QClipboard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QClipboard5clearENS_4ModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:67
// index:0
// Public Visibility=Default Availability=Available
// [1] bool supportsSelection() const

/*
Returns true if the clipboard supports mouse selection; otherwise returns false.
*/
impl /*struct*/ QClipboard {
  pub fn supportsSelection_0<RetType, T: QClipboard_supportsSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportsSelection_0(self);
    // return 1;
  }
}
pub trait QClipboard_supportsSelection_0<RetType> {
  fn supportsSelection_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_supportsSelection_0<bool> for () {
  fn supportsSelection_0(self , rsthis: & QClipboard) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QClipboard17supportsSelectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:68
// index:0
// Public Visibility=Default Availability=Available
// [1] bool supportsFindBuffer() const

/*
Returns true if the clipboard supports a separate search buffer; otherwise returns false.
*/
impl /*struct*/ QClipboard {
  pub fn supportsFindBuffer_0<RetType, T: QClipboard_supportsFindBuffer_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportsFindBuffer_0(self);
    // return 1;
  }
}
pub trait QClipboard_supportsFindBuffer_0<RetType> {
  fn supportsFindBuffer_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_supportsFindBuffer_0<bool> for () {
  fn supportsFindBuffer_0(self , rsthis: & QClipboard) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QClipboard18supportsFindBufferEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:70
// index:0
// Public Visibility=Default Availability=Available
// [1] bool ownsSelection() const

/*
Returns true if this clipboard object owns the mouse selection data; otherwise returns false.
*/
impl /*struct*/ QClipboard {
  pub fn ownsSelection_0<RetType, T: QClipboard_ownsSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ownsSelection_0(self);
    // return 1;
  }
}
pub trait QClipboard_ownsSelection_0<RetType> {
  fn ownsSelection_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_ownsSelection_0<bool> for () {
  fn ownsSelection_0(self , rsthis: & QClipboard) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QClipboard13ownsSelectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:71
// index:0
// Public Visibility=Default Availability=Available
// [1] bool ownsClipboard() const

/*
Returns true if this clipboard object owns the clipboard data; otherwise returns false.
*/
impl /*struct*/ QClipboard {
  pub fn ownsClipboard_0<RetType, T: QClipboard_ownsClipboard_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ownsClipboard_0(self);
    // return 1;
  }
}
pub trait QClipboard_ownsClipboard_0<RetType> {
  fn ownsClipboard_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_ownsClipboard_0<bool> for () {
  fn ownsClipboard_0(self , rsthis: & QClipboard) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QClipboard13ownsClipboardEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:72
// index:0
// Public Visibility=Default Availability=Available
// [1] bool ownsFindBuffer() const

/*
Returns true if this clipboard object owns the find buffer data; otherwise returns false.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QClipboard {
  pub fn ownsFindBuffer_0<RetType, T: QClipboard_ownsFindBuffer_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ownsFindBuffer_0(self);
    // return 1;
  }
}
pub trait QClipboard_ownsFindBuffer_0<RetType> {
  fn ownsFindBuffer_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_ownsFindBuffer_0<bool> for () {
  fn ownsFindBuffer_0(self , rsthis: & QClipboard) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QClipboard14ownsFindBufferEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:74
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text(QClipboard::Mode) const

/*
Returns the clipboard text as plain text, or an empty string if the clipboard does not contain any text.

The mode argument is used to control which part of the system clipboard is used. If mode is QClipboard::Clipboard, the text is retrieved from the global clipboard. If mode is QClipboard::Selection, the text is retrieved from the global mouse selection. If mode is QClipboard::FindBuffer, the text is retrieved from the search string buffer.

See also setText() and mimeData().
*/
impl /*struct*/ QClipboard {
  pub fn text_0<RetType, T: QClipboard_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QClipboard_text_0<RetType> {
  fn text_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_text_0<usize> for (i32) {
  fn text_0(self , rsthis: & QClipboard) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QClipboard4textENS_4ModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:75
// index:1
// Public Visibility=Default Availability=Available
// [8] QString text(QString &, QClipboard::Mode) const

/*
Returns the clipboard text as plain text, or an empty string if the clipboard does not contain any text.

The mode argument is used to control which part of the system clipboard is used. If mode is QClipboard::Clipboard, the text is retrieved from the global clipboard. If mode is QClipboard::Selection, the text is retrieved from the global mouse selection. If mode is QClipboard::FindBuffer, the text is retrieved from the search string buffer.

See also setText() and mimeData().
*/
impl /*struct*/ QClipboard {
  pub fn text_1<RetType, T: QClipboard_text_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_1(self);
    // return 1;
  }
}
pub trait QClipboard_text_1<RetType> {
  fn text_1(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_text_1<usize> for (usize,i32) {
  fn text_1(self , rsthis: & QClipboard) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QClipboard4textER7QStringNS_4ModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:76
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setText(const QString &, QClipboard::Mode)

/*
Copies text into the clipboard as plain text.

The mode argument is used to control which part of the system clipboard is used. If mode is QClipboard::Clipboard, the text is stored in the global clipboard. If mode is QClipboard::Selection, the text is stored in the global mouse selection. If mode is QClipboard::FindBuffer, the text is stored in the search string buffer.

See also text() and setMimeData().
*/
impl /*struct*/ QClipboard {
  pub fn setText_0<RetType, T: QClipboard_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QClipboard_setText_0<RetType> {
  fn setText_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_setText_0<(/*void*/)> for (usize,i32) {
  fn setText_0(self , rsthis: & QClipboard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QClipboard7setTextERK7QStringNS_4ModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] const QMimeData * mimeData(QClipboard::Mode) const

/*
Returns a pointer to a QMimeData representation of the current clipboard data (can be NULL if the given mode is not supported by the platform).

The mode argument is used to control which part of the system clipboard is used. If mode is QClipboard::Clipboard, the data is retrieved from the global clipboard. If mode is QClipboard::Selection, the data is retrieved from the global mouse selection. If mode is QClipboard::FindBuffer, the data is retrieved from the search string buffer.

The text(), image(), and pixmap() functions are simpler wrappers for retrieving text, image, and pixmap data.

Note: The pointer returned might become invalidated when the contents of the clipboard changes; either by calling one of the setter functions or externally by the system clipboard changing.

See also setMimeData().
*/
impl /*struct*/ QClipboard {
  pub fn mimeData_0<RetType, T: QClipboard_mimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeData_0(self);
    // return 1;
  }
}
pub trait QClipboard_mimeData_0<RetType> {
  fn mimeData_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_mimeData_0<usize> for (i32) {
  fn mimeData_0(self , rsthis: & QClipboard) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QClipboard8mimeDataENS_4ModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMimeData(QMimeData *, QClipboard::Mode)

/*
Sets the clipboard data to src. Ownership of the data is transferred to the clipboard. If you want to remove the data either call clear() or call setMimeData() again with new data.

The mode argument is used to control which part of the system clipboard is used. If mode is QClipboard::Clipboard, the data is stored in the global clipboard. If mode is QClipboard::Selection, the data is stored in the global mouse selection. If mode is QClipboard::FindBuffer, the data is stored in the search string buffer.

The setText(), setImage() and setPixmap() functions are simpler wrappers for setting text, image and pixmap data respectively.

See also mimeData().
*/
impl /*struct*/ QClipboard {
  pub fn setMimeData_0<RetType, T: QClipboard_setMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMimeData_0(self);
    // return 1;
  }
}
pub trait QClipboard_setMimeData_0<RetType> {
  fn setMimeData_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_setMimeData_0<(/*void*/)> for (usize,i32) {
  fn setMimeData_0(self , rsthis: & QClipboard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QClipboard11setMimeDataEP9QMimeDataNS_4ModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:81
// index:0
// Public Visibility=Default Availability=Available
// [32] QImage image(QClipboard::Mode) const

/*
Returns the clipboard image, or returns a null image if the clipboard does not contain an image or if it contains an image in an unsupported image format.

The mode argument is used to control which part of the system clipboard is used. If mode is QClipboard::Clipboard, the image is retrieved from the global clipboard. If mode is QClipboard::Selection, the image is retrieved from the global mouse selection.

See also setImage(), pixmap(), mimeData(), and QImage::isNull().
*/
impl /*struct*/ QClipboard {
  pub fn image_0<RetType, T: QClipboard_image_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.image_0(self);
    // return 1;
  }
}
pub trait QClipboard_image_0<RetType> {
  fn image_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_image_0<usize> for (i32) {
  fn image_0(self , rsthis: & QClipboard) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QClipboard5imageENS_4ModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:82
// index:0
// Public Visibility=Default Availability=Available
// [32] QPixmap pixmap(QClipboard::Mode) const

/*
Returns the clipboard pixmap, or null if the clipboard does not contain a pixmap. Note that this can lose information. For example, if the image is 24-bit and the display is 8-bit, the result is converted to 8 bits, and if the image has an alpha channel, the result just has a mask.

The mode argument is used to control which part of the system clipboard is used. If mode is QClipboard::Clipboard, the pixmap is retrieved from the global clipboard. If mode is QClipboard::Selection, the pixmap is retrieved from the global mouse selection.

See also setPixmap(), image(), mimeData(), and QPixmap::convertFromImage().
*/
impl /*struct*/ QClipboard {
  pub fn pixmap_0<RetType, T: QClipboard_pixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixmap_0(self);
    // return 1;
  }
}
pub trait QClipboard_pixmap_0<RetType> {
  fn pixmap_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_pixmap_0<usize> for (i32) {
  fn pixmap_0(self , rsthis: & QClipboard) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QClipboard6pixmapENS_4ModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setImage(const QImage &, QClipboard::Mode)

/*
Copies the image into the clipboard.

The mode argument is used to control which part of the system clipboard is used. If mode is QClipboard::Clipboard, the image is stored in the global clipboard. If mode is QClipboard::Selection, the data is stored in the global mouse selection.

This is shorthand for:


  QMimeData *data = new QMimeData;
  data->setImageData(image);
  clipboard->setMimeData(data, mode);



See also image(), setPixmap(), and setMimeData().
*/
impl /*struct*/ QClipboard {
  pub fn setImage_0<RetType, T: QClipboard_setImage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setImage_0(self);
    // return 1;
  }
}
pub trait QClipboard_setImage_0<RetType> {
  fn setImage_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_setImage_0<(/*void*/)> for (usize,i32) {
  fn setImage_0(self , rsthis: & QClipboard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QClipboard8setImageERK6QImageNS_4ModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPixmap(const QPixmap &, QClipboard::Mode)

/*
Copies pixmap into the clipboard. Note that this is slower than setImage() because it needs to convert the QPixmap to a QImage first.

The mode argument is used to control which part of the system clipboard is used. If mode is QClipboard::Clipboard, the pixmap is stored in the global clipboard. If mode is QClipboard::Selection, the pixmap is stored in the global mouse selection.

See also pixmap(), setImage(), and setMimeData().
*/
impl /*struct*/ QClipboard {
  pub fn setPixmap_0<RetType, T: QClipboard_setPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPixmap_0(self);
    // return 1;
  }
}
pub trait QClipboard_setPixmap_0<RetType> {
  fn setPixmap_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_setPixmap_0<(/*void*/)> for (usize,i32) {
  fn setPixmap_0(self , rsthis: & QClipboard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QClipboard9setPixmapERK7QPixmapNS_4ModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void changed(QClipboard::Mode)

/*
This signal is emitted when the data for the given clipboard mode is changed.

This function was introduced in  Qt 4.2.

See also dataChanged(), selectionChanged(), and findBufferChanged().
*/
impl /*struct*/ QClipboard {
  pub fn changed_0<RetType, T: QClipboard_changed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changed_0(self);
    // return 1;
  }
}
pub trait QClipboard_changed_0<RetType> {
  fn changed_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_changed_0<(/*void*/)> for (i32) {
  fn changed_0(self , rsthis: & QClipboard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QClipboard7changedENS_4ModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectionChanged()

/*
This signal is emitted when the selection is changed. This only applies to windowing systems that support selections, e.g. X11. Windows and macOS don't support selections.

See also dataChanged(), findBufferChanged(), and changed().
*/
impl /*struct*/ QClipboard {
  pub fn selectionChanged_0<RetType, T: QClipboard_selectionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged_0(self);
    // return 1;
  }
}
pub trait QClipboard_selectionChanged_0<RetType> {
  fn selectionChanged_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_selectionChanged_0<(/*void*/)> for () {
  fn selectionChanged_0(self , rsthis: & QClipboard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QClipboard16selectionChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void findBufferChanged()

/*
This signal is emitted when the find buffer is changed. This only applies to macOS.

With Qt version 4.3 or higher, clipboard changes made by other applications will only be detected when the application is activated.

This function was introduced in  Qt 4.2.

See also dataChanged(), selectionChanged(), and changed().
*/
impl /*struct*/ QClipboard {
  pub fn findBufferChanged_0<RetType, T: QClipboard_findBufferChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.findBufferChanged_0(self);
    // return 1;
  }
}
pub trait QClipboard_findBufferChanged_0<RetType> {
  fn findBufferChanged_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_findBufferChanged_0<(/*void*/)> for () {
  fn findBufferChanged_0(self , rsthis: & QClipboard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QClipboard17findBufferChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qclipboard.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void dataChanged()

/*
This signal is emitted when the clipboard data is changed.

On macOS and with Qt version 4.3 or higher, clipboard changes made by other applications will only be detected when the application is activated.

See also findBufferChanged(), selectionChanged(), and changed().
*/
impl /*struct*/ QClipboard {
  pub fn dataChanged_0<RetType, T: QClipboard_dataChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dataChanged_0(self);
    // return 1;
  }
}
pub trait QClipboard_dataChanged_0<RetType> {
  fn dataChanged_0(self , rsthis: & QClipboard) -> RetType;
}
impl<'a> /*trait*/ QClipboard_dataChanged_0<(/*void*/)> for () {
  fn dataChanged_0(self , rsthis: & QClipboard) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QClipboard11dataChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQClipboard(this :*mut QClipboard) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN10QClipboardD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum type is used to control which part of the system clipboard is used by QClipboard::mimeData(), QClipboard::setMimeData() and related functions.



See also QClipboard::supportsSelection().

*/
pub type QClipboard__Mode = i32;
// indicates that data should be stored and retrieved from the global clipboard.
pub const QClipboard__Clipboard :QClipboard__Mode = 0;
// 
pub const QClipboard__Selection :QClipboard__Mode = 1;
// indicates that data should be stored and retrieved from the Find buffer. This mode is used for holding search strings on macOS.
pub const QClipboard__FindBuffer :QClipboard__Mode = 2;
// 
pub const QClipboard__LastMode :QClipboard__Mode = 2;
pub fn QClipboard_ModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QClipboard", val);
}
pub fn QClipboard_ModeItemName_s(val: i32) ->String {
  //var nilthis *QClipboard
  //return nilthis.ModeItemName(val);
  return QClipboard_ModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
