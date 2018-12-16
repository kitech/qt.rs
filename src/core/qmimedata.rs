

// mod ::core::QMimeData
// package qtcore
// /usr/include/qt/QtCore/qmimedata.h
// #include <qmimedata.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 4
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// QVariant retrieveData(const QString &, QVariant::Type)
// func (this *QMimeData) InheritRetrieveData(f func(mimetype string, preferredType int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "retrieveData", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QMimeData)=16
pub struct QMimeData {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMimeData_ITF interface {
//    QObject_ITF
//    QMimeData_PTR() *QMimeData
//}
//func (ptr *QMimeData) QMimeData_PTR() *QMimeData { return ptr }

impl /*struct*/ QMimeData {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMimeData {
    return QMimeData{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMimeData {
//  type Target = QMimeDataBASE;
//
//  fn deref(&self) -> &QMimeDataBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMimeDataBASE> for QMimeData {
//  fn as_ref(& self) -> & QMimeDataBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qmimedata.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QMimeData {
  pub fn metaObject_0<RetType, T: QMimeData_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QMimeData_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QMimeData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeData10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:56
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QMimeData()

/*
Constructs a new MIME data object with no data in it.
*/
// QMimeData() ctx.fn_proto_cpp
impl /*struct*/ QMimeData {
  pub fn QMimeData_0<T: QMimeData_QMimeData_0>(value: T) -> QMimeData {
    let rsthis = value.QMimeData_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMimeData_QMimeData_0 {
  fn QMimeData_0(self) -> QMimeData;
}
// QMimeData() ctx.fn_proto_cpp
impl<'a> /*trait*/ QMimeData_QMimeData_0 for () {
  fn QMimeData_0(self) -> QMimeData {
    // unsafe{_ZN9QMimeDataC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QMimeDataC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMimeData{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QMimeData()

/*

*/
pub fn DeleteQMimeData(this :*mut QMimeData) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QMimeDataD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qmimedata.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] QList<QUrl> urls() const

/*
Returns a list of URLs contained within the MIME data object.

URLs correspond to the MIME type text/uri-list.

See also setUrls(), hasUrls(), and data().
*/
impl /*struct*/ QMimeData {
  pub fn urls_0<RetType, T: QMimeData_urls_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.urls_0(self);
    // return 1;
  }
}
pub trait QMimeData_urls_0<RetType> {
  fn urls_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_urls_0<usize> for () {
  fn urls_0(self , rsthis: & QMimeData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeData4urlsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:61
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasUrls() const

/*
Returns true if the object can return a list of urls; otherwise returns false.

URLs correspond to the MIME type text/uri-list.

See also setUrls(), urls(), and hasFormat().
*/
impl /*struct*/ QMimeData {
  pub fn hasUrls_0<RetType, T: QMimeData_hasUrls_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasUrls_0(self);
    // return 1;
  }
}
pub trait QMimeData_hasUrls_0<RetType> {
  fn hasUrls_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_hasUrls_0<bool> for () {
  fn hasUrls_0(self , rsthis: & QMimeData) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeData7hasUrlsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:63
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text() const

/*
Returns a plain text (MIME type text/plain) representation of the data.

See also setText(), hasText(), html(), and data().
*/
impl /*struct*/ QMimeData {
  pub fn text_0<RetType, T: QMimeData_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QMimeData_text_0<RetType> {
  fn text_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_text_0<usize> for () {
  fn text_0(self , rsthis: & QMimeData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeData4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setText(const QString &)

/*
Sets text as the plain text (MIME type text/plain) used to represent the data.

See also text(), hasText(), setHtml(), and setData().
*/
impl /*struct*/ QMimeData {
  pub fn setText_0<RetType, T: QMimeData_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QMimeData_setText_0<RetType> {
  fn setText_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_setText_0<(/*void*/)> for (usize) {
  fn setText_0(self , rsthis: & QMimeData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QMimeData7setTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:65
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasText() const

/*
Returns true if the object can return plain text (MIME type text/plain); otherwise returns false.

See also setText(), text(), hasHtml(), and hasFormat().
*/
impl /*struct*/ QMimeData {
  pub fn hasText_0<RetType, T: QMimeData_hasText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasText_0(self);
    // return 1;
  }
}
pub trait QMimeData_hasText_0<RetType> {
  fn hasText_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_hasText_0<bool> for () {
  fn hasText_0(self , rsthis: & QMimeData) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeData7hasTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:67
// index:0
// Public Visibility=Default Availability=Available
// [8] QString html() const

/*
Returns a string if the data stored in the object is HTML (MIME type text/html); otherwise returns an empty string.

See also setHtml(), hasHtml(), and setData().
*/
impl /*struct*/ QMimeData {
  pub fn html_0<RetType, T: QMimeData_html_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.html_0(self);
    // return 1;
  }
}
pub trait QMimeData_html_0<RetType> {
  fn html_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_html_0<usize> for () {
  fn html_0(self , rsthis: & QMimeData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeData4htmlEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHtml(const QString &)

/*
Sets html as the HTML (MIME type text/html) used to represent the data.

See also html(), hasHtml(), setText(), and setData().
*/
impl /*struct*/ QMimeData {
  pub fn setHtml_0<RetType, T: QMimeData_setHtml_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHtml_0(self);
    // return 1;
  }
}
pub trait QMimeData_setHtml_0<RetType> {
  fn setHtml_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_setHtml_0<(/*void*/)> for (usize) {
  fn setHtml_0(self , rsthis: & QMimeData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QMimeData7setHtmlERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:69
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasHtml() const

/*
Returns true if the object can return HTML (MIME type text/html); otherwise returns false.

See also setHtml(), html(), and hasFormat().
*/
impl /*struct*/ QMimeData {
  pub fn hasHtml_0<RetType, T: QMimeData_hasHtml_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasHtml_0(self);
    // return 1;
  }
}
pub trait QMimeData_hasHtml_0<RetType> {
  fn hasHtml_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_hasHtml_0<bool> for () {
  fn hasHtml_0(self , rsthis: & QMimeData) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeData7hasHtmlEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:71
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant imageData() const

/*
Returns a QVariant storing a QImage if the object can return an image; otherwise returns a null variant.

A QVariant is used because QMimeData belongs to the Qt Core module, whereas QImage belongs to Qt GUI. To convert the QVariant to a QImage, simply use qvariant_cast(). For example:


  if (event->mimeData()->hasImage()) {
      QImage image = qvariant_cast<QImage>(event->mimeData()->imageData());
      ...
  }



See also setImageData() and hasImage().
*/
impl /*struct*/ QMimeData {
  pub fn imageData_0<RetType, T: QMimeData_imageData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.imageData_0(self);
    // return 1;
  }
}
pub trait QMimeData_imageData_0<RetType> {
  fn imageData_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_imageData_0<usize> for () {
  fn imageData_0(self , rsthis: & QMimeData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeData9imageDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setImageData(const QVariant &)

/*
Sets the data in the object to the given image.

A QVariant is used because QMimeData belongs to the Qt Core module, whereas QImage belongs to Qt GUI. The conversion from QImage to QVariant is implicit. For example:


  mimeData->setImageData(QImage("beautifulfjord.png"));



See also imageData(), hasImage(), and setData().
*/
impl /*struct*/ QMimeData {
  pub fn setImageData_0<RetType, T: QMimeData_setImageData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setImageData_0(self);
    // return 1;
  }
}
pub trait QMimeData_setImageData_0<RetType> {
  fn setImageData_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_setImageData_0<(/*void*/)> for (usize) {
  fn setImageData_0(self , rsthis: & QMimeData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QMimeData12setImageDataERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:73
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasImage() const

/*
Returns true if the object can return an image; otherwise returns false.

See also setImageData(), imageData(), and hasFormat().
*/
impl /*struct*/ QMimeData {
  pub fn hasImage_0<RetType, T: QMimeData_hasImage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasImage_0(self);
    // return 1;
  }
}
pub trait QMimeData_hasImage_0<RetType> {
  fn hasImage_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_hasImage_0<bool> for () {
  fn hasImage_0(self , rsthis: & QMimeData) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeData8hasImageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:75
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant colorData() const

/*
Returns a color if the data stored in the object represents a color (MIME type application/x-color); otherwise returns a null variant.

A QVariant is used because QMimeData belongs to the Qt Core module, whereas QColor belongs to Qt GUI. To convert the QVariant to a QColor, simply use qvariant_cast(). For example:


  if (event->mimeData()->hasColor()) {
      QColor color = qvariant_cast<QColor>(event->mimeData()->colorData());
      ...
  }



See also hasColor(), setColorData(), and data().
*/
impl /*struct*/ QMimeData {
  pub fn colorData_0<RetType, T: QMimeData_colorData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.colorData_0(self);
    // return 1;
  }
}
pub trait QMimeData_colorData_0<RetType> {
  fn colorData_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_colorData_0<usize> for () {
  fn colorData_0(self , rsthis: & QMimeData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeData9colorDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:76
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColorData(const QVariant &)

/*
Sets the color data in the object to the given color.

Colors correspond to the MIME type application/x-color.

See also colorData(), hasColor(), and setData().
*/
impl /*struct*/ QMimeData {
  pub fn setColorData_0<RetType, T: QMimeData_setColorData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColorData_0(self);
    // return 1;
  }
}
pub trait QMimeData_setColorData_0<RetType> {
  fn setColorData_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_setColorData_0<(/*void*/)> for (usize) {
  fn setColorData_0(self , rsthis: & QMimeData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QMimeData12setColorDataERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:77
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasColor() const

/*
Returns true if the object can return a color (MIME type application/x-color); otherwise returns false.

See also setColorData(), colorData(), and hasFormat().
*/
impl /*struct*/ QMimeData {
  pub fn hasColor_0<RetType, T: QMimeData_hasColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasColor_0(self);
    // return 1;
  }
}
pub trait QMimeData_hasColor_0<RetType> {
  fn hasColor_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_hasColor_0<bool> for () {
  fn hasColor_0(self , rsthis: & QMimeData) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeData8hasColorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:79
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray data(const QString &) const

/*
Returns the data stored in the object in the format described by the MIME type specified by mimeType.

See also setData().
*/
impl /*struct*/ QMimeData {
  pub fn data_0<RetType, T: QMimeData_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QMimeData_data_0<RetType> {
  fn data_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_data_0<usize> for (usize) {
  fn data_0(self , rsthis: & QMimeData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeData4dataERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setData(const QString &, const QByteArray &)

/*
Sets the data associated with the MIME type given by mimeType to the specified data.

For the most common types of data, you can call the higher-level functions setText(), setHtml(), setUrls(), setImageData(), and setColorData() instead.

Note that if you want to use a custom data type in an item view drag and drop operation, you must register it as a Qt meta type, using the Q_DECLARE_METATYPE() macro, and implement stream operators for it. The stream operators must then be registered with the qRegisterMetaTypeStreamOperators() function.

See also data(), hasFormat(), QMetaType, and qRegisterMetaTypeStreamOperators().
*/
impl /*struct*/ QMimeData {
  pub fn setData_0<RetType, T: QMimeData_setData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setData_0(self);
    // return 1;
  }
}
pub trait QMimeData_setData_0<RetType> {
  fn setData_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_setData_0<(/*void*/)> for (usize,usize) {
  fn setData_0(self , rsthis: & QMimeData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QMimeData7setDataERK7QStringRK10QByteArray", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeFormat(const QString &)

/*
Removes the data entry for mimeType in the object.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QMimeData {
  pub fn removeFormat_0<RetType, T: QMimeData_removeFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeFormat_0(self);
    // return 1;
  }
}
pub trait QMimeData_removeFormat_0<RetType> {
  fn removeFormat_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_removeFormat_0<(/*void*/)> for (usize) {
  fn removeFormat_0(self , rsthis: & QMimeData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QMimeData12removeFormatERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:83
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool hasFormat(const QString &) const

/*
Returns true if the object can return data for the MIME type specified by mimeType; otherwise returns false.

For the most common types of data, you can call the higher-level functions hasText(), hasHtml(), hasUrls(), hasImage(), and hasColor() instead.

See also formats(), setData(), and data().
*/
impl /*struct*/ QMimeData {
  pub fn hasFormat_0<RetType, T: QMimeData_hasFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasFormat_0(self);
    // return 1;
  }
}
pub trait QMimeData_hasFormat_0<RetType> {
  fn hasFormat_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_hasFormat_0<bool> for (usize) {
  fn hasFormat_0(self , rsthis: & QMimeData) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeData9hasFormatERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:84
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QStringList formats() const

/*
Returns a list of formats supported by the object. This is a list of MIME types for which the object can return suitable data. The formats in the list are in a priority order.

For the most common types of data, you can call the higher-level functions hasText(), hasHtml(), hasUrls(), hasImage(), and hasColor() instead.

See also hasFormat(), setData(), and data().
*/
impl /*struct*/ QMimeData {
  pub fn formats_0<RetType, T: QMimeData_formats_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.formats_0(self);
    // return 1;
  }
}
pub trait QMimeData_formats_0<RetType> {
  fn formats_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_formats_0<usize> for () {
  fn formats_0(self , rsthis: & QMimeData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeData7formatsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Removes all the MIME type and data entries in the object.
*/
impl /*struct*/ QMimeData {
  pub fn clear_0<RetType, T: QMimeData_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QMimeData_clear_0<RetType> {
  fn clear_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QMimeData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QMimeData5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmimedata.h:88
// index:0
// Protected virtual Visibility=Default Availability=Available
// [16] QVariant retrieveData(const QString &, QVariant::Type) const

/*
Returns a variant with the given type containing data for the MIME type specified by mimeType. If the object does not support the MIME type or variant type given, a null variant is returned instead.

This function is called by the general data() getter and by the convenience getters (text(), html(), urls(), imageData(), and colorData()). You can reimplement it if you want to store your data using a custom data structure (instead of a QByteArray, which is what setData() provides). You would then also need to reimplement hasFormat() and formats().

See also data().
*/
impl /*struct*/ QMimeData {
  pub fn retrieveData_0<RetType, T: QMimeData_retrieveData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.retrieveData_0(self);
    // return 1;
  }
}
pub trait QMimeData_retrieveData_0<RetType> {
  fn retrieveData_0(self , rsthis: & QMimeData) -> RetType;
}
impl<'a> /*trait*/ QMimeData_retrieveData_0<usize> for (usize,i32) {
  fn retrieveData_0(self , rsthis: & QMimeData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeData12retrieveDataERK7QStringN8QVariant4TypeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
