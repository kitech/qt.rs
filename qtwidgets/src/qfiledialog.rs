

// mod ::widgets::QFileDialog
// package qtwidgets
// /usr/include/qt/QtWidgets/qfiledialog.h
// #include <qfiledialog.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 8
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

// void done(int)
// func (this *QFileDialog) InheritDone(f func(result int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "done", f)
// }

// void accept()
// func (this *QFileDialog) InheritAccept(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "accept", f)
// }

// void changeEvent(QEvent *)
// func (this *QFileDialog) InheritChangeEvent(f func(e *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QFileDialog)=48
pub struct QFileDialog {
  qbase: QDialog,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFileDialog_ITF interface {
//    QDialog_ITF
//    QFileDialog_PTR() *QFileDialog
//}
//func (ptr *QFileDialog) QFileDialog_PTR() *QFileDialog { return ptr }

impl /*struct*/ QFileDialog {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFileDialog {
    return QFileDialog{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFileDialog {
//  type Target = QFileDialogBASE;
//
//  fn deref(&self) -> &QFileDialogBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFileDialogBASE> for QFileDialog {
//  fn as_ref(& self) -> & QFileDialogBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qfiledialog.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QFileDialog {
  pub fn metaObject_0<RetType, T: QFileDialog_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QFileDialog_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFileDialog(QWidget *, Qt::WindowFlags)

/*
Constructs a file dialog with the given parent and widget flags.
*/
// QFileDialog(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QFileDialog {
  pub fn QFileDialog_0<T: QFileDialog_QFileDialog_0>(value: T) -> QFileDialog {
    let rsthis = value.QFileDialog_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFileDialog_QFileDialog_0 {
  fn QFileDialog_0(self) -> QFileDialog;
}
// QFileDialog(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFileDialog_QFileDialog_0 for (usize,i32) {
  fn QFileDialog_0(self) -> QFileDialog {
    // unsafe{_ZN11QFileDialogC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QFileDialogC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFileDialog{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:101
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QFileDialog(QWidget *, const QString &, const QString &, const QString &)

/*
Constructs a file dialog with the given parent and widget flags.
*/
// QFileDialog(QWidget *, const QString &, const QString &, const QString &) ctx.fn_proto_cpp
impl /*struct*/ QFileDialog {
  pub fn QFileDialog_1<T: QFileDialog_QFileDialog_1>(value: T) -> QFileDialog {
    let rsthis = value.QFileDialog_1();
    return rsthis;
    // return 1;
  }
}

pub trait QFileDialog_QFileDialog_1 {
  fn QFileDialog_1(self) -> QFileDialog;
}
// QFileDialog(QWidget *, const QString &, const QString &, const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFileDialog_QFileDialog_1 for (usize,usize,usize,usize) {
  fn QFileDialog_1(self) -> QFileDialog {
    // unsafe{_ZN11QFileDialogC2EP7QWidgetRK7QStringS4_S4_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QFileDialogC2EP7QWidgetRK7QStringS4_S4_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFileDialog{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:105
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QFileDialog()

/*

*/
pub fn DeleteQFileDialog(this :*mut QFileDialog) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QFileDialogD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qfiledialog.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDirectory(const QString &)

/*
Sets the file dialog's current directory.

Note: On iOS, if you set directory to QStandardPaths::standardLocations(QStandardPaths::PicturesLocation).last(), a native image picker dialog will be used for accessing the user's photo album. The filename returned can be loaded using QFile and related APIs. For this to be enabled, the Info.plist assigned to QMAKE_INFO_PLIST in the project file must contain the key NSPhotoLibraryUsageDescription. See Info.plist documentation from Apple for more information regarding this key. This feature was added in Qt 5.5.

See also directory().
*/
impl /*struct*/ QFileDialog {
  pub fn setDirectory_0<RetType, T: QFileDialog_setDirectory_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDirectory_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setDirectory_0<RetType> {
  fn setDirectory_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setDirectory_0<(/*void*/)> for (usize) {
  fn setDirectory_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog12setDirectoryERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:108
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setDirectory(const QDir &)

/*
Sets the file dialog's current directory.

Note: On iOS, if you set directory to QStandardPaths::standardLocations(QStandardPaths::PicturesLocation).last(), a native image picker dialog will be used for accessing the user's photo album. The filename returned can be loaded using QFile and related APIs. For this to be enabled, the Info.plist assigned to QMAKE_INFO_PLIST in the project file must contain the key NSPhotoLibraryUsageDescription. See Info.plist documentation from Apple for more information regarding this key. This feature was added in Qt 5.5.

See also directory().
*/
impl /*struct*/ QFileDialog {
  pub fn setDirectory_1<RetType, T: QFileDialog_setDirectory_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDirectory_1(self);
    // return 1;
  }
}
pub trait QFileDialog_setDirectory_1<RetType> {
  fn setDirectory_1(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setDirectory_1<(/*void*/)> for (usize) {
  fn setDirectory_1(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog12setDirectoryERK4QDir", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:109
// index:0
// Public Visibility=Default Availability=Available
// [8] QDir directory() const

/*
Returns the directory currently being displayed in the dialog.

See also setDirectory().
*/
impl /*struct*/ QFileDialog {
  pub fn directory_0<RetType, T: QFileDialog_directory_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.directory_0(self);
    // return 1;
  }
}
pub trait QFileDialog_directory_0<RetType> {
  fn directory_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_directory_0<usize> for () {
  fn directory_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog9directoryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDirectoryUrl(const QUrl &)

/*
Sets the file dialog's current directory url.

Note: The non-native QFileDialog supports only local files.

Note: On Windows, it is possible to pass URLs representing one of the virtual folders, such as "Computer" or "Network". This is done by passing a QUrl using the scheme clsid followed by the CLSID value with the curly braces removed. For example the URL clsid:374DE290-123F-4565-9164-39C4925E467B denotes the download location. For a complete list of possible values, see the MSDN documentation on KNOWNFOLDERID. This feature was added in Qt 5.5.

This function was introduced in  Qt 5.2.

See also directoryUrl() and QUuid.
*/
impl /*struct*/ QFileDialog {
  pub fn setDirectoryUrl_0<RetType, T: QFileDialog_setDirectoryUrl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDirectoryUrl_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setDirectoryUrl_0<RetType> {
  fn setDirectoryUrl_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setDirectoryUrl_0<(/*void*/)> for (usize) {
  fn setDirectoryUrl_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog15setDirectoryUrlERK4QUrl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:112
// index:0
// Public Visibility=Default Availability=Available
// [8] QUrl directoryUrl() const

/*
Returns the url of the directory currently being displayed in the dialog.

This function was introduced in  Qt 5.2.

See also setDirectoryUrl().
*/
impl /*struct*/ QFileDialog {
  pub fn directoryUrl_0<RetType, T: QFileDialog_directoryUrl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.directoryUrl_0(self);
    // return 1;
  }
}
pub trait QFileDialog_directoryUrl_0<RetType> {
  fn directoryUrl_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_directoryUrl_0<usize> for () {
  fn directoryUrl_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog12directoryUrlEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:114
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectFile(const QString &)

/*
Selects the given filename in the file dialog.

See also selectedFiles().
*/
impl /*struct*/ QFileDialog {
  pub fn selectFile_0<RetType, T: QFileDialog_selectFile_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectFile_0(self);
    // return 1;
  }
}
pub trait QFileDialog_selectFile_0<RetType> {
  fn selectFile_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_selectFile_0<(/*void*/)> for (usize) {
  fn selectFile_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog10selectFileERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:115
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList selectedFiles() const

/*
Returns a list of strings containing the absolute paths of the selected files in the dialog. If no files are selected, or the mode is not ExistingFiles or ExistingFile, selectedFiles() contains the current path in the viewport.

See also selectedNameFilter() and selectFile().
*/
impl /*struct*/ QFileDialog {
  pub fn selectedFiles_0<RetType, T: QFileDialog_selectedFiles_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedFiles_0(self);
    // return 1;
  }
}
pub trait QFileDialog_selectedFiles_0<RetType> {
  fn selectedFiles_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_selectedFiles_0<usize> for () {
  fn selectedFiles_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog13selectedFilesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectUrl(const QUrl &)

/*
Selects the given url in the file dialog.

Note: The non-native QFileDialog supports only local files.

This function was introduced in  Qt 5.2.

See also selectedUrls().
*/
impl /*struct*/ QFileDialog {
  pub fn selectUrl_0<RetType, T: QFileDialog_selectUrl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectUrl_0(self);
    // return 1;
  }
}
pub trait QFileDialog_selectUrl_0<RetType> {
  fn selectUrl_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_selectUrl_0<(/*void*/)> for (usize) {
  fn selectUrl_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog9selectUrlERK4QUrl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:118
// index:0
// Public Visibility=Default Availability=Available
// [-2] QList<QUrl> selectedUrls() const

/*
Returns a list of urls containing the selected files in the dialog. If no files are selected, or the mode is not ExistingFiles or ExistingFile, selectedUrls() contains the current path in the viewport.

This function was introduced in  Qt 5.2.

See also selectedNameFilter() and selectUrl().
*/
impl /*struct*/ QFileDialog {
  pub fn selectedUrls_0<RetType, T: QFileDialog_selectedUrls_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedUrls_0(self);
    // return 1;
  }
}
pub trait QFileDialog_selectedUrls_0<RetType> {
  fn selectedUrls_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_selectedUrls_0<usize> for () {
  fn selectedUrls_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog12selectedUrlsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:120
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNameFilterDetailsVisible(bool)

/*

*/
impl /*struct*/ QFileDialog {
  pub fn setNameFilterDetailsVisible_0<RetType, T: QFileDialog_setNameFilterDetailsVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNameFilterDetailsVisible_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setNameFilterDetailsVisible_0<RetType> {
  fn setNameFilterDetailsVisible_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setNameFilterDetailsVisible_0<(/*void*/)> for (bool) {
  fn setNameFilterDetailsVisible_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog27setNameFilterDetailsVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:121
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNameFilterDetailsVisible() const

/*

*/
impl /*struct*/ QFileDialog {
  pub fn isNameFilterDetailsVisible_0<RetType, T: QFileDialog_isNameFilterDetailsVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNameFilterDetailsVisible_0(self);
    // return 1;
  }
}
pub trait QFileDialog_isNameFilterDetailsVisible_0<RetType> {
  fn isNameFilterDetailsVisible_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_isNameFilterDetailsVisible_0<bool> for () {
  fn isNameFilterDetailsVisible_0(self , rsthis: & QFileDialog) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog26isNameFilterDetailsVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:123
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNameFilter(const QString &)

/*
Sets the filter used in the file dialog to the given filter.

If filter contains a pair of parentheses containing one or more filename-wildcard patterns, separated by spaces, then only the text contained in the parentheses is used as the filter. This means that these calls are all equivalent:


  dialog.setNameFilter("All C++ files (*.cpp *.cc *.C *.cxx *.c++)");
  dialog.setNameFilter("*.cpp *.cc *.C *.cxx *.c++");



This function was introduced in  Qt 4.4.

See also setMimeTypeFilters() and setNameFilters().
*/
impl /*struct*/ QFileDialog {
  pub fn setNameFilter_0<RetType, T: QFileDialog_setNameFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNameFilter_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setNameFilter_0<RetType> {
  fn setNameFilter_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setNameFilter_0<(/*void*/)> for (usize) {
  fn setNameFilter_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog13setNameFilterERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:124
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNameFilters(const QStringList &)

/*
Sets the filters used in the file dialog.

Note that the filter *.* is not portable, because the historical assumption that the file extension determines the file type is not consistent on every operating system. It is possible to have a file with no dot in its name (for example, Makefile). In a native Windows file dialog, *.* will match such files, while in other types of file dialogs it may not. So it is better to use * if you mean to select any file.


  QStringList filters;
  filters << "Image files (*.png *.xpm *.jpg)"
          << "Text files (*.txt)"
          << "Any files (*)";

  QFileDialog dialog(this);
  dialog.setNameFilters(filters);
  dialog.exec();



setMimeTypeFilters() has the advantage of providing all possible name filters for each file type. For example, JPEG images have three possible extensions; if your application can open such files, selecting the image/jpeg mime type as a filter will allow you to open all of them.

This function was introduced in  Qt 4.4.

See also nameFilters().
*/
impl /*struct*/ QFileDialog {
  pub fn setNameFilters_0<RetType, T: QFileDialog_setNameFilters_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNameFilters_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setNameFilters_0<RetType> {
  fn setNameFilters_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setNameFilters_0<(/*void*/)> for (usize) {
  fn setNameFilters_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog14setNameFiltersERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:125
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList nameFilters() const

/*
Returns the file type filters that are in operation on this file dialog.

This function was introduced in  Qt 4.4.

See also setNameFilters().
*/
impl /*struct*/ QFileDialog {
  pub fn nameFilters_0<RetType, T: QFileDialog_nameFilters_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nameFilters_0(self);
    // return 1;
  }
}
pub trait QFileDialog_nameFilters_0<RetType> {
  fn nameFilters_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_nameFilters_0<usize> for () {
  fn nameFilters_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog11nameFiltersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectNameFilter(const QString &)

/*
Sets the current file type filter. Multiple filters can be passed in filter by separating them with semicolons or spaces.

This function was introduced in  Qt 4.4.

See also setNameFilter(), setNameFilters(), and selectedNameFilter().
*/
impl /*struct*/ QFileDialog {
  pub fn selectNameFilter_0<RetType, T: QFileDialog_selectNameFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectNameFilter_0(self);
    // return 1;
  }
}
pub trait QFileDialog_selectNameFilter_0<RetType> {
  fn selectNameFilter_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_selectNameFilter_0<(/*void*/)> for (usize) {
  fn selectNameFilter_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog16selectNameFilterERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:127
// index:0
// Public Visibility=Default Availability=Available
// [8] QString selectedMimeTypeFilter() const

/*
Returns The mimetype of the file that the user selected in the file dialog.

This function was introduced in  Qt 5.9.
*/
impl /*struct*/ QFileDialog {
  pub fn selectedMimeTypeFilter_0<RetType, T: QFileDialog_selectedMimeTypeFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedMimeTypeFilter_0(self);
    // return 1;
  }
}
pub trait QFileDialog_selectedMimeTypeFilter_0<RetType> {
  fn selectedMimeTypeFilter_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_selectedMimeTypeFilter_0<usize> for () {
  fn selectedMimeTypeFilter_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog22selectedMimeTypeFilterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:128
// index:0
// Public Visibility=Default Availability=Available
// [8] QString selectedNameFilter() const

/*
Returns the filter that the user selected in the file dialog.

This function was introduced in  Qt 4.4.

See also selectedFiles().
*/
impl /*struct*/ QFileDialog {
  pub fn selectedNameFilter_0<RetType, T: QFileDialog_selectedNameFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedNameFilter_0(self);
    // return 1;
  }
}
pub trait QFileDialog_selectedNameFilter_0<RetType> {
  fn selectedNameFilter_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_selectedNameFilter_0<usize> for () {
  fn selectedNameFilter_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog18selectedNameFilterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:131
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMimeTypeFilters(const QStringList &)

/*
Sets the filters used in the file dialog, from a list of MIME types.

Convenience method for setNameFilters(). Uses QMimeType to create a name filter from the glob patterns and description defined in each MIME type.

Use application/octet-stream for the "All files (*)" filter, since that is the base MIME type for all files.

Calling setMimeTypeFilters overrides any previously set name filters, and changes the return value of nameFilters().


  QStringList mimeTypeFilters;
  mimeTypeFilters << "image/jpeg" // will show "JPEG image (*.jpeg *.jpg *.jpe)
              << "image/png"  // will show "PNG image (*.png)"
              << "application/octet-stream"; // will show "All files (*)"

  QFileDialog dialog(this);
  dialog.setMimeTypeFilters(mimeTypeFilters);
  dialog.exec();



This function was introduced in  Qt 5.2.

See also mimeTypeFilters().
*/
impl /*struct*/ QFileDialog {
  pub fn setMimeTypeFilters_0<RetType, T: QFileDialog_setMimeTypeFilters_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMimeTypeFilters_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setMimeTypeFilters_0<RetType> {
  fn setMimeTypeFilters_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setMimeTypeFilters_0<(/*void*/)> for (usize) {
  fn setMimeTypeFilters_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog18setMimeTypeFiltersERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:132
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList mimeTypeFilters() const

/*
Returns the MIME type filters that are in operation on this file dialog.

This function was introduced in  Qt 5.2.

See also setMimeTypeFilters().
*/
impl /*struct*/ QFileDialog {
  pub fn mimeTypeFilters_0<RetType, T: QFileDialog_mimeTypeFilters_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeTypeFilters_0(self);
    // return 1;
  }
}
pub trait QFileDialog_mimeTypeFilters_0<RetType> {
  fn mimeTypeFilters_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_mimeTypeFilters_0<usize> for () {
  fn mimeTypeFilters_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog15mimeTypeFiltersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectMimeTypeFilter(const QString &)

/*
Sets the current MIME type filter.

This function was introduced in  Qt 5.2.
*/
impl /*struct*/ QFileDialog {
  pub fn selectMimeTypeFilter_0<RetType, T: QFileDialog_selectMimeTypeFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectMimeTypeFilter_0(self);
    // return 1;
  }
}
pub trait QFileDialog_selectMimeTypeFilter_0<RetType> {
  fn selectMimeTypeFilter_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_selectMimeTypeFilter_0<(/*void*/)> for (usize) {
  fn selectMimeTypeFilter_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog20selectMimeTypeFilterERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:136
// index:0
// Public Visibility=Default Availability=Available
// [4] QDir::Filters filter() const

/*
Returns the filter that is used when displaying files.

This function was introduced in  Qt 4.4.

See also setFilter().
*/
impl /*struct*/ QFileDialog {
  pub fn filter_0<RetType, T: QFileDialog_filter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filter_0(self);
    // return 1;
  }
}
pub trait QFileDialog_filter_0<RetType> {
  fn filter_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_filter_0<i32> for () {
  fn filter_0(self , rsthis: & QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog6filterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:137
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFilter(QDir::Filters)

/*
Sets the filter used by the model to filters. The filter is used to specify the kind of files that should be shown.

This function was introduced in  Qt 4.4.

See also filter().
*/
impl /*struct*/ QFileDialog {
  pub fn setFilter_0<RetType, T: QFileDialog_setFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFilter_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setFilter_0<RetType> {
  fn setFilter_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setFilter_0<(/*void*/)> for (i32) {
  fn setFilter_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog9setFilterE6QFlagsIN4QDir6FilterEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:139
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setViewMode(QFileDialog::ViewMode)

/*

*/
impl /*struct*/ QFileDialog {
  pub fn setViewMode_0<RetType, T: QFileDialog_setViewMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setViewMode_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setViewMode_0<RetType> {
  fn setViewMode_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setViewMode_0<(/*void*/)> for (i32) {
  fn setViewMode_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog11setViewModeENS_8ViewModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:140
// index:0
// Public Visibility=Default Availability=Available
// [4] QFileDialog::ViewMode viewMode() const

/*

*/
impl /*struct*/ QFileDialog {
  pub fn viewMode_0<RetType, T: QFileDialog_viewMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewMode_0(self);
    // return 1;
  }
}
pub trait QFileDialog_viewMode_0<RetType> {
  fn viewMode_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_viewMode_0<i32> for () {
  fn viewMode_0(self , rsthis: & QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog8viewModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:142
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFileMode(QFileDialog::FileMode)

/*

*/
impl /*struct*/ QFileDialog {
  pub fn setFileMode_0<RetType, T: QFileDialog_setFileMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFileMode_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setFileMode_0<RetType> {
  fn setFileMode_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setFileMode_0<(/*void*/)> for (i32) {
  fn setFileMode_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog11setFileModeENS_8FileModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:143
// index:0
// Public Visibility=Default Availability=Available
// [4] QFileDialog::FileMode fileMode() const

/*

*/
impl /*struct*/ QFileDialog {
  pub fn fileMode_0<RetType, T: QFileDialog_fileMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileMode_0(self);
    // return 1;
  }
}
pub trait QFileDialog_fileMode_0<RetType> {
  fn fileMode_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_fileMode_0<i32> for () {
  fn fileMode_0(self , rsthis: & QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog8fileModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:145
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAcceptMode(QFileDialog::AcceptMode)

/*

*/
impl /*struct*/ QFileDialog {
  pub fn setAcceptMode_0<RetType, T: QFileDialog_setAcceptMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAcceptMode_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setAcceptMode_0<RetType> {
  fn setAcceptMode_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setAcceptMode_0<(/*void*/)> for (i32) {
  fn setAcceptMode_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog13setAcceptModeENS_10AcceptModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:146
// index:0
// Public Visibility=Default Availability=Available
// [4] QFileDialog::AcceptMode acceptMode() const

/*

*/
impl /*struct*/ QFileDialog {
  pub fn acceptMode_0<RetType, T: QFileDialog_acceptMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.acceptMode_0(self);
    // return 1;
  }
}
pub trait QFileDialog_acceptMode_0<RetType> {
  fn acceptMode_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_acceptMode_0<i32> for () {
  fn acceptMode_0(self , rsthis: & QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog10acceptModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:148
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setReadOnly(bool)

/*

*/
impl /*struct*/ QFileDialog {
  pub fn setReadOnly_0<RetType, T: QFileDialog_setReadOnly_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setReadOnly_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setReadOnly_0<RetType> {
  fn setReadOnly_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setReadOnly_0<(/*void*/)> for (bool) {
  fn setReadOnly_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog11setReadOnlyEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:149
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isReadOnly() const

/*

*/
impl /*struct*/ QFileDialog {
  pub fn isReadOnly_0<RetType, T: QFileDialog_isReadOnly_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isReadOnly_0(self);
    // return 1;
  }
}
pub trait QFileDialog_isReadOnly_0<RetType> {
  fn isReadOnly_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_isReadOnly_0<bool> for () {
  fn isReadOnly_0(self , rsthis: & QFileDialog) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog10isReadOnlyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:151
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setResolveSymlinks(bool)

/*

*/
impl /*struct*/ QFileDialog {
  pub fn setResolveSymlinks_0<RetType, T: QFileDialog_setResolveSymlinks_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setResolveSymlinks_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setResolveSymlinks_0<RetType> {
  fn setResolveSymlinks_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setResolveSymlinks_0<(/*void*/)> for (bool) {
  fn setResolveSymlinks_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog18setResolveSymlinksEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:152
// index:0
// Public Visibility=Default Availability=Available
// [1] bool resolveSymlinks() const

/*

*/
impl /*struct*/ QFileDialog {
  pub fn resolveSymlinks_0<RetType, T: QFileDialog_resolveSymlinks_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resolveSymlinks_0(self);
    // return 1;
  }
}
pub trait QFileDialog_resolveSymlinks_0<RetType> {
  fn resolveSymlinks_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_resolveSymlinks_0<bool> for () {
  fn resolveSymlinks_0(self , rsthis: & QFileDialog) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog15resolveSymlinksEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:155
// index:0
// Public Visibility=Default Availability=Available
// [-2] QList<QUrl> sidebarUrls() const

/*
Returns a list of urls that are currently in the sidebar

This function was introduced in  Qt 4.3.

See also setSidebarUrls().
*/
impl /*struct*/ QFileDialog {
  pub fn sidebarUrls_0<RetType, T: QFileDialog_sidebarUrls_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sidebarUrls_0(self);
    // return 1;
  }
}
pub trait QFileDialog_sidebarUrls_0<RetType> {
  fn sidebarUrls_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_sidebarUrls_0<usize> for () {
  fn sidebarUrls_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog11sidebarUrlsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:157
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray saveState() const

/*
Saves the state of the dialog's layout, history and current directory.

Typically this is used in conjunction with QSettings to remember the size for a future session. A version number is stored as part of the data.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QFileDialog {
  pub fn saveState_0<RetType, T: QFileDialog_saveState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.saveState_0(self);
    // return 1;
  }
}
pub trait QFileDialog_saveState_0<RetType> {
  fn saveState_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_saveState_0<usize> for () {
  fn saveState_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog9saveStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:158
// index:0
// Public Visibility=Default Availability=Available
// [1] bool restoreState(const QByteArray &)

/*
Restores the dialogs's layout, history and current directory to the state specified.

Typically this is used in conjunction with QSettings to restore the size from a past session.

Returns false if there are errors

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QFileDialog {
  pub fn restoreState_0<RetType, T: QFileDialog_restoreState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.restoreState_0(self);
    // return 1;
  }
}
pub trait QFileDialog_restoreState_0<RetType> {
  fn restoreState_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_restoreState_0<bool> for (usize) {
  fn restoreState_0(self , rsthis: & QFileDialog) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDialog12restoreStateERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:160
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setConfirmOverwrite(bool)

/*

*/
impl /*struct*/ QFileDialog {
  pub fn setConfirmOverwrite_0<RetType, T: QFileDialog_setConfirmOverwrite_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setConfirmOverwrite_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setConfirmOverwrite_0<RetType> {
  fn setConfirmOverwrite_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setConfirmOverwrite_0<(/*void*/)> for (bool) {
  fn setConfirmOverwrite_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog19setConfirmOverwriteEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:161
// index:0
// Public Visibility=Default Availability=Available
// [1] bool confirmOverwrite() const

/*

*/
impl /*struct*/ QFileDialog {
  pub fn confirmOverwrite_0<RetType, T: QFileDialog_confirmOverwrite_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.confirmOverwrite_0(self);
    // return 1;
  }
}
pub trait QFileDialog_confirmOverwrite_0<RetType> {
  fn confirmOverwrite_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_confirmOverwrite_0<bool> for () {
  fn confirmOverwrite_0(self , rsthis: & QFileDialog) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog16confirmOverwriteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:163
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultSuffix(const QString &)

/*

*/
impl /*struct*/ QFileDialog {
  pub fn setDefaultSuffix_0<RetType, T: QFileDialog_setDefaultSuffix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultSuffix_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setDefaultSuffix_0<RetType> {
  fn setDefaultSuffix_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setDefaultSuffix_0<(/*void*/)> for (usize) {
  fn setDefaultSuffix_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog16setDefaultSuffixERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:164
// index:0
// Public Visibility=Default Availability=Available
// [8] QString defaultSuffix() const

/*

*/
impl /*struct*/ QFileDialog {
  pub fn defaultSuffix_0<RetType, T: QFileDialog_defaultSuffix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultSuffix_0(self);
    // return 1;
  }
}
pub trait QFileDialog_defaultSuffix_0<RetType> {
  fn defaultSuffix_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_defaultSuffix_0<usize> for () {
  fn defaultSuffix_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog13defaultSuffixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:166
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHistory(const QStringList &)

/*
Sets the browsing history of the filedialog to contain the given paths.

See also history().
*/
impl /*struct*/ QFileDialog {
  pub fn setHistory_0<RetType, T: QFileDialog_setHistory_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHistory_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setHistory_0<RetType> {
  fn setHistory_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setHistory_0<(/*void*/)> for (usize) {
  fn setHistory_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog10setHistoryERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:167
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList history() const

/*
Returns the browsing history of the filedialog as a list of paths.

See also setHistory().
*/
impl /*struct*/ QFileDialog {
  pub fn history_0<RetType, T: QFileDialog_history_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.history_0(self);
    // return 1;
  }
}
pub trait QFileDialog_history_0<RetType> {
  fn history_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_history_0<usize> for () {
  fn history_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog7historyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:169
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemDelegate(QAbstractItemDelegate *)

/*
Sets the item delegate used to render items in the views in the file dialog to the given delegate.

Warning: You should not share the same instance of a delegate between views. Doing so can cause incorrect or unintuitive editing behavior since each view connected to a given delegate may receive the closeEditor() signal, and attempt to access, modify or close an editor that has already been closed.

Note that the model used is QFileSystemModel. It has custom item data roles, which is described by the Roles enum. You can use a QFileIconProvider if you only want custom icons.

See also itemDelegate(), setIconProvider(), and QFileSystemModel.
*/
impl /*struct*/ QFileDialog {
  pub fn setItemDelegate_0<RetType, T: QFileDialog_setItemDelegate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemDelegate_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setItemDelegate_0<RetType> {
  fn setItemDelegate_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setItemDelegate_0<(/*void*/)> for (usize) {
  fn setItemDelegate_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog15setItemDelegateEP21QAbstractItemDelegate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:170
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractItemDelegate * itemDelegate() const

/*
Returns the item delegate used to render the items in the views in the filedialog.

See also setItemDelegate().
*/
impl /*struct*/ QFileDialog {
  pub fn itemDelegate_0<RetType, T: QFileDialog_itemDelegate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemDelegate_0(self);
    // return 1;
  }
}
pub trait QFileDialog_itemDelegate_0<RetType> {
  fn itemDelegate_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_itemDelegate_0<usize> for () {
  fn itemDelegate_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog12itemDelegateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:172
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIconProvider(QFileIconProvider *)

/*
Sets the icon provider used by the filedialog to the specified provider.

See also iconProvider().
*/
impl /*struct*/ QFileDialog {
  pub fn setIconProvider_0<RetType, T: QFileDialog_setIconProvider_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIconProvider_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setIconProvider_0<RetType> {
  fn setIconProvider_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setIconProvider_0<(/*void*/)> for (usize) {
  fn setIconProvider_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog15setIconProviderEP17QFileIconProvider", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:173
// index:0
// Public Visibility=Default Availability=Available
// [8] QFileIconProvider * iconProvider() const

/*
Returns the icon provider used by the filedialog.

See also setIconProvider().
*/
impl /*struct*/ QFileDialog {
  pub fn iconProvider_0<RetType, T: QFileDialog_iconProvider_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iconProvider_0(self);
    // return 1;
  }
}
pub trait QFileDialog_iconProvider_0<RetType> {
  fn iconProvider_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_iconProvider_0<usize> for () {
  fn iconProvider_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog12iconProviderEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:175
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLabelText(QFileDialog::DialogLabel, const QString &)

/*
Sets the text shown in the filedialog in the specified label.

See also labelText().
*/
impl /*struct*/ QFileDialog {
  pub fn setLabelText_0<RetType, T: QFileDialog_setLabelText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLabelText_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setLabelText_0<RetType> {
  fn setLabelText_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setLabelText_0<(/*void*/)> for (i32,usize) {
  fn setLabelText_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog12setLabelTextENS_11DialogLabelERK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:176
// index:0
// Public Visibility=Default Availability=Available
// [8] QString labelText(QFileDialog::DialogLabel) const

/*
Returns the text shown in the filedialog in the specified label.

See also setLabelText().
*/
impl /*struct*/ QFileDialog {
  pub fn labelText_0<RetType, T: QFileDialog_labelText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.labelText_0(self);
    // return 1;
  }
}
pub trait QFileDialog_labelText_0<RetType> {
  fn labelText_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_labelText_0<usize> for (i32) {
  fn labelText_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog9labelTextENS_11DialogLabelE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:178
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSupportedSchemes(const QStringList &)

/*

*/
impl /*struct*/ QFileDialog {
  pub fn setSupportedSchemes_0<RetType, T: QFileDialog_setSupportedSchemes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSupportedSchemes_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setSupportedSchemes_0<RetType> {
  fn setSupportedSchemes_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setSupportedSchemes_0<(/*void*/)> for (usize) {
  fn setSupportedSchemes_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog19setSupportedSchemesERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:179
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList supportedSchemes() const

/*

*/
impl /*struct*/ QFileDialog {
  pub fn supportedSchemes_0<RetType, T: QFileDialog_supportedSchemes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportedSchemes_0(self);
    // return 1;
  }
}
pub trait QFileDialog_supportedSchemes_0<RetType> {
  fn supportedSchemes_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_supportedSchemes_0<usize> for () {
  fn supportedSchemes_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog16supportedSchemesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:182
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setProxyModel(QAbstractProxyModel *)

/*
Sets the model for the views to the given proxyModel. This is useful if you want to modify the underlying model; for example, to add columns, filter data or add drives.

Any existing proxy model will be removed, but not deleted. The file dialog will take ownership of the proxyModel.

This function was introduced in  Qt 4.3.

See also proxyModel().
*/
impl /*struct*/ QFileDialog {
  pub fn setProxyModel_0<RetType, T: QFileDialog_setProxyModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setProxyModel_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setProxyModel_0<RetType> {
  fn setProxyModel_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setProxyModel_0<(/*void*/)> for (usize) {
  fn setProxyModel_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog13setProxyModelEP19QAbstractProxyModel", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:183
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractProxyModel * proxyModel() const

/*
Returns the proxy model used by the file dialog. By default no proxy is set.

See also setProxyModel().
*/
impl /*struct*/ QFileDialog {
  pub fn proxyModel_0<RetType, T: QFileDialog_proxyModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.proxyModel_0(self);
    // return 1;
  }
}
pub trait QFileDialog_proxyModel_0<RetType> {
  fn proxyModel_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_proxyModel_0<usize> for () {
  fn proxyModel_0(self , rsthis: & QFileDialog) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog10proxyModelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:186
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOption(QFileDialog::Option, bool)

/*
Sets the given option to be enabled if on is true; otherwise, clears the given option.

This function was introduced in  Qt 4.5.

See also options and testOption().
*/
impl /*struct*/ QFileDialog {
  pub fn setOption_0<RetType, T: QFileDialog_setOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOption_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setOption_0<RetType> {
  fn setOption_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setOption_0<(/*void*/)> for (i32,bool) {
  fn setOption_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog9setOptionENS_6OptionEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:187
// index:0
// Public Visibility=Default Availability=Available
// [1] bool testOption(QFileDialog::Option) const

/*
Returns true if the given option is enabled; otherwise, returns false.

This function was introduced in  Qt 4.5.

See also options and setOption().
*/
impl /*struct*/ QFileDialog {
  pub fn testOption_0<RetType, T: QFileDialog_testOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.testOption_0(self);
    // return 1;
  }
}
pub trait QFileDialog_testOption_0<RetType> {
  fn testOption_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_testOption_0<bool> for (i32) {
  fn testOption_0(self , rsthis: & QFileDialog) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog10testOptionENS_6OptionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:188
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOptions(QFileDialog::Options)

/*

*/
impl /*struct*/ QFileDialog {
  pub fn setOptions_0<RetType, T: QFileDialog_setOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOptions_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setOptions_0<RetType> {
  fn setOptions_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setOptions_0<(/*void*/)> for (i32) {
  fn setOptions_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog10setOptionsE6QFlagsINS_6OptionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:189
// index:0
// Public Visibility=Default Availability=Available
// [4] QFileDialog::Options options() const

/*

*/
impl /*struct*/ QFileDialog {
  pub fn options_0<RetType, T: QFileDialog_options_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.options_0(self);
    // return 1;
  }
}
pub trait QFileDialog_options_0<RetType> {
  fn options_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_options_0<i32> for () {
  fn options_0(self , rsthis: & QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDialog7optionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:192
// index:0
// Public Visibility=Default Availability=Available
// [-2] void open(QObject *, const char *)

/*
This is an overloaded function.

This function connects one of its signals to the slot specified by receiver and member. The specific signal depends is filesSelected() if fileMode is ExistingFiles and fileSelected() if fileMode is anything else.

The signal will be disconnected from the slot when the dialog is closed.

This function was introduced in  Qt 4.5.
*/
impl /*struct*/ QFileDialog {
  pub fn open_0<RetType, T: QFileDialog_open_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.open_0(self);
    // return 1;
  }
}
pub trait QFileDialog_open_0<RetType> {
  fn open_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_open_0<(/*void*/)> for (usize,usize) {
  fn open_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog4openEP7QObjectPKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:193
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setVisible(bool)

/*
Reimplemented from QWidget::setVisible().
*/
impl /*struct*/ QFileDialog {
  pub fn setVisible_0<RetType, T: QFileDialog_setVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisible_0(self);
    // return 1;
  }
}
pub trait QFileDialog_setVisible_0<RetType> {
  fn setVisible_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_setVisible_0<(/*void*/)> for (bool) {
  fn setVisible_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog10setVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:196
// index:0
// Public Visibility=Default Availability=Available
// [-2] void fileSelected(const QString &)

/*
When the selection changes for local operations and the dialog is accepted, this signal is emitted with the (possibly empty) selected file.

See also currentChanged() and QDialog::Accepted.
*/
impl /*struct*/ QFileDialog {
  pub fn fileSelected_0<RetType, T: QFileDialog_fileSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileSelected_0(self);
    // return 1;
  }
}
pub trait QFileDialog_fileSelected_0<RetType> {
  fn fileSelected_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_fileSelected_0<(/*void*/)> for (usize) {
  fn fileSelected_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog12fileSelectedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:197
// index:0
// Public Visibility=Default Availability=Available
// [-2] void filesSelected(const QStringList &)

/*
When the selection changes for local operations and the dialog is accepted, this signal is emitted with the (possibly empty) list of selected files.

See also currentChanged() and QDialog::Accepted.
*/
impl /*struct*/ QFileDialog {
  pub fn filesSelected_0<RetType, T: QFileDialog_filesSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filesSelected_0(self);
    // return 1;
  }
}
pub trait QFileDialog_filesSelected_0<RetType> {
  fn filesSelected_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_filesSelected_0<(/*void*/)> for (usize) {
  fn filesSelected_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog13filesSelectedERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:198
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentChanged(const QString &)

/*
When the current file changes for local operations, this signal is emitted with the new file name as the path parameter.

See also filesSelected().
*/
impl /*struct*/ QFileDialog {
  pub fn currentChanged_0<RetType, T: QFileDialog_currentChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentChanged_0(self);
    // return 1;
  }
}
pub trait QFileDialog_currentChanged_0<RetType> {
  fn currentChanged_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_currentChanged_0<(/*void*/)> for (usize) {
  fn currentChanged_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog14currentChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:199
// index:0
// Public Visibility=Default Availability=Available
// [-2] void directoryEntered(const QString &)

/*
This signal is emitted for local operations when the user enters a directory.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QFileDialog {
  pub fn directoryEntered_0<RetType, T: QFileDialog_directoryEntered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.directoryEntered_0(self);
    // return 1;
  }
}
pub trait QFileDialog_directoryEntered_0<RetType> {
  fn directoryEntered_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_directoryEntered_0<(/*void*/)> for (usize) {
  fn directoryEntered_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog16directoryEnteredERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:201
// index:0
// Public Visibility=Default Availability=Available
// [-2] void urlSelected(const QUrl &)

/*
When the selection changes and the dialog is accepted, this signal is emitted with the (possibly empty) selected url.

This function was introduced in  Qt 5.2.

See also currentUrlChanged() and QDialog::Accepted.
*/
impl /*struct*/ QFileDialog {
  pub fn urlSelected_0<RetType, T: QFileDialog_urlSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.urlSelected_0(self);
    // return 1;
  }
}
pub trait QFileDialog_urlSelected_0<RetType> {
  fn urlSelected_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_urlSelected_0<(/*void*/)> for (usize) {
  fn urlSelected_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog11urlSelectedERK4QUrl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:203
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentUrlChanged(const QUrl &)

/*
When the current file changes, this signal is emitted with the new file URL as the url parameter.

This function was introduced in  Qt 5.2.

See also urlsSelected().
*/
impl /*struct*/ QFileDialog {
  pub fn currentUrlChanged_0<RetType, T: QFileDialog_currentUrlChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentUrlChanged_0(self);
    // return 1;
  }
}
pub trait QFileDialog_currentUrlChanged_0<RetType> {
  fn currentUrlChanged_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_currentUrlChanged_0<(/*void*/)> for (usize) {
  fn currentUrlChanged_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog17currentUrlChangedERK4QUrl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:204
// index:0
// Public Visibility=Default Availability=Available
// [-2] void directoryUrlEntered(const QUrl &)

/*
This signal is emitted when the user enters a directory.

This function was introduced in  Qt 5.2.
*/
impl /*struct*/ QFileDialog {
  pub fn directoryUrlEntered_0<RetType, T: QFileDialog_directoryUrlEntered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.directoryUrlEntered_0(self);
    // return 1;
  }
}
pub trait QFileDialog_directoryUrlEntered_0<RetType> {
  fn directoryUrlEntered_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_directoryUrlEntered_0<(/*void*/)> for (usize) {
  fn directoryUrlEntered_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog19directoryUrlEnteredERK4QUrl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:206
// index:0
// Public Visibility=Default Availability=Available
// [-2] void filterSelected(const QString &)

/*
This signal is emitted when the user selects a filter.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QFileDialog {
  pub fn filterSelected_0<RetType, T: QFileDialog_filterSelected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filterSelected_0(self);
    // return 1;
  }
}
pub trait QFileDialog_filterSelected_0<RetType> {
  fn filterSelected_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_filterSelected_0<(/*void*/)> for (usize) {
  fn filterSelected_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog14filterSelectedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:210
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString getOpenFileName(QWidget *, const QString &, const QString &, const QString &, QString *, QFileDialog::Options)

/*
This is a convenience static function that returns an existing file selected by the user. If the user presses Cancel, it returns a null string.


  QString fileName = QFileDialog::getOpenFileName(this, tr("Open File"),
                                                  "/home",
                                                  tr("Images (*.png *.xpm *.jpg)"));



The function creates a modal file dialog with the given parent widget. If parent is not 0, the dialog will be shown centered over the parent widget.

The file dialog's working directory will be set to dir. If dir includes a file name, the file will be selected. Only files that match the given filter are shown. The filter selected is set to selectedFilter. The parameters dir, selectedFilter, and filter may be empty strings. If you want multiple filters, separate them with ';;', for example:


  "Images (*.png *.xpm *.jpg);;Text files (*.txt);;XML files (*.xml)"



The options argument holds various options about how to run the dialog, see the QFileDialog::Option enum for more information on the flags you can pass.

The dialog's caption is set to caption. If caption is not specified then a default caption will be used.

On Windows, and macOS, this static function will use the native file dialog and not a QFileDialog.

On Windows the dialog will spin a blocking modal event loop that will not dispatch any QTimers, and if parent is not 0 then it will position the dialog just below the parent's title bar.

On Unix/X11, the normal behavior of the file dialog is to resolve and follow symlinks. For example, if /usr/tmp is a symlink to /var/tmp, the file dialog will change to /var/tmp after entering /usr/tmp. If options includes DontResolveSymlinks, the file dialog will treat symlinks as regular directories.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QFileDialog constructors.

See also getOpenFileNames(), getSaveFileName(), and getExistingDirectory().
*/
impl /*struct*/ QFileDialog {
  pub fn getOpenFileName_0<RetType, T: QFileDialog_getOpenFileName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.getOpenFileName_0();
    // return 1;
  }
}
pub trait QFileDialog_getOpenFileName_0<RetType> {
  fn getOpenFileName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_getOpenFileName_0<usize> for (usize,usize,usize,usize,usize,i32) {
  fn getOpenFileName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDialog15getOpenFileNameEP7QWidgetRK7QStringS4_S4_PS2_6QFlagsINS_6OptionEE", 6,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:217
// index:0
// Public static Visibility=Default Availability=Available
// [8] QUrl getOpenFileUrl(QWidget *, const QString &, const QUrl &, const QString &, QString *, QFileDialog::Options, const QStringList &)

/*
This is a convenience static function that returns an existing file selected by the user. If the user presses Cancel, it returns an empty url.

The function is used similarly to QFileDialog::getOpenFileName(). In particular parent, caption, dir, filter, selectedFilter and options are used in the exact same way.

The main difference with QFileDialog::getOpenFileName() comes from the ability offered to the user to select a remote file. That's why the return type and the type of dir is QUrl.

The supportedSchemes argument allows to restrict the type of URLs the user will be able to select. It is a way for the application to declare the protocols it will support to fetch the file content. An empty list means that no restriction is applied (the default). Supported for local files ("file" scheme) is implicit and always enabled; it is not necessary to include it in the restriction.

When possible, this static function will use the native file dialog and not a QFileDialog. On platforms which don't support selecting remote files, Qt will allow to select only local files.

This function was introduced in  Qt 5.2.

See also getOpenFileName(), getOpenFileUrls(), getSaveFileUrl(), and getExistingDirectoryUrl().
*/
impl /*struct*/ QFileDialog {
  pub fn getOpenFileUrl_0<RetType, T: QFileDialog_getOpenFileUrl_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.getOpenFileUrl_0();
    // return 1;
  }
}
pub trait QFileDialog_getOpenFileUrl_0<RetType> {
  fn getOpenFileUrl_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_getOpenFileUrl_0<usize> for (usize,usize,usize,usize,usize,i32,usize) {
  fn getOpenFileUrl_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDialog14getOpenFileUrlEP7QWidgetRK7QStringRK4QUrlS4_PS2_6QFlagsINS_6OptionEERK11QStringList", 7,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:225
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString getSaveFileName(QWidget *, const QString &, const QString &, const QString &, QString *, QFileDialog::Options)

/*
This is a convenience static function that will return a file name selected by the user. The file does not have to exist.

It creates a modal file dialog with the given parent widget. If parent is not 0, the dialog will be shown centered over the parent widget.


  QString fileName = QFileDialog::getSaveFileName(this, tr("Save File"),
                             "/home/jana/untitled.png",
                             tr("Images (*.png *.xpm *.jpg)"));



The file dialog's working directory will be set to dir. If dir includes a file name, the file will be selected. Only files that match the filter are shown. The filter selected is set to selectedFilter. The parameters dir, selectedFilter, and filter may be empty strings. Multiple filters are separated with ';;'. For instance:


  "Images (*.png *.xpm *.jpg);;Text files (*.txt);;XML files (*.xml)"



The options argument holds various options about how to run the dialog, see the QFileDialog::Option enum for more information on the flags you can pass.

The default filter can be chosen by setting selectedFilter to the desired value.

The dialog's caption is set to caption. If caption is not specified, a default caption will be used.

On Windows, and macOS, this static function will use the native file dialog and not a QFileDialog.

On Windows the dialog will spin a blocking modal event loop that will not dispatch any QTimers, and if parent is not 0 then it will position the dialog just below the parent's title bar. On macOS, with its native file dialog, the filter argument is ignored.

On Unix/X11, the normal behavior of the file dialog is to resolve and follow symlinks. For example, if /usr/tmp is a symlink to /var/tmp, the file dialog will change to /var/tmp after entering /usr/tmp. If options includes DontResolveSymlinks the file dialog will treat symlinks as regular directories.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QFileDialog constructors.

See also getOpenFileName(), getOpenFileNames(), and getExistingDirectory().
*/
impl /*struct*/ QFileDialog {
  pub fn getSaveFileName_0<RetType, T: QFileDialog_getSaveFileName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.getSaveFileName_0();
    // return 1;
  }
}
pub trait QFileDialog_getSaveFileName_0<RetType> {
  fn getSaveFileName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_getSaveFileName_0<usize> for (usize,usize,usize,usize,usize,i32) {
  fn getSaveFileName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDialog15getSaveFileNameEP7QWidgetRK7QStringS4_S4_PS2_6QFlagsINS_6OptionEE", 6,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:232
// index:0
// Public static Visibility=Default Availability=Available
// [8] QUrl getSaveFileUrl(QWidget *, const QString &, const QUrl &, const QString &, QString *, QFileDialog::Options, const QStringList &)

/*
This is a convenience static function that returns a file selected by the user. The file does not have to exist. If the user presses Cancel, it returns an empty url.

The function is used similarly to QFileDialog::getSaveFileName(). In particular parent, caption, dir, filter, selectedFilter and options are used in the exact same way.

The main difference with QFileDialog::getSaveFileName() comes from the ability offered to the user to select a remote file. That's why the return type and the type of dir is QUrl.

The supportedSchemes argument allows to restrict the type of URLs the user will be able to select. It is a way for the application to declare the protocols it will support to save the file content. An empty list means that no restriction is applied (the default). Supported for local files ("file" scheme) is implicit and always enabled; it is not necessary to include it in the restriction.

When possible, this static function will use the native file dialog and not a QFileDialog. On platforms which don't support selecting remote files, Qt will allow to select only local files.

This function was introduced in  Qt 5.2.

See also getSaveFileName(), getOpenFileUrl(), getOpenFileUrls(), and getExistingDirectoryUrl().
*/
impl /*struct*/ QFileDialog {
  pub fn getSaveFileUrl_0<RetType, T: QFileDialog_getSaveFileUrl_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.getSaveFileUrl_0();
    // return 1;
  }
}
pub trait QFileDialog_getSaveFileUrl_0<RetType> {
  fn getSaveFileUrl_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_getSaveFileUrl_0<usize> for (usize,usize,usize,usize,usize,i32,usize) {
  fn getSaveFileUrl_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDialog14getSaveFileUrlEP7QWidgetRK7QStringRK4QUrlS4_PS2_6QFlagsINS_6OptionEERK11QStringList", 7,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:240
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString getExistingDirectory(QWidget *, const QString &, const QString &, QFileDialog::Options)

/*
This is a convenience static function that will return an existing directory selected by the user.


  QString dir = QFileDialog::getExistingDirectory(this, tr("Open Directory"),
                                                  "/home",
                                                  QFileDialog::ShowDirsOnly
                                                  | QFileDialog::DontResolveSymlinks);



This function creates a modal file dialog with the given parent widget. If parent is not 0, the dialog will be shown centered over the parent widget.

The dialog's working directory is set to dir, and the caption is set to caption. Either of these may be an empty string in which case the current directory and a default caption will be used respectively.

The options argument holds various options about how to run the dialog, see the QFileDialog::Option enum for more information on the flags you can pass. To ensure a native file dialog, ShowDirsOnly must be set.

On Windows and macOS, this static function will use the native file dialog and not a QFileDialog. However, the native Windows file dialog does not support displaying files in the directory chooser. You need to pass DontUseNativeDialog to display files using a QFileDialog.

On Unix/X11, the normal behavior of the file dialog is to resolve and follow symlinks. For example, if /usr/tmp is a symlink to /var/tmp, the file dialog will change to /var/tmp after entering /usr/tmp. If options includes DontResolveSymlinks, the file dialog will treat symlinks as regular directories.

On Windows, the dialog will spin a blocking modal event loop that will not dispatch any QTimers, and if parent is not 0 then it will position the dialog just below the parent's title bar.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QFileDialog constructors.

See also getOpenFileName(), getOpenFileNames(), and getSaveFileName().
*/
impl /*struct*/ QFileDialog {
  pub fn getExistingDirectory_0<RetType, T: QFileDialog_getExistingDirectory_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.getExistingDirectory_0();
    // return 1;
  }
}
pub trait QFileDialog_getExistingDirectory_0<RetType> {
  fn getExistingDirectory_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_getExistingDirectory_0<usize> for (usize,usize,usize,i32) {
  fn getExistingDirectory_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDialog20getExistingDirectoryEP7QWidgetRK7QStringS4_6QFlagsINS_6OptionEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:245
// index:0
// Public static Visibility=Default Availability=Available
// [8] QUrl getExistingDirectoryUrl(QWidget *, const QString &, const QUrl &, QFileDialog::Options, const QStringList &)

/*
This is a convenience static function that will return an existing directory selected by the user. If the user presses Cancel, it returns an empty url.

The function is used similarly to QFileDialog::getExistingDirectory(). In particular parent, caption, dir and options are used in the exact same way.

The main difference with QFileDialog::getExistingDirectory() comes from the ability offered to the user to select a remote directory. That's why the return type and the type of dir is QUrl.

The supportedSchemes argument allows to restrict the type of URLs the user will be able to select. It is a way for the application to declare the protocols it will support to fetch the file content. An empty list means that no restriction is applied (the default). Supported for local files ("file" scheme) is implicit and always enabled; it is not necessary to include it in the restriction.

When possible, this static function will use the native file dialog and not a QFileDialog. On platforms which don't support selecting remote files, Qt will allow to select only local files.

This function was introduced in  Qt 5.2.

See also getExistingDirectory(), getOpenFileUrl(), getOpenFileUrls(), and getSaveFileUrl().
*/
impl /*struct*/ QFileDialog {
  pub fn getExistingDirectoryUrl_0<RetType, T: QFileDialog_getExistingDirectoryUrl_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.getExistingDirectoryUrl_0();
    // return 1;
  }
}
pub trait QFileDialog_getExistingDirectoryUrl_0<RetType> {
  fn getExistingDirectoryUrl_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_getExistingDirectoryUrl_0<usize> for (usize,usize,usize,i32,usize) {
  fn getExistingDirectoryUrl_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDialog23getExistingDirectoryUrlEP7QWidgetRK7QStringRK4QUrl6QFlagsINS_6OptionEERK11QStringList", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:251
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStringList getOpenFileNames(QWidget *, const QString &, const QString &, const QString &, QString *, QFileDialog::Options)

/*
This is a convenience static function that will return one or more existing files selected by the user.


  QStringList files = QFileDialog::getOpenFileNames(
                          this,
                          "Select one or more files to open",
                          "/home",
                          "Images (*.png *.xpm *.jpg)");



This function creates a modal file dialog with the given parent widget. If parent is not 0, the dialog will be shown centered over the parent widget.

The file dialog's working directory will be set to dir. If dir includes a file name, the file will be selected. The filter is set to filter so that only those files which match the filter are shown. The filter selected is set to selectedFilter. The parameters dir, selectedFilter and filter may be empty strings. If you need multiple filters, separate them with ';;', for instance:


  "Images (*.png *.xpm *.jpg);;Text files (*.txt);;XML files (*.xml)"



The dialog's caption is set to caption. If caption is not specified then a default caption will be used.

On Windows, and macOS, this static function will use the native file dialog and not a QFileDialog.

On Windows the dialog will spin a blocking modal event loop that will not dispatch any QTimers, and if parent is not 0 then it will position the dialog just below the parent's title bar.

On Unix/X11, the normal behavior of the file dialog is to resolve and follow symlinks. For example, if /usr/tmp is a symlink to /var/tmp, the file dialog will change to /var/tmp after entering /usr/tmp. The options argument holds various options about how to run the dialog, see the QFileDialog::Option enum for more information on the flags you can pass.

Warning: Do not delete parent during the execution of the dialog. If you want to do this, you should create the dialog yourself using one of the QFileDialog constructors.

See also getOpenFileName(), getSaveFileName(), and getExistingDirectory().
*/
impl /*struct*/ QFileDialog {
  pub fn getOpenFileNames_0<RetType, T: QFileDialog_getOpenFileNames_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.getOpenFileNames_0();
    // return 1;
  }
}
pub trait QFileDialog_getOpenFileNames_0<RetType> {
  fn getOpenFileNames_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_getOpenFileNames_0<usize> for (usize,usize,usize,usize,usize,i32) {
  fn getOpenFileNames_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDialog16getOpenFileNamesEP7QWidgetRK7QStringS4_S4_PS2_6QFlagsINS_6OptionEE", 6,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:258
// index:0
// Public static Visibility=Default Availability=Available
// [-2] QList<QUrl> getOpenFileUrls(QWidget *, const QString &, const QUrl &, const QString &, QString *, QFileDialog::Options, const QStringList &)

/*
This is a convenience static function that will return one or more existing files selected by the user. If the user presses Cancel, it returns an empty list.

The function is used similarly to QFileDialog::getOpenFileNames(). In particular parent, caption, dir, filter, selectedFilter and options are used in the exact same way.

The main difference with QFileDialog::getOpenFileNames() comes from the ability offered to the user to select remote files. That's why the return type and the type of dir are respectively QList<QUrl> and QUrl.

The supportedSchemes argument allows to restrict the type of URLs the user will be able to select. It is a way for the application to declare the protocols it will support to fetch the file content. An empty list means that no restriction is applied (the default). Supported for local files ("file" scheme) is implicit and always enabled; it is not necessary to include it in the restriction.

When possible, this static function will use the native file dialog and not a QFileDialog. On platforms which don't support selecting remote files, Qt will allow to select only local files.

This function was introduced in  Qt 5.2.

See also getOpenFileNames(), getOpenFileUrl(), getSaveFileUrl(), and getExistingDirectoryUrl().
*/
impl /*struct*/ QFileDialog {
  pub fn getOpenFileUrls_0<RetType, T: QFileDialog_getOpenFileUrls_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.getOpenFileUrls_0();
    // return 1;
  }
}
pub trait QFileDialog_getOpenFileUrls_0<RetType> {
  fn getOpenFileUrls_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_getOpenFileUrls_0<usize> for (usize,usize,usize,usize,usize,i32,usize) {
  fn getOpenFileUrls_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDialog15getOpenFileUrlsEP7QWidgetRK7QStringRK4QUrlS4_PS2_6QFlagsINS_6OptionEERK11QStringList", 7,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:269
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void done(int)

/*
Reimplemented from QDialog::done().
*/
impl /*struct*/ QFileDialog {
  pub fn done_0<RetType, T: QFileDialog_done_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.done_0(self);
    // return 1;
  }
}
pub trait QFileDialog_done_0<RetType> {
  fn done_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_done_0<(/*void*/)> for (i32) {
  fn done_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog4doneEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:270
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void accept()

/*
Reimplemented from QDialog::accept().
*/
impl /*struct*/ QFileDialog {
  pub fn accept_0<RetType, T: QFileDialog_accept_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.accept_0(self);
    // return 1;
  }
}
pub trait QFileDialog_accept_0<RetType> {
  fn accept_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_accept_0<(/*void*/)> for () {
  fn accept_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QFileDialog6acceptEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qfiledialog.h:271
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QFileDialog {
  pub fn changeEvent_0<RetType, T: QFileDialog_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QFileDialog_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QFileDialog) -> RetType;
}
impl<'a> /*trait*/ QFileDialog_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QFileDialog) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFileDialog11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum describes the view mode of the file dialog; i.e. what information about each file will be displayed.



See also setViewMode().

*/
pub type QFileDialog__ViewMode = i32;
// Displays an icon, a name, and details for each item in the directory.
pub const QFileDialog__Detail :QFileDialog__ViewMode = 0;
// Displays only an icon and a name for each item in the directory.
pub const QFileDialog__List :QFileDialog__ViewMode = 1;
pub fn QFileDialog_ViewModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFileDialog", val);
}
pub fn QFileDialog_ViewModeItemName_s(val: i32) ->String {
  //var nilthis *QFileDialog
  //return nilthis.ViewModeItemName(val);
  return QFileDialog_ViewModeItemName(val);
}


/*
This enum is used to indicate what the user may select in the file dialog; i.e. what the dialog will return if the user clicks OK.



This value is obsolete since Qt 4.5:



See also setFileMode().

*/
pub type QFileDialog__FileMode = i32;
// The name of a file, whether it exists or not.
pub const QFileDialog__AnyFile :QFileDialog__FileMode = 0;
// The name of a single existing file.
pub const QFileDialog__ExistingFile :QFileDialog__FileMode = 1;
// The name of a directory. Both files and directories are displayed. However, the native Windows file dialog does not support displaying files in the directory chooser.
pub const QFileDialog__Directory :QFileDialog__FileMode = 2;
// The names of zero or more existing files.
pub const QFileDialog__ExistingFiles :QFileDialog__FileMode = 3;
// Use Directory and setOption(ShowDirsOnly, true) instead.
pub const QFileDialog__DirectoryOnly :QFileDialog__FileMode = 4;
pub fn QFileDialog_FileModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFileDialog", val);
}
pub fn QFileDialog_FileModeItemName_s(val: i32) ->String {
  //var nilthis *QFileDialog
  //return nilthis.FileModeItemName(val);
  return QFileDialog_FileModeItemName(val);
}


/*
ConstantValue
QFileDialog::AcceptOpen0
QFileDialog::AcceptSave1

*/
pub type QFileDialog__AcceptMode = i32;
// 
pub const QFileDialog__AcceptOpen :QFileDialog__AcceptMode = 0;
// 
pub const QFileDialog__AcceptSave :QFileDialog__AcceptMode = 1;
pub fn QFileDialog_AcceptModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFileDialog", val);
}
pub fn QFileDialog_AcceptModeItemName_s(val: i32) ->String {
  //var nilthis *QFileDialog
  //return nilthis.AcceptModeItemName(val);
  return QFileDialog_AcceptModeItemName(val);
}


/*
ConstantValue
QFileDialog::LookIn0
QFileDialog::FileName1
QFileDialog::FileType2
QFileDialog::Accept3
QFileDialog::Reject4

*/
pub type QFileDialog__DialogLabel = i32;
// 
pub const QFileDialog__LookIn :QFileDialog__DialogLabel = 0;
// 
pub const QFileDialog__FileName :QFileDialog__DialogLabel = 1;
// 
pub const QFileDialog__FileType :QFileDialog__DialogLabel = 2;
// 
pub const QFileDialog__Accept :QFileDialog__DialogLabel = 3;
// 
pub const QFileDialog__Reject :QFileDialog__DialogLabel = 4;
pub fn QFileDialog_DialogLabelItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFileDialog", val);
}
pub fn QFileDialog_DialogLabelItemName_s(val: i32) ->String {
  //var nilthis *QFileDialog
  //return nilthis.DialogLabelItemName(val);
  return QFileDialog_DialogLabelItemName(val);
}


/*


*/
pub type QFileDialog__Option = i32;
// 
pub const QFileDialog__ShowDirsOnly :QFileDialog__Option = 1;
// 
pub const QFileDialog__DontResolveSymlinks :QFileDialog__Option = 2;
// 
pub const QFileDialog__DontConfirmOverwrite :QFileDialog__Option = 4;
// 
pub const QFileDialog__DontUseSheet :QFileDialog__Option = 8;
// 
pub const QFileDialog__DontUseNativeDialog :QFileDialog__Option = 16;
// 
pub const QFileDialog__ReadOnly :QFileDialog__Option = 32;
// 
pub const QFileDialog__HideNameFilterDetails :QFileDialog__Option = 64;
// 
pub const QFileDialog__DontUseCustomDirectoryIcons :QFileDialog__Option = 128;
pub fn QFileDialog_OptionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFileDialog", val);
}
pub fn QFileDialog_OptionItemName_s(val: i32) ->String {
  //var nilthis *QFileDialog
  //return nilthis.OptionItemName(val);
  return QFileDialog_OptionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
