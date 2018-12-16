

// mod ::widgets::QWizardPage
// package qtwidgets
// /usr/include/qt/QtWidgets/qwizard.h
// #include <qwizard.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 51
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

// void setField(const QString &, const QVariant &)
// func (this *QWizardPage) InheritSetField(f func(name string, value *qtcore.QVariant) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setField", f)
// }

// QVariant field(const QString &)
// func (this *QWizardPage) InheritField(f func(name string) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "field", f)
// }

// void registerField(const QString &, QWidget *, const char *, const char *)
// func (this *QWizardPage) InheritRegisterField(f func(name string, widget *QWidget/*777 QWidget **/, property string, changedSignal string) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "registerField", f)
// }

// QWizard * wizard()
// func (this *QWizardPage) InheritWizard(f func() unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "wizard", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QWizardPage)=48
pub struct QWizardPage {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QWizardPage_ITF interface {
//    QWidget_ITF
//    QWizardPage_PTR() *QWizardPage
//}
//func (ptr *QWizardPage) QWizardPage_PTR() *QWizardPage { return ptr }

impl /*struct*/ QWizardPage {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QWizardPage {
    return QWizardPage{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QWizardPage {
//  type Target = QWizardPageBASE;
//
//  fn deref(&self) -> &QWizardPageBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QWizardPageBASE> for QWizardPage {
//  fn as_ref(& self) -> & QWizardPageBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qwizard.h:213
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QWizardPage {
  pub fn metaObject_0<RetType, T: QWizardPage_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QWizardPage_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QWizardPage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWizardPage10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:218
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QWizardPage(QWidget *)

/*

*/
// QWizardPage(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QWizardPage {
  pub fn QWizardPage_0<T: QWizardPage_QWizardPage_0>(value: T) -> QWizardPage {
    let rsthis = value.QWizardPage_0();
    return rsthis;
    // return 1;
  }
}

pub trait QWizardPage_QWizardPage_0 {
  fn QWizardPage_0(self) -> QWizardPage;
}
// QWizardPage(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QWizardPage_QWizardPage_0 for (usize) {
  fn QWizardPage_0(self) -> QWizardPage {
    // unsafe{_ZN11QWizardPageC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QWizardPageC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QWizardPage{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:219
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QWizardPage()

/*

*/
pub fn DeleteQWizardPage(this :*mut QWizardPage) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QWizardPageD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qwizard.h:221
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTitle(const QString &)

/*

*/
impl /*struct*/ QWizardPage {
  pub fn setTitle_0<RetType, T: QWizardPage_setTitle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTitle_0(self);
    // return 1;
  }
}
pub trait QWizardPage_setTitle_0<RetType> {
  fn setTitle_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_setTitle_0<(/*void*/)> for (usize) {
  fn setTitle_0(self , rsthis: & QWizardPage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QWizardPage8setTitleERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:222
// index:0
// Public Visibility=Default Availability=Available
// [8] QString title() const

/*

*/
impl /*struct*/ QWizardPage {
  pub fn title_0<RetType, T: QWizardPage_title_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.title_0(self);
    // return 1;
  }
}
pub trait QWizardPage_title_0<RetType> {
  fn title_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_title_0<usize> for () {
  fn title_0(self , rsthis: & QWizardPage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWizardPage5titleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:223
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSubTitle(const QString &)

/*

*/
impl /*struct*/ QWizardPage {
  pub fn setSubTitle_0<RetType, T: QWizardPage_setSubTitle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSubTitle_0(self);
    // return 1;
  }
}
pub trait QWizardPage_setSubTitle_0<RetType> {
  fn setSubTitle_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_setSubTitle_0<(/*void*/)> for (usize) {
  fn setSubTitle_0(self , rsthis: & QWizardPage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QWizardPage11setSubTitleERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:224
// index:0
// Public Visibility=Default Availability=Available
// [8] QString subTitle() const

/*

*/
impl /*struct*/ QWizardPage {
  pub fn subTitle_0<RetType, T: QWizardPage_subTitle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subTitle_0(self);
    // return 1;
  }
}
pub trait QWizardPage_subTitle_0<RetType> {
  fn subTitle_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_subTitle_0<usize> for () {
  fn subTitle_0(self , rsthis: & QWizardPage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWizardPage8subTitleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:225
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPixmap(QWizard::WizardPixmap, const QPixmap &)

/*
Sets the pixmap for role which to pixmap.

The pixmaps are used by QWizard when displaying a page. Which pixmaps are actually used depend on the wizard style.

Pixmaps can also be set for a specific page using QWizardPage::setPixmap().

See also pixmap(), QWizardPage::setPixmap(), and Elements of a Wizard Page.
*/
impl /*struct*/ QWizardPage {
  pub fn setPixmap_0<RetType, T: QWizardPage_setPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPixmap_0(self);
    // return 1;
  }
}
pub trait QWizardPage_setPixmap_0<RetType> {
  fn setPixmap_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_setPixmap_0<(/*void*/)> for (i32,usize) {
  fn setPixmap_0(self , rsthis: & QWizardPage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QWizardPage9setPixmapEN7QWizard12WizardPixmapERK7QPixmap", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:226
// index:0
// Public Visibility=Default Availability=Available
// [32] QPixmap pixmap(QWizard::WizardPixmap) const

/*
Returns the pixmap set for role which.

By default, the only pixmap that is set is the BackgroundPixmap on macOS.

See also setPixmap(), QWizardPage::pixmap(), and Elements of a Wizard Page.
*/
impl /*struct*/ QWizardPage {
  pub fn pixmap_0<RetType, T: QWizardPage_pixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixmap_0(self);
    // return 1;
  }
}
pub trait QWizardPage_pixmap_0<RetType> {
  fn pixmap_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_pixmap_0<usize> for (i32) {
  fn pixmap_0(self , rsthis: & QWizardPage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWizardPage6pixmapEN7QWizard12WizardPixmapE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:227
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFinalPage(bool)

/*

*/
impl /*struct*/ QWizardPage {
  pub fn setFinalPage_0<RetType, T: QWizardPage_setFinalPage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFinalPage_0(self);
    // return 1;
  }
}
pub trait QWizardPage_setFinalPage_0<RetType> {
  fn setFinalPage_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_setFinalPage_0<(/*void*/)> for (bool) {
  fn setFinalPage_0(self , rsthis: & QWizardPage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QWizardPage12setFinalPageEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:228
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isFinalPage() const

/*

*/
impl /*struct*/ QWizardPage {
  pub fn isFinalPage_0<RetType, T: QWizardPage_isFinalPage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFinalPage_0(self);
    // return 1;
  }
}
pub trait QWizardPage_isFinalPage_0<RetType> {
  fn isFinalPage_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_isFinalPage_0<bool> for () {
  fn isFinalPage_0(self , rsthis: & QWizardPage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWizardPage11isFinalPageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:229
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCommitPage(bool)

/*

*/
impl /*struct*/ QWizardPage {
  pub fn setCommitPage_0<RetType, T: QWizardPage_setCommitPage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCommitPage_0(self);
    // return 1;
  }
}
pub trait QWizardPage_setCommitPage_0<RetType> {
  fn setCommitPage_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_setCommitPage_0<(/*void*/)> for (bool) {
  fn setCommitPage_0(self , rsthis: & QWizardPage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QWizardPage13setCommitPageEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:230
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isCommitPage() const

/*

*/
impl /*struct*/ QWizardPage {
  pub fn isCommitPage_0<RetType, T: QWizardPage_isCommitPage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCommitPage_0(self);
    // return 1;
  }
}
pub trait QWizardPage_isCommitPage_0<RetType> {
  fn isCommitPage_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_isCommitPage_0<bool> for () {
  fn isCommitPage_0(self , rsthis: & QWizardPage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWizardPage12isCommitPageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:231
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setButtonText(QWizard::WizardButton, const QString &)

/*
Sets the text on button which to be text.

By default, the text on buttons depends on the wizardStyle. For example, on macOS, the Next button is called Continue.

To add extra buttons to the wizard (e.g., a Print button), one way is to call setButtonText() with CustomButton1, CustomButton2, or CustomButton3 to set their text, and make the buttons visible using the HaveCustomButton1, HaveCustomButton2, and/or HaveCustomButton3 options.

Button texts may also be set on a per-page basis using QWizardPage::setButtonText().

See also buttonText(), setButton(), button(), setButtonLayout(), setOptions(), and QWizardPage::setButtonText().
*/
impl /*struct*/ QWizardPage {
  pub fn setButtonText_0<RetType, T: QWizardPage_setButtonText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setButtonText_0(self);
    // return 1;
  }
}
pub trait QWizardPage_setButtonText_0<RetType> {
  fn setButtonText_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_setButtonText_0<(/*void*/)> for (i32,usize) {
  fn setButtonText_0(self , rsthis: & QWizardPage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QWizardPage13setButtonTextEN7QWizard12WizardButtonERK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:232
// index:0
// Public Visibility=Default Availability=Available
// [8] QString buttonText(QWizard::WizardButton) const

/*
Returns the text on button which.

If a text has ben set using setButtonText(), this text is returned.

By default, the text on buttons depends on the wizardStyle. For example, on macOS, the Next button is called Continue.

See also button(), setButton(), setButtonText(), QWizardPage::buttonText(), and QWizardPage::setButtonText().
*/
impl /*struct*/ QWizardPage {
  pub fn buttonText_0<RetType, T: QWizardPage_buttonText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonText_0(self);
    // return 1;
  }
}
pub trait QWizardPage_buttonText_0<RetType> {
  fn buttonText_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_buttonText_0<usize> for (i32) {
  fn buttonText_0(self , rsthis: & QWizardPage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWizardPage10buttonTextEN7QWizard12WizardButtonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:234
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void initializePage()

/*
This virtual function is called by QWizard to prepare page id just before it is shown either as a result of QWizard::restart() being called, or as a result of the user clicking Next. (However, if the QWizard::IndependentPages option is set, this function is only called the first time the page is shown.)

By reimplementing this function, you can ensure that the page's fields are properly initialized based on fields from previous pages.

The default implementation calls QWizardPage::initializePage() on page(id).

See also QWizardPage::initializePage() and cleanupPage().
*/
impl /*struct*/ QWizardPage {
  pub fn initializePage_0<RetType, T: QWizardPage_initializePage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initializePage_0(self);
    // return 1;
  }
}
pub trait QWizardPage_initializePage_0<RetType> {
  fn initializePage_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_initializePage_0<(/*void*/)> for () {
  fn initializePage_0(self , rsthis: & QWizardPage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QWizardPage14initializePageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:235
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void cleanupPage()

/*
This virtual function is called by QWizard to clean up page id just before the user leaves it by clicking Back (unless the QWizard::IndependentPages option is set).

The default implementation calls QWizardPage::cleanupPage() on page(id).

See also QWizardPage::cleanupPage() and initializePage().
*/
impl /*struct*/ QWizardPage {
  pub fn cleanupPage_0<RetType, T: QWizardPage_cleanupPage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cleanupPage_0(self);
    // return 1;
  }
}
pub trait QWizardPage_cleanupPage_0<RetType> {
  fn cleanupPage_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_cleanupPage_0<(/*void*/)> for () {
  fn cleanupPage_0(self , rsthis: & QWizardPage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QWizardPage11cleanupPageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:236
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool validatePage()

/*

*/
impl /*struct*/ QWizardPage {
  pub fn validatePage_0<RetType, T: QWizardPage_validatePage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.validatePage_0(self);
    // return 1;
  }
}
pub trait QWizardPage_validatePage_0<RetType> {
  fn validatePage_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_validatePage_0<bool> for () {
  fn validatePage_0(self , rsthis: & QWizardPage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QWizardPage12validatePageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:237
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool isComplete() const

/*

*/
impl /*struct*/ QWizardPage {
  pub fn isComplete_0<RetType, T: QWizardPage_isComplete_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isComplete_0(self);
    // return 1;
  }
}
pub trait QWizardPage_isComplete_0<RetType> {
  fn isComplete_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_isComplete_0<bool> for () {
  fn isComplete_0(self , rsthis: & QWizardPage) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWizardPage10isCompleteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:238
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int nextId() const

/*
This virtual function is called by QWizard to find out which page to show when the user clicks the Next button.

The return value is the ID of the next page, or -1 if no page follows.

The default implementation calls QWizardPage::nextId() on the currentPage().

By reimplementing this function, you can specify a dynamic page order.

See also QWizardPage::nextId() and currentPage().
*/
impl /*struct*/ QWizardPage {
  pub fn nextId_0<RetType, T: QWizardPage_nextId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nextId_0(self);
    // return 1;
  }
}
pub trait QWizardPage_nextId_0<RetType> {
  fn nextId_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_nextId_0<i32> for () {
  fn nextId_0(self , rsthis: & QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWizardPage6nextIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:241
// index:0
// Public Visibility=Default Availability=Available
// [-2] void completeChanged()

/*

*/
impl /*struct*/ QWizardPage {
  pub fn completeChanged_0<RetType, T: QWizardPage_completeChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.completeChanged_0(self);
    // return 1;
  }
}
pub trait QWizardPage_completeChanged_0<RetType> {
  fn completeChanged_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_completeChanged_0<(/*void*/)> for () {
  fn completeChanged_0(self , rsthis: & QWizardPage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QWizardPage15completeChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:244
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setField(const QString &, const QVariant &)

/*
Sets the value of the field called name to value.

This function can be used to set fields on any page of the wizard.

See also QWizardPage::registerField(), QWizardPage::setField(), and field().
*/
impl /*struct*/ QWizardPage {
  pub fn setField_0<RetType, T: QWizardPage_setField_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setField_0(self);
    // return 1;
  }
}
pub trait QWizardPage_setField_0<RetType> {
  fn setField_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_setField_0<(/*void*/)> for (usize,usize) {
  fn setField_0(self , rsthis: & QWizardPage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QWizardPage8setFieldERK7QStringRK8QVariant", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:245
// index:0
// Protected Visibility=Default Availability=Available
// [16] QVariant field(const QString &) const

/*
Returns the value of the field called name.

This function can be used to access fields on any page of the wizard.

See also QWizardPage::registerField(), QWizardPage::field(), and setField().
*/
impl /*struct*/ QWizardPage {
  pub fn field_0<RetType, T: QWizardPage_field_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.field_0(self);
    // return 1;
  }
}
pub trait QWizardPage_field_0<RetType> {
  fn field_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_field_0<usize> for (usize) {
  fn field_0(self , rsthis: & QWizardPage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWizardPage5fieldERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:246
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void registerField(const QString &, QWidget *, const char *, const char *)

/*

*/
impl /*struct*/ QWizardPage {
  pub fn registerField_0<RetType, T: QWizardPage_registerField_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.registerField_0(self);
    // return 1;
  }
}
pub trait QWizardPage_registerField_0<RetType> {
  fn registerField_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_registerField_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn registerField_0(self , rsthis: & QWizardPage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (self.2) as *const usize as usize;
    let arg3 = (self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QWizardPage13registerFieldERK7QStringP7QWidgetPKcS6_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qwizard.h:248
// index:0
// Protected Visibility=Default Availability=Available
// [8] QWizard * wizard() const

/*

*/
impl /*struct*/ QWizardPage {
  pub fn wizard_0<RetType, T: QWizardPage_wizard_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wizard_0(self);
    // return 1;
  }
}
pub trait QWizardPage_wizard_0<RetType> {
  fn wizard_0(self , rsthis: & QWizardPage) -> RetType;
}
impl<'a> /*trait*/ QWizardPage_wizard_0<usize> for () {
  fn wizard_0(self , rsthis: & QWizardPage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWizardPage6wizardEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
