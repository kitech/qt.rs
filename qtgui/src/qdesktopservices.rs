

// mod ::gui::QDesktopServices
// package qtgui
// /usr/include/qt/QtGui/qdesktopservices.h
// #include <qdesktopservices.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 20
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
#[derive(Default)] // class sizeof(QDesktopServices)=1
pub struct QDesktopServices {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDesktopServices_ITF interface {
//    QDesktopServices_PTR() *QDesktopServices
//}
//func (ptr *QDesktopServices) QDesktopServices_PTR() *QDesktopServices { return ptr }

impl /*struct*/ QDesktopServices {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDesktopServices {
    return QDesktopServices{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDesktopServices {
//  type Target = QDesktopServicesBASE;
//
//  fn deref(&self) -> &QDesktopServicesBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDesktopServicesBASE> for QDesktopServices {
//  fn as_ref(& self) -> & QDesktopServicesBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qdesktopservices.h:59
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool openUrl(const QUrl &)

/*
Opens the given url in the appropriate Web browser for the user's desktop environment, and returns true if successful; otherwise returns false.

If the URL is a reference to a local file (i.e., the URL scheme is "file") then it will be opened with a suitable application instead of a Web browser.

The following example opens a file on the Windows file system residing on a path that contains spaces:


  QDesktopServices::openUrl(QUrl("file:///C:/Documents and Settings/All Users/Desktop", QUrl::TolerantMode));



If a mailto URL is specified, the user's e-mail client will be used to open a composer window containing the options specified in the URL, similar to the way mailto links are handled by a Web browser.

For example, the following URL contains a recipient (user@foo.com), a subject (Test), and a message body (Just a test):


  mailto:user@foo.com?subject=Test&body=Just a test



Warning: Although many e-mail clients can send attachments and are Unicode-aware, the user may have configured their client without these features. Also, certain e-mail clients (e.g., Lotus Notes) have problems with long URLs.

Warning: A return value of true indicates that the application has successfully requested the operating system to open the URL in an external application. The external application may still fail to launch or fail to open the requested URL. This result will not be reported back to the application.

Warning: URLs passed to this function on iOS will not load unless their schemes are listed in the LSApplicationQueriesSchemes key of the application's Info.plist file. For more information, see the Apple Developer Documentation for canOpenURL(_:). For example, the following lines enable URLs with the HTTPS scheme:


  <key>LSApplicationQueriesSchemes</key>
  <array>
      <string>https</string>
  </array>



See also setUrlHandler().
*/
impl /*struct*/ QDesktopServices {
  pub fn openUrl_0<RetType, T: QDesktopServices_openUrl_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.openUrl_0();
    // return 1;
  }
}
pub trait QDesktopServices_openUrl_0<RetType> {
  fn openUrl_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDesktopServices_openUrl_0<bool> for (usize) {
  fn openUrl_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QDesktopServices7openUrlERK4QUrl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qdesktopservices.h:60
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setUrlHandler(const QString &, QObject *, const char *)

/*
Sets the handler for the given scheme to be the handler method provided by the receiver object.

This function provides a way to customize the behavior of openUrl(). If openUrl() is called with a URL with the specified scheme then the given method on the receiver object is called instead of QDesktopServices launching an external application.

The provided method must be implemented as a slot that only accepts a single QUrl argument.

If setUrlHandler() is used to set a new handler for a scheme which already has a handler, the existing handler is simply replaced with the new one. Since QDesktopServices does not take ownership of handlers, no objects are deleted when a handler is replaced.

Note that the handler will always be called from within the same thread that calls QDesktopServices::openUrl().

See also openUrl() and unsetUrlHandler().
*/
impl /*struct*/ QDesktopServices {
  pub fn setUrlHandler_0<RetType, T: QDesktopServices_setUrlHandler_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setUrlHandler_0();
    // return 1;
  }
}
pub trait QDesktopServices_setUrlHandler_0<RetType> {
  fn setUrlHandler_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDesktopServices_setUrlHandler_0<(/*void*/)> for (usize,usize,usize) {
  fn setUrlHandler_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (self.2) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QDesktopServices13setUrlHandlerERK7QStringP7QObjectPKc", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qdesktopservices.h:61
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void unsetUrlHandler(const QString &)

/*
Removes a previously set URL handler for the specified scheme.

See also setUrlHandler().
*/
impl /*struct*/ QDesktopServices {
  pub fn unsetUrlHandler_0<RetType, T: QDesktopServices_unsetUrlHandler_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.unsetUrlHandler_0();
    // return 1;
  }
}
pub trait QDesktopServices_unsetUrlHandler_0<RetType> {
  fn unsetUrlHandler_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDesktopServices_unsetUrlHandler_0<(/*void*/)> for (usize) {
  fn unsetUrlHandler_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QDesktopServices15unsetUrlHandlerERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQDesktopServices(this :*mut QDesktopServices) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN16QDesktopServicesD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
